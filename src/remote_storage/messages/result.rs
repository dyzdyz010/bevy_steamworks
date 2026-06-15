use bevy_ecs::message::Message;

use super::{
    SteamworksRemoteStorageCommand, SteamworksRemoteStorageError, SteamworksRemoteStorageOperation,
};

/// Result message emitted by [`crate::SteamworksRemoteStoragePlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksRemoteStorageResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksRemoteStorageOperation),
    /// The command failed synchronously or through a Steam async call result.
    Err {
        /// Command that failed.
        command: SteamworksRemoteStorageCommand,
        /// Failure reason.
        error: SteamworksRemoteStorageError,
    },
}

crate::result_ext::impl_steamworks_result_helpers!(
    SteamworksRemoteStorageResult,
    SteamworksRemoteStorageOperation,
    SteamworksRemoteStorageCommand,
    SteamworksRemoteStorageError
);
