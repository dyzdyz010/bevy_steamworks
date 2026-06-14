use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::{SteamworksClient, SteamworksEvent};

use super::{
    async_results::SteamworksMatchmakingAsyncResults, callbacks::process_matchmaking_steam_events,
    filters::apply_lobby_list_filter, validation::validate_command, SteamworksLobbyGameServer,
    SteamworksMatchmakingCommand, SteamworksMatchmakingError, SteamworksMatchmakingOperation,
    SteamworksMatchmakingResult, SteamworksMatchmakingState,
};

pub(super) fn process_matchmaking_commands(
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

#[cfg(test)]
mod tests {
    use super::super::SteamworksLobbyListFilter;
    use super::*;

    #[test]
    fn async_commands_get_unique_request_ids() {
        let mut state = SteamworksMatchmakingState::default();
        let command =
            SteamworksMatchmakingCommand::request_lobby_list(SteamworksLobbyListFilter::new());

        assert_eq!(async_command_request_id(&command, &mut state), Some(0));
        assert_eq!(async_command_request_id(&command, &mut state), Some(1));
        assert_eq!(
            async_command_request_id(
                &SteamworksMatchmakingCommand::GetLobbyDataCount {
                    lobby: steamworks::LobbyId::from_raw(1),
                },
                &mut state,
            ),
            None
        );
    }
}
