use std::path::PathBuf;

use bevy_ecs::message::Message;

mod constructors;

/// A high-level command for Steam screenshot workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksScreenshotsCommand {
    /// Set whether this app handles Steam screenshot requests itself.
    ///
    /// When enabled, Steam emits [`crate::SteamworksEvent::ScreenshotRequested`]
    /// and [`crate::SteamworksScreenshotsOperation::ScreenshotRequested`], and the game
    /// is expected to capture and submit a screenshot.
    HookScreenshots {
        /// Whether screenshots should be hooked by the app.
        hook: bool,
    },
    /// Read whether this app is currently hooking Steam screenshots.
    IsScreenshotsHooked,
    /// Trigger a Steam screenshot.
    ///
    /// Depending on hook state, Steam may emit
    /// [`crate::SteamworksEvent::ScreenshotRequested`] /
    /// [`crate::SteamworksScreenshotsOperation::ScreenshotRequested`] and later
    /// [`crate::SteamworksEvent::ScreenshotReady`] /
    /// [`crate::SteamworksScreenshotsOperation::ScreenshotReady`].
    TriggerScreenshot,
    /// Add an existing screenshot image file to the user's Steam screenshot library.
    ///
    /// This submits the request and returns a handle immediately if Steam accepts
    /// it. Final save confirmation arrives later through both
    /// [`crate::SteamworksEvent::ScreenshotReady`] and
    /// [`crate::SteamworksScreenshotsOperation::ScreenshotReady`].
    ///
    /// The upstream wrapper canonicalizes the provided paths before submitting
    /// them to Steam, so use local paths and keep this command low-frequency.
    AddScreenshotToLibrary {
        /// Screenshot image file path.
        filename: PathBuf,
        /// Optional thumbnail image file path.
        thumbnail_filename: Option<PathBuf>,
        /// Screenshot width in pixels.
        width: i32,
        /// Screenshot height in pixels.
        height: i32,
    },
}
