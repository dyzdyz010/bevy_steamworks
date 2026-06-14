use bevy_ecs::message::{MessageReader, MessageWriter};

use crate::SteamworksEvent;

use super::{
    messages::{SteamworksScreenshotsOperation, SteamworksScreenshotsResult},
    state::SteamworksScreenshotsState,
    types::SteamworksScreenshotReady,
};

pub(super) fn process_screenshots_steam_events(
    state: &mut SteamworksScreenshotsState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksScreenshotsResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::ScreenshotRequested(_) => {
                SteamworksScreenshotsOperation::ScreenshotRequested {
                    count: state.screenshot_requested_count().saturating_add(1),
                }
            }
            SteamworksEvent::ScreenshotReady(event) => {
                SteamworksScreenshotsOperation::ScreenshotReady {
                    ready: SteamworksScreenshotReady {
                        local_handle: event.local_handle.clone().map_err(Into::into),
                    },
                }
            }
            _ => continue,
        };

        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks screenshots callback"
        );
        results.write(SteamworksScreenshotsResult::Ok(operation));
    }
}
