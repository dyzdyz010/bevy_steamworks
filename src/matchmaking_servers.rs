//! High-level Bevy ECS integration for Steam Matchmaking Servers.
//!
//! This module builds on top of the upstream
//! [`steamworks::MatchmakingServers`] API. It exposes Steam server-browser
//! list requests through Bevy commands/results while keeping the upstream
//! request handles owned by the plugin.

use std::sync::{Arc, Mutex};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{MessageWriter, Messages},
    prelude::{Res, ResMut},
    schedule::IntoScheduleConfigs,
};

use crate::{SteamworksClient, SteamworksSystem};

mod messages;
mod requests;
mod state;
#[cfg(test)]
mod tests;
mod types;

pub use messages::*;
use requests::*;
pub use state::SteamworksMatchmakingServersState;
pub use types::*;

/// Maximum byte length for one Steam server-list filter key or value.
pub const STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES: usize = 255;

/// Bevy plugin for high-level Steam Matchmaking Servers commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksMatchmakingServersCommand`] and
/// [`SteamworksMatchmakingServersResult`] messages and processes commands in
/// [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksMatchmakingServersPlugin;

impl SteamworksMatchmakingServersPlugin {
    /// Creates a Matchmaking Servers plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksMatchmakingServersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksMatchmakingServersState>()
            .init_resource::<SteamworksMatchmakingServersAsyncResults>()
            .init_resource::<SteamworksMatchmakingServerListRequests>()
            .add_message::<SteamworksMatchmakingServersCommand>()
            .add_message::<SteamworksMatchmakingServersResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessMatchmakingServersCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_matchmaking_servers_commands
                    .in_set(SteamworksSystem::ProcessMatchmakingServersCommands),
            );
    }
}

fn process_matchmaking_servers_commands(
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
        let request = match &command {
            SteamworksMatchmakingServersCommand::RequestServerList { .. } => {
                Some(state.next_request_id())
            }
            _ => None,
        };

        match handle_matchmaking_servers_command(
            &client,
            &async_results,
            &requests,
            command.clone(),
            request,
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

fn handle_matchmaking_servers_command(
    client: &SteamworksClient,
    async_results: &SteamworksMatchmakingServersAsyncResults,
    requests: &SteamworksMatchmakingServerListRequests,
    command: SteamworksMatchmakingServersCommand,
    request: Option<SteamworksServerListRequestId>,
) -> Result<SteamworksMatchmakingServersOperation, SteamworksMatchmakingServersError> {
    validate_command(&command)?;

    match command {
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

fn server_list_callbacks(
    request: SteamworksServerListRequestId,
    async_results: SteamworksMatchmakingServersAsyncResults,
) -> steamworks::ServerListCallbacks {
    let responded_results = async_results.clone();
    let failed_results = async_results.clone();

    steamworks::ServerListCallbacks::new(
        Box::new(move |list, server_index| {
            let result = list
                .lock()
                .expect("Steamworks server-list request mutex was poisoned")
                .get_server_details(server_index);
            responded_results.push(match result {
                Ok(server) => SteamworksMatchmakingServersResult::Ok(
                    SteamworksMatchmakingServersOperation::ServerResponded {
                        request,
                        server_index,
                        server: SteamworksGameServerItem::from_steam(server),
                    },
                ),
                Err(()) => SteamworksMatchmakingServersResult::Err {
                    command: SteamworksMatchmakingServersCommand::GetServerDetails {
                        request,
                        server: server_index,
                    },
                    error: SteamworksMatchmakingServersError::ServerDetailsUnavailable {
                        request,
                        server: server_index,
                    },
                },
            });
        }),
        Box::new(move |_list, server_index| {
            failed_results.push(SteamworksMatchmakingServersResult::Ok(
                SteamworksMatchmakingServersOperation::ServerFailedToRespond {
                    request,
                    server_index,
                },
            ));
        }),
        Box::new(move |_list, response| {
            async_results.push(SteamworksMatchmakingServersResult::Ok(
                SteamworksMatchmakingServersOperation::ServerListRefreshCompleted {
                    request,
                    response: response.into(),
                },
            ));
        }),
    )
}

fn request_handle(
    requests: &SteamworksMatchmakingServerListRequests,
    request: SteamworksServerListRequestId,
) -> Result<Arc<Mutex<steamworks::ServerListRequest>>, SteamworksMatchmakingServersError> {
    requests
        .get(request)
        .ok_or(SteamworksMatchmakingServersError::ServerListRequestNotFound { request })
}

fn validate_server_index_in_request(
    handle: &Arc<Mutex<steamworks::ServerListRequest>>,
    request: SteamworksServerListRequestId,
    server: i32,
) -> Result<(), SteamworksMatchmakingServersError> {
    let count = handle
        .lock()
        .expect("Steamworks server-list request mutex was poisoned")
        .get_server_count()
        .map_err(|_| SteamworksMatchmakingServersError::ServerListRequestReleased { request })?;
    if server >= count {
        return Err(SteamworksMatchmakingServersError::ServerIndexOutOfRange {
            request,
            server,
            count,
        });
    }

    Ok(())
}

fn validate_command(
    command: &SteamworksMatchmakingServersCommand,
) -> Result<(), SteamworksMatchmakingServersError> {
    match command {
        SteamworksMatchmakingServersCommand::RequestServerList { kind, filters, .. } => {
            validate_filters(*kind, filters)
        }
        SteamworksMatchmakingServersCommand::RefreshServer { server, .. }
        | SteamworksMatchmakingServersCommand::GetServerDetails { server, .. } => {
            validate_server_index(*server)
        }
        _ => Ok(()),
    }
}

fn validate_filters(
    kind: SteamworksServerListKind,
    filters: &SteamworksServerListFilters,
) -> Result<(), SteamworksMatchmakingServersError> {
    if kind == SteamworksServerListKind::Lan && !filters.is_empty() {
        return Err(SteamworksMatchmakingServersError::LanFiltersUnsupported);
    }

    for (key, value) in filters.entries() {
        validate_filter_text("filter key", key)?;
        validate_filter_text("filter value", value)?;
    }

    Ok(())
}

fn validate_filter_text(
    field: &'static str,
    value: &str,
) -> Result<(), SteamworksMatchmakingServersError> {
    if value.as_bytes().contains(&0) {
        return Err(SteamworksMatchmakingServersError::invalid_string(field));
    }
    if value.len() > STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES {
        return Err(SteamworksMatchmakingServersError::FilterTooLong {
            field,
            requested: value.len(),
            max_supported: STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES,
        });
    }
    Ok(())
}

fn validate_server_index(server: i32) -> Result<(), SteamworksMatchmakingServersError> {
    if server < 0 {
        Err(SteamworksMatchmakingServersError::InvalidServerIndex { server })
    } else {
        Ok(())
    }
}
