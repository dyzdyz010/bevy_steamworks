use super::{
    filters::lobby_key, SteamworksLobbyListFilter, SteamworksMatchmakingCommand,
    SteamworksMatchmakingError, MAX_LOBBY_CHAT_MESSAGE_BYTES, MAX_LOBBY_LIST_RESULTS,
    MAX_LOBBY_MEMBERS,
};

pub(super) fn validate_command(
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

pub(super) fn validate_filter(
    filter: &SteamworksLobbyListFilter,
) -> Result<(), SteamworksMatchmakingError> {
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
