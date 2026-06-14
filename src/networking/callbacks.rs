use bevy_ecs::message::{MessageReader, MessageWriter};

use crate::SteamworksEvent;

use super::{
    messages::{SteamworksNetworkingOperation, SteamworksNetworkingResult},
    state::SteamworksNetworkingState,
};

pub(super) fn process_networking_steam_events(
    state: &mut SteamworksNetworkingState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksNetworkingResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::P2PSessionRequest(event) => {
                Some(SteamworksNetworkingOperation::SessionRequestReceived {
                    remote: event.remote,
                })
            }
            SteamworksEvent::P2PSessionConnectFail(event) => {
                Some(SteamworksNetworkingOperation::SessionConnectFailed {
                    remote: event.remote,
                    error: steamworks::P2PSessionError::from(event.error),
                })
            }
            _ => None,
        };

        if let Some(operation) = operation {
            state.record_operation(&operation);
            tracing::debug!(
                target: "bevy_steamworks",
                operation = ?operation,
                "processed Steamworks legacy P2P callback"
            );
            results.write(SteamworksNetworkingResult::Ok(operation));
        }
    }
}
