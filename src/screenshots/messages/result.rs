use bevy_ecs::message::Message;

use super::{
    SteamworksScreenshotsCommand, SteamworksScreenshotsError, SteamworksScreenshotsOperation,
};

/// Result message emitted by [`crate::SteamworksScreenshotsPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksScreenshotsResult {
    /// The command or observed callback was processed successfully.
    Ok(SteamworksScreenshotsOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksScreenshotsCommand,
        /// Failure reason.
        error: SteamworksScreenshotsError,
    },
}

crate::result_ext::impl_steamworks_result_helpers!(
    SteamworksScreenshotsResult,
    SteamworksScreenshotsOperation,
    SteamworksScreenshotsCommand,
    SteamworksScreenshotsError
);
