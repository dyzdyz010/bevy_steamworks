//! High-level Bevy ECS integration for Steam matchmaking and lobbies.
//!
//! This module builds on top of the upstream [`steamworks::Matchmaking`] API.
//! It keeps async Steam call results and lobby callbacks flowing through Bevy
//! messages, while avoiding blocking work in the frame loop.

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
    schedule::IntoScheduleConfigs,
};

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

mod async_results;
mod callbacks;
mod filters;
mod messages;
mod state;
#[cfg(test)]
mod tests;
mod types;
mod validation;

use async_results::SteamworksMatchmakingAsyncResults;
use callbacks::process_matchmaking_steam_events;
use filters::apply_lobby_list_filter;
use validation::validate_command;

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
