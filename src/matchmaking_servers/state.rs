use bevy_ecs::prelude::Resource;

use super::*;

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
    server_list_request_count: u64,
    server_list_release_count: u64,
    server_list_refresh_request_count: u64,
    server_refresh_request_count: u64,
    server_list_count_read_count: u64,
    server_list_refreshing_read_count: u64,
    server_response_count: u64,
    server_failure_count: u64,
    refresh_complete_count: u64,
    next_request_id: u64,
}

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

    pub(super) fn record_error(&mut self, error: SteamworksMatchmakingServersError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksMatchmakingServersOperation) {
        match operation {
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

    pub(super) fn sync_request_count(&mut self, active_server_list_requests: usize) {
        self.active_server_list_requests = active_server_list_requests;
    }

    pub(super) fn next_request_id(&mut self) -> SteamworksServerListRequestId {
        let request_id = self.next_request_id;
        self.next_request_id = self.next_request_id.wrapping_add(1);
        SteamworksServerListRequestId::from_raw(request_id)
    }
}
