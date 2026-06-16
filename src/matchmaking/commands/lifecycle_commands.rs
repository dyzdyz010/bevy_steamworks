use super::super::{
    async_results::SteamworksMatchmakingAsyncResults, filters::apply_lobby_list_filter,
    SteamworksMatchmakingCommand, SteamworksMatchmakingError, SteamworksMatchmakingOperation,
    SteamworksMatchmakingResult,
};

pub(super) fn request_lobby_list(
    matchmaking: &steamworks::Matchmaking,
    async_results: &SteamworksMatchmakingAsyncResults,
    request_id: u64,
    filter: super::super::SteamworksLobbyListFilter,
) -> Result<SteamworksMatchmakingOperation, SteamworksMatchmakingError> {
    apply_lobby_list_filter(matchmaking, &filter)?;
    let async_results = async_results.clone();
    let command = SteamworksMatchmakingCommand::RequestLobbyList {
        filter: filter.clone(),
    };
    let request_filter = filter.clone();
    matchmaking.request_lobby_list(move |result| {
        async_results.push(match result {
            Ok(lobbies) => {
                SteamworksMatchmakingResult::Ok(SteamworksMatchmakingOperation::LobbyListReceived {
                    request_id,
                    filter: request_filter.clone(),
                    lobbies,
                })
            }
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

pub(super) fn create_lobby(
    matchmaking: &steamworks::Matchmaking,
    async_results: &SteamworksMatchmakingAsyncResults,
    request_id: u64,
    lobby_type: steamworks::LobbyType,
    max_members: u32,
) -> SteamworksMatchmakingOperation {
    let async_results = async_results.clone();
    let command = SteamworksMatchmakingCommand::CreateLobby {
        lobby_type,
        max_members,
    };
    matchmaking.create_lobby(lobby_type, max_members, move |result| {
        async_results.push(match result {
            Ok(lobby) => {
                SteamworksMatchmakingResult::Ok(SteamworksMatchmakingOperation::LobbyCreated {
                    request_id,
                    lobby_type,
                    max_members,
                    lobby,
                })
            }
            Err(source) => SteamworksMatchmakingResult::Err {
                command,
                error: SteamworksMatchmakingError::steam_error("matchmaking.create_lobby", source),
            },
        });
    });
    SteamworksMatchmakingOperation::LobbyCreateRequested {
        request_id,
        lobby_type,
        max_members,
    }
}

pub(super) fn join_lobby(
    matchmaking: &steamworks::Matchmaking,
    async_results: &SteamworksMatchmakingAsyncResults,
    request_id: u64,
    lobby: steamworks::LobbyId,
) -> SteamworksMatchmakingOperation {
    let async_results = async_results.clone();
    let command = SteamworksMatchmakingCommand::JoinLobby { lobby };
    let requested_lobby = lobby;
    matchmaking.join_lobby(lobby, move |result| {
        async_results.push(match result {
            Ok(lobby) => {
                SteamworksMatchmakingResult::Ok(SteamworksMatchmakingOperation::LobbyJoined {
                    request_id,
                    requested_lobby,
                    lobby,
                })
            }
            Err(()) => SteamworksMatchmakingResult::Err {
                command,
                error: SteamworksMatchmakingError::operation_failed("matchmaking.join_lobby"),
            },
        });
    });
    SteamworksMatchmakingOperation::LobbyJoinRequested { request_id, lobby }
}

pub(super) fn leave_lobby(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
) -> SteamworksMatchmakingOperation {
    matchmaking.leave_lobby(lobby);
    SteamworksMatchmakingOperation::LobbyLeft { lobby }
}
