use bevy_ecs::message::Message;

use super::{SteamworksAppsCommand, SteamworksAppsError, SteamworksAppsOperation};

/// Result message emitted by [`crate::SteamworksAppsPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksAppsResult {
    /// The command or observed callback was processed successfully.
    Ok(SteamworksAppsOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksAppsCommand,
        /// Failure reason.
        error: SteamworksAppsError,
    },
}
