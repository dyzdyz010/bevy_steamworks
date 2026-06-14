use bevy_ecs::message::{MessageReader, MessageWriter};

use crate::SteamworksEvent;

use super::{
    SteamworksUgcDownloadItemResult, SteamworksUgcOperation, SteamworksUgcResult,
    SteamworksUgcState,
};

pub(super) fn process_ugc_steam_events(
    state: &mut SteamworksUgcState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksUgcResult>,
) {
    for event in steam_events.read() {
        let SteamworksEvent::DownloadItemResult(event) = event else {
            continue;
        };

        let operation = SteamworksUgcOperation::DownloadItemResultReceived {
            result: SteamworksUgcDownloadItemResult {
                app_id: event.app_id,
                item: event.published_file_id,
                error: event.error,
            },
        };
        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks UGC callback"
        );
        results.write(SteamworksUgcResult::Ok(operation));
    }
}
