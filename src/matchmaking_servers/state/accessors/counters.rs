use crate::matchmaking_servers::*;

impl SteamworksMatchmakingServersState {
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
