use std::time::Duration;

use bevy_ecs::message::Message;

use super::super::{SteamworksTimelineEventInfo, SteamworksTimelineGameMode};

mod constructors;

/// A high-level command for Steam Timeline workflows.
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksTimelineCommand {
    /// Set the current Timeline game mode.
    SetGameMode {
        /// Game mode to submit to Steam.
        mode: SteamworksTimelineGameMode,
    },
    /// Set a Timeline state description tooltip.
    SetStateDescription {
        /// Tooltip text.
        description: String,
        /// Duration over which Steam should apply the change.
        duration: Duration,
    },
    /// Clear the current Timeline state description tooltip.
    ClearStateDescription {
        /// Duration over which Steam should apply the change.
        duration: Duration,
    },
    /// Add an event marker to the Steam Timeline.
    AddEvent {
        /// Event details.
        event: SteamworksTimelineEventInfo,
    },
}
