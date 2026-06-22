use std::path::PathBuf;

use super::SteamworksScreenshotsCommand;

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
