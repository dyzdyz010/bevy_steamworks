use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::{SteamworksClient, SteamworksEvent};

mod chat_commands;
mod data_commands;
mod lifecycle_commands;
mod member_commands;
mod server_commands;

use super::{
    async_results::SteamworksMatchmakingAsyncResults, callbacks::process_matchmaking_steam_events,
    validation::validate_command, SteamworksMatchmakingCommand, SteamworksMatchmakingError,
    SteamworksMatchmakingOperation, SteamworksMatchmakingResult, SteamworksMatchmakingState,
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
            lifecycle_commands::request_lobby_list(
                &matchmaking,
                async_results,
                request_id.expect("async matchmaking command missing request id"),
                filter,
            )
        }
        SteamworksMatchmakingCommand::CreateLobby {
            lobby_type,
            max_members,
        } => Ok(lifecycle_commands::create_lobby(
            &matchmaking,
            async_results,
            request_id.expect("async matchmaking command missing request id"),
            lobby_type,
            max_members,
        )),
        SteamworksMatchmakingCommand::JoinLobby { lobby } => Ok(lifecycle_commands::join_lobby(
            &matchmaking,
            async_results,
            request_id.expect("async matchmaking command missing request id"),
            lobby,
        )),
        SteamworksMatchmakingCommand::LeaveLobby { lobby } => {
            Ok(lifecycle_commands::leave_lobby(&matchmaking, lobby))
        }
        SteamworksMatchmakingCommand::GetLobbyDataCount { lobby } => {
            Ok(data_commands::read_lobby_data_count(&matchmaking, lobby))
        }
        SteamworksMatchmakingCommand::GetLobbyData { lobby, key } => {
            Ok(data_commands::read_lobby_data(&matchmaking, lobby, key))
        }
        SteamworksMatchmakingCommand::GetLobbyDataByIndex { lobby, index } => Ok(
            data_commands::read_lobby_data_by_index(&matchmaking, lobby, index),
        ),
        SteamworksMatchmakingCommand::GetAllLobbyData { lobby } => {
            Ok(data_commands::read_all_lobby_data(&matchmaking, lobby))
        }
        SteamworksMatchmakingCommand::SetLobbyData { lobby, key, value } => {
            data_commands::set_lobby_data(&matchmaking, lobby, key, value)
        }
        SteamworksMatchmakingCommand::DeleteLobbyData { lobby, key } => {
            data_commands::delete_lobby_data(&matchmaking, lobby, key)
        }
        SteamworksMatchmakingCommand::SetLobbyMemberData { lobby, key, value } => Ok(
            member_commands::set_lobby_member_data(&matchmaking, lobby, key, value),
        ),
        SteamworksMatchmakingCommand::GetLobbyMemberData { lobby, user, key } => Ok(
            member_commands::read_lobby_member_data(&matchmaking, lobby, user, key),
        ),
        SteamworksMatchmakingCommand::GetLobbyMemberLimit { lobby } => Ok(
            member_commands::read_lobby_member_limit(&matchmaking, lobby),
        ),
        SteamworksMatchmakingCommand::GetLobbyOwner { lobby } => {
            Ok(member_commands::read_lobby_owner(&matchmaking, lobby))
        }
        SteamworksMatchmakingCommand::GetLobbyMemberCount { lobby } => Ok(
            member_commands::read_lobby_member_count(&matchmaking, lobby),
        ),
        SteamworksMatchmakingCommand::ListLobbyMembers { lobby } => {
            Ok(member_commands::list_lobby_members(&matchmaking, lobby))
        }
        SteamworksMatchmakingCommand::SetLobbyJoinable { lobby, joinable } => {
            chat_commands::set_lobby_joinable(&matchmaking, lobby, joinable)
        }
        SteamworksMatchmakingCommand::SendLobbyChatMessage { lobby, data } => {
            chat_commands::send_lobby_chat_message(&matchmaking, lobby, data)
        }
        SteamworksMatchmakingCommand::GetLobbyChatEntry {
            lobby,
            chat_id,
            max_bytes,
        } => Ok(chat_commands::read_lobby_chat_entry(
            &matchmaking,
            lobby,
            chat_id,
            max_bytes,
        )),
        SteamworksMatchmakingCommand::SetLobbyGameServer {
            lobby,
            address,
            steam_id,
        } => Ok(server_commands::set_lobby_game_server(
            &matchmaking,
            lobby,
            address,
            steam_id,
        )),
        SteamworksMatchmakingCommand::GetLobbyGameServer { lobby } => {
            Ok(server_commands::read_lobby_game_server(&matchmaking, lobby))
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
