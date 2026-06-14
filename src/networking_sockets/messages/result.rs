use bevy_ecs::message::Message;

use super::{
    command::SteamworksNetworkingSocketsCommand, error::SteamworksNetworkingSocketsError,
    operation::SteamworksNetworkingSocketsOperation,
};

/// Result message emitted by [`crate::SteamworksNetworkingSocketsPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksNetworkingSocketsResult {
    /// The command or event succeeded.
    Ok(SteamworksNetworkingSocketsOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksNetworkingSocketsCommand,
        /// Failure reason.
        error: SteamworksNetworkingSocketsError,
    },
}
