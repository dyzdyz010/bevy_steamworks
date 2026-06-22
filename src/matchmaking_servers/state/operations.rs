use super::{
    push_bounded, upsert_by, SteamworksCachedServerListServer, SteamworksMatchmakingServersState,
};
use crate::matchmaking_servers::{
    SteamworksMatchmakingServersError, SteamworksMatchmakingServersOperation,
    SteamworksServerListCount, SteamworksServerListRefreshing, SteamworksServerListRequestId,
    SteamworksServerListRequestInfo, SteamworksServerListServerIndex, SteamworksServerQueryId,
};

impl SteamworksMatchmakingServersState {
    pub(in crate::matchmaking_servers) fn record_error(
        &mut self,
        error: SteamworksMatchmakingServersError,
    ) {
        self.last_error = Some(error);
    }

    pub(in crate::matchmaking_servers) fn record_operation(
        &mut self,
        operation: &SteamworksMatchmakingServersOperation,
    ) {
        match operation {
            SteamworksMatchmakingServersOperation::ServerQuerySubmitted { query } => {
                upsert_by(&mut self.server_queries, query.query, *query, |cached| {
                    cached.query
                });
                self.last_server_query = Some(*query);
                self.server_query_count = self.server_query_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerPingResponded { ping } => {
                self.last_server = Some(ping.server.clone());
                upsert_by(&mut self.server_pings, ping.query, ping.clone(), |cached| {
                    cached.query
                });
                self.last_server_ping = Some(ping.clone());
                self.server_ping_response_count = self.server_ping_response_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerPingFailed { query } => {
                push_bounded(&mut self.failed_server_pings, *query);
                self.last_failed_server_ping = Some(*query);
                self.server_ping_failure_count = self.server_ping_failure_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerPlayerDetailsReceived { details } => {
                upsert_by(
                    &mut self.server_player_details,
                    details.query,
                    details.clone(),
                    |cached| cached.query,
                );
                self.last_server_player_details = Some(details.clone());
                self.server_player_details_count =
                    self.server_player_details_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerPlayerDetailsFailed { query } => {
                push_bounded(&mut self.failed_server_player_details, *query);
                self.last_failed_server_player_details = Some(*query);
                self.server_player_details_failure_count =
                    self.server_player_details_failure_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerRulesReceived { rules } => {
                upsert_by(
                    &mut self.server_rules,
                    rules.query,
                    rules.clone(),
                    |cached| cached.query,
                );
                self.last_server_rules = Some(rules.clone());
                self.server_rules_count = self.server_rules_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerRulesFailed { query } => {
                push_bounded(&mut self.failed_server_rules, *query);
                self.last_failed_server_rules = Some(*query);
                self.server_rules_failure_count = self.server_rules_failure_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerListRequested {
                request,
                app_id,
                kind,
                filters,
            } => {
                let info = SteamworksServerListRequestInfo {
                    request: *request,
                    app_id: *app_id,
                    kind: *kind,
                    filters: filters.clone(),
                };
                upsert_by(
                    &mut self.server_list_requests,
                    *request,
                    info.clone(),
                    |cached| cached.request,
                );
                self.last_server_list_request = Some(info);
                self.server_list_request_count = self.server_list_request_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerResponded {
                request,
                server_index,
                server,
            } => {
                self.last_server = Some(server.clone());
                let index = SteamworksServerListServerIndex {
                    request: *request,
                    server_index: *server_index,
                };
                upsert_by(
                    &mut self.servers,
                    index,
                    SteamworksCachedServerListServer {
                        index,
                        server: server.clone(),
                    },
                    |cached| cached.index,
                );
                self.last_server_response = Some(index);
                self.server_response_count = self.server_response_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerFailedToRespond {
                request,
                server_index,
            } => {
                self.last_server_failure = Some(SteamworksServerListServerIndex {
                    request: *request,
                    server_index: *server_index,
                });
                self.server_failure_count = self.server_failure_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerListRefreshCompleted {
                request,
                response,
                ..
            } => {
                self.last_refresh_completion_request = Some(*request);
                self.last_refresh_response = Some(*response);
                self.refresh_complete_count = self.refresh_complete_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerListRefreshSubmitted { request } => {
                self.last_server_list_refresh_request = Some(*request);
                self.server_list_refresh_request_count =
                    self.server_list_refresh_request_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerRefreshSubmitted {
                request,
                server_index,
            } => {
                self.last_server_refresh_request = Some(SteamworksServerListServerIndex {
                    request: *request,
                    server_index: *server_index,
                });
                self.server_refresh_request_count =
                    self.server_refresh_request_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerListCountRead { request, count } => {
                let count = SteamworksServerListCount {
                    request: *request,
                    count: *count,
                };
                upsert_by(&mut self.server_list_counts, *request, count, |cached| {
                    cached.request
                });
                self.last_server_list_count = Some(count);
                self.server_list_count_read_count =
                    self.server_list_count_read_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerDetailsRead {
                request,
                server_index,
                server,
            } => {
                let index = SteamworksServerListServerIndex {
                    request: *request,
                    server_index: *server_index,
                };
                upsert_by(
                    &mut self.servers,
                    index,
                    SteamworksCachedServerListServer {
                        index,
                        server: server.clone(),
                    },
                    |cached| cached.index,
                );
                self.last_server_details_read = Some(index);
                self.last_server = Some(server.clone());
            }
            SteamworksMatchmakingServersOperation::ServerListRefreshingRead {
                request,
                refreshing,
            } => {
                let refreshing = SteamworksServerListRefreshing {
                    request: *request,
                    refreshing: *refreshing,
                };
                upsert_by(
                    &mut self.server_list_refreshing_states,
                    *request,
                    refreshing,
                    |cached| cached.request,
                );
                self.last_server_list_refreshing = Some(refreshing);
                self.server_list_refreshing_read_count =
                    self.server_list_refreshing_read_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerListReleased { request } => {
                self.server_list_requests
                    .retain(|info| info.request != *request);
                self.server_list_counts
                    .retain(|count| count.request != *request);
                self.server_list_refreshing_states
                    .retain(|refreshing| refreshing.request != *request);
                self.servers
                    .retain(|server| server.index.request != *request);
                self.last_released_server_list_request = Some(*request);
                self.server_list_release_count = self.server_list_release_count.saturating_add(1);
            }
        }
    }

    pub(in crate::matchmaking_servers) fn sync_request_count(
        &mut self,
        active_server_list_requests: usize,
    ) {
        self.active_server_list_requests = active_server_list_requests;
    }

    pub(in crate::matchmaking_servers) fn next_request_id(
        &mut self,
    ) -> SteamworksServerListRequestId {
        let request_id = self.next_request_id;
        self.next_request_id = self.next_request_id.wrapping_add(1);
        SteamworksServerListRequestId::from_raw(request_id)
    }

    pub(in crate::matchmaking_servers) fn next_query_id(&mut self) -> SteamworksServerQueryId {
        let query_id = self.next_query_id;
        self.next_query_id = self.next_query_id.wrapping_add(1);
        SteamworksServerQueryId::from_raw(query_id)
    }
}
