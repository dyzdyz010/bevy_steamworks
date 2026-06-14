//! High-level Bevy ECS integration for Steam matchmaking and lobbies.
//!
//! This module builds on top of the upstream [`steamworks::Matchmaking`] API.
//! It keeps async Steam call results and lobby callbacks flowing through Bevy
//! messages, while avoiding blocking work in the frame loop.

use std::sync::{Arc, Mutex};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

mod messages;
mod state;
#[cfg(test)]
mod tests;
mod types;

pub use messages::*;
pub use state::SteamworksMatchmakingState;
pub use types::*;

const MAX_LOBBY_MEMBERS: u32 = 250;
const MAX_LOBBY_CHAT_MESSAGE_BYTES: usize = 4096;
const MAX_LOBBY_LIST_RESULTS: u64 = i32::MAX as u64;

/// Bevy plugin for high-level Steam matchmaking and lobby commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksMatchmakingCommand`] and [`SteamworksMatchmakingResult`]
/// messages and runs its command processor in [`bevy_app::First`] after Steam
/// callbacks. It also mirrors lobby callbacks into matchmaking results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksMatchmakingPlugin;

impl SteamworksMatchmakingPlugin {
    /// Creates a matchmaking plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksMatchmakingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksMatchmakingState>()
            .init_resource::<SteamworksMatchmakingAsyncResults>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksMatchmakingCommand>()
            .add_message::<SteamworksMatchmakingResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessMatchmakingCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_matchmaking_commands.in_set(SteamworksSystem::ProcessMatchmakingCommands),
            );
    }
}

#[derive(Clone, Debug, Default, Resource)]
struct SteamworksMatchmakingAsyncResults {
    queue: Arc<Mutex<Vec<SteamworksMatchmakingResult>>>,
}

impl SteamworksMatchmakingAsyncResults {
    fn push(&self, result: SteamworksMatchmakingResult) {
        self.queue
            .lock()
            .expect("Steamworks matchmaking async result mutex was poisoned")
            .push(result);
    }

    fn drain(&self) -> Vec<SteamworksMatchmakingResult> {
        self.queue
            .lock()
            .expect("Steamworks matchmaking async result mutex was poisoned")
            .drain(..)
            .collect()
    }
}

fn process_matchmaking_commands(
    client: Option<Res<SteamworksClient>>,
    async_results: Res<SteamworksMatchmakingAsyncResults>,
    mut state: ResMut<SteamworksMatchmakingState>,
    mut commands: ResMut<Messages<SteamworksMatchmakingCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksMatchmakingResult>,
) {
    for result in async_results.drain() {
        record_matchmaking_result(&mut state, &result);
        results.write(result);
    }

    process_matchmaking_steam_events(&mut state, &mut steam_events, &mut results);

    let Some(client) = client else {
        let error = SteamworksMatchmakingError::ClientUnavailable;
        for command in commands.drain() {
            state.record_error(error.clone());
            results.write(SteamworksMatchmakingResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        let request_id = async_command_request_id(&command, &mut state);
        match handle_matchmaking_command(&client, &async_results, command.clone(), request_id) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks matchmaking command"
                );
                results.write(SteamworksMatchmakingResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks matchmaking command failed"
                );
                results.write(SteamworksMatchmakingResult::Err { command, error });
            }
        }
    }
}

fn process_matchmaking_steam_events(
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

fn record_matchmaking_result(
    state: &mut SteamworksMatchmakingState,
    result: &SteamworksMatchmakingResult,
) {
    match result {
        SteamworksMatchmakingResult::Ok(operation) => state.record_operation(operation),
        SteamworksMatchmakingResult::Err { error, .. } => state.record_error(error.clone()),
    }
}

fn async_command_request_id(
    command: &SteamworksMatchmakingCommand,
    state: &mut SteamworksMatchmakingState,
) -> Option<u64> {
    matches!(
        command,
        SteamworksMatchmakingCommand::RequestLobbyList { .. }
            | SteamworksMatchmakingCommand::CreateLobby { .. }
            | SteamworksMatchmakingCommand::JoinLobby { .. }
    )
    .then(|| state.next_request_id())
}

fn handle_matchmaking_command(
    client: &SteamworksClient,
    async_results: &SteamworksMatchmakingAsyncResults,
    command: SteamworksMatchmakingCommand,
    request_id: Option<u64>,
) -> Result<SteamworksMatchmakingOperation, SteamworksMatchmakingError> {
    validate_command(&command)?;

    let matchmaking = client.matchmaking();
    match command {
        SteamworksMatchmakingCommand::RequestLobbyList { filter } => {
            let request_id = request_id.expect("async matchmaking command missing request id");
            apply_lobby_list_filter(&matchmaking, &filter)?;
            let async_results = async_results.clone();
            let command = SteamworksMatchmakingCommand::RequestLobbyList {
                filter: filter.clone(),
            };
            let request_filter = filter.clone();
            matchmaking.request_lobby_list(move |result| {
                async_results.push(match result {
                    Ok(lobbies) => SteamworksMatchmakingResult::Ok(
                        SteamworksMatchmakingOperation::LobbyListReceived {
                            request_id,
                            filter: request_filter.clone(),
                            lobbies,
                        },
                    ),
                    Err(source) => SteamworksMatchmakingResult::Err {
                        command,
                        error: SteamworksMatchmakingError::steam_error(
                            "matchmaking.request_lobby_list",
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksMatchmakingOperation::LobbyListRequested { request_id, filter })
        }
        SteamworksMatchmakingCommand::CreateLobby {
            lobby_type,
            max_members,
        } => {
            let request_id = request_id.expect("async matchmaking command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksMatchmakingCommand::CreateLobby {
                lobby_type,
                max_members,
            };
            matchmaking.create_lobby(lobby_type, max_members, move |result| {
                async_results.push(match result {
                    Ok(lobby) => SteamworksMatchmakingResult::Ok(
                        SteamworksMatchmakingOperation::LobbyCreated {
                            request_id,
                            lobby_type,
                            max_members,
                            lobby,
                        },
                    ),
                    Err(source) => SteamworksMatchmakingResult::Err {
                        command,
                        error: SteamworksMatchmakingError::steam_error(
                            "matchmaking.create_lobby",
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksMatchmakingOperation::LobbyCreateRequested {
                request_id,
                lobby_type,
                max_members,
            })
        }
        SteamworksMatchmakingCommand::JoinLobby { lobby } => {
            let request_id = request_id.expect("async matchmaking command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksMatchmakingCommand::JoinLobby { lobby };
            let requested_lobby = lobby;
            matchmaking.join_lobby(lobby, move |result| {
                async_results.push(match result {
                    Ok(lobby) => SteamworksMatchmakingResult::Ok(
                        SteamworksMatchmakingOperation::LobbyJoined {
                            request_id,
                            requested_lobby,
                            lobby,
                        },
                    ),
                    Err(()) => SteamworksMatchmakingResult::Err {
                        command,
                        error: SteamworksMatchmakingError::operation_failed(
                            "matchmaking.join_lobby",
                        ),
                    },
                });
            });
            Ok(SteamworksMatchmakingOperation::LobbyJoinRequested { request_id, lobby })
        }
        SteamworksMatchmakingCommand::LeaveLobby { lobby } => {
            matchmaking.leave_lobby(lobby);
            Ok(SteamworksMatchmakingOperation::LobbyLeft { lobby })
        }
        SteamworksMatchmakingCommand::GetLobbyDataCount { lobby } => {
            Ok(SteamworksMatchmakingOperation::LobbyDataCountRead {
                lobby,
                count: matchmaking.lobby_data_count(lobby),
            })
        }
        SteamworksMatchmakingCommand::GetLobbyData { lobby, key } => {
            Ok(SteamworksMatchmakingOperation::LobbyDataRead {
                lobby,
                value: matchmaking.lobby_data(lobby, &key),
                key,
            })
        }
        SteamworksMatchmakingCommand::GetLobbyDataByIndex { lobby, index } => {
            Ok(SteamworksMatchmakingOperation::LobbyDataByIndexRead {
                lobby,
                index,
                entry: matchmaking.lobby_data_by_index(lobby, index),
            })
        }
        SteamworksMatchmakingCommand::GetAllLobbyData { lobby } => {
            let entries = (0..matchmaking.lobby_data_count(lobby))
                .filter_map(|index| matchmaking.lobby_data_by_index(lobby, index))
                .collect();
            Ok(SteamworksMatchmakingOperation::AllLobbyDataRead { lobby, entries })
        }
        SteamworksMatchmakingCommand::SetLobbyData { lobby, key, value } => {
            if matchmaking.set_lobby_data(lobby, &key, &value) {
                Ok(SteamworksMatchmakingOperation::LobbyDataSet { lobby, key })
            } else {
                Err(SteamworksMatchmakingError::operation_failed(
                    "matchmaking.set_lobby_data",
                ))
            }
        }
        SteamworksMatchmakingCommand::DeleteLobbyData { lobby, key } => {
            if matchmaking.delete_lobby_data(lobby, &key) {
                Ok(SteamworksMatchmakingOperation::LobbyDataDeleted { lobby, key })
            } else {
                Err(SteamworksMatchmakingError::operation_failed(
                    "matchmaking.delete_lobby_data",
                ))
            }
        }
        SteamworksMatchmakingCommand::SetLobbyMemberData { lobby, key, value } => {
            matchmaking.set_lobby_member_data(lobby, &key, &value);
            Ok(SteamworksMatchmakingOperation::LobbyMemberDataSet { lobby, key })
        }
        SteamworksMatchmakingCommand::GetLobbyMemberData { lobby, user, key } => {
            Ok(SteamworksMatchmakingOperation::LobbyMemberDataRead {
                lobby,
                user,
                value: matchmaking.get_lobby_member_data(lobby, user, &key),
                key,
            })
        }
        SteamworksMatchmakingCommand::GetLobbyMemberLimit { lobby } => {
            Ok(SteamworksMatchmakingOperation::LobbyMemberLimitRead {
                lobby,
                limit: matchmaking.lobby_member_limit(lobby),
            })
        }
        SteamworksMatchmakingCommand::GetLobbyOwner { lobby } => {
            Ok(SteamworksMatchmakingOperation::LobbyOwnerRead {
                lobby,
                owner: matchmaking.lobby_owner(lobby),
            })
        }
        SteamworksMatchmakingCommand::GetLobbyMemberCount { lobby } => {
            Ok(SteamworksMatchmakingOperation::LobbyMemberCountRead {
                lobby,
                count: matchmaking.lobby_member_count(lobby),
            })
        }
        SteamworksMatchmakingCommand::ListLobbyMembers { lobby } => {
            Ok(SteamworksMatchmakingOperation::LobbyMembersListed {
                lobby,
                members: matchmaking.lobby_members(lobby),
            })
        }
        SteamworksMatchmakingCommand::SetLobbyJoinable { lobby, joinable } => {
            if matchmaking.set_lobby_joinable(lobby, joinable) {
                Ok(SteamworksMatchmakingOperation::LobbyJoinableSet { lobby, joinable })
            } else {
                Err(SteamworksMatchmakingError::operation_failed(
                    "matchmaking.set_lobby_joinable",
                ))
            }
        }
        SteamworksMatchmakingCommand::SendLobbyChatMessage { lobby, data } => matchmaking
            .send_lobby_chat_message(lobby, &data)
            .map(|()| SteamworksMatchmakingOperation::LobbyChatMessageSent {
                lobby,
                len: data.len(),
            })
            .map_err(|source| {
                SteamworksMatchmakingError::steam_error(
                    "matchmaking.send_lobby_chat_message",
                    source,
                )
            }),
        SteamworksMatchmakingCommand::GetLobbyChatEntry {
            lobby,
            chat_id,
            max_bytes,
        } => {
            let mut buffer = vec![0; max_bytes];
            let data = matchmaking
                .get_lobby_chat_entry(lobby, chat_id, &mut buffer)
                .to_vec();
            Ok(SteamworksMatchmakingOperation::LobbyChatEntryRead {
                lobby,
                chat_id,
                data,
            })
        }
        SteamworksMatchmakingCommand::SetLobbyGameServer {
            lobby,
            address,
            steam_id,
        } => {
            matchmaking.set_lobby_game_server(lobby, address, steam_id);
            Ok(SteamworksMatchmakingOperation::LobbyGameServerSet {
                lobby,
                server: SteamworksLobbyGameServer { address, steam_id },
            })
        }
        SteamworksMatchmakingCommand::GetLobbyGameServer { lobby } => {
            let server = matchmaking
                .get_lobby_game_server(lobby)
                .map(|(address, steam_id)| SteamworksLobbyGameServer { address, steam_id });
            Ok(SteamworksMatchmakingOperation::LobbyGameServerRead { lobby, server })
        }
    }
}

fn apply_lobby_list_filter(
    matchmaking: &steamworks::Matchmaking,
    filter: &SteamworksLobbyListFilter,
) -> Result<(), SteamworksMatchmakingError> {
    for item in &filter.string {
        let key = lobby_key(&item.key)?;
        matchmaking.add_request_lobby_list_string_filter(steamworks::StringFilter(
            key,
            &item.value,
            item.comparison,
        ));
    }

    for item in &filter.number {
        let key = lobby_key(&item.key)?;
        matchmaking.add_request_lobby_list_numerical_filter(steamworks::NumberFilter(
            key,
            item.value,
            item.comparison,
        ));
    }

    for item in &filter.near_value {
        let key = lobby_key(&item.key)?;
        matchmaking
            .add_request_lobby_list_near_value_filter(steamworks::NearFilter(key, item.value));
    }

    if let Some(open_slots) = filter.open_slots {
        matchmaking.set_request_lobby_list_slots_available_filter(open_slots);
    }
    if let Some(distance) = filter.distance {
        matchmaking.set_request_lobby_list_distance_filter(distance);
    }
    if let Some(max_results) = filter.max_results {
        matchmaking.set_request_lobby_list_result_count_filter(max_results);
    }

    Ok(())
}

fn lobby_key(key: &str) -> Result<steamworks::LobbyKey<'_>, SteamworksMatchmakingError> {
    steamworks::LobbyKey::try_new(key)
        .map_err(|_| SteamworksMatchmakingError::lobby_key_too_long(key))
}

fn validate_command(
    command: &SteamworksMatchmakingCommand,
) -> Result<(), SteamworksMatchmakingError> {
    match command {
        SteamworksMatchmakingCommand::RequestLobbyList { filter } => validate_filter(filter),
        SteamworksMatchmakingCommand::CreateLobby { max_members, .. } => {
            if *max_members > MAX_LOBBY_MEMBERS {
                Err(SteamworksMatchmakingError::MaxLobbyMembersExceeded {
                    requested: *max_members,
                    max_supported: MAX_LOBBY_MEMBERS,
                })
            } else {
                Ok(())
            }
        }
        SteamworksMatchmakingCommand::GetLobbyData { key, .. }
        | SteamworksMatchmakingCommand::DeleteLobbyData { key, .. } => validate_lobby_key(key),
        SteamworksMatchmakingCommand::SetLobbyData { key, value, .. }
        | SteamworksMatchmakingCommand::SetLobbyMemberData { key, value, .. } => {
            validate_lobby_key(key)?;
            validate_steam_string("value", value)
        }
        SteamworksMatchmakingCommand::GetLobbyMemberData { key, .. } => validate_lobby_key(key),
        SteamworksMatchmakingCommand::SendLobbyChatMessage { data, .. } => {
            validate_lobby_chat_message(data.len())
        }
        SteamworksMatchmakingCommand::GetLobbyChatEntry { max_bytes, .. } => {
            validate_lobby_chat_message(*max_bytes)
        }
        _ => Ok(()),
    }
}

fn validate_filter(filter: &SteamworksLobbyListFilter) -> Result<(), SteamworksMatchmakingError> {
    for item in &filter.string {
        validate_lobby_key(&item.key)?;
        validate_steam_string("value", &item.value)?;
    }
    for item in &filter.number {
        validate_lobby_key(&item.key)?;
    }
    for item in &filter.near_value {
        validate_lobby_key(&item.key)?;
    }
    if let Some(max_results) = filter.max_results {
        validate_lobby_list_result_count(max_results)?;
    }
    Ok(())
}

fn validate_lobby_key(key: &str) -> Result<(), SteamworksMatchmakingError> {
    validate_steam_string("key", key)?;
    lobby_key(key).map(|_| ())
}

fn validate_steam_string(
    field: &'static str,
    value: &str,
) -> Result<(), SteamworksMatchmakingError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksMatchmakingError::invalid_string(field))
    } else {
        Ok(())
    }
}

fn validate_lobby_chat_message(len: usize) -> Result<(), SteamworksMatchmakingError> {
    if len == 0 || len > MAX_LOBBY_CHAT_MESSAGE_BYTES {
        Err(SteamworksMatchmakingError::InvalidChatMessageLength {
            requested: len,
            max_supported: MAX_LOBBY_CHAT_MESSAGE_BYTES,
        })
    } else {
        Ok(())
    }
}

fn validate_lobby_list_result_count(count: u64) -> Result<(), SteamworksMatchmakingError> {
    if count > MAX_LOBBY_LIST_RESULTS {
        Err(SteamworksMatchmakingError::MaxLobbyListResultsExceeded {
            requested: count,
            max_supported: MAX_LOBBY_LIST_RESULTS,
        })
    } else {
        Ok(())
    }
}
