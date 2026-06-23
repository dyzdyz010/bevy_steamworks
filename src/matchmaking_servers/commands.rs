use std::sync::{Arc, Mutex};

use bevy_ecs::{
    message::{MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::SteamworksClient;

use super::{
    callbacks::{
        ping_callbacks, player_details_callbacks, server_list_callbacks, server_rules_callbacks,
    },
    requests::{SteamworksMatchmakingServerListRequests, SteamworksMatchmakingServersAsyncResults},
    state::SteamworksMatchmakingServersState,
    validation::{validate_command, validate_server_index_in_request},
    SteamworksGameServerItem, SteamworksMatchmakingServersCommand,
    SteamworksMatchmakingServersError, SteamworksMatchmakingServersOperation,
    SteamworksMatchmakingServersResult, SteamworksServerListFilters, SteamworksServerListKind,
    SteamworksServerListRequestId, SteamworksServerQueryId, SteamworksServerQueryInfo,
    SteamworksServerQueryKind,
};

pub(super) fn process_matchmaking_servers_commands(
    client: Option<Res<SteamworksClient>>,
    async_results: Res<SteamworksMatchmakingServersAsyncResults>,
    requests: Res<SteamworksMatchmakingServerListRequests>,
    mut state: ResMut<SteamworksMatchmakingServersState>,
    mut commands: ResMut<Messages<SteamworksMatchmakingServersCommand>>,
    mut results: MessageWriter<SteamworksMatchmakingServersResult>,
) {
    for result in async_results.drain() {
        match &result {
            SteamworksMatchmakingServersResult::Ok(operation) => {
                state.record_operation(operation);
                state.sync_request_count(requests.len());
            }
            SteamworksMatchmakingServersResult::Err { error, .. } => {
                state.record_error(error.clone());
                state.sync_request_count(requests.len());
            }
        }
        results.write(result);
    }

    let Some(client) = client else {
        let error = SteamworksMatchmakingServersError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks matchmaking servers command failed"
            );
            results.write(SteamworksMatchmakingServersResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        let (request, query) = match command_ids_after_validation(&command, &mut state) {
            Ok(ids) => ids,
            Err(error) => {
                state.record_error(error.clone());
                state.sync_request_count(requests.len());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks matchmaking servers command failed"
                );
                results.write(SteamworksMatchmakingServersResult::Err { command, error });
                continue;
            }
        };

        match handle_matchmaking_servers_command(
            &client,
            &async_results,
            &requests,
            command.clone(),
            request,
            query,
        ) {
            Ok(operation) => {
                state.record_operation(&operation);
                state.sync_request_count(requests.len());
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks matchmaking servers command"
                );
                results.write(SteamworksMatchmakingServersResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                state.sync_request_count(requests.len());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks matchmaking servers command failed"
                );
                results.write(SteamworksMatchmakingServersResult::Err { command, error });
            }
        }
    }
}

fn command_ids_after_validation(
    command: &SteamworksMatchmakingServersCommand,
    state: &mut SteamworksMatchmakingServersState,
) -> Result<
    (
        Option<SteamworksServerListRequestId>,
        Option<SteamworksServerQueryId>,
    ),
    SteamworksMatchmakingServersError,
> {
    validate_command(command)?;

    let request = match command {
        SteamworksMatchmakingServersCommand::RequestServerList { .. } => {
            Some(state.next_request_id())
        }
        _ => None,
    };
    let query = match command {
        SteamworksMatchmakingServersCommand::PingServer { .. }
        | SteamworksMatchmakingServersCommand::QueryPlayerDetails { .. }
        | SteamworksMatchmakingServersCommand::QueryServerRules { .. } => {
            Some(state.next_query_id())
        }
        _ => None,
    };

    Ok((request, query))
}

fn handle_matchmaking_servers_command(
    client: &SteamworksClient,
    async_results: &SteamworksMatchmakingServersAsyncResults,
    requests: &SteamworksMatchmakingServerListRequests,
    command: SteamworksMatchmakingServersCommand,
    request: Option<SteamworksServerListRequestId>,
    query: Option<SteamworksServerQueryId>,
) -> Result<SteamworksMatchmakingServersOperation, SteamworksMatchmakingServersError> {
    validate_command(&command)?;

    match command {
        SteamworksMatchmakingServersCommand::PingServer { target } => {
            let query = query.expect("server query command missing query id");
            client.matchmaking_servers().ping_server(
                target.address,
                target.query_port,
                ping_callbacks(query, target, async_results.clone()),
            );
            Ok(
                SteamworksMatchmakingServersOperation::ServerQuerySubmitted {
                    query: SteamworksServerQueryInfo {
                        query,
                        kind: SteamworksServerQueryKind::Ping,
                        target,
                    },
                },
            )
        }
        SteamworksMatchmakingServersCommand::QueryPlayerDetails { target } => {
            let query = query.expect("server query command missing query id");
            client.matchmaking_servers().player_details(
                target.address,
                target.query_port,
                player_details_callbacks(query, target, async_results.clone()),
            );
            Ok(
                SteamworksMatchmakingServersOperation::ServerQuerySubmitted {
                    query: SteamworksServerQueryInfo {
                        query,
                        kind: SteamworksServerQueryKind::PlayerDetails,
                        target,
                    },
                },
            )
        }
        SteamworksMatchmakingServersCommand::QueryServerRules { target } => {
            let query = query.expect("server query command missing query id");
            client.matchmaking_servers().server_rules(
                target.address,
                target.query_port,
                server_rules_callbacks(query, target, async_results.clone()),
            );
            Ok(
                SteamworksMatchmakingServersOperation::ServerQuerySubmitted {
                    query: SteamworksServerQueryInfo {
                        query,
                        kind: SteamworksServerQueryKind::Rules,
                        target,
                    },
                },
            )
        }
        SteamworksMatchmakingServersCommand::RequestServerList {
            app_id,
            kind,
            filters,
        } => {
            let request = request.expect("server-list request command missing request id");
            let handle = request_server_list(
                client,
                async_results.clone(),
                request,
                app_id,
                kind,
                &filters,
            )?;
            requests.insert(request, client, handle);
            Ok(SteamworksMatchmakingServersOperation::ServerListRequested {
                request,
                app_id,
                kind,
                filters,
            })
        }
        SteamworksMatchmakingServersCommand::RefreshServerList { request } => {
            let handle = request_handle(requests, request)?;
            handle
                .lock()
                .expect("Steamworks server-list request mutex was poisoned")
                .refresh_query()
                .map_err(
                    |_| SteamworksMatchmakingServersError::ServerListRequestRejected {
                        request,
                        operation: "refresh_query",
                    },
                )?;
            Ok(SteamworksMatchmakingServersOperation::ServerListRefreshSubmitted { request })
        }
        SteamworksMatchmakingServersCommand::RefreshServer { request, server } => {
            let handle = request_handle(requests, request)?;
            validate_server_index_in_request(&handle, request, server)?;
            handle
                .lock()
                .expect("Steamworks server-list request mutex was poisoned")
                .refresh_server(server)
                .map_err(
                    |_| SteamworksMatchmakingServersError::ServerListRequestRejected {
                        request,
                        operation: "refresh_server",
                    },
                )?;
            Ok(
                SteamworksMatchmakingServersOperation::ServerRefreshSubmitted {
                    request,
                    server_index: server,
                },
            )
        }
        SteamworksMatchmakingServersCommand::GetServerListCount { request } => {
            let handle = request_handle(requests, request)?;
            let count = handle
                .lock()
                .expect("Steamworks server-list request mutex was poisoned")
                .get_server_count()
                .map_err(
                    |_| SteamworksMatchmakingServersError::ServerListRequestReleased { request },
                )?;
            Ok(SteamworksMatchmakingServersOperation::ServerListCountRead { request, count })
        }
        SteamworksMatchmakingServersCommand::GetServerDetails { request, server } => {
            let handle = request_handle(requests, request)?;
            validate_server_index_in_request(&handle, request, server)?;
            let item = handle
                .lock()
                .expect("Steamworks server-list request mutex was poisoned")
                .get_server_details(server)
                .map_err(
                    |_| SteamworksMatchmakingServersError::ServerListRequestReleased { request },
                )
                .map(SteamworksGameServerItem::from_steam)?;
            Ok(SteamworksMatchmakingServersOperation::ServerDetailsRead {
                request,
                server_index: server,
                server: item,
            })
        }
        SteamworksMatchmakingServersCommand::IsServerListRefreshing { request } => {
            let handle = request_handle(requests, request)?;
            let refreshing = handle
                .lock()
                .expect("Steamworks server-list request mutex was poisoned")
                .is_refreshing()
                .map_err(
                    |_| SteamworksMatchmakingServersError::ServerListRequestReleased { request },
                )?;
            Ok(
                SteamworksMatchmakingServersOperation::ServerListRefreshingRead {
                    request,
                    refreshing,
                },
            )
        }
        SteamworksMatchmakingServersCommand::ReleaseServerList { request } => {
            let handle = request_handle(requests, request)?;
            handle
                .lock()
                .expect("Steamworks server-list request mutex was poisoned")
                .release()
                .map_err(
                    |source| SteamworksMatchmakingServersError::ServerListReleaseFailed {
                        request,
                        reason: source.into(),
                    },
                )?;
            requests.remove(request);
            Ok(SteamworksMatchmakingServersOperation::ServerListReleased { request })
        }
    }
}

fn request_server_list(
    client: &SteamworksClient,
    async_results: SteamworksMatchmakingServersAsyncResults,
    request: SteamworksServerListRequestId,
    app_id: steamworks::AppId,
    kind: SteamworksServerListKind,
    filters: &SteamworksServerListFilters,
) -> Result<Arc<Mutex<steamworks::ServerListRequest>>, SteamworksMatchmakingServersError> {
    let servers = client.matchmaking_servers();
    let callbacks = server_list_callbacks(request, async_results);
    match kind {
        SteamworksServerListKind::Lan => Ok(servers.lan_server_list(app_id, callbacks)),
        SteamworksServerListKind::Internet => servers
            .internet_server_list(app_id, &filters.as_upstream_map(), callbacks)
            .map_err(|_| SteamworksMatchmakingServersError::ServerListQueryRejected { kind }),
        SteamworksServerListKind::Favorites => servers
            .favorites_server_list(app_id, &filters.as_upstream_map(), callbacks)
            .map_err(|_| SteamworksMatchmakingServersError::ServerListQueryRejected { kind }),
        SteamworksServerListKind::History => servers
            .history_server_list(app_id, &filters.as_upstream_map(), callbacks)
            .map_err(|_| SteamworksMatchmakingServersError::ServerListQueryRejected { kind }),
        SteamworksServerListKind::Friends => servers
            .friends_server_list(app_id, &filters.as_upstream_map(), callbacks)
            .map_err(|_| SteamworksMatchmakingServersError::ServerListQueryRejected { kind }),
    }
}

fn request_handle(
    requests: &SteamworksMatchmakingServerListRequests,
    request: SteamworksServerListRequestId,
) -> Result<Arc<Mutex<steamworks::ServerListRequest>>, SteamworksMatchmakingServersError> {
    requests
        .get(request)
        .ok_or(SteamworksMatchmakingServersError::ServerListRequestNotFound { request })
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::*;

    #[test]
    fn invalid_server_list_request_does_not_consume_request_id() {
        let mut state = SteamworksMatchmakingServersState::default();
        let command = SteamworksMatchmakingServersCommand::RequestServerList {
            app_id: steamworks::AppId(480),
            kind: SteamworksServerListKind::Lan,
            filters: SteamworksServerListFilters::new().with("map", "arena"),
        };

        assert_eq!(
            command_ids_after_validation(&command, &mut state),
            Err(SteamworksMatchmakingServersError::LanFiltersUnsupported)
        );
        assert_eq!(state.next_request_id().raw(), 0);
    }

    #[test]
    fn valid_request_and_query_commands_allocate_predictable_ids() {
        let mut state = SteamworksMatchmakingServersState::default();

        let (request, query) = command_ids_after_validation(
            &SteamworksMatchmakingServersCommand::request_internet_server_list(
                480,
                SteamworksServerListFilters::new(),
            ),
            &mut state,
        )
        .expect("valid server-list request should allocate an id");
        assert_eq!(request.map(SteamworksServerListRequestId::raw), Some(0));
        assert_eq!(query, None);

        let (request, query) = command_ids_after_validation(
            &SteamworksMatchmakingServersCommand::ping_server(Ipv4Addr::LOCALHOST, 27015),
            &mut state,
        )
        .expect("valid direct query should allocate an id");
        assert_eq!(request, None);
        assert_eq!(query.map(SteamworksServerQueryId::raw), Some(0));
        assert_eq!(state.next_request_id().raw(), 1);
        assert_eq!(state.next_query_id().raw(), 1);
    }
}
