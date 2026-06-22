use std::net::SocketAddrV4;

use bevy_ecs::message::Message;

use super::super::SteamworksLobbyListFilter;

mod constructors;
mod debug;

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
