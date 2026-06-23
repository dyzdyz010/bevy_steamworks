use std::time::Duration;

use super::SteamworksMatchmakingServersState;
use crate::matchmaking_servers::{
    SteamworksGameServerItem, SteamworksMatchmakingServersError, SteamworksServerListCount,
    SteamworksServerListRefreshing, SteamworksServerListRequestId, SteamworksServerListRequestInfo,
    SteamworksServerListResponse, SteamworksServerListServerIndex, SteamworksServerPing,
    SteamworksServerPlayerDetails, SteamworksServerPlayerInfo, SteamworksServerQueryId,
    SteamworksServerQueryInfo, SteamworksServerQueryKind, SteamworksServerQueryTarget,
    SteamworksServerRule, SteamworksServerRules,
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

    /// Returns a cached server-list request by ID, if it is still owned by this plugin.
    pub fn server_list_request(
        &self,
        request: SteamworksServerListRequestId,
    ) -> Option<&SteamworksServerListRequestInfo> {
        self.server_list_requests
            .iter()
            .find(|info| info.request == request)
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

    /// Returns the latest cached server count for one server-list request.
    pub fn server_list_count(&self, request: SteamworksServerListRequestId) -> Option<i32> {
        self.server_list_counts
            .iter()
            .find(|count| count.request == request)
            .map(|count| count.count)
    }

    /// Returns the most recent server-list refreshing state read through this plugin.
    pub fn last_server_list_refreshing(&self) -> Option<SteamworksServerListRefreshing> {
        self.last_server_list_refreshing
    }

    /// Returns the latest cached refreshing state for one server-list request.
    pub fn server_list_refreshing(&self, request: SteamworksServerListRequestId) -> Option<bool> {
        self.server_list_refreshing_states
            .iter()
            .find(|refreshing| refreshing.request == request)
            .map(|refreshing| refreshing.refreshing)
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

    /// Returns a cached server snapshot by server-list request and server index.
    pub fn server(
        &self,
        request: SteamworksServerListRequestId,
        server_index: i32,
    ) -> Option<&SteamworksGameServerItem> {
        self.servers
            .iter()
            .find(|server| {
                server.index.request == request && server.index.server_index == server_index
            })
            .map(|server| &server.server)
    }

    /// Returns the most recent server-list refresh completion response.
    pub fn last_refresh_response(&self) -> Option<SteamworksServerListResponse> {
        self.last_refresh_response
    }

    /// Returns the most recent direct server query submitted through this plugin.
    pub fn last_server_query(&self) -> Option<SteamworksServerQueryInfo> {
        self.last_server_query
    }

    /// Returns a cached direct server query context by query ID.
    pub fn server_query(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<SteamworksServerQueryInfo> {
        self.server_queries
            .iter()
            .find(|info| info.query == query)
            .copied()
    }

    /// Returns the target endpoint for a cached direct server query.
    pub fn server_query_target(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<SteamworksServerQueryTarget> {
        self.server_query(query).map(|info| info.target)
    }

    /// Returns the kind of a cached direct server query.
    pub fn server_query_kind(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<SteamworksServerQueryKind> {
        self.server_query(query).map(|info| info.kind)
    }

    /// Returns the most recent direct server ping response.
    pub fn last_server_ping(&self) -> Option<&SteamworksServerPing> {
        self.last_server_ping.as_ref()
    }

    /// Returns a cached direct server ping response by query ID.
    pub fn server_ping(&self, query: SteamworksServerQueryId) -> Option<&SteamworksServerPing> {
        self.server_pings.iter().find(|ping| ping.query == query)
    }

    /// Returns the target endpoint from a cached direct ping response.
    pub fn server_ping_target(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<SteamworksServerQueryTarget> {
        self.server_ping(query).map(|ping| ping.target)
    }

    /// Returns the server item from a cached direct ping response.
    pub fn server_ping_server(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<&SteamworksGameServerItem> {
        self.server_ping(query).map(|ping| &ping.server)
    }

    /// Returns the ping duration from a cached direct ping response.
    pub fn server_ping_latency(&self, query: SteamworksServerQueryId) -> Option<Duration> {
        self.server_ping_server(query).map(|server| server.ping)
    }

    /// Returns the server name from a cached direct ping response.
    pub fn server_ping_server_name(&self, query: SteamworksServerQueryId) -> Option<&str> {
        self.server_ping_server(query)
            .map(|server| server.server_name.as_str())
    }

    /// Returns the map name from a cached direct ping response.
    pub fn server_ping_map(&self, query: SteamworksServerQueryId) -> Option<&str> {
        self.server_ping_server(query)
            .map(|server| server.map.as_str())
    }

    /// Returns the current player count from a cached direct ping response.
    pub fn server_ping_player_count(&self, query: SteamworksServerQueryId) -> Option<i32> {
        self.server_ping_server(query).map(|server| server.players)
    }

    /// Returns the maximum player count from a cached direct ping response.
    pub fn server_ping_max_players(&self, query: SteamworksServerQueryId) -> Option<i32> {
        self.server_ping_server(query)
            .map(|server| server.max_players)
    }

    /// Returns the most recent direct server ping query that failed.
    pub fn last_failed_server_ping(&self) -> Option<SteamworksServerQueryId> {
        self.last_failed_server_ping
    }

    /// Returns whether a direct server ping query has failed.
    pub fn server_ping_failed(&self, query: SteamworksServerQueryId) -> bool {
        self.failed_server_pings.contains(&query)
    }

    /// Returns the most recent direct server player-details response.
    pub fn last_server_player_details(&self) -> Option<&SteamworksServerPlayerDetails> {
        self.last_server_player_details.as_ref()
    }

    /// Returns cached direct server player details by query ID.
    pub fn server_player_details(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<&SteamworksServerPlayerDetails> {
        self.server_player_details
            .iter()
            .find(|details| details.query == query)
    }

    /// Returns the target endpoint from cached direct player details.
    pub fn server_player_details_target(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<SteamworksServerQueryTarget> {
        self.server_player_details(query)
            .map(|details| details.target)
    }

    /// Returns cached direct player rows for one query.
    pub fn server_players(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<&[SteamworksServerPlayerInfo]> {
        self.server_player_details(query)
            .map(|details| details.players.as_slice())
    }

    /// Returns the number of cached direct player rows for one query.
    pub fn server_player_count(&self, query: SteamworksServerQueryId) -> Option<usize> {
        self.server_players(query).map(|players| players.len())
    }

    /// Returns the number of player rows in the most recent direct player-details response.
    pub fn last_server_player_count(&self) -> Option<usize> {
        self.last_server_player_details
            .as_ref()
            .map(|details| details.players.len())
    }

    /// Returns a cached direct player row by player name.
    pub fn server_player(
        &self,
        query: SteamworksServerQueryId,
        name: &str,
    ) -> Option<&SteamworksServerPlayerInfo> {
        self.server_players(query)
            .and_then(|players| players.iter().find(|player| player.name == name))
    }

    /// Returns whether cached direct player details contain a player name.
    pub fn server_has_player(&self, query: SteamworksServerQueryId, name: &str) -> Option<bool> {
        self.server_players(query)
            .map(|players| players.iter().any(|player| player.name == name))
    }

    /// Returns a cached direct player score by player name.
    pub fn server_player_score(&self, query: SteamworksServerQueryId, name: &str) -> Option<i32> {
        self.server_player(query, name).map(|player| player.score)
    }

    /// Returns cached direct player time-played by player name.
    pub fn server_player_time_played(
        &self,
        query: SteamworksServerQueryId,
        name: &str,
    ) -> Option<Duration> {
        self.server_player(query, name)
            .map(|player| player.time_played)
    }

    /// Returns the most recent direct server player-details query that failed.
    pub fn last_failed_server_player_details(&self) -> Option<SteamworksServerQueryId> {
        self.last_failed_server_player_details
    }

    /// Returns whether a direct player-details query has failed.
    pub fn server_player_details_failed(&self, query: SteamworksServerQueryId) -> bool {
        self.failed_server_player_details.contains(&query)
    }

    /// Returns the most recent direct server-rules response.
    pub fn last_server_rules(&self) -> Option<&SteamworksServerRules> {
        self.last_server_rules.as_ref()
    }

    /// Returns cached direct server rules by query ID.
    pub fn server_rules(&self, query: SteamworksServerQueryId) -> Option<&SteamworksServerRules> {
        self.server_rules.iter().find(|rules| rules.query == query)
    }

    /// Returns the target endpoint from cached direct server rules.
    pub fn server_rules_target(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<SteamworksServerQueryTarget> {
        self.server_rules(query).map(|rules| rules.target)
    }

    /// Returns cached direct server rule rows for one query.
    pub fn server_rule_entries(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<&[SteamworksServerRule]> {
        self.server_rules(query).map(|rules| rules.rules.as_slice())
    }

    /// Returns the number of cached direct server rule rows for one query.
    pub fn server_rule_count(&self, query: SteamworksServerQueryId) -> Option<usize> {
        self.server_rule_entries(query).map(|rules| rules.len())
    }

    /// Returns the number of rule rows in the most recent direct server-rules response.
    pub fn last_server_rule_count(&self) -> Option<usize> {
        self.last_server_rules
            .as_ref()
            .map(|rules| rules.rules.len())
    }

    /// Returns one cached direct server rule, preserving a cached query without that key as `Some(None)`.
    pub fn server_rule(&self, query: SteamworksServerQueryId, key: &str) -> Option<Option<&str>> {
        self.server_rule_entries(query).map(|rules| {
            rules
                .iter()
                .find(|rule| rule.key == key)
                .map(|rule| rule.value.as_str())
        })
    }

    /// Returns whether cached direct server rules contain a key.
    pub fn server_has_rule(&self, query: SteamworksServerQueryId, key: &str) -> Option<bool> {
        self.server_rule_entries(query)
            .map(|rules| rules.iter().any(|rule| rule.key == key))
    }

    /// Returns the most recent direct server-rules query that failed.
    pub fn last_failed_server_rules(&self) -> Option<SteamworksServerQueryId> {
        self.last_failed_server_rules
    }

    /// Returns whether a direct server-rules query has failed.
    pub fn server_rules_failed(&self, query: SteamworksServerQueryId) -> bool {
        self.failed_server_rules.contains(&query)
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
