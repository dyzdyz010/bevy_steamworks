use bevy_ecs::prelude::Resource;

use super::{
    messages::SteamworksScreenshotsError,
    types::{SteamworksScreenshotReady, SteamworksSubmittedScreenshot},
};

mod accessors;
mod operations;

pub(in crate::screenshots) const STEAMWORKS_SCREENSHOTS_STATE_CACHE_LIMIT: usize = 1_024;

/// Runtime state for [`super::SteamworksScreenshotsPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksScreenshotsState {
    last_error: Option<SteamworksScreenshotsError>,
    screenshots_hooked: Option<bool>,
    added_screenshots: Vec<steamworks::screenshots::ScreenshotHandle>,
    submitted_screenshots: Vec<SteamworksSubmittedScreenshot>,
    screenshot_trigger_count: u64,
    screenshot_requested_count: u64,
    screenshot_ready_count: u64,
    screenshot_ready_events: Vec<SteamworksScreenshotReady>,
    last_screenshot_ready: Option<SteamworksScreenshotReady>,
}
