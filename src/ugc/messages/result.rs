use bevy_ecs::message::Message;

use super::{
    command::SteamworksUgcCommand, error::SteamworksUgcError, operation::SteamworksUgcOperation,
};

/// Result message emitted by [`crate::SteamworksUgcPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksUgcResult {
    /// The command was submitted to Steamworks, completed, or read synchronously.
    Ok(SteamworksUgcOperation),
    /// The command failed synchronously or through an async Steam call result.
    Err {
        /// Command that failed.
        command: SteamworksUgcCommand,
        /// Failure reason.
        error: SteamworksUgcError,
    },
}
