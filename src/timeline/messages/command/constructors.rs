use std::time::Duration;

use super::super::super::{SteamworksTimelineEventInfo, SteamworksTimelineGameMode};
use super::SteamworksTimelineCommand;

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
