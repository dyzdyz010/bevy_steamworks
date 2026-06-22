use super::SteamworksTimelineState;
use crate::timeline::{
    SteamworksTimelineError, SteamworksTimelineEventInfo, SteamworksTimelineGameMode,
    SteamworksTimelineStateDescription,
};

impl SteamworksTimelineState {
    /// Returns the most recent synchronous error observed by the Timeline plugin.
    pub fn last_error(&self) -> Option<&SteamworksTimelineError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent Timeline game mode submitted through the plugin.
    pub fn game_mode(&self) -> Option<SteamworksTimelineGameMode> {
        self.game_mode
    }

    /// Returns the current Timeline state description tracked by the plugin.
    pub fn state_description(&self) -> Option<&SteamworksTimelineStateDescription> {
        self.state_description.as_ref()
    }

    /// Returns the most recent Timeline event submitted through the plugin.
    pub fn last_event(&self) -> Option<&SteamworksTimelineEventInfo> {
        self.last_event.as_ref()
    }

    /// Returns the number of Timeline events submitted through the plugin.
    pub fn event_count(&self) -> u64 {
        self.event_count
    }
}
