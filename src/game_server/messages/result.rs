use bevy_ecs::message::Message;

use super::{
    command::SteamworksServerCommand, error::SteamworksServerError,
    operation::SteamworksServerOperation,
};

/// Result message emitted by [`crate::SteamworksServerPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksServerResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksServerOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksServerCommand,
        /// Failure reason.
        error: SteamworksServerError,
    },
}
