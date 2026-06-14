use bevy_ecs::message::{MessageReader, MessageWriter};

use crate::SteamworksEvent;

use super::{
    messages::{SteamworksRemotePlayOperation, SteamworksRemotePlayResult},
    state::SteamworksRemotePlayState,
};

pub(super) fn process_remote_play_steam_events(
    state: &mut SteamworksRemotePlayState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksRemotePlayResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::RemotePlayConnected(event) => {
                SteamworksRemotePlayOperation::SessionConnected {
                    session: event.session,
                }
            }
            SteamworksEvent::RemotePlayDisconnected(event) => {
                SteamworksRemotePlayOperation::SessionDisconnected {
                    session: event.session,
                }
            }
            _ => continue,
        };

        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks Remote Play callback"
        );
        results.write(SteamworksRemotePlayResult::Ok(operation));
    }
}
