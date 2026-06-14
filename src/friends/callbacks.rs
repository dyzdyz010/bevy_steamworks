use bevy_ecs::message::{MessageReader, MessageWriter};

use crate::SteamworksEvent;

use super::{
    messages::{SteamworksFriendsOperation, SteamworksFriendsResult},
    state::SteamworksFriendsState,
    types::{
        SteamworksLobbyJoinRequest, SteamworksPersonaStateChange, SteamworksRichPresenceJoinRequest,
    },
};

pub(super) fn process_friends_steam_events(
    state: &mut SteamworksFriendsState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksFriendsResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::GameOverlayActivated(event) => {
                SteamworksFriendsOperation::GameOverlayActivationChanged {
                    active: event.active,
                }
            }
            SteamworksEvent::PersonaStateChange(event) => {
                SteamworksFriendsOperation::PersonaStateChanged {
                    change: SteamworksPersonaStateChange {
                        steam_id: event.steam_id,
                        flags: event.flags,
                    },
                }
            }
            SteamworksEvent::GameLobbyJoinRequested(event) => {
                SteamworksFriendsOperation::GameLobbyJoinRequested {
                    request: SteamworksLobbyJoinRequest {
                        lobby: event.lobby_steam_id,
                        friend_steam_id: event.friend_steam_id,
                    },
                }
            }
            SteamworksEvent::GameRichPresenceJoinRequested(event) => {
                SteamworksFriendsOperation::GameRichPresenceJoinRequested {
                    request: SteamworksRichPresenceJoinRequest {
                        friend_steam_id: event.friend_steam_id,
                        connect: event.connect.clone(),
                    },
                }
            }
            _ => continue,
        };

        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks friends callback"
        );
        results.write(SteamworksFriendsResult::Ok(operation));
    }
}
