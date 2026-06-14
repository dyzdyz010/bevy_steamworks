use bevy_ecs::message::Message;

use super::{
    SteamworksNetworkingCommand, SteamworksNetworkingError, SteamworksNetworkingOperation,
};

/// Result message emitted by [`crate::SteamworksNetworkingPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksNetworkingResult {
    /// The command, read operation, or callback succeeded.
    Ok(SteamworksNetworkingOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksNetworkingCommand,
        /// Failure reason.
        error: SteamworksNetworkingError,
    },
}
