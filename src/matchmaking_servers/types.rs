use std::{
    collections::{BTreeMap, HashMap},
    net::Ipv4Addr,
    time::Duration,
};

/// Opaque ID for a server-list request handle owned by this plugin.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SteamworksServerListRequestId(u64);

impl SteamworksServerListRequestId {
    /// Creates an ID from a raw integer.
    pub fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw integer backing this plugin-owned ID.
    pub fn raw(self) -> u64 {
        self.0
    }
}

/// Submitted server-list request context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksServerListRequestInfo {
    /// Plugin-owned request ID.
    pub request: SteamworksServerListRequestId,
    /// Steam app ID queried.
    pub app_id: steamworks::AppId,
    /// Server-list source.
    pub kind: SteamworksServerListKind,
    /// Filters applied to the request.
    pub filters: SteamworksServerListFilters,
}

/// Server-list request plus server index context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SteamworksServerListServerIndex {
    /// Plugin-owned request ID.
    pub request: SteamworksServerListRequestId,
    /// Server index inside the request.
    pub server_index: i32,
}

/// Opaque ID for a single-server query owned by this plugin.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SteamworksServerQueryId(u64);

impl SteamworksServerQueryId {
    /// Creates an ID from a raw integer.
    pub fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw integer backing this plugin-owned ID.
    pub fn raw(self) -> u64 {
        self.0
    }
}

/// Target address for direct Steam matchmaking server queries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SteamworksServerQueryTarget {
    /// IPv4 address of the server.
    pub address: Ipv4Addr,
    /// Query port of the server.
    pub query_port: u16,
}

/// Direct server query kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SteamworksServerQueryKind {
    /// Ping one server and read its server item snapshot.
    Ping,
    /// Query player details from one server.
    PlayerDetails,
    /// Query server rules from one server.
    Rules,
}

/// Submitted direct server query context.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SteamworksServerQueryInfo {
    /// Plugin-owned query ID.
    pub query: SteamworksServerQueryId,
    /// Direct query kind.
    pub kind: SteamworksServerQueryKind,
    /// Target server endpoint.
    pub target: SteamworksServerQueryTarget,
}

/// Server-list count read context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SteamworksServerListCount {
    /// Plugin-owned request ID.
    pub request: SteamworksServerListRequestId,
    /// Server count reported by Steam.
    pub count: i32,
}

/// Server-list refreshing state read context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SteamworksServerListRefreshing {
    /// Plugin-owned request ID.
    pub request: SteamworksServerListRequestId,
    /// Whether the request is currently refreshing.
    pub refreshing: bool,
}

/// Server-list source supported by Steam Matchmaking Servers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SteamworksServerListKind {
    /// Query servers on the local network.
    Lan,
    /// Query the public Internet master server.
    Internet,
    /// Query the user's favorite servers.
    Favorites,
    /// Query the user's server history.
    History,
    /// Query servers associated with Steam friends.
    Friends,
}

/// Owned keyed filter set for Steam server-list requests.
///
/// The upstream `steamworks` wrapper accepts server-list filters as a map, so
/// this type intentionally models simple unique key/value filters. It does not
/// represent repeated or order-sensitive boolean filter clauses.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SteamworksServerListFilters {
    entries: BTreeMap<String, String>,
}

impl SteamworksServerListFilters {
    /// Creates an empty filter set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts one filter and returns the updated filter set.
    ///
    /// If a key already exists, the value is replaced.
    pub fn with(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.insert(key, value);
        self
    }

    /// Inserts or replaces one filter value.
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) -> Option<String> {
        self.entries.insert(key.into(), value.into())
    }

    /// Returns the number of filters.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether this filter set is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the owned filter entries.
    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.entries
    }

    pub(super) fn as_upstream_map(&self) -> HashMap<&str, &str> {
        self.entries
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect()
    }
}

impl FromIterator<(String, String)> for SteamworksServerListFilters {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        Self {
            entries: iter.into_iter().collect(),
        }
    }
}

/// Owned snapshot of one Steam game server item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksGameServerItem {
    /// Steam app ID hosted by the server.
    pub app_id: u32,
    /// Current human player count.
    pub players: i32,
    /// Whether Steam says this server should not be refreshed.
    pub do_not_refresh: bool,
    /// Whether the latest query had a successful response.
    pub successful_response: bool,
    /// Whether the server requires a password.
    pub have_password: bool,
    /// Whether the server is secure.
    pub secure: bool,
    /// Current bot player count.
    pub bot_players: i32,
    /// Query ping reported by Steam.
    pub ping: Duration,
    /// Maximum player capacity.
    pub max_players: i32,
    /// Server version reported by Steam.
    pub server_version: i32,
    /// Steam ID associated with the server.
    pub steam_id: u64,
    /// Last time this server was played, as reported by Steam.
    pub last_time_played: Duration,
    /// IPv4 address of the server.
    pub address: Ipv4Addr,
    /// Query port of the server.
    pub query_port: u16,
    /// Connection port of the server.
    pub connection_port: u16,
    /// Game description reported by the server.
    pub game_description: String,
    /// Server name.
    pub server_name: String,
    /// Game directory.
    pub game_dir: String,
    /// Current map name.
    pub map: String,
    /// Server tags.
    pub tags: String,
}

/// Successful direct ping result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksServerPing {
    /// Plugin-owned query ID.
    pub query: SteamworksServerQueryId,
    /// Target server endpoint.
    pub target: SteamworksServerQueryTarget,
    /// Server snapshot returned by Steam.
    pub server: SteamworksGameServerItem,
}

/// Player row returned by a direct player-details query.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksServerPlayerInfo {
    /// Player name reported by the server.
    pub name: String,
    /// Player score reported by the server.
    pub score: i32,
    /// Time played reported by the server.
    pub time_played: Duration,
}

impl SteamworksServerPlayerInfo {
    pub(super) fn from_steam(name: &std::ffi::CStr, score: i32, time_played: f32) -> Self {
        let seconds = if time_played.is_finite() && time_played > 0.0 {
            time_played
        } else {
            0.0
        };

        Self {
            name: name.to_string_lossy().into_owned(),
            score,
            time_played: Duration::from_secs_f32(seconds),
        }
    }
}

/// Successful direct player-details query result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksServerPlayerDetails {
    /// Plugin-owned query ID.
    pub query: SteamworksServerQueryId,
    /// Target server endpoint.
    pub target: SteamworksServerQueryTarget,
    /// Players returned by the server.
    pub players: Vec<SteamworksServerPlayerInfo>,
}

/// Rule row returned by a direct server-rules query.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksServerRule {
    /// Rule key reported by the server.
    pub key: String,
    /// Rule value reported by the server.
    pub value: String,
}

impl SteamworksServerRule {
    pub(super) fn from_steam(key: &std::ffi::CStr, value: &std::ffi::CStr) -> Self {
        Self {
            key: key.to_string_lossy().into_owned(),
            value: value.to_string_lossy().into_owned(),
        }
    }
}

/// Successful direct server-rules query result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksServerRules {
    /// Plugin-owned query ID.
    pub query: SteamworksServerQueryId,
    /// Target server endpoint.
    pub target: SteamworksServerQueryTarget,
    /// Rules returned by the server.
    pub rules: Vec<SteamworksServerRule>,
}

impl SteamworksGameServerItem {
    pub(super) fn from_steam(server: steamworks::GameServerItem) -> Self {
        Self {
            app_id: server.appid,
            players: server.players,
            do_not_refresh: server.do_not_refresh,
            successful_response: server.successful_response,
            have_password: server.have_password,
            secure: server.secure,
            bot_players: server.bot_players,
            ping: server.ping,
            max_players: server.max_players,
            server_version: server.server_version,
            steam_id: server.steamid,
            last_time_played: server.last_time_played,
            address: server.addr,
            query_port: server.query_port,
            connection_port: server.connection_port,
            game_description: server.game_description,
            server_name: server.server_name,
            game_dir: server.game_dir,
            map: server.map,
            tags: server.tags,
        }
    }
}

/// Server-list refresh completion response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SteamworksServerListResponse {
    /// At least one server responded.
    ServerResponded,
    /// A server failed to respond.
    ServerFailedToRespond,
    /// The master server listed no servers.
    NoServersListedOnMasterServer,
}

impl From<steamworks::ServerResponse> for SteamworksServerListResponse {
    fn from(value: steamworks::ServerResponse) -> Self {
        match value {
            steamworks::ServerResponse::ServerResponded => Self::ServerResponded,
            steamworks::ServerResponse::ServerFailedToRespond => Self::ServerFailedToRespond,
            steamworks::ServerResponse::NoServersListedOnMasterServer => {
                Self::NoServersListedOnMasterServer
            }
        }
    }
}

/// Reason an upstream server-list release failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SteamworksServerListReleaseError {
    /// The request was already released.
    Released,
    /// The upstream wrapper rejected release while the request is refreshing.
    Refreshing,
}

impl From<steamworks::ReleaseError> for SteamworksServerListReleaseError {
    fn from(value: steamworks::ReleaseError) -> Self {
        match value {
            steamworks::ReleaseError::Released => Self::Released,
            steamworks::ReleaseError::Refreshing => Self::Refreshing,
        }
    }
}
