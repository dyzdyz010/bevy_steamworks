use std::net::SocketAddrV4;

use crate::matchmaking::SteamworksLobbyListFilter;

use super::SteamworksMatchmakingCommand;

impl SteamworksMatchmakingCommand {
    /// Creates a [`SteamworksMatchmakingCommand::RequestLobbyList`] command.
    pub fn request_lobby_list(filter: SteamworksLobbyListFilter) -> Self {
        Self::RequestLobbyList { filter }
    }

    /// Creates a [`SteamworksMatchmakingCommand::CreateLobby`] command.
    pub fn create_lobby(lobby_type: steamworks::LobbyType, max_members: u32) -> Self {
        Self::CreateLobby {
            lobby_type,
            max_members,
        }
    }

    /// Creates a [`SteamworksMatchmakingCommand::JoinLobby`] command.
    pub fn join_lobby(lobby: steamworks::LobbyId) -> Self {
        Self::JoinLobby { lobby }
    }

    /// Creates a [`SteamworksMatchmakingCommand::LeaveLobby`] command.
    pub fn leave_lobby(lobby: steamworks::LobbyId) -> Self {
        Self::LeaveLobby { lobby }
    }

    /// Creates a [`SteamworksMatchmakingCommand::GetLobbyDataCount`] command.
    pub fn get_lobby_data_count(lobby: steamworks::LobbyId) -> Self {
        Self::GetLobbyDataCount { lobby }
    }

    /// Creates a [`SteamworksMatchmakingCommand::GetLobbyData`] command.
    pub fn get_lobby_data(lobby: steamworks::LobbyId, key: impl Into<String>) -> Self {
        Self::GetLobbyData {
            lobby,
            key: key.into(),
        }
    }

    /// Creates a [`SteamworksMatchmakingCommand::GetLobbyDataByIndex`] command.
    pub fn get_lobby_data_by_index(lobby: steamworks::LobbyId, index: u32) -> Self {
        Self::GetLobbyDataByIndex { lobby, index }
    }

    /// Creates a [`SteamworksMatchmakingCommand::GetAllLobbyData`] command.
    pub fn get_all_lobby_data(lobby: steamworks::LobbyId) -> Self {
        Self::GetAllLobbyData { lobby }
    }

    /// Creates a [`SteamworksMatchmakingCommand::SetLobbyData`] command.
    pub fn set_lobby_data(
        lobby: steamworks::LobbyId,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self::SetLobbyData {
            lobby,
            key: key.into(),
            value: value.into(),
        }
    }

    /// Creates a [`SteamworksMatchmakingCommand::DeleteLobbyData`] command.
    pub fn delete_lobby_data(lobby: steamworks::LobbyId, key: impl Into<String>) -> Self {
        Self::DeleteLobbyData {
            lobby,
            key: key.into(),
        }
    }

    /// Creates a [`SteamworksMatchmakingCommand::SetLobbyMemberData`] command.
    pub fn set_lobby_member_data(
        lobby: steamworks::LobbyId,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self::SetLobbyMemberData {
            lobby,
            key: key.into(),
            value: value.into(),
        }
    }

    /// Creates a [`SteamworksMatchmakingCommand::GetLobbyMemberData`] command.
    pub fn get_lobby_member_data(
        lobby: steamworks::LobbyId,
        user: steamworks::SteamId,
        key: impl Into<String>,
    ) -> Self {
        Self::GetLobbyMemberData {
            lobby,
            user,
            key: key.into(),
        }
    }

    /// Creates a [`SteamworksMatchmakingCommand::GetLobbyMemberLimit`] command.
    pub fn get_lobby_member_limit(lobby: steamworks::LobbyId) -> Self {
        Self::GetLobbyMemberLimit { lobby }
    }

    /// Creates a [`SteamworksMatchmakingCommand::GetLobbyOwner`] command.
    pub fn get_lobby_owner(lobby: steamworks::LobbyId) -> Self {
        Self::GetLobbyOwner { lobby }
    }

    /// Creates a [`SteamworksMatchmakingCommand::GetLobbyMemberCount`] command.
    pub fn get_lobby_member_count(lobby: steamworks::LobbyId) -> Self {
        Self::GetLobbyMemberCount { lobby }
    }

    /// Creates a [`SteamworksMatchmakingCommand::ListLobbyMembers`] command.
    pub fn list_lobby_members(lobby: steamworks::LobbyId) -> Self {
        Self::ListLobbyMembers { lobby }
    }

    /// Creates a [`SteamworksMatchmakingCommand::SetLobbyJoinable`] command.
    pub fn set_lobby_joinable(lobby: steamworks::LobbyId, joinable: bool) -> Self {
        Self::SetLobbyJoinable { lobby, joinable }
    }

    /// Creates a [`SteamworksMatchmakingCommand::SendLobbyChatMessage`] command.
    pub fn send_lobby_chat_message(lobby: steamworks::LobbyId, data: impl Into<Vec<u8>>) -> Self {
        Self::SendLobbyChatMessage {
            lobby,
            data: data.into(),
        }
    }

    /// Creates a [`SteamworksMatchmakingCommand::GetLobbyChatEntry`] command.
    pub fn get_lobby_chat_entry(
        lobby: steamworks::LobbyId,
        chat_id: i32,
        max_bytes: usize,
    ) -> Self {
        Self::GetLobbyChatEntry {
            lobby,
            chat_id,
            max_bytes,
        }
    }

    /// Creates a [`SteamworksMatchmakingCommand::SetLobbyGameServer`] command.
    pub fn set_lobby_game_server(
        lobby: steamworks::LobbyId,
        address: SocketAddrV4,
        steam_id: Option<steamworks::SteamId>,
    ) -> Self {
        Self::SetLobbyGameServer {
            lobby,
            address,
            steam_id,
        }
    }

    /// Creates a [`SteamworksMatchmakingCommand::GetLobbyGameServer`] command.
    pub fn get_lobby_game_server(lobby: steamworks::LobbyId) -> Self {
        Self::GetLobbyGameServer { lobby }
    }
}
