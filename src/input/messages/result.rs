use bevy_ecs::message::Message;

use super::{SteamworksInputCommand, SteamworksInputError, SteamworksInputOperation};

/// Result message emitted by [`crate::SteamworksInputPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksInputResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksInputOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksInputCommand,
        /// Failure reason.
        error: SteamworksInputError,
    },
}

crate::result_ext::impl_steamworks_result_helpers!(
    SteamworksInputResult,
    SteamworksInputOperation,
    SteamworksInputCommand,
    SteamworksInputError
);
