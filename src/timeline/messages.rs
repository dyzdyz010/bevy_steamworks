use std::time::Duration;

use bevy_ecs::message::Message;
use thiserror::Error;

use super::types::{SteamworksTimelineEventInfo, SteamworksTimelineGameMode};

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
    /// Creates a [`SteamworksTimelineCommand::SetGameMode`] command.
    pub fn set_game_mode(mode: SteamworksTimelineGameMode) -> Self {
        Self::SetGameMode { mode }
    }

    /// Creates a [`SteamworksTimelineCommand::SetStateDescription`] command.
    pub fn set_state_description(description: impl Into<String>, duration: Duration) -> Self {
        Self::SetStateDescription {
            description: description.into(),
            duration,
        }
    }

    /// Creates a [`SteamworksTimelineCommand::ClearStateDescription`] command.
    pub fn clear_state_description(duration: Duration) -> Self {
        Self::ClearStateDescription { duration }
    }

    /// Creates a [`SteamworksTimelineCommand::AddEvent`] command.
    pub fn add_event(event: SteamworksTimelineEventInfo) -> Self {
        Self::AddEvent { event }
    }
}

/// A Steam Timeline operation accepted by this command layer.
///
/// The operation has been submitted to the upstream `steamworks` wrapper. Steam
/// may still no-op the request when the runtime Timeline interface is
/// unavailable.
#[derive(Clone, Debug, PartialEq)]
pub enum SteamworksTimelineOperation {
    /// Timeline game mode was submitted.
    GameModeSet {
        /// Game mode submitted to Steam.
        mode: SteamworksTimelineGameMode,
    },
    /// Timeline state description was submitted.
    StateDescriptionSet {
        /// Tooltip text.
        description: String,
        /// Duration over which Steam should apply the change.
        duration: Duration,
    },
    /// Timeline state description clear was submitted.
    StateDescriptionCleared {
        /// Duration over which Steam should apply the change.
        duration: Duration,
    },
    /// Timeline event was submitted.
    TimelineEventAdded {
        /// Event details submitted to Steam.
        event: SteamworksTimelineEventInfo,
    },
}

/// Result message emitted by [`super::SteamworksTimelinePlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksTimelineResult {
    /// The command was accepted and submitted to the upstream wrapper.
    ///
    /// This does not guarantee that Steam applied the request; the upstream
    /// Timeline wrapper no-ops when the runtime interface is unavailable.
    Ok(SteamworksTimelineOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksTimelineCommand,
        /// Failure reason.
        error: SteamworksTimelineError,
    },
}

/// Synchronous errors from [`super::SteamworksTimelinePlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksTimelineError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks Timeline command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A floating-point value is not finite.
    #[error("Steamworks Timeline command field {field} must be finite")]
    InvalidFloat {
        /// Field whose value was invalid.
        field: &'static str,
    },
}

impl SteamworksTimelineError {
    pub(super) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    pub(super) fn invalid_float(field: &'static str) -> Self {
        Self::InvalidFloat { field }
    }
}
