use crate::matchmaking_servers::*;

impl SteamworksMatchmakingServersState {
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
}
