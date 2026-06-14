use bevy_ecs::message::Message;

use super::{
    SteamworksMatchmakingServersCommand, SteamworksMatchmakingServersError,
    SteamworksMatchmakingServersOperation,
};

/// Result message emitted by [`crate::SteamworksMatchmakingServersPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksMatchmakingServersResult {
    /// The command was submitted to Steamworks, a value was read, or a callback was observed.
    Ok(SteamworksMatchmakingServersOperation),
    /// The command failed synchronously or callback processing failed.
    Err {
        /// Command that failed.
        command: SteamworksMatchmakingServersCommand,
        /// Failure reason.
        error: SteamworksMatchmakingServersError,
    },
}
