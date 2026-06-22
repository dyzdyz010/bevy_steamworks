use super::SteamworksMatchmakingServersState;
use crate::matchmaking_servers::{
    SteamworksGameServerItem, SteamworksMatchmakingServersError, SteamworksServerListCount,
    SteamworksServerListRefreshing, SteamworksServerListRequestId, SteamworksServerListRequestInfo,
    SteamworksServerListResponse, SteamworksServerListServerIndex, SteamworksServerPing,
    SteamworksServerPlayerDetails, SteamworksServerQueryId, SteamworksServerQueryInfo,
    SteamworksServerRules,
};

impl SteamworksMatchmakingServersState {
    /// Returns the most recent synchronous or callback error observed by the plugin.
    pub fn last_error(&self) -> Option<&SteamworksMatchmakingServersError> {
        self.last_error.as_ref()
    }

    /// Returns the number of active server-list request handles owned by the plugin.
    pub fn active_server_list_requests(&self) -> usize {
        self.active_server_list_requests
    }

    /// Returns the most recent server-list request submitted through this plugin.
    pub fn last_server_list_request(&self) -> Option<&SteamworksServerListRequestInfo> {
        self.last_server_list_request.as_ref()
    }

    /// Returns the most recent server-list request released through this plugin.
    pub fn last_released_server_list_request(&self) -> Option<SteamworksServerListRequestId> {
        self.last_released_server_list_request
    }

    /// Returns the most recent server-list refresh request submitted through this plugin.
    pub fn last_server_list_refresh_request(&self) -> Option<SteamworksServerListRequestId> {
        self.last_server_list_refresh_request
    }

    /// Returns the most recent server-list request whose refresh completed.
    pub fn last_refresh_completion_request(&self) -> Option<SteamworksServerListRequestId> {
        self.last_refresh_completion_request
    }

    /// Returns the most recent single-server refresh request submitted through this plugin.
    pub fn last_server_refresh_request(&self) -> Option<SteamworksServerListServerIndex> {
        self.last_server_refresh_request
    }

    /// Returns the most recent server-list count read through this plugin.
    pub fn last_server_list_count(&self) -> Option<SteamworksServerListCount> {
        self.last_server_list_count
    }

    /// Returns the most recent server-list refreshing state read through this plugin.
    pub fn last_server_list_refreshing(&self) -> Option<SteamworksServerListRefreshing> {
        self.last_server_list_refreshing
    }

    /// Returns the most recent server response callback context.
    pub fn last_server_response(&self) -> Option<SteamworksServerListServerIndex> {
        self.last_server_response
    }

    /// Returns the most recent server failure callback context.
    pub fn last_server_failure(&self) -> Option<SteamworksServerListServerIndex> {
        self.last_server_failure
    }

    /// Returns the most recent server details read context.
    pub fn last_server_details_read(&self) -> Option<SteamworksServerListServerIndex> {
        self.last_server_details_read
    }

    /// Returns the most recent server snapshot read or received by callback.
    pub fn last_server(&self) -> Option<&SteamworksGameServerItem> {
        self.last_server.as_ref()
    }

    /// Returns the most recent server-list refresh completion response.
    pub fn last_refresh_response(&self) -> Option<SteamworksServerListResponse> {
        self.last_refresh_response
    }

    /// Returns the most recent direct server query submitted through this plugin.
    pub fn last_server_query(&self) -> Option<SteamworksServerQueryInfo> {
        self.last_server_query
    }

    /// Returns the most recent direct server ping response.
    pub fn last_server_ping(&self) -> Option<&SteamworksServerPing> {
        self.last_server_ping.as_ref()
    }

    /// Returns the most recent direct server ping query that failed.
    pub fn last_failed_server_ping(&self) -> Option<SteamworksServerQueryId> {
        self.last_failed_server_ping
    }

    /// Returns the most recent direct server player-details response.
    pub fn last_server_player_details(&self) -> Option<&SteamworksServerPlayerDetails> {
        self.last_server_player_details.as_ref()
    }

    /// Returns the most recent direct server player-details query that failed.
    pub fn last_failed_server_player_details(&self) -> Option<SteamworksServerQueryId> {
        self.last_failed_server_player_details
    }

    /// Returns the most recent direct server-rules response.
    pub fn last_server_rules(&self) -> Option<&SteamworksServerRules> {
        self.last_server_rules.as_ref()
    }

    /// Returns the most recent direct server-rules query that failed.
    pub fn last_failed_server_rules(&self) -> Option<SteamworksServerQueryId> {
        self.last_failed_server_rules
    }

    /// Returns how many server-list requests were submitted.
    pub fn server_list_request_count(&self) -> u64 {
        self.server_list_request_count
    }

    /// Returns how many server-list requests were released.
    pub fn server_list_release_count(&self) -> u64 {
        self.server_list_release_count
    }

    /// Returns how many server-list refresh commands were submitted.
    pub fn server_list_refresh_request_count(&self) -> u64 {
        self.server_list_refresh_request_count
    }

    /// Returns how many single-server refresh commands were submitted.
    pub fn server_refresh_request_count(&self) -> u64 {
        self.server_refresh_request_count
    }

    /// Returns how many server-list count reads were observed.
    pub fn server_list_count_read_count(&self) -> u64 {
        self.server_list_count_read_count
    }

    /// Returns how many server-list refreshing state reads were observed.
    pub fn server_list_refreshing_read_count(&self) -> u64 {
        self.server_list_refreshing_read_count
    }

    /// Returns how many server responded callbacks were observed.
    pub fn server_response_count(&self) -> u64 {
        self.server_response_count
    }

    /// Returns how many server failed callbacks were observed.
    pub fn server_failure_count(&self) -> u64 {
        self.server_failure_count
    }

    /// Returns how many server-list refresh completion callbacks were observed.
    pub fn refresh_complete_count(&self) -> u64 {
        self.refresh_complete_count
    }

    /// Returns how many direct server queries were submitted.
    pub fn server_query_count(&self) -> u64 {
        self.server_query_count
    }

    /// Returns how many direct server ping responses were observed.
    pub fn server_ping_response_count(&self) -> u64 {
        self.server_ping_response_count
    }

    /// Returns how many direct server ping failures were observed.
    pub fn server_ping_failure_count(&self) -> u64 {
        self.server_ping_failure_count
    }

    /// Returns how many direct server player-details responses were observed.
    pub fn server_player_details_count(&self) -> u64 {
        self.server_player_details_count
    }

    /// Returns how many direct server player-details failures were observed.
    pub fn server_player_details_failure_count(&self) -> u64 {
        self.server_player_details_failure_count
    }

    /// Returns how many direct server-rules responses were observed.
    pub fn server_rules_count(&self) -> u64 {
        self.server_rules_count
    }

    /// Returns how many direct server-rules failures were observed.
    pub fn server_rules_failure_count(&self) -> u64 {
        self.server_rules_failure_count
    }
}
