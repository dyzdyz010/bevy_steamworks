use std::path::Path;

use super::SteamworksScreenshotsState;
use crate::screenshots::{
    SteamworksScreenshotReady, SteamworksScreenshotReadyError, SteamworksScreenshotsError,
    SteamworksSubmittedScreenshot,
};

impl SteamworksScreenshotsState {
    /// Returns the most recent synchronous error observed by the screenshots plugin.
    pub fn last_error(&self) -> Option<&SteamworksScreenshotsError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent screenshot hook state observed or submitted.
    pub fn screenshots_hooked(&self) -> Option<bool> {
        self.screenshots_hooked
    }

    /// Returns screenshot handles successfully submitted through this command layer.
    ///
    /// Final save confirmation arrives later through both
    /// [`crate::SteamworksEvent::ScreenshotReady`] and
    /// [`crate::SteamworksScreenshotsOperation::ScreenshotReady`].
    pub fn added_screenshots(&self) -> &[steamworks::screenshots::ScreenshotHandle] {
        &self.added_screenshots
    }

    /// Returns screenshot library submissions accepted by Steam through this command layer.
    pub fn submitted_screenshots(&self) -> &[SteamworksSubmittedScreenshot] {
        &self.submitted_screenshots
    }

    /// Returns one submitted screenshot snapshot by Steam screenshot handle.
    pub fn submitted_screenshot(
        &self,
        handle: steamworks::screenshots::ScreenshotHandle,
    ) -> Option<&SteamworksSubmittedScreenshot> {
        self.submitted_screenshots
            .iter()
            .find(|submission| submission.handle == handle)
    }

    /// Returns the most recent submitted screenshot snapshot for an image file path.
    pub fn submitted_screenshot_by_filename(
        &self,
        filename: impl AsRef<Path>,
    ) -> Option<&SteamworksSubmittedScreenshot> {
        let filename = filename.as_ref();
        self.submitted_screenshots
            .iter()
            .rev()
            .find(|submission| submission.filename == filename)
    }

    /// Returns the submitted screenshot dimensions for a Steam screenshot handle.
    pub fn submitted_screenshot_dimensions(
        &self,
        handle: steamworks::screenshots::ScreenshotHandle,
    ) -> Option<(i32, i32)> {
        self.submitted_screenshot(handle)
            .map(|submission| (submission.width, submission.height))
    }

    /// Returns the submitted screenshot thumbnail path, preserving a submitted screenshot without thumbnail as `Some(None)`.
    pub fn submitted_screenshot_thumbnail(
        &self,
        handle: steamworks::screenshots::ScreenshotHandle,
    ) -> Option<Option<&Path>> {
        self.submitted_screenshot(handle)
            .map(|submission| submission.thumbnail_filename.as_deref())
    }

    /// Returns the most recent screenshot library submission accepted by Steam.
    pub fn last_submitted_screenshot(&self) -> Option<&SteamworksSubmittedScreenshot> {
        self.submitted_screenshots.last()
    }

    /// Returns how many trigger screenshot commands this plugin successfully submitted.
    pub fn screenshot_trigger_count(&self) -> u64 {
        self.screenshot_trigger_count
    }

    /// Returns how many screenshot requested callbacks this plugin observed.
    pub fn screenshot_requested_count(&self) -> u64 {
        self.screenshot_requested_count
    }

    /// Returns how many screenshot ready callbacks this plugin observed.
    pub fn screenshot_ready_count(&self) -> u64 {
        self.screenshot_ready_count
    }

    /// Returns how many successful screenshot ready callbacks are retained in state.
    pub fn screenshot_ready_success_count(&self) -> usize {
        self.screenshot_ready_events
            .iter()
            .filter(|ready| ready.local_handle.is_ok())
            .count()
    }

    /// Returns how many failed screenshot ready callbacks are retained in state.
    pub fn screenshot_ready_error_count(&self) -> usize {
        self.screenshot_ready_events
            .iter()
            .filter(|ready| ready.local_handle.is_err())
            .count()
    }

    /// Returns bounded screenshot ready callback snapshots observed by this plugin.
    pub fn screenshot_ready_events(&self) -> &[SteamworksScreenshotReady] {
        &self.screenshot_ready_events
    }

    /// Returns the most recent ready callback snapshot for a screenshot handle.
    pub fn screenshot_ready(
        &self,
        handle: steamworks::screenshots::ScreenshotHandle,
    ) -> Option<&SteamworksScreenshotReady> {
        self.screenshot_ready_events
            .iter()
            .rev()
            .find(|ready| match &ready.local_handle {
                Ok(ready_handle) => *ready_handle == handle,
                Err(_) => false,
            })
    }

    /// Returns the most recent screenshot ready callback snapshot.
    pub fn last_screenshot_ready(&self) -> Option<&SteamworksScreenshotReady> {
        self.last_screenshot_ready.as_ref()
    }

    /// Returns the most recent screenshot ready callback error, if the latest callback failed.
    pub fn last_screenshot_ready_error(&self) -> Option<SteamworksScreenshotReadyError> {
        self.last_screenshot_ready
            .as_ref()
            .and_then(|ready| ready.local_handle.err())
    }
}
