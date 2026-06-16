use super::super::{SteamworksMatchmakingError, SteamworksMatchmakingOperation};

pub(super) fn read_lobby_data_count(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
) -> SteamworksMatchmakingOperation {
    SteamworksMatchmakingOperation::LobbyDataCountRead {
        lobby,
        count: matchmaking.lobby_data_count(lobby),
    }
}

pub(super) fn read_lobby_data(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
    key: String,
) -> SteamworksMatchmakingOperation {
    SteamworksMatchmakingOperation::LobbyDataRead {
        lobby,
        value: matchmaking.lobby_data(lobby, &key),
        key,
    }
}

pub(super) fn read_lobby_data_by_index(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
    index: u32,
) -> SteamworksMatchmakingOperation {
    SteamworksMatchmakingOperation::LobbyDataByIndexRead {
        lobby,
        index,
        entry: matchmaking.lobby_data_by_index(lobby, index),
    }
}

pub(super) fn read_all_lobby_data(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
) -> SteamworksMatchmakingOperation {
    let entries = (0..matchmaking.lobby_data_count(lobby))
        .filter_map(|index| matchmaking.lobby_data_by_index(lobby, index))
        .collect();
    SteamworksMatchmakingOperation::AllLobbyDataRead { lobby, entries }
}

pub(super) fn set_lobby_data(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
    key: String,
    value: String,
) -> Result<SteamworksMatchmakingOperation, SteamworksMatchmakingError> {
    if matchmaking.set_lobby_data(lobby, &key, &value) {
        Ok(SteamworksMatchmakingOperation::LobbyDataSet { lobby, key })
    } else {
        Err(SteamworksMatchmakingError::operation_failed(
            "matchmaking.set_lobby_data",
        ))
    }
}

pub(super) fn delete_lobby_data(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
    key: String,
) -> Result<SteamworksMatchmakingOperation, SteamworksMatchmakingError> {
    if matchmaking.delete_lobby_data(lobby, &key) {
        Ok(SteamworksMatchmakingOperation::LobbyDataDeleted { lobby, key })
    } else {
        Err(SteamworksMatchmakingError::operation_failed(
            "matchmaking.delete_lobby_data",
        ))
    }
}
