use std::net::SocketAddrV4;

/// Owned lobby-list filters for [`crate::SteamworksMatchmakingCommand::RequestLobbyList`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SteamworksLobbyListFilter {
    /// String lobby metadata filters.
    pub string: Vec<SteamworksLobbyStringFilter>,
    /// Numeric lobby metadata filters.
    pub number: Vec<SteamworksLobbyNumberFilter>,
    /// Near-value sort filters.
    pub near_value: Vec<SteamworksLobbyNearFilter>,
    /// Minimum available open slots.
    pub open_slots: Option<u8>,
    /// Distance bucket used by Steam's lobby search.
    pub distance: Option<steamworks::DistanceFilter>,
    /// Maximum number of lobby results to return.
    pub max_results: Option<u64>,
}

impl SteamworksLobbyListFilter {
    /// Creates an empty lobby-list filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a string metadata filter.
    pub fn with_string(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
        comparison: steamworks::StringFilterKind,
    ) -> Self {
        self.string.push(SteamworksLobbyStringFilter {
            key: key.into(),
            value: value.into(),
            comparison,
        });
        self
    }

    /// Adds a numeric metadata filter.
    pub fn with_number(
        mut self,
        key: impl Into<String>,
        value: i32,
        comparison: steamworks::ComparisonFilter,
    ) -> Self {
        self.number.push(SteamworksLobbyNumberFilter {
            key: key.into(),
            value,
            comparison,
        });
        self
    }

    /// Adds a near-value sort filter.
    pub fn with_near_value(mut self, key: impl Into<String>, value: i32) -> Self {
        self.near_value.push(SteamworksLobbyNearFilter {
            key: key.into(),
            value,
        });
        self
    }

    /// Sets the minimum available open slots.
    pub fn with_open_slots(mut self, open_slots: u8) -> Self {
        self.open_slots = Some(open_slots);
        self
    }

    /// Sets the Steam lobby search distance.
    pub fn with_distance(mut self, distance: steamworks::DistanceFilter) -> Self {
        self.distance = Some(distance);
        self
    }

    /// Sets the maximum number of lobby results.
    pub fn with_max_results(mut self, max_results: u64) -> Self {
        self.max_results = Some(max_results);
        self
    }
}

/// A string lobby metadata filter.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyStringFilter {
    /// Lobby metadata key.
    pub key: String,
    /// Lobby metadata value.
    pub value: String,
    /// String comparison mode.
    pub comparison: steamworks::StringFilterKind,
}

/// A numeric lobby metadata filter.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyNumberFilter {
    /// Lobby metadata key.
    pub key: String,
    /// Numeric comparison value.
    pub value: i32,
    /// Numeric comparison mode.
    pub comparison: steamworks::ComparisonFilter,
}

/// A near-value lobby sort filter.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyNearFilter {
    /// Lobby metadata key.
    pub key: String,
    /// Value used for proximity sorting.
    pub value: i32,
}

/// Game-server data associated with a Steam lobby.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyGameServer {
    /// Server IPv4 address and port.
    pub address: SocketAddrV4,
    /// Optional Steam ID for the game server.
    pub steam_id: Option<steamworks::SteamId>,
}

/// Lobby created callback snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyCreatedCallback {
    /// Raw Steam result code reported by the upstream callback.
    pub result: u32,
    /// Lobby created by Steam, or zero when creation failed.
    pub lobby: steamworks::LobbyId,
}

/// Lobby enter callback snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyEnterCallback {
    /// Lobby entered by the local user.
    pub lobby: steamworks::LobbyId,
    /// Raw chat permissions reported by Steam.
    pub chat_permissions: u32,
    /// Whether Steam reported the lobby as locked.
    pub blocked: bool,
    /// Steam lobby enter response.
    pub chat_room_enter_response: steamworks::ChatRoomEnterResponse,
}

/// Lobby chat message callback snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyChatMessage {
    /// Lobby that received the chat entry.
    pub lobby: steamworks::LobbyId,
    /// User who sent the message.
    pub user: steamworks::SteamId,
    /// Chat entry kind reported by Steam.
    pub chat_entry_type: steamworks::ChatEntryType,
    /// Chat entry index reported by Steam.
    pub chat_id: i32,
}

/// Lobby member state change callback snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyChatUpdate {
    /// Lobby where the membership change occurred.
    pub lobby: steamworks::LobbyId,
    /// User whose lobby state changed.
    pub user_changed: steamworks::SteamId,
    /// User who caused the change.
    pub making_change: steamworks::SteamId,
    /// Member state change reported by Steam.
    pub member_state_change: steamworks::ChatMemberStateChange,
}

/// Lobby metadata update callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyDataUpdate {
    /// Lobby whose metadata changed.
    pub lobby: steamworks::LobbyId,
    /// Lobby member whose data changed, or the lobby ID when room data changed.
    pub member: steamworks::SteamId,
    /// Whether Steam reported the metadata update as successful.
    pub success: bool,
}

/// Submitted lobby-list request context.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyListRequest {
    /// Unique request ID assigned by the plugin.
    pub request_id: u64,
    /// Filters applied to the request.
    pub filter: SteamworksLobbyListFilter,
}

/// Submitted lobby-create request context.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyCreateRequest {
    /// Unique request ID assigned by the plugin.
    pub request_id: u64,
    /// Lobby visibility requested.
    pub lobby_type: steamworks::LobbyType,
    /// Maximum members requested.
    pub max_members: u32,
}

/// Submitted lobby-join request context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksMatchmakingLobbyJoinRequest {
    /// Unique request ID assigned by the plugin.
    pub request_id: u64,
    /// Lobby requested by the command.
    pub lobby: steamworks::LobbyId,
}

/// Completed lobby creation context.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyCreated {
    /// Unique request ID assigned by the plugin.
    pub request_id: u64,
    /// Lobby visibility requested.
    pub lobby_type: steamworks::LobbyType,
    /// Maximum members requested.
    pub max_members: u32,
    /// Created lobby.
    pub lobby: steamworks::LobbyId,
}

/// Completed lobby join context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyJoined {
    /// Unique request ID assigned by the plugin.
    pub request_id: u64,
    /// Lobby requested by the command.
    pub requested_lobby: steamworks::LobbyId,
    /// Lobby joined according to Steam.
    pub lobby: steamworks::LobbyId,
}

/// Lobby metadata count snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyDataCount {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Metadata entry count.
    pub count: u32,
}

/// Lobby metadata value snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyDataValue {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Metadata key.
    pub key: String,
    /// Metadata value, if Steam had one.
    pub value: Option<String>,
}

/// Lobby metadata entry snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyDataEntry {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Metadata entry index.
    pub index: u32,
    /// Metadata key/value pair, if Steam had one.
    pub entry: Option<(String, String)>,
}

/// Full lobby metadata snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyDataEntries {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Metadata key/value pairs.
    pub entries: Vec<(String, String)>,
}

/// Lobby metadata mutation snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyDataMutation {
    /// Lobby mutated.
    pub lobby: steamworks::LobbyId,
    /// Metadata key.
    pub key: String,
}

/// Lobby member metadata value snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyMemberDataValue {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Member inspected.
    pub user: steamworks::SteamId,
    /// Metadata key.
    pub key: String,
    /// Metadata value, if Steam had one.
    pub value: Option<String>,
}

/// Lobby member limit snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyMemberLimit {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Member limit, if known.
    pub limit: Option<usize>,
}

/// Lobby owner snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyOwner {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Owner Steam ID.
    pub owner: steamworks::SteamId,
}

/// Lobby member count snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyMemberCount {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Member count.
    pub count: usize,
}

/// Lobby member list snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyMembers {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Member Steam IDs.
    pub members: Vec<steamworks::SteamId>,
}

/// Lobby joinability mutation snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyJoinability {
    /// Lobby mutated.
    pub lobby: steamworks::LobbyId,
    /// Joinable value submitted.
    pub joinable: bool,
}

/// Lobby chat send snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyChatMessageSent {
    /// Lobby sent into.
    pub lobby: steamworks::LobbyId,
    /// Message length in bytes.
    pub len: usize,
}

/// Lobby chat entry bytes snapshot.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksLobbyChatEntry {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Chat entry index.
    pub chat_id: i32,
    /// Message bytes read from Steam.
    pub data: Vec<u8>,
}

impl std::fmt::Debug for SteamworksLobbyChatEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SteamworksLobbyChatEntry")
            .field("lobby", &self.lobby)
            .field("chat_id", &self.chat_id)
            .field("data_len", &self.data.len())
            .finish()
    }
}

/// Lobby game-server assignment snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyGameServerAssignment {
    /// Lobby mutated.
    pub lobby: steamworks::LobbyId,
    /// Game-server data submitted.
    pub server: SteamworksLobbyGameServer,
}

/// Lobby game-server lookup snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyGameServerLookup {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Game-server data, if Steam had one.
    pub server: Option<SteamworksLobbyGameServer>,
}
