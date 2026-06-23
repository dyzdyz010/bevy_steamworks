use super::SteamworksTimelineState;
use crate::timeline::{
    SteamworksTimelineError, SteamworksTimelineEventClipPriority, SteamworksTimelineEventInfo,
    SteamworksTimelineGameMode, SteamworksTimelineStateDescription,
};
use std::time::Duration;

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

    /// Returns the current Timeline state description text tracked by the plugin.
    pub fn state_description_text(&self) -> Option<&str> {
        self.state_description()
            .map(|description| description.description.as_str())
    }

    /// Returns the current Timeline state description duration tracked by the plugin.
    pub fn state_description_duration(&self) -> Option<Duration> {
        self.state_description()
            .map(|description| description.duration)
    }

    /// Returns bounded Timeline event snapshots submitted through the plugin.
    pub fn events(&self) -> &[SteamworksTimelineEventInfo] {
        &self.events
    }

    /// Returns the most recent Timeline event submitted through the plugin.
    pub fn last_event(&self) -> Option<&SteamworksTimelineEventInfo> {
        self.last_event.as_ref()
    }

    /// Returns the most recent Timeline event with the given icon.
    pub fn last_event_with_icon(
        &self,
        icon: impl AsRef<str>,
    ) -> Option<&SteamworksTimelineEventInfo> {
        let icon = icon.as_ref();
        self.events.iter().rev().find(|event| event.icon == icon)
    }

    /// Returns the most recent Timeline event icon.
    pub fn last_event_icon(&self) -> Option<&str> {
        self.last_event().map(|event| event.icon.as_str())
    }

    /// Returns the most recent Timeline event title.
    pub fn last_event_title(&self) -> Option<&str> {
        self.last_event().map(|event| event.title.as_str())
    }

    /// Returns the most recent Timeline event description.
    pub fn last_event_description(&self) -> Option<&str> {
        self.last_event().map(|event| event.description.as_str())
    }

    /// Returns the most recent Timeline event priority.
    pub fn last_event_priority(&self) -> Option<u32> {
        self.last_event().map(|event| event.priority)
    }

    /// Returns the most recent Timeline event start offset in seconds.
    pub fn last_event_start_offset_seconds(&self) -> Option<f32> {
        self.last_event().map(|event| event.start_offset_seconds)
    }

    /// Returns the most recent Timeline event duration.
    pub fn last_event_duration(&self) -> Option<Duration> {
        self.last_event().map(|event| event.duration)
    }

    /// Returns the most recent Timeline event clip priority.
    pub fn last_event_clip_priority(&self) -> Option<SteamworksTimelineEventClipPriority> {
        self.last_event().map(|event| event.clip_priority)
    }

    /// Returns the number of Timeline events submitted through the plugin.
    pub fn event_count(&self) -> u64 {
        self.event_count
    }
}
