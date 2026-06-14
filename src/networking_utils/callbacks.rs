use bevy_ecs::message::{MessageReader, MessageWriter};

use crate::{SteamworksClient, SteamworksEvent};

use super::{
    messages::{SteamworksNetworkingUtilsOperation, SteamworksNetworkingUtilsResult},
    state::SteamworksNetworkingUtilsState,
    types::SteamworksRelayNetworkStatus,
};

pub(super) fn process_networking_utils_steam_events(
    client: &SteamworksClient,
    state: &mut SteamworksNetworkingUtilsState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksNetworkingUtilsResult>,
) {
    for event in steam_events.read() {
        if let SteamworksEvent::RelayNetworkStatusCallback(_) = event {
            let operation = SteamworksNetworkingUtilsOperation::RelayNetworkStatusChanged {
                status: SteamworksRelayNetworkStatus::from_steam(
                    client.networking_utils().detailed_relay_network_status(),
                ),
            };
            state.record_operation(&operation);
            tracing::debug!(
                target: "bevy_steamworks",
                operation = ?operation,
                "processed Steamworks relay network status callback"
            );
            results.write(SteamworksNetworkingUtilsResult::Ok(operation));
        }
    }
}
