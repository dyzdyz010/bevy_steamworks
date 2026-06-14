use bevy_ecs::prelude::Resource;

use super::{
    messages::{SteamworksTimelineError, SteamworksTimelineOperation},
    types::{
        SteamworksTimelineEventInfo, SteamworksTimelineGameMode, SteamworksTimelineStateDescription,
    },
};

/// Runtime state for [`super::SteamworksTimelinePlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksTimelineState {
    last_error: Option<SteamworksTimelineError>,
    game_mode: Option<SteamworksTimelineGameMode>,
    state_description: Option<SteamworksTimelineStateDescription>,
    last_event: Option<SteamworksTimelineEventInfo>,
    event_count: u64,
}

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

    #[cfg(test)]
    pub(super) fn set_event_count_for_test(&mut self, event_count: u64) {
        self.event_count = event_count;
    }

    pub(super) fn record_error(&mut self, error: SteamworksTimelineError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksTimelineOperation) {
        match operation {
            SteamworksTimelineOperation::GameModeSet { mode } => {
                self.game_mode = Some(*mode);
            }
            SteamworksTimelineOperation::StateDescriptionSet {
                description,
                duration,
            } => {
                self.state_description = Some(SteamworksTimelineStateDescription {
                    description: description.clone(),
                    duration: *duration,
                });
            }
            SteamworksTimelineOperation::StateDescriptionCleared { .. } => {
                self.state_description = None;
            }
            SteamworksTimelineOperation::TimelineEventAdded { event } => {
                self.last_event = Some(event.clone());
                self.event_count = self.event_count.saturating_add(1);
            }
        }
    }
}
