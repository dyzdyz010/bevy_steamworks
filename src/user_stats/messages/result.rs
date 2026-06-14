use bevy_ecs::message::Message;

use super::{
    command::SteamworksStatsCommand, error::SteamworksStatsError,
    operation::SteamworksStatsOperation,
};

/// Result message emitted by [`crate::SteamworksStatsPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksStatsResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksStatsOperation),
    /// The command failed synchronously or an asynchronous callback failed.
    Err {
        /// Command that failed.
        command: SteamworksStatsCommand,
        /// Failure reason.
        error: SteamworksStatsError,
    },
}
