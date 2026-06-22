use super::SteamworksScreenshotsState;
use crate::screenshots::{
    SteamworksScreenshotReady, SteamworksScreenshotsError, SteamworksSubmittedScreenshot,
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

    /// Returns the most recent screenshot ready callback snapshot.
    pub fn last_screenshot_ready(&self) -> Option<&SteamworksScreenshotReady> {
        self.last_screenshot_ready.as_ref()
    }
}
