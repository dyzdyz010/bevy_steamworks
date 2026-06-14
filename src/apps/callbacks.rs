use bevy_ecs::message::{MessageReader, MessageWriter};

use crate::SteamworksEvent;

use super::{
    messages::{SteamworksAppsOperation, SteamworksAppsResult},
    state::SteamworksAppsState,
};

pub(super) fn process_apps_steam_events(
    state: &mut SteamworksAppsState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksAppsResult>,
) {
    for event in steam_events.read() {
        if !matches!(event, SteamworksEvent::NewUrlLaunchParameters(_)) {
            continue;
        }

        let operation = SteamworksAppsOperation::NewUrlLaunchParametersReceived {
            count: state.new_url_launch_parameters_count().saturating_add(1),
        };
        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks apps callback"
        );
        results.write(SteamworksAppsResult::Ok(operation));
    }
}
