use super::super::{SteamworksStatsError, SteamworksStatsState};

impl SteamworksStatsState {
    /// Returns whether this plugin has requested stats for the current user.
    pub fn current_user_stats_requested(&self) -> bool {
        self.current_user_stats_requested
    }

    /// Returns whether successful writes are waiting for `store_stats`.
    pub fn pending_store(&self) -> bool {
        self.pending_store
    }

    /// Returns the most recent command or asynchronous callback error observed by the stats plugin.
    pub fn last_error(&self) -> Option<&SteamworksStatsError> {
        self.last_error.as_ref()
    }
}
