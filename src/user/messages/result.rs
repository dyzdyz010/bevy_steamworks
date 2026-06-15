use bevy_ecs::message::Message;

use super::{SteamworksUserCommand, SteamworksUserError, SteamworksUserOperation};

/// Result message emitted by [`crate::SteamworksUserPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksUserResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksUserOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksUserCommand,
        /// Failure reason.
        error: SteamworksUserError,
    },
}

crate::result_ext::impl_steamworks_result_helpers!(
    SteamworksUserResult,
    SteamworksUserOperation,
    SteamworksUserCommand,
    SteamworksUserError
);
