use bevy_ecs::message::Message;

use super::{
    SteamworksMatchmakingCommand, SteamworksMatchmakingError, SteamworksMatchmakingOperation,
};

/// Result message emitted by [`crate::SteamworksMatchmakingPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksMatchmakingResult {
    /// The command, async call result, or observed callback was processed successfully.
    Ok(SteamworksMatchmakingOperation),
    /// The command failed synchronously or through a Steam async call result.
    Err {
        /// Command that failed.
        command: SteamworksMatchmakingCommand,
        /// Failure reason.
        error: SteamworksMatchmakingError,
    },
}

crate::result_ext::impl_steamworks_result_helpers!(
    SteamworksMatchmakingResult,
    SteamworksMatchmakingOperation,
    SteamworksMatchmakingCommand,
    SteamworksMatchmakingError
);
