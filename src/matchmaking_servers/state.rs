use bevy_ecs::prelude::Resource;

use crate::cache::trim_oldest;

use super::*;

mod accessors;
mod operations;

pub(in crate::matchmaking_servers) const STEAMWORKS_MATCHMAKING_SERVERS_STATE_CACHE_LIMIT: usize =
    1_024;

/// Runtime state for [`crate::SteamworksMatchmakingServersPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksMatchmakingServersState {
    last_error: Option<SteamworksMatchmakingServersError>,
    active_server_list_requests: usize,
    server_list_requests: Vec<SteamworksServerListRequestInfo>,
    last_server_list_request: Option<SteamworksServerListRequestInfo>,
    last_released_server_list_request: Option<SteamworksServerListRequestId>,
    last_server_list_refresh_request: Option<SteamworksServerListRequestId>,
    last_refresh_completion_request: Option<SteamworksServerListRequestId>,
    last_server_refresh_request: Option<SteamworksServerListServerIndex>,
    server_list_counts: Vec<SteamworksServerListCount>,
    last_server_list_count: Option<SteamworksServerListCount>,
    server_list_refreshing_states: Vec<SteamworksServerListRefreshing>,
    last_server_list_refreshing: Option<SteamworksServerListRefreshing>,
    last_server_response: Option<SteamworksServerListServerIndex>,
    last_server_failure: Option<SteamworksServerListServerIndex>,
    last_server_details_read: Option<SteamworksServerListServerIndex>,
    servers: Vec<SteamworksCachedServerListServer>,
    last_server: Option<SteamworksGameServerItem>,
    last_refresh_response: Option<SteamworksServerListResponse>,
    server_queries: Vec<SteamworksServerQueryInfo>,
    last_server_query: Option<SteamworksServerQueryInfo>,
    server_pings: Vec<SteamworksServerPing>,
    last_server_ping: Option<SteamworksServerPing>,
    failed_server_pings: Vec<SteamworksServerQueryId>,
    last_failed_server_ping: Option<SteamworksServerQueryId>,
    server_player_details: Vec<SteamworksServerPlayerDetails>,
    last_server_player_details: Option<SteamworksServerPlayerDetails>,
    failed_server_player_details: Vec<SteamworksServerQueryId>,
    last_failed_server_player_details: Option<SteamworksServerQueryId>,
    server_rules: Vec<SteamworksServerRules>,
    last_server_rules: Option<SteamworksServerRules>,
    failed_server_rules: Vec<SteamworksServerQueryId>,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::matchmaking_servers) struct SteamworksCachedServerListServer {
    pub index: SteamworksServerListServerIndex,
    pub server: SteamworksGameServerItem,
}

pub(super) fn upsert_by<T, K: PartialEq>(
    items: &mut Vec<T>,
    key: K,
    value: T,
    key_for: impl Fn(&T) -> K,
) {
    if let Some(existing) = items.iter_mut().find(|existing| key_for(existing) == key) {
        *existing = value;
    } else {
        items.push(value);
        trim_oldest(items, STEAMWORKS_MATCHMAKING_SERVERS_STATE_CACHE_LIMIT);
    }
}

pub(super) fn push_bounded<T>(items: &mut Vec<T>, value: T) {
    items.push(value);
    trim_oldest(items, STEAMWORKS_MATCHMAKING_SERVERS_STATE_CACHE_LIMIT);
}
