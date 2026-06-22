use super::SteamworksMatchmakingServersState;
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
                self.last_server_query = Some(*query);
                self.server_query_count = self.server_query_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerPingResponded { ping } => {
                self.last_server = Some(ping.server.clone());
                self.last_server_ping = Some(ping.clone());
                self.server_ping_response_count = self.server_ping_response_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerPingFailed { query } => {
                self.last_failed_server_ping = Some(*query);
                self.server_ping_failure_count = self.server_ping_failure_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerPlayerDetailsReceived { details } => {
                self.last_server_player_details = Some(details.clone());
                self.server_player_details_count =
                    self.server_player_details_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerPlayerDetailsFailed { query } => {
                self.last_failed_server_player_details = Some(*query);
                self.server_player_details_failure_count =
                    self.server_player_details_failure_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerRulesReceived { rules } => {
                self.last_server_rules = Some(rules.clone());
                self.server_rules_count = self.server_rules_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerRulesFailed { query } => {
                self.last_failed_server_rules = Some(*query);
                self.server_rules_failure_count = self.server_rules_failure_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerListRequested {
                request,
                app_id,
                kind,
                filters,
            } => {
                self.last_server_list_request = Some(SteamworksServerListRequestInfo {
                    request: *request,
                    app_id: *app_id,
                    kind: *kind,
                    filters: filters.clone(),
                });
                self.server_list_request_count = self.server_list_request_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerResponded {
                request,
                server_index,
                server,
            } => {
                self.last_server = Some(server.clone());
                self.last_server_response = Some(SteamworksServerListServerIndex {
                    request: *request,
                    server_index: *server_index,
                });
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
                self.last_server_list_count = Some(SteamworksServerListCount {
                    request: *request,
                    count: *count,
                });
                self.server_list_count_read_count =
                    self.server_list_count_read_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerDetailsRead {
                request,
                server_index,
                server,
            } => {
                self.last_server_details_read = Some(SteamworksServerListServerIndex {
                    request: *request,
                    server_index: *server_index,
                });
                self.last_server = Some(server.clone());
            }
            SteamworksMatchmakingServersOperation::ServerListRefreshingRead {
                request,
                refreshing,
            } => {
                self.last_server_list_refreshing = Some(SteamworksServerListRefreshing {
                    request: *request,
                    refreshing: *refreshing,
                });
                self.server_list_refreshing_read_count =
                    self.server_list_refreshing_read_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerListReleased { request } => {
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
