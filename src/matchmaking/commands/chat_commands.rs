use super::super::{SteamworksMatchmakingError, SteamworksMatchmakingOperation};

pub(super) fn set_lobby_joinable(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
    joinable: bool,
) -> Result<SteamworksMatchmakingOperation, SteamworksMatchmakingError> {
    if matchmaking.set_lobby_joinable(lobby, joinable) {
        Ok(SteamworksMatchmakingOperation::LobbyJoinableSet { lobby, joinable })
    } else {
        Err(SteamworksMatchmakingError::operation_failed(
            "matchmaking.set_lobby_joinable",
        ))
    }
}

pub(super) fn send_lobby_chat_message(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
    data: Vec<u8>,
) -> Result<SteamworksMatchmakingOperation, SteamworksMatchmakingError> {
    matchmaking
        .send_lobby_chat_message(lobby, &data)
        .map(|()| SteamworksMatchmakingOperation::LobbyChatMessageSent {
            lobby,
            len: data.len(),
        })
        .map_err(|source| {
            SteamworksMatchmakingError::steam_error("matchmaking.send_lobby_chat_message", source)
        })
}

pub(super) fn read_lobby_chat_entry(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
    chat_id: i32,
    max_bytes: usize,
) -> SteamworksMatchmakingOperation {
    let mut buffer = vec![0; max_bytes];
    let data = matchmaking
        .get_lobby_chat_entry(lobby, chat_id, &mut buffer)
        .to_vec();
    SteamworksMatchmakingOperation::LobbyChatEntryRead {
        lobby,
        chat_id,
        data,
    }
}
