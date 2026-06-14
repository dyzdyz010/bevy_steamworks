//! High-level Bevy ECS integration for Steam Workshop / UGC.
//!
//! This module builds on top of the upstream [`steamworks::UGC`] API. It keeps
//! common Workshop queries, subscriptions, downloads, and playtime tracking in
//! Bevy messages, while converting asynchronous Steam call results and download
//! callbacks into owned ECS-safe result messages.

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
    schedule::IntoScheduleConfigs,
};

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

/// Maximum number of item IDs accepted by one UGC details or playtime command.
///
/// The raw Steam call takes a `u32` count and is not intended for unbounded
/// frame-loop payloads. Larger batches should be split by the caller.
pub const STEAMWORKS_UGC_MAX_ITEMS_PER_COMMAND: usize = 1_000;

/// Maximum item title bytes accepted before the trailing NUL terminator.
pub const STEAMWORKS_UGC_MAX_UPDATE_TITLE_BYTES: usize = 128;

/// Maximum item description bytes accepted before the trailing NUL terminator.
pub const STEAMWORKS_UGC_MAX_UPDATE_DESCRIPTION_BYTES: usize = 7_999;

/// Maximum developer metadata bytes accepted before the trailing NUL terminator.
pub const STEAMWORKS_UGC_MAX_UPDATE_METADATA_BYTES: usize = 4_999;

/// Maximum item tag bytes accepted by Steam.
pub const STEAMWORKS_UGC_MAX_UPDATE_TAG_BYTES: usize = 255;

/// Maximum key/value tag removals accepted by one item update.
pub const STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAG_REMOVALS: usize = 100;

/// Maximum key/value tag additions accepted by one item update.
pub const STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAGS: usize = 100;

/// Bevy plugin for high-level Steam Workshop / UGC commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksUgcCommand`] and [`SteamworksUgcResult`] messages and runs its
/// command processor in [`bevy_app::First`] after Steam callbacks. It also
/// mirrors Workshop download completion callbacks into UGC results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksUgcPlugin;

impl SteamworksUgcPlugin {
    /// Creates a UGC plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksUgcPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksUgcState>()
            .init_resource::<SteamworksUgcAsyncResults>()
            .init_resource::<SteamworksUgcUpdateWatches>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksUgcCommand>()
            .add_message::<SteamworksUgcResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessUgcCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_ugc_commands.in_set(SteamworksSystem::ProcessUgcCommands),
            );
    }
}

mod async_results;
mod item_updates;
mod messages;
mod queries;
mod snapshots;
mod state;
mod types;
mod update_watches;
mod validation;

use async_results::SteamworksUgcAsyncResults;
use item_updates::apply_item_update;
use queries::{apply_query_options, create_query};
use snapshots::snapshot_query_results;
use update_watches::SteamworksUgcUpdateWatches;
use validation::validate_command;

pub use messages::*;
pub use state::SteamworksUgcState;
pub use types::*;

fn process_ugc_commands(
    client: Option<Res<SteamworksClient>>,
    async_results: Res<SteamworksUgcAsyncResults>,
    update_watches: Res<SteamworksUgcUpdateWatches>,
    mut state: ResMut<SteamworksUgcState>,
    mut commands: ResMut<Messages<SteamworksUgcCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksUgcResult>,
) {
    for result in async_results.drain() {
        record_ugc_result(&mut state, &result);
        state.sync_active_item_updates(&update_watches);
        results.write(result);
    }

    process_ugc_steam_events(&mut state, &mut steam_events, &mut results);

    let Some(client) = client else {
        let error = SteamworksUgcError::ClientUnavailable;
        for command in commands.drain() {
            state.record_error(error.clone());
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks UGC command failed"
            );
            results.write(SteamworksUgcResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        if let Err(error) = validate_command(&command) {
            state.record_error(error.clone());
            state.sync_active_item_updates(&update_watches);
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
            &client,
            &async_results,
            &update_watches,
            command.clone(),
            request_id,
        ) {
            Ok(operation) => {
                state.record_operation(&operation);
                state.sync_active_item_updates(&update_watches);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks UGC command"
                );
                results.write(SteamworksUgcResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                state.sync_active_item_updates(&update_watches);
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

fn process_ugc_steam_events(
    state: &mut SteamworksUgcState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksUgcResult>,
) {
    for event in steam_events.read() {
        let SteamworksEvent::DownloadItemResult(event) = event else {
            continue;
        };

        let operation = SteamworksUgcOperation::DownloadItemResultReceived {
            result: SteamworksUgcDownloadItemResult {
                app_id: event.app_id,
                item: event.published_file_id,
                error: event.error,
            },
        };
        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks UGC callback"
        );
        results.write(SteamworksUgcResult::Ok(operation));
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
            Ok(SteamworksUgcOperation::DownloadsSuspended { suspend })
        }
        SteamworksUgcCommand::ListSubscribedItems {
            include_locally_disabled,
        } => {
            let items = ugc.subscribed_items(include_locally_disabled);
            Ok(SteamworksUgcOperation::SubscribedItemsListed {
                include_locally_disabled,
                items,
            })
        }
        SteamworksUgcCommand::GetItemState { item } => Ok(SteamworksUgcOperation::ItemStateRead {
            info: SteamworksUgcItemStateInfo {
                item,
                state: ugc.item_state(item),
            },
        }),
        SteamworksUgcCommand::GetItemDownloadInfo { item } => {
            Ok(SteamworksUgcOperation::ItemDownloadInfoRead {
                info: SteamworksUgcItemDownloadInfoResult {
                    item,
                    info: ugc
                        .item_download_info(item)
                        .map(
                            |(downloaded_bytes, total_bytes)| SteamworksUgcItemDownloadInfo {
                                downloaded_bytes,
                                total_bytes,
                            },
                        ),
                },
            })
        }
        SteamworksUgcCommand::GetItemInstallInfo { item } => {
            Ok(SteamworksUgcOperation::ItemInstallInfoRead {
                info: SteamworksUgcItemInstallInfoResult {
                    item,
                    info: ugc
                        .item_install_info(item)
                        .map(|info| SteamworksUgcItemInstallInfo {
                            folder: info.folder,
                            size_on_disk: info.size_on_disk,
                            timestamp: info.timestamp,
                        }),
                },
            })
        }
        SteamworksUgcCommand::DownloadItem {
            item,
            high_priority,
        } => {
            if !ugc.download_item(item, high_priority) {
                return Err(SteamworksUgcError::operation_failed("ugc.download_item"));
            }
            Ok(SteamworksUgcOperation::DownloadItemSubmitted {
                item,
                high_priority,
            })
        }
        SteamworksUgcCommand::Query { query } => {
            let request_id = request_id.expect("async UGC query command missing request id");
            let options = query.options().clone();
            let query_handle = create_query(&ugc, &query)?;
            let query_handle = apply_query_options(query_handle, &options)?;
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::Query {
                query: query.clone(),
            };
            let callback_query = query.clone();
            query_handle.fetch(move |result| {
                async_results.push(match result {
                    Ok(results) => {
                        SteamworksUgcResult::Ok(SteamworksUgcOperation::QueryCompleted {
                            request_id,
                            query: callback_query,
                            results: snapshot_query_results(&results, &options),
                        })
                    }
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.query.fetch",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::QueryRequested { request_id, query })
        }
        SteamworksUgcCommand::CreateItem { app_id, file_type } => {
            let request_id = request_id.expect("async UGC create command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::CreateItem { app_id, file_type };
            ugc.create_item(app_id, file_type, move |result| {
                async_results.push(match result {
                    Ok((item, legal)) => {
                        SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemCreated {
                            request_id,
                            item,
                            user_needs_to_accept_workshop_legal_agreement: legal,
                        })
                    }
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.create_item",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::ItemCreateRequested {
                request_id,
                app_id,
                file_type,
            })
        }
        SteamworksUgcCommand::SubmitItemUpdate {
            app_id,
            item,
            update,
        } => {
            let request_id = request_id.expect("async UGC item update command missing request id");
            let update_handle = ugc.start_item_update(app_id, item);
            let update_handle = apply_item_update(update_handle, &update)?;
            let async_results = async_results.clone();
            let update_watches_for_callback = update_watches.clone();
            let command = SteamworksUgcCommand::SubmitItemUpdate {
                app_id,
                item,
                update: update.clone(),
            };
            let watch = update_handle.submit(update.change_note.as_deref(), move |result| {
                update_watches_for_callback.remove(request_id);
                async_results.push(match result {
                    Ok((updated_item, legal)) => {
                        SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemUpdated {
                            request_id,
                            item: updated_item,
                            user_needs_to_accept_workshop_legal_agreement: legal,
                        })
                    }
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.submit_item_update",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            update_watches.insert(request_id, watch);
            Ok(SteamworksUgcOperation::ItemUpdateSubmitted {
                request_id,
                app_id,
                item,
                update,
            })
        }
        SteamworksUgcCommand::GetItemUpdateProgress { request_id } => {
            let progress = update_watches
                .progress(request_id)
                .ok_or(SteamworksUgcError::ItemUpdateNotFound { request_id })?;
            Ok(SteamworksUgcOperation::ItemUpdateProgressRead { progress })
        }
        SteamworksUgcCommand::ForgetItemUpdate { request_id } => {
            if update_watches.remove(request_id) {
                Ok(SteamworksUgcOperation::ItemUpdateForgotten { request_id })
            } else {
                Err(SteamworksUgcError::ItemUpdateNotFound { request_id })
            }
        }
        SteamworksUgcCommand::SubscribeItem { item } => {
            let request_id = request_id.expect("async UGC subscribe command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::SubscribeItem { item };
            ugc.subscribe_item(item, move |result| {
                async_results.push(match result {
                    Ok(()) => SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemSubscribed {
                        request_id,
                        item,
                    }),
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.subscribe_item",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::ItemSubscribeRequested { request_id, item })
        }
        SteamworksUgcCommand::UnsubscribeItem { item } => {
            let request_id = request_id.expect("async UGC unsubscribe command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::UnsubscribeItem { item };
            ugc.unsubscribe_item(item, move |result| {
                async_results.push(match result {
                    Ok(()) => SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemUnsubscribed {
                        request_id,
                        item,
                    }),
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.unsubscribe_item",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::ItemUnsubscribeRequested { request_id, item })
        }
        SteamworksUgcCommand::DeleteItem { item } => {
            let request_id = request_id.expect("async UGC delete command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::DeleteItem { item };
            ugc.delete_item(item, move |result| {
                async_results.push(match result {
                    Ok(()) => SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemDeleted {
                        request_id,
                        item,
                    }),
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.delete_item",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::ItemDeleteRequested { request_id, item })
        }
        SteamworksUgcCommand::StartPlaytimeTracking { items } => {
            let request_id =
                request_id.expect("async UGC start playtime command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::StartPlaytimeTracking {
                items: items.clone(),
            };
            let callback_items = items.clone();
            ugc.start_playtime_tracking(&items, move |result| {
                async_results.push(match result {
                    Ok(()) => {
                        SteamworksUgcResult::Ok(SteamworksUgcOperation::PlaytimeTrackingStarted {
                            request_id,
                            items: callback_items,
                        })
                    }
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.start_playtime_tracking",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::PlaytimeTrackingStartRequested { request_id, items })
        }
        SteamworksUgcCommand::StopPlaytimeTracking { items } => {
            let request_id =
                request_id.expect("async UGC stop playtime command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::StopPlaytimeTracking {
                items: items.clone(),
            };
            let callback_items = items.clone();
            ugc.stop_playtime_tracking(&items, move |result| {
                async_results.push(match result {
                    Ok(()) => {
                        SteamworksUgcResult::Ok(SteamworksUgcOperation::PlaytimeTrackingStopped {
                            request_id,
                            items: callback_items,
                        })
                    }
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.stop_playtime_tracking",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::PlaytimeTrackingStopRequested { request_id, items })
        }
        SteamworksUgcCommand::StopPlaytimeTrackingForAllItems => {
            let request_id =
                request_id.expect("async UGC stop all playtime command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::StopPlaytimeTrackingForAllItems;
            ugc.stop_playtime_tracking_for_all_items(move |result| {
                async_results.push(match result {
                    Ok(()) => SteamworksUgcResult::Ok(
                        SteamworksUgcOperation::PlaytimeTrackingForAllItemsStopped { request_id },
                    ),
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.stop_playtime_tracking_for_all_items",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::PlaytimeTrackingForAllItemsStopRequested { request_id })
        }
    }
}

#[cfg(test)]
mod tests;
