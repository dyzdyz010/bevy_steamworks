use crate::matchmaking_servers::*;

impl SteamworksMatchmakingServersState {
    /// Returns the most recent synchronous or callback error observed by the plugin.
    pub fn last_error(&self) -> Option<&SteamworksMatchmakingServersError> {
        self.last_error.as_ref()
    }

    /// Returns the number of active server-list request handles owned by the plugin.
    pub fn active_server_list_requests(&self) -> usize {
        self.active_server_list_requests
    }
}
