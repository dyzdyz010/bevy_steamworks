use bevy_ecs::message::Message;

use super::{
    SteamworksRemotePlayCommand, SteamworksRemotePlayError, SteamworksRemotePlayOperation,
};

/// Result message emitted by [`crate::SteamworksRemotePlayPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksRemotePlayResult {
    /// The command or observed callback was processed successfully.
    Ok(SteamworksRemotePlayOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksRemotePlayCommand,
        /// Failure reason.
        error: SteamworksRemotePlayError,
    },
}
