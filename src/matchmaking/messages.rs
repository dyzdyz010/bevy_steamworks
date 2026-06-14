use std::net::SocketAddrV4;

use bevy_ecs::message::Message;
use thiserror::Error;

use super::*;

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
    /// [`SteamworksMatchmakingOperation::LobbyCreated`] with a request ID. Steam
    /// may also emit [`SteamworksMatchmakingOperation::LobbyCreateCallbackReceived`]
    /// and [`SteamworksMatchmakingOperation::LobbyEnterCallbackReceived`] as
    /// callback observations.
    CreateLobby {
        /// Lobby visibility.
        lobby_type: steamworks::LobbyType,
        /// Maximum lobby members. Steam supports at most 250.
        max_members: u32,
    },
    /// Join a lobby.
    ///
    /// The async command result emits [`SteamworksMatchmakingOperation::LobbyJoined`]
    /// with a request ID. Steam may also emit
    /// [`SteamworksMatchmakingOperation::LobbyEnterCallbackReceived`] as a
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

/// A successfully submitted Steam matchmaking operation or synchronous read.
#[derive(Clone, PartialEq)]
pub enum SteamworksMatchmakingOperation {
    /// Lobby list request was submitted.
    LobbyListRequested {
        /// Unique request ID assigned by the plugin.
        request_id: u64,
        /// Filters applied to the request.
        filter: SteamworksLobbyListFilter,
    },
    /// Lobby list request completed.
    LobbyListReceived {
        /// Unique request ID assigned by the plugin.
        request_id: u64,
        /// Filters applied to the request.
        filter: SteamworksLobbyListFilter,
        /// Matching lobby IDs.
        lobbies: Vec<steamworks::LobbyId>,
    },
    /// Lobby creation was submitted.
    LobbyCreateRequested {
        /// Unique request ID assigned by the plugin.
        request_id: u64,
        /// Lobby visibility.
        lobby_type: steamworks::LobbyType,
        /// Maximum members requested.
        max_members: u32,
    },
    /// Lobby creation completed.
    LobbyCreated {
        /// Unique request ID assigned by the plugin.
        request_id: u64,
        /// Lobby visibility requested.
        lobby_type: steamworks::LobbyType,
        /// Maximum members requested.
        max_members: u32,
        /// Created lobby.
        lobby: steamworks::LobbyId,
    },
    /// Lobby join was submitted.
    LobbyJoinRequested {
        /// Unique request ID assigned by the plugin.
        request_id: u64,
        /// Lobby requested.
        lobby: steamworks::LobbyId,
    },
    /// Lobby join completed.
    LobbyJoined {
        /// Unique request ID assigned by the plugin.
        request_id: u64,
        /// Lobby requested by the command.
        requested_lobby: steamworks::LobbyId,
        /// Joined lobby.
        lobby: steamworks::LobbyId,
    },
    /// Lobby was left.
    LobbyLeft {
        /// Left lobby.
        lobby: steamworks::LobbyId,
    },
    /// Lobby metadata count was read.
    LobbyDataCountRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Metadata entry count.
        count: u32,
    },
    /// Lobby metadata value was read.
    LobbyDataRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Metadata key.
        key: String,
        /// Metadata value, if Steam had one.
        value: Option<String>,
    },
    /// Lobby metadata entry was read by index.
    LobbyDataByIndexRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Metadata entry index.
        index: u32,
        /// Metadata key/value pair, if Steam had one.
        entry: Option<(String, String)>,
    },
    /// All currently cached lobby metadata was read.
    AllLobbyDataRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Metadata key/value pairs.
        entries: Vec<(String, String)>,
    },
    /// Lobby metadata was set.
    LobbyDataSet {
        /// Lobby mutated.
        lobby: steamworks::LobbyId,
        /// Metadata key.
        key: String,
    },
    /// Lobby metadata was deleted.
    LobbyDataDeleted {
        /// Lobby mutated.
        lobby: steamworks::LobbyId,
        /// Metadata key.
        key: String,
    },
    /// Local-user lobby metadata was set.
    LobbyMemberDataSet {
        /// Lobby mutated.
        lobby: steamworks::LobbyId,
        /// Metadata key.
        key: String,
    },
    /// Member metadata was read.
    LobbyMemberDataRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Member inspected.
        user: steamworks::SteamId,
        /// Metadata key.
        key: String,
        /// Metadata value, if Steam had one.
        value: Option<String>,
    },
    /// Lobby member limit was read.
    LobbyMemberLimitRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Member limit, if known.
        limit: Option<usize>,
    },
    /// Lobby owner was read.
    LobbyOwnerRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Owner Steam ID.
        owner: steamworks::SteamId,
    },
    /// Lobby member count was read.
    LobbyMemberCountRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Member count.
        count: usize,
    },
    /// Lobby members were read.
    LobbyMembersListed {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Member Steam IDs.
        members: Vec<steamworks::SteamId>,
    },
    /// Lobby joinability was set.
    LobbyJoinableSet {
        /// Lobby mutated.
        lobby: steamworks::LobbyId,
        /// Joinable value submitted.
        joinable: bool,
    },
    /// Lobby chat message was sent.
    LobbyChatMessageSent {
        /// Lobby sent into.
        lobby: steamworks::LobbyId,
        /// Message length in bytes.
        len: usize,
    },
    /// Lobby chat entry bytes were read.
    LobbyChatEntryRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Chat entry index.
        chat_id: i32,
        /// Message bytes read from Steam.
        data: Vec<u8>,
    },
    /// Lobby game-server data was set.
    LobbyGameServerSet {
        /// Lobby mutated.
        lobby: steamworks::LobbyId,
        /// Game-server data submitted.
        server: SteamworksLobbyGameServer,
    },
    /// Lobby game-server data was read.
    LobbyGameServerRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Game-server data, if Steam had one.
        server: Option<SteamworksLobbyGameServer>,
    },
    /// A lobby created callback was observed.
    LobbyCreateCallbackReceived {
        /// Callback snapshot.
        callback: SteamworksLobbyCreatedCallback,
    },
    /// A lobby enter callback was observed.
    LobbyEnterCallbackReceived {
        /// Callback snapshot.
        callback: SteamworksLobbyEnterCallback,
    },
    /// A lobby chat message callback was observed.
    LobbyChatMessageReceived {
        /// Callback snapshot.
        message: SteamworksLobbyChatMessage,
    },
    /// A lobby membership change callback was observed.
    LobbyChatUpdateReceived {
        /// Callback snapshot.
        update: SteamworksLobbyChatUpdate,
    },
    /// A lobby metadata update callback was observed.
    LobbyDataUpdateReceived {
        /// Callback snapshot.
        update: SteamworksLobbyDataUpdate,
    },
}

impl std::fmt::Debug for SteamworksMatchmakingOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LobbyListRequested { request_id, filter } => f
                .debug_struct("LobbyListRequested")
                .field("request_id", request_id)
                .field("filter", filter)
                .finish(),
            Self::LobbyListReceived {
                request_id,
                filter,
                lobbies,
            } => f
                .debug_struct("LobbyListReceived")
                .field("request_id", request_id)
                .field("filter", filter)
                .field("lobbies", lobbies)
                .finish(),
            Self::LobbyCreateRequested {
                request_id,
                lobby_type,
                max_members,
            } => f
                .debug_struct("LobbyCreateRequested")
                .field("request_id", request_id)
                .field("lobby_type", lobby_type)
                .field("max_members", max_members)
                .finish(),
            Self::LobbyCreated {
                request_id,
                lobby_type,
                max_members,
                lobby,
            } => f
                .debug_struct("LobbyCreated")
                .field("request_id", request_id)
                .field("lobby_type", lobby_type)
                .field("max_members", max_members)
                .field("lobby", lobby)
                .finish(),
            Self::LobbyJoinRequested { request_id, lobby } => f
                .debug_struct("LobbyJoinRequested")
                .field("request_id", request_id)
                .field("lobby", lobby)
                .finish(),
            Self::LobbyJoined {
                request_id,
                requested_lobby,
                lobby,
            } => f
                .debug_struct("LobbyJoined")
                .field("request_id", request_id)
                .field("requested_lobby", requested_lobby)
                .field("lobby", lobby)
                .finish(),
            Self::LobbyLeft { lobby } => f.debug_struct("LobbyLeft").field("lobby", lobby).finish(),
            Self::LobbyDataCountRead { lobby, count } => f
                .debug_struct("LobbyDataCountRead")
                .field("lobby", lobby)
                .field("count", count)
                .finish(),
            Self::LobbyDataRead { lobby, key, value } => f
                .debug_struct("LobbyDataRead")
                .field("lobby", lobby)
                .field("key", key)
                .field("value", value)
                .finish(),
            Self::LobbyDataByIndexRead {
                lobby,
                index,
                entry,
            } => f
                .debug_struct("LobbyDataByIndexRead")
                .field("lobby", lobby)
                .field("index", index)
                .field("entry", entry)
                .finish(),
            Self::AllLobbyDataRead { lobby, entries } => f
                .debug_struct("AllLobbyDataRead")
                .field("lobby", lobby)
                .field("entries", entries)
                .finish(),
            Self::LobbyDataSet { lobby, key } => f
                .debug_struct("LobbyDataSet")
                .field("lobby", lobby)
                .field("key", key)
                .finish(),
            Self::LobbyDataDeleted { lobby, key } => f
                .debug_struct("LobbyDataDeleted")
                .field("lobby", lobby)
                .field("key", key)
                .finish(),
            Self::LobbyMemberDataSet { lobby, key } => f
                .debug_struct("LobbyMemberDataSet")
                .field("lobby", lobby)
                .field("key", key)
                .finish(),
            Self::LobbyMemberDataRead {
                lobby,
                user,
                key,
                value,
            } => f
                .debug_struct("LobbyMemberDataRead")
                .field("lobby", lobby)
                .field("user", user)
                .field("key", key)
                .field("value", value)
                .finish(),
            Self::LobbyMemberLimitRead { lobby, limit } => f
                .debug_struct("LobbyMemberLimitRead")
                .field("lobby", lobby)
                .field("limit", limit)
                .finish(),
            Self::LobbyOwnerRead { lobby, owner } => f
                .debug_struct("LobbyOwnerRead")
                .field("lobby", lobby)
                .field("owner", owner)
                .finish(),
            Self::LobbyMemberCountRead { lobby, count } => f
                .debug_struct("LobbyMemberCountRead")
                .field("lobby", lobby)
                .field("count", count)
                .finish(),
            Self::LobbyMembersListed { lobby, members } => f
                .debug_struct("LobbyMembersListed")
                .field("lobby", lobby)
                .field("members", members)
                .finish(),
            Self::LobbyJoinableSet { lobby, joinable } => f
                .debug_struct("LobbyJoinableSet")
                .field("lobby", lobby)
                .field("joinable", joinable)
                .finish(),
            Self::LobbyChatMessageSent { lobby, len } => f
                .debug_struct("LobbyChatMessageSent")
                .field("lobby", lobby)
                .field("len", len)
                .finish(),
            Self::LobbyChatEntryRead {
                lobby,
                chat_id,
                data,
            } => f
                .debug_struct("LobbyChatEntryRead")
                .field("lobby", lobby)
                .field("chat_id", chat_id)
                .field("data_len", &data.len())
                .finish(),
            Self::LobbyGameServerSet { lobby, server } => f
                .debug_struct("LobbyGameServerSet")
                .field("lobby", lobby)
                .field("server", server)
                .finish(),
            Self::LobbyGameServerRead { lobby, server } => f
                .debug_struct("LobbyGameServerRead")
                .field("lobby", lobby)
                .field("server", server)
                .finish(),
            Self::LobbyCreateCallbackReceived { callback } => f
                .debug_struct("LobbyCreateCallbackReceived")
                .field("callback", callback)
                .finish(),
            Self::LobbyEnterCallbackReceived { callback } => f
                .debug_struct("LobbyEnterCallbackReceived")
                .field("callback", callback)
                .finish(),
            Self::LobbyChatMessageReceived { message } => f
                .debug_struct("LobbyChatMessageReceived")
                .field("message", message)
                .finish(),
            Self::LobbyChatUpdateReceived { update } => f
                .debug_struct("LobbyChatUpdateReceived")
                .field("update", update)
                .finish(),
            Self::LobbyDataUpdateReceived { update } => f
                .debug_struct("LobbyDataUpdateReceived")
                .field("update", update)
                .finish(),
        }
    }
}

/// Result message emitted by [`crate::SteamworksMatchmakingPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksMatchmakingResult {
    /// The command, async call result, or observed callback was processed successfully.
    Ok(SteamworksMatchmakingOperation),
    /// The command failed synchronously or through a Steam async call result.
    Err {
        /// Command that failed.
        command: SteamworksMatchmakingCommand,
        /// Failure reason.
        error: SteamworksMatchmakingError,
    },
}

/// Synchronous and async errors from [`crate::SteamworksMatchmakingPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksMatchmakingError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks matchmaking command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A lobby metadata key is longer than Steam supports.
    #[error("Steamworks lobby key is too long: {key}")]
    LobbyKeyTooLong {
        /// Key rejected by the upstream Steamworks API wrapper.
        key: String,
    },
    /// A lobby creation request exceeded Steam's member limit.
    #[error("Steamworks lobbies support at most {max_supported} members, got {requested}")]
    MaxLobbyMembersExceeded {
        /// Requested member count.
        requested: u32,
        /// Maximum supported member count.
        max_supported: u32,
    },
    /// A lobby list result count exceeded the upstream Steam API wrapper's safe range.
    #[error("Steamworks lobby list result count must be <= {max_supported}, got {requested}")]
    MaxLobbyListResultsExceeded {
        /// Requested result count.
        requested: u64,
        /// Maximum supported result count before upstream integer truncation.
        max_supported: u64,
    },
    /// A lobby chat message length is outside Steam's supported range.
    #[error("Steamworks lobby chat messages must be 1..={max_supported} bytes, got {requested}")]
    InvalidChatMessageLength {
        /// Requested message length.
        requested: usize,
        /// Maximum supported message length.
        max_supported: usize,
    },
    /// The upstream Steamworks API rejected the operation.
    #[error("Steamworks matchmaking operation failed: {operation}")]
    OperationFailed {
        /// Operation that failed.
        operation: &'static str,
    },
    /// The upstream Steamworks API returned an explicit Steam error.
    #[error("Steamworks matchmaking operation {operation} failed: {source}")]
    SteamError {
        /// Operation that failed.
        operation: &'static str,
        /// Steamworks error.
        source: steamworks::SteamError,
    },
}

impl SteamworksMatchmakingError {
    pub(super) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    pub(super) fn lobby_key_too_long(key: impl Into<String>) -> Self {
        Self::LobbyKeyTooLong { key: key.into() }
    }

    pub(super) fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }

    pub(super) fn steam_error(operation: &'static str, source: steamworks::SteamError) -> Self {
        Self::SteamError { operation, source }
    }
}
