use bevy_ecs::message::Message;

use super::{SteamworksUtilsCommand, SteamworksUtilsError, SteamworksUtilsOperation};

/// Result message emitted by [`crate::SteamworksUtilsPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksUtilsResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksUtilsOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksUtilsCommand,
        /// Failure reason.
        error: SteamworksUtilsError,
    },
}
