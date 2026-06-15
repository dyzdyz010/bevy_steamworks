use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
    system::SystemParam,
};

use crate::{SteamworksClient, SteamworksEvent, SteamworksServer};

use super::{
    async_results::SteamworksUgcAsyncResults, callbacks::process_ugc_steam_events,
    item_updates::apply_item_update, queries::apply_query_options, queries::create_query,
    snapshots::snapshot_query_results, update_watches::SteamworksUgcUpdateWatches,
    validation::validate_command, SteamworksUgcCommand, SteamworksUgcError,
    SteamworksUgcItemDownloadInfo, SteamworksUgcItemDownloadInfoResult,
    SteamworksUgcItemInstallInfo, SteamworksUgcItemInstallInfoResult, SteamworksUgcItemStateInfo,
    SteamworksUgcOperation, SteamworksUgcResult, SteamworksUgcState,
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
        } => {
            let server = server.ok_or(SteamworksUgcError::ServerUnavailable)?;
            if !server
                .ugc()
                .init_for_game_server(workshop_depot.raw(), &folder)
            {
                return Err(SteamworksUgcError::operation_failed(
                    "ugc.init_for_game_server",
                ));
            }
            Ok(SteamworksUgcOperation::GameServerWorkshopInitialized {
                workshop_depot,
                folder,
            })
        }
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
        SteamworksUgcCommand::QueryTotal { query } => {
            let request_id = request_id.expect("async UGC total query command missing request id");
            let options = query_options_without_payload_shape_flags(&query);
            let query_handle = create_query(&ugc, &query)?;
            let query_handle = apply_query_options(query_handle, &options)?;
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::QueryTotal {
                query: query.clone(),
            };
            let callback_query = query.clone();
            query_handle.fetch_total(move |result| {
                async_results.push(match result {
                    Ok(total_results) => {
                        SteamworksUgcResult::Ok(SteamworksUgcOperation::QueryTotalCompleted {
                            request_id,
                            query: callback_query.clone(),
                            total: super::SteamworksUgcQueryTotal { total_results },
                        })
                    }
                    Err(source) => SteamworksUgcResult::Err {
                        command: command.clone(),
                        error: SteamworksUgcError::steam_error(
                            "ugc.query.fetch_total",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::QueryTotalRequested { request_id, query })
        }
        SteamworksUgcCommand::QueryIds { query } => {
            let request_id = request_id.expect("async UGC ID query command missing request id");
            let options = query_options_without_payload_shape_flags(&query);
            let query_handle = create_query(&ugc, &query)?;
            let query_handle = apply_query_options(query_handle, &options)?;
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::QueryIds {
                query: query.clone(),
            };
            let callback_query = query.clone();
            query_handle.fetch_ids(move |result| {
                async_results.push(match result {
                    Ok(items) => {
                        SteamworksUgcResult::Ok(SteamworksUgcOperation::QueryIdsCompleted {
                            request_id,
                            query: callback_query.clone(),
                            ids: super::SteamworksUgcQueryIds { items },
                        })
                    }
                    Err(source) => SteamworksUgcResult::Err {
                        command: command.clone(),
                        error: SteamworksUgcError::steam_error(
                            "ugc.query.fetch_ids",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::QueryIdsRequested { request_id, query })
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
        SteamworksUgcCommand::InitWorkshopForGameServer { .. } => {
            unreachable!("server-only UGC command should be handled before client dispatch")
        }
    }
}

fn query_options_without_payload_shape_flags(
    query: &super::SteamworksUgcQuery,
) -> super::SteamworksUgcQueryOptions {
    let mut options = query.options().clone();
    options.return_only_ids = false;
    options.return_total_only = false;
    options
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

    #[test]
    fn specialized_query_commands_ignore_payload_shape_option_flags() {
        let query = SteamworksUgcQuery::item(steamworks::PublishedFileId(1)).with_options(
            super::super::SteamworksUgcQueryOptions::new()
                .with_return_only_ids(true)
                .with_return_total_only(true),
        );

        let options = query_options_without_payload_shape_flags(&query);

        assert!(!options.return_only_ids);
        assert!(!options.return_total_only);
    }
}
