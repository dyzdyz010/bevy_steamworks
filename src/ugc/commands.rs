use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
    system::SystemParam,
};

use crate::{SteamworksClient, SteamworksEvent, SteamworksServer};

mod item_commands;
mod query_commands;
mod read_commands;
mod server_commands;
mod subscription_commands;

use super::{
    async_results::SteamworksUgcAsyncResults, callbacks::process_ugc_steam_events,
    update_watches::SteamworksUgcUpdateWatches, validation::validate_command, SteamworksUgcCommand,
    SteamworksUgcError, SteamworksUgcOperation, SteamworksUgcResult, SteamworksUgcState,
};

#[derive(SystemParam)]
pub(super) struct SteamworksUgcIo<'w, 's> {
    client: Option<Res<'w, SteamworksClient>>,
    server: Option<Res<'w, SteamworksServer>>,
    async_results: Res<'w, SteamworksUgcAsyncResults>,
    update_watches: Res<'w, SteamworksUgcUpdateWatches>,
    commands: ResMut<'w, Messages<SteamworksUgcCommand>>,
    steam_events: MessageReader<'w, 's, SteamworksEvent>,
}

pub(super) fn process_ugc_commands(
    mut state: ResMut<SteamworksUgcState>,
    mut io: SteamworksUgcIo,
    mut results: MessageWriter<SteamworksUgcResult>,
) {
    for result in io.async_results.drain() {
        record_ugc_result(&mut state, &result);
        state.sync_active_item_updates(&io.update_watches);
        results.write(result);
    }

    process_ugc_steam_events(&mut state, &mut io.steam_events, &mut results);

    for command in io.commands.drain() {
        if let Err(error) = validate_command(&command) {
            state.record_error(error.clone());
            state.sync_active_item_updates(&io.update_watches);
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks UGC command failed"
            );
            results.write(SteamworksUgcResult::Err { command, error });
            continue;
        }

        let request_id = async_command_request_id(&command, &mut state);
        match handle_ugc_command(
            io.client.as_deref(),
            io.server.as_deref(),
            &io.async_results,
            &io.update_watches,
            command.clone(),
            request_id,
        ) {
            Ok(operation) => {
                state.record_operation(&operation);
                state.sync_active_item_updates(&io.update_watches);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks UGC command"
                );
                results.write(SteamworksUgcResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                state.sync_active_item_updates(&io.update_watches);
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks UGC command failed"
                );
                results.write(SteamworksUgcResult::Err { command, error });
            }
        }
    }
}

fn record_ugc_result(state: &mut SteamworksUgcState, result: &SteamworksUgcResult) {
    match result {
        SteamworksUgcResult::Ok(operation) => state.record_operation(operation),
        SteamworksUgcResult::Err { error, .. } => {
            if error.async_request_id().is_some() {
                state.record_failed_async_operation();
            }
            state.record_error(error.clone());
        }
    }
}

fn async_command_request_id(
    command: &SteamworksUgcCommand,
    state: &mut SteamworksUgcState,
) -> Option<u64> {
    matches!(
        command,
        SteamworksUgcCommand::Query { .. }
            | SteamworksUgcCommand::QueryTotal { .. }
            | SteamworksUgcCommand::QueryIds { .. }
            | SteamworksUgcCommand::CreateItem { .. }
            | SteamworksUgcCommand::SubmitItemUpdate { .. }
            | SteamworksUgcCommand::SubscribeItem { .. }
            | SteamworksUgcCommand::UnsubscribeItem { .. }
            | SteamworksUgcCommand::DeleteItem { .. }
            | SteamworksUgcCommand::StartPlaytimeTracking { .. }
            | SteamworksUgcCommand::StopPlaytimeTracking { .. }
            | SteamworksUgcCommand::StopPlaytimeTrackingForAllItems
    )
    .then(|| state.next_request_id())
}

fn handle_ugc_command(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
    async_results: &SteamworksUgcAsyncResults,
    update_watches: &SteamworksUgcUpdateWatches,
    command: SteamworksUgcCommand,
    request_id: Option<u64>,
) -> Result<SteamworksUgcOperation, SteamworksUgcError> {
    match command {
        SteamworksUgcCommand::InitWorkshopForGameServer {
            workshop_depot,
            folder,
        } => server_commands::init_workshop_for_game_server(server, workshop_depot, folder),
        command => handle_client_ugc_command(
            client.ok_or(SteamworksUgcError::ClientUnavailable)?,
            async_results,
            update_watches,
            command,
            request_id,
        ),
    }
}

fn handle_client_ugc_command(
    client: &SteamworksClient,
    async_results: &SteamworksUgcAsyncResults,
    update_watches: &SteamworksUgcUpdateWatches,
    command: SteamworksUgcCommand,
    request_id: Option<u64>,
) -> Result<SteamworksUgcOperation, SteamworksUgcError> {
    let ugc = client.ugc();
    match command {
        SteamworksUgcCommand::SuspendDownloads { suspend } => {
            ugc.suspend_downloads(suspend);
            Ok(read_commands::suspend_downloads(suspend))
        }
        SteamworksUgcCommand::ListSubscribedItems {
            include_locally_disabled,
        } => Ok(read_commands::list_subscribed_items(
            &ugc,
            include_locally_disabled,
        )),
        SteamworksUgcCommand::GetItemState { item } => {
            Ok(read_commands::read_item_state(&ugc, item))
        }
        SteamworksUgcCommand::GetItemDownloadInfo { item } => {
            Ok(read_commands::read_item_download_info(&ugc, item))
        }
        SteamworksUgcCommand::GetItemInstallInfo { item } => {
            Ok(read_commands::read_item_install_info(&ugc, item))
        }
        SteamworksUgcCommand::DownloadItem {
            item,
            high_priority,
        } => read_commands::download_item(&ugc, item, high_priority),
        SteamworksUgcCommand::Query { query } => {
            let request_id = request_id.expect("async UGC query command missing request id");
            query_commands::query(&ugc, async_results, request_id, query)
        }
        SteamworksUgcCommand::QueryTotal { query } => {
            let request_id = request_id.expect("async UGC total query command missing request id");
            query_commands::query_total(&ugc, async_results, request_id, query)
        }
        SteamworksUgcCommand::QueryIds { query } => {
            let request_id = request_id.expect("async UGC ID query command missing request id");
            query_commands::query_ids(&ugc, async_results, request_id, query)
        }
        SteamworksUgcCommand::CreateItem { app_id, file_type } => {
            let request_id = request_id.expect("async UGC create command missing request id");
            Ok(item_commands::create_item(
                &ugc,
                async_results,
                request_id,
                app_id,
                file_type,
            ))
        }
        SteamworksUgcCommand::SubmitItemUpdate {
            app_id,
            item,
            update,
        } => {
            let request_id = request_id.expect("async UGC item update command missing request id");
            item_commands::submit_item_update(
                &ugc,
                async_results,
                update_watches,
                request_id,
                app_id,
                item,
                update,
            )
        }
        SteamworksUgcCommand::GetItemUpdateProgress { request_id } => {
            item_commands::read_item_update_progress(update_watches, request_id)
        }
        SteamworksUgcCommand::ForgetItemUpdate { request_id } => {
            item_commands::forget_item_update(update_watches, request_id)
        }
        SteamworksUgcCommand::SubscribeItem { item } => {
            let request_id = request_id.expect("async UGC subscribe command missing request id");
            Ok(subscription_commands::subscribe_item(
                &ugc,
                async_results,
                request_id,
                item,
            ))
        }
        SteamworksUgcCommand::UnsubscribeItem { item } => {
            let request_id = request_id.expect("async UGC unsubscribe command missing request id");
            Ok(subscription_commands::unsubscribe_item(
                &ugc,
                async_results,
                request_id,
                item,
            ))
        }
        SteamworksUgcCommand::DeleteItem { item } => {
            let request_id = request_id.expect("async UGC delete command missing request id");
            Ok(subscription_commands::delete_item(
                &ugc,
                async_results,
                request_id,
                item,
            ))
        }
        SteamworksUgcCommand::StartPlaytimeTracking { items } => {
            let request_id =
                request_id.expect("async UGC start playtime command missing request id");
            Ok(subscription_commands::start_playtime_tracking(
                &ugc,
                async_results,
                request_id,
                items,
            ))
        }
        SteamworksUgcCommand::StopPlaytimeTracking { items } => {
            let request_id =
                request_id.expect("async UGC stop playtime command missing request id");
            Ok(subscription_commands::stop_playtime_tracking(
                &ugc,
                async_results,
                request_id,
                items,
            ))
        }
        SteamworksUgcCommand::StopPlaytimeTrackingForAllItems => {
            let request_id =
                request_id.expect("async UGC stop all playtime command missing request id");
            Ok(subscription_commands::stop_playtime_tracking_for_all_items(
                &ugc,
                async_results,
                request_id,
            ))
        }
        SteamworksUgcCommand::InitWorkshopForGameServer { .. } => {
            unreachable!("server-only UGC command should be handled before client dispatch")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{SteamworksUgcItemUpdate, SteamworksUgcQuery};
    use super::*;

    #[test]
    fn async_commands_get_unique_request_ids() {
        let mut state = SteamworksUgcState::default();
        let query = SteamworksUgcQuery::item(steamworks::PublishedFileId(1));

        assert_eq!(
            async_command_request_id(&SteamworksUgcCommand::query(query.clone()), &mut state),
            Some(0)
        );
        assert_eq!(
            async_command_request_id(
                &SteamworksUgcCommand::query_total(query.clone()),
                &mut state
            ),
            Some(1)
        );
        assert_eq!(
            async_command_request_id(&SteamworksUgcCommand::query_ids(query), &mut state),
            Some(2)
        );
        assert_eq!(
            async_command_request_id(
                &SteamworksUgcCommand::download_item(steamworks::PublishedFileId(1), false),
                &mut state,
            ),
            None
        );
        assert_eq!(
            async_command_request_id(
                &SteamworksUgcCommand::subscribe_item(steamworks::PublishedFileId(1)),
                &mut state,
            ),
            Some(3)
        );
        assert_eq!(
            async_command_request_id(
                &SteamworksUgcCommand::submit_item_update(
                    steamworks::AppId(480),
                    steamworks::PublishedFileId(1),
                    SteamworksUgcItemUpdate::new().with_title("Title"),
                ),
                &mut state,
            ),
            Some(4)
        );
    }

    #[test]
    fn state_counts_async_failures_as_completed() {
        let mut state = SteamworksUgcState::default();
        let result = SteamworksUgcResult::Err {
            command: SteamworksUgcCommand::subscribe_item(steamworks::PublishedFileId(1)),
            error: SteamworksUgcError::steam_error(
                "ugc.subscribe_item",
                Some(7),
                steamworks::SteamError::IOFailure,
            ),
        };

        record_ugc_result(&mut state, &result);

        assert_eq!(state.successful_async_operations(), 0);
        assert_eq!(state.failed_async_operations(), 1);
        assert_eq!(state.completed_async_operations(), 1);
        assert_eq!(
            state.last_error(),
            Some(&SteamworksUgcError::steam_error(
                "ugc.subscribe_item",
                Some(7),
                steamworks::SteamError::IOFailure,
            ))
        );
    }
}
