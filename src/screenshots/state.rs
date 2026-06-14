use bevy_ecs::prelude::Resource;

use super::{
    messages::{SteamworksScreenshotsError, SteamworksScreenshotsOperation},
    types::{SteamworksScreenshotReady, SteamworksSubmittedScreenshot},
};

/// Runtime state for [`super::SteamworksScreenshotsPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksScreenshotsState {
    last_error: Option<SteamworksScreenshotsError>,
    screenshots_hooked: Option<bool>,
    added_screenshots: Vec<steamworks::screenshots::ScreenshotHandle>,
    submitted_screenshots: Vec<SteamworksSubmittedScreenshot>,
    screenshot_trigger_count: u64,
    screenshot_requested_count: u64,
    last_screenshot_ready: Option<SteamworksScreenshotReady>,
}

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
    /// [`super::SteamworksScreenshotsOperation::ScreenshotReady`].
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

    pub(super) fn record_error(&mut self, error: SteamworksScreenshotsError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksScreenshotsOperation) {
        match operation {
            SteamworksScreenshotsOperation::ScreenshotsHookSet { hook }
            | SteamworksScreenshotsOperation::ScreenshotsHookedRead { hooked: hook } => {
                self.screenshots_hooked = Some(*hook);
            }
            SteamworksScreenshotsOperation::ScreenshotTriggered => {
                self.screenshot_trigger_count = self.screenshot_trigger_count.saturating_add(1);
            }
            SteamworksScreenshotsOperation::ScreenshotLibraryAddSubmitted {
                handle,
                filename,
                thumbnail_filename,
                width,
                height,
            } => {
                if !self.added_screenshots.contains(handle) {
                    self.added_screenshots.push(*handle);
                }
                upsert_submitted_screenshot(
                    &mut self.submitted_screenshots,
                    SteamworksSubmittedScreenshot {
                        handle: *handle,
                        filename: filename.clone(),
                        thumbnail_filename: thumbnail_filename.clone(),
                        width: *width,
                        height: *height,
                    },
                );
            }
            SteamworksScreenshotsOperation::ScreenshotRequested { count } => {
                self.screenshot_requested_count = *count;
            }
            SteamworksScreenshotsOperation::ScreenshotReady { ready } => {
                self.last_screenshot_ready = Some(ready.clone());
            }
        }
    }
}

fn upsert_submitted_screenshot(
    submissions: &mut Vec<SteamworksSubmittedScreenshot>,
    submission: SteamworksSubmittedScreenshot,
) {
    if let Some(index) = submissions
        .iter()
        .position(|existing| existing.handle == submission.handle)
    {
        submissions.remove(index);
    }

    submissions.push(submission);
}
