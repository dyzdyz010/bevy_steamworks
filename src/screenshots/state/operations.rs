use super::{SteamworksScreenshotsState, STEAMWORKS_SCREENSHOTS_STATE_CACHE_LIMIT};
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
                    trim_cache(&mut self.added_screenshots);
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
    trim_cache(submissions);
}

fn trim_cache<T>(items: &mut Vec<T>) {
    if items.len() > STEAMWORKS_SCREENSHOTS_STATE_CACHE_LIMIT {
        let overflow = items.len() - STEAMWORKS_SCREENSHOTS_STATE_CACHE_LIMIT;
        items.drain(0..overflow);
    }
}
