use super::super::{
    SteamworksLobbyChatMessage, SteamworksLobbyChatUpdate, SteamworksLobbyCreatedCallback,
    SteamworksLobbyDataUpdate, SteamworksLobbyEnterCallback, SteamworksLobbyGameServer,
    SteamworksLobbyListFilter,
};

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
