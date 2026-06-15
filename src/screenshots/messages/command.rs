use std::path::PathBuf;

use bevy_ecs::message::Message;

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

impl SteamworksScreenshotsCommand {
    /// Creates a [`crate::SteamworksScreenshotsCommand::HookScreenshots`] command.
    pub fn hook_screenshots(hook: bool) -> Self {
        Self::HookScreenshots { hook }
    }

    /// Creates a [`crate::SteamworksScreenshotsCommand::IsScreenshotsHooked`] command.
    pub fn is_screenshots_hooked() -> Self {
        Self::IsScreenshotsHooked
    }

    /// Creates a [`crate::SteamworksScreenshotsCommand::TriggerScreenshot`] command.
    pub fn trigger_screenshot() -> Self {
        Self::TriggerScreenshot
    }

    /// Creates a [`crate::SteamworksScreenshotsCommand::AddScreenshotToLibrary`] command.
    pub fn add_screenshot_to_library(
        filename: impl Into<PathBuf>,
        thumbnail_filename: Option<impl Into<PathBuf>>,
        width: i32,
        height: i32,
    ) -> Self {
        Self::AddScreenshotToLibrary {
            filename: filename.into(),
            thumbnail_filename: thumbnail_filename.map(Into::into),
            width,
            height,
        }
    }
}
