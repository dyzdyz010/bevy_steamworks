use bevy_ecs::message::Message;

use super::{SteamworksTimelineCommand, SteamworksTimelineError, SteamworksTimelineOperation};

/// Result message emitted by [`crate::SteamworksTimelinePlugin`].
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
