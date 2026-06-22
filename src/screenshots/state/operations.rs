use super::{SteamworksScreenshotsState, STEAMWORKS_SCREENSHOTS_STATE_CACHE_LIMIT};
use crate::cache::trim_oldest;
use crate::screenshots::{
    SteamworksScreenshotsError, SteamworksScreenshotsOperation, SteamworksSubmittedScreenshot,
};

impl SteamworksScreenshotsState {
    pub(in crate::screenshots) fn record_error(&mut self, error: SteamworksScreenshotsError) {
        self.last_error = Some(error);
    }

    pub(in crate::screenshots) fn record_operation(
        &mut self,
        operation: &SteamworksScreenshotsOperation,
    ) {
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
                    trim_oldest(
                        &mut self.added_screenshots,
                        STEAMWORKS_SCREENSHOTS_STATE_CACHE_LIMIT,
                    );
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
                self.screenshot_ready_count = self.screenshot_ready_count.saturating_add(1);
                self.screenshot_ready_events.push(ready.clone());
                trim_oldest(
                    &mut self.screenshot_ready_events,
                    STEAMWORKS_SCREENSHOTS_STATE_CACHE_LIMIT,
                );
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
    trim_oldest(submissions, STEAMWORKS_SCREENSHOTS_STATE_CACHE_LIMIT);
}
