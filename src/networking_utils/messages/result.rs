use bevy_ecs::message::Message;

use super::{
    SteamworksNetworkingUtilsCommand, SteamworksNetworkingUtilsError,
    SteamworksNetworkingUtilsOperation,
};

/// Result message emitted by [`crate::SteamworksNetworkingUtilsPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksNetworkingUtilsResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksNetworkingUtilsOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksNetworkingUtilsCommand,
        /// Failure reason.
        error: SteamworksNetworkingUtilsError,
    },
}

crate::result_ext::impl_steamworks_result_helpers!(
    SteamworksNetworkingUtilsResult,
    SteamworksNetworkingUtilsOperation,
    SteamworksNetworkingUtilsCommand,
    SteamworksNetworkingUtilsError
);
