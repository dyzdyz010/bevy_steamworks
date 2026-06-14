use bevy_ecs::message::Message;

use super::{SteamworksFriendsCommand, SteamworksFriendsError, SteamworksFriendsOperation};

/// Result message emitted by [`crate::SteamworksFriendsPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksFriendsResult {
    /// The command or observed callback was processed successfully.
    Ok(SteamworksFriendsOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksFriendsCommand,
        /// Failure reason.
        error: SteamworksFriendsError,
    },
}
