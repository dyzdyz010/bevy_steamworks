use bevy_ecs::message::Message;

use super::{
    SteamworksNetworkingMessagesCommand, SteamworksNetworkingMessagesError,
    SteamworksNetworkingMessagesOperation,
};

/// Result message emitted by [`crate::SteamworksNetworkingMessagesPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksNetworkingMessagesResult {
    /// The command, receive operation, or callback succeeded.
    Ok(SteamworksNetworkingMessagesOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksNetworkingMessagesCommand,
        /// Failure reason.
        error: SteamworksNetworkingMessagesError,
    },
}
