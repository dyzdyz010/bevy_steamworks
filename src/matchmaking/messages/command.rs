use std::net::SocketAddrV4;

use bevy_ecs::message::Message;

use super::super::SteamworksLobbyListFilter;

/// A high-level command for Steam matchmaking and lobbies.
#[derive(Clone, Message, PartialEq)]
pub enum SteamworksMatchmakingCommand {
    /// Request a lobby list from Steam.
    RequestLobbyList {
        /// Owned filters to apply before requesting the lobby list.
        filter: SteamworksLobbyListFilter,
    },
    /// Create a lobby.
    ///
    /// The async command result emits
    /// [`super::SteamworksMatchmakingOperation::LobbyCreated`] with a request ID. Steam
    /// may also emit [`super::SteamworksMatchmakingOperation::LobbyCreateCallbackReceived`]
    /// and [`super::SteamworksMatchmakingOperation::LobbyEnterCallbackReceived`] as
    /// callback observations.
    CreateLobby {
        /// Lobby visibility.
        lobby_type: steamworks::LobbyType,
        /// Maximum lobby members. Steam supports at most 250.
        max_members: u32,
    },
    /// Join a lobby.
    ///
    /// The async command result emits [`super::SteamworksMatchmakingOperation::LobbyJoined`]
    /// with a request ID. Steam may also emit
    /// [`super::SteamworksMatchmakingOperation::LobbyEnterCallbackReceived`] as a
    /// callback observation.
    JoinLobby {
        /// Lobby to join.
        lobby: steamworks::LobbyId,
    },
    /// Leave a lobby.
    LeaveLobby {
        /// Lobby to leave.
        lobby: steamworks::LobbyId,
    },
    /// Read the number of lobby metadata entries.
    GetLobbyDataCount {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
    },
    /// Read one lobby metadata value.
    GetLobbyData {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
        /// Metadata key.
        key: String,
    },
    /// Read one lobby metadata entry by index.
    GetLobbyDataByIndex {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
        /// Metadata entry index.
        index: u32,
    },
    /// Read all lobby metadata entries currently cached by Steam.
    GetAllLobbyData {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
    },
    /// Set one lobby metadata value.
    SetLobbyData {
        /// Lobby to mutate.
        lobby: steamworks::LobbyId,
        /// Metadata key.
        key: String,
        /// Metadata value.
        value: String,
    },
    /// Delete one lobby metadata value.
    DeleteLobbyData {
        /// Lobby to mutate.
        lobby: steamworks::LobbyId,
        /// Metadata key.
        key: String,
    },
    /// Set local-user metadata inside a lobby.
    SetLobbyMemberData {
        /// Lobby to mutate.
        lobby: steamworks::LobbyId,
        /// Metadata key.
        key: String,
        /// Metadata value.
        value: String,
    },
    /// Read one member metadata value.
    GetLobbyMemberData {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
        /// Member to inspect.
        user: steamworks::SteamId,
        /// Metadata key.
        key: String,
    },
    /// Read a lobby's member limit.
    GetLobbyMemberLimit {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
    },
    /// Read a lobby's owner.
    GetLobbyOwner {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
    },
    /// Read a lobby's member count.
    GetLobbyMemberCount {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
    },
    /// Read all currently known lobby members.
    ListLobbyMembers {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
    },
    /// Set whether a lobby is joinable.
    SetLobbyJoinable {
        /// Lobby to mutate.
        lobby: steamworks::LobbyId,
        /// Whether the lobby should be joinable.
        joinable: bool,
    },
    /// Send a lobby chat message.
    SendLobbyChatMessage {
        /// Lobby to send into.
        lobby: steamworks::LobbyId,
        /// Message bytes. Steam supports up to 4096 bytes.
        data: Vec<u8>,
    },
    /// Read the bytes for a lobby chat entry.
    ///
    /// Steam treats chat entry IDs as callback-scope values. Prefer a lower-level
    /// callback registered through [`crate::SteamworksCallbackRegistry`] when
    /// bytes must be copied immediately and reliably. This command is retained
    /// for callers that know the entry is still available through Steam's lobby
    /// cache.
    GetLobbyChatEntry {
        /// Lobby that received the chat entry.
        lobby: steamworks::LobbyId,
        /// Chat entry index from the Steam callback.
        chat_id: i32,
        /// Maximum bytes to read, up to 4096.
        max_bytes: usize,
    },
    /// Set game-server information for a lobby.
    SetLobbyGameServer {
        /// Lobby to mutate.
        lobby: steamworks::LobbyId,
        /// Server IPv4 address and port.
        address: SocketAddrV4,
        /// Optional Steam ID for the game server.
        steam_id: Option<steamworks::SteamId>,
    },
    /// Read game-server information for a lobby.
    GetLobbyGameServer {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
    },
}

impl std::fmt::Debug for SteamworksMatchmakingCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestLobbyList { filter } => f
                .debug_struct("RequestLobbyList")
                .field("filter", filter)
                .finish(),
            Self::CreateLobby {
                lobby_type,
                max_members,
            } => f
                .debug_struct("CreateLobby")
                .field("lobby_type", lobby_type)
                .field("max_members", max_members)
                .finish(),
            Self::JoinLobby { lobby } => f.debug_struct("JoinLobby").field("lobby", lobby).finish(),
            Self::LeaveLobby { lobby } => {
                f.debug_struct("LeaveLobby").field("lobby", lobby).finish()
            }
            Self::GetLobbyDataCount { lobby } => f
                .debug_struct("GetLobbyDataCount")
                .field("lobby", lobby)
                .finish(),
            Self::GetLobbyData { lobby, key } => f
                .debug_struct("GetLobbyData")
                .field("lobby", lobby)
                .field("key", key)
                .finish(),
            Self::GetLobbyDataByIndex { lobby, index } => f
                .debug_struct("GetLobbyDataByIndex")
                .field("lobby", lobby)
                .field("index", index)
                .finish(),
            Self::GetAllLobbyData { lobby } => f
                .debug_struct("GetAllLobbyData")
                .field("lobby", lobby)
                .finish(),
            Self::SetLobbyData { lobby, key, value } => f
                .debug_struct("SetLobbyData")
                .field("lobby", lobby)
                .field("key", key)
                .field("value", value)
                .finish(),
            Self::DeleteLobbyData { lobby, key } => f
                .debug_struct("DeleteLobbyData")
                .field("lobby", lobby)
                .field("key", key)
                .finish(),
            Self::SetLobbyMemberData { lobby, key, value } => f
                .debug_struct("SetLobbyMemberData")
                .field("lobby", lobby)
                .field("key", key)
                .field("value", value)
                .finish(),
            Self::GetLobbyMemberData { lobby, user, key } => f
                .debug_struct("GetLobbyMemberData")
                .field("lobby", lobby)
                .field("user", user)
                .field("key", key)
                .finish(),
            Self::GetLobbyMemberLimit { lobby } => f
                .debug_struct("GetLobbyMemberLimit")
                .field("lobby", lobby)
                .finish(),
            Self::GetLobbyOwner { lobby } => f
                .debug_struct("GetLobbyOwner")
                .field("lobby", lobby)
                .finish(),
            Self::GetLobbyMemberCount { lobby } => f
                .debug_struct("GetLobbyMemberCount")
                .field("lobby", lobby)
                .finish(),
            Self::ListLobbyMembers { lobby } => f
                .debug_struct("ListLobbyMembers")
                .field("lobby", lobby)
                .finish(),
            Self::SetLobbyJoinable { lobby, joinable } => f
                .debug_struct("SetLobbyJoinable")
                .field("lobby", lobby)
                .field("joinable", joinable)
                .finish(),
            Self::SendLobbyChatMessage { lobby, data } => f
                .debug_struct("SendLobbyChatMessage")
                .field("lobby", lobby)
                .field("data_len", &data.len())
                .finish(),
            Self::GetLobbyChatEntry {
                lobby,
                chat_id,
                max_bytes,
            } => f
                .debug_struct("GetLobbyChatEntry")
                .field("lobby", lobby)
                .field("chat_id", chat_id)
                .field("max_bytes", max_bytes)
                .finish(),
            Self::SetLobbyGameServer {
                lobby,
                address,
                steam_id,
            } => f
                .debug_struct("SetLobbyGameServer")
                .field("lobby", lobby)
                .field("address", address)
                .field("steam_id", steam_id)
                .finish(),
            Self::GetLobbyGameServer { lobby } => f
                .debug_struct("GetLobbyGameServer")
                .field("lobby", lobby)
                .finish(),
        }
    }
}

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

    /// Creates a [`SteamworksMatchmakingCommand::GetLobbyData`] command.
    pub fn get_lobby_data(lobby: steamworks::LobbyId, key: impl Into<String>) -> Self {
        Self::GetLobbyData {
            lobby,
            key: key.into(),
        }
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

    /// Creates a [`SteamworksMatchmakingCommand::SendLobbyChatMessage`] command.
    pub fn send_lobby_chat_message(lobby: steamworks::LobbyId, data: impl Into<Vec<u8>>) -> Self {
        Self::SendLobbyChatMessage {
            lobby,
            data: data.into(),
        }
    }
}
