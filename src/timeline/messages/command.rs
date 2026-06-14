use std::time::Duration;

use bevy_ecs::message::Message;

use super::super::{SteamworksTimelineEventInfo, SteamworksTimelineGameMode};

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

impl SteamworksTimelineCommand {
    /// Creates a [`crate::SteamworksTimelineCommand::SetGameMode`] command.
    pub fn set_game_mode(mode: SteamworksTimelineGameMode) -> Self {
        Self::SetGameMode { mode }
    }

    /// Creates a [`crate::SteamworksTimelineCommand::SetStateDescription`] command.
    pub fn set_state_description(description: impl Into<String>, duration: Duration) -> Self {
        Self::SetStateDescription {
            description: description.into(),
            duration,
        }
    }

    /// Creates a [`crate::SteamworksTimelineCommand::ClearStateDescription`] command.
    pub fn clear_state_description(duration: Duration) -> Self {
        Self::ClearStateDescription { duration }
    }

    /// Creates a [`crate::SteamworksTimelineCommand::AddEvent`] command.
    pub fn add_event(event: SteamworksTimelineEventInfo) -> Self {
        Self::AddEvent { event }
    }
}
