use bevy_ecs::message::{MessageReader, MessageWriter};

use crate::SteamworksEvent;

use super::{
    SteamworksLobbyChatMessage, SteamworksLobbyChatUpdate, SteamworksLobbyCreatedCallback,
    SteamworksLobbyDataUpdate, SteamworksLobbyEnterCallback, SteamworksMatchmakingOperation,
    SteamworksMatchmakingResult, SteamworksMatchmakingState,
};

pub(super) fn process_matchmaking_steam_events(
    state: &mut SteamworksMatchmakingState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksMatchmakingResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::LobbyCreated(event) => {
                SteamworksMatchmakingOperation::LobbyCreateCallbackReceived {
                    callback: SteamworksLobbyCreatedCallback {
                        result: event.result,
                        lobby: event.lobby,
                    },
                }
            }
            SteamworksEvent::LobbyEnter(event) => {
                SteamworksMatchmakingOperation::LobbyEnterCallbackReceived {
                    callback: SteamworksLobbyEnterCallback {
                        lobby: event.lobby,
                        chat_permissions: event.chat_permissions,
                        blocked: event.blocked,
                        chat_room_enter_response: event.chat_room_enter_response,
                    },
                }
            }
            SteamworksEvent::LobbyChatMsg(event) => {
                SteamworksMatchmakingOperation::LobbyChatMessageReceived {
                    message: snapshot_lobby_chat_message(event),
                }
            }
            SteamworksEvent::LobbyChatUpdate(event) => {
                SteamworksMatchmakingOperation::LobbyChatUpdateReceived {
                    update: SteamworksLobbyChatUpdate {
                        lobby: event.lobby,
                        user_changed: event.user_changed,
                        making_change: event.making_change,
                        member_state_change: event.member_state_change.clone(),
                    },
                }
            }
            SteamworksEvent::LobbyDataUpdate(event) => {
                SteamworksMatchmakingOperation::LobbyDataUpdateReceived {
                    update: SteamworksLobbyDataUpdate {
                        lobby: event.lobby,
                        member: event.member,
                        success: event.success,
                    },
                }
            }
            _ => continue,
        };

        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks matchmaking callback"
        );
        results.write(SteamworksMatchmakingResult::Ok(operation));
    }
}

fn snapshot_lobby_chat_message(event: &steamworks::LobbyChatMsg) -> SteamworksLobbyChatMessage {
    SteamworksLobbyChatMessage {
        lobby: event.lobby,
        user: event.user,
        chat_entry_type: event.chat_entry_type,
        chat_id: event.chat_id,
    }
}
