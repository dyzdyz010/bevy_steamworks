use bevy_ecs::prelude::Resource;

use super::*;

mod accessors;
mod operations;

/// Runtime state for [`crate::SteamworksMatchmakingServersPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksMatchmakingServersState {
    last_error: Option<SteamworksMatchmakingServersError>,
    active_server_list_requests: usize,
    last_server_list_request: Option<SteamworksServerListRequestInfo>,
    last_released_server_list_request: Option<SteamworksServerListRequestId>,
    last_server_list_refresh_request: Option<SteamworksServerListRequestId>,
    last_refresh_completion_request: Option<SteamworksServerListRequestId>,
    last_server_refresh_request: Option<SteamworksServerListServerIndex>,
    last_server_list_count: Option<SteamworksServerListCount>,
    last_server_list_refreshing: Option<SteamworksServerListRefreshing>,
    last_server_response: Option<SteamworksServerListServerIndex>,
    last_server_failure: Option<SteamworksServerListServerIndex>,
    last_server_details_read: Option<SteamworksServerListServerIndex>,
    last_server: Option<SteamworksGameServerItem>,
    last_refresh_response: Option<SteamworksServerListResponse>,
    last_server_query: Option<SteamworksServerQueryInfo>,
    last_server_ping: Option<SteamworksServerPing>,
    last_failed_server_ping: Option<SteamworksServerQueryId>,
    last_server_player_details: Option<SteamworksServerPlayerDetails>,
    last_failed_server_player_details: Option<SteamworksServerQueryId>,
    last_server_rules: Option<SteamworksServerRules>,
    last_failed_server_rules: Option<SteamworksServerQueryId>,
    server_list_request_count: u64,
    server_list_release_count: u64,
    server_list_refresh_request_count: u64,
    server_refresh_request_count: u64,
    server_list_count_read_count: u64,
    server_list_refreshing_read_count: u64,
    server_response_count: u64,
    server_failure_count: u64,
    refresh_complete_count: u64,
    server_query_count: u64,
    server_ping_response_count: u64,
    server_ping_failure_count: u64,
    server_player_details_count: u64,
    server_player_details_failure_count: u64,
    server_rules_count: u64,
    server_rules_failure_count: u64,
    next_request_id: u64,
    next_query_id: u64,
}
