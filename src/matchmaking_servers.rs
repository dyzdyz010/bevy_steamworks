//! High-level Bevy ECS integration for Steam Matchmaking Servers.
//!
//! This module builds on top of the upstream
//! [`steamworks::MatchmakingServers`] API. It exposes Steam server-browser
//! list requests through Bevy commands/results while keeping the upstream
//! request handles owned by the plugin.

use std::{
    collections::{BTreeMap, HashMap},
    net::Ipv4Addr,
    sync::{Arc, Mutex},
    time::Duration,
};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksSystem};

/// Maximum byte length for one Steam server-list filter key or value.
pub const STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES: usize = 255;

/// Bevy plugin for high-level Steam Matchmaking Servers commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksMatchmakingServersCommand`] and
/// [`SteamworksMatchmakingServersResult`] messages and processes commands in
/// [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksMatchmakingServersPlugin;

impl SteamworksMatchmakingServersPlugin {
    /// Creates a Matchmaking Servers plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksMatchmakingServersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksMatchmakingServersState>()
            .init_resource::<SteamworksMatchmakingServersAsyncResults>()
            .init_resource::<SteamworksMatchmakingServerListRequests>()
            .add_message::<SteamworksMatchmakingServersCommand>()
            .add_message::<SteamworksMatchmakingServersResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessMatchmakingServersCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_matchmaking_servers_commands
                    .in_set(SteamworksSystem::ProcessMatchmakingServersCommands),
            );
    }
}

/// Runtime state for [`SteamworksMatchmakingServersPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksMatchmakingServersState {
    last_error: Option<SteamworksMatchmakingServersError>,
    active_server_list_requests: usize,
    last_server: Option<SteamworksGameServerItem>,
    last_refresh_response: Option<SteamworksServerListResponse>,
    server_response_count: u64,
    server_failure_count: u64,
    refresh_complete_count: u64,
    next_request_id: u64,
}

impl SteamworksMatchmakingServersState {
    /// Returns the most recent synchronous or callback error observed by the plugin.
    pub fn last_error(&self) -> Option<&SteamworksMatchmakingServersError> {
        self.last_error.as_ref()
    }

    /// Returns the number of active server-list request handles owned by the plugin.
    pub fn active_server_list_requests(&self) -> usize {
        self.active_server_list_requests
    }

    /// Returns the most recent server snapshot read or received by callback.
    pub fn last_server(&self) -> Option<&SteamworksGameServerItem> {
        self.last_server.as_ref()
    }

    /// Returns the most recent server-list refresh completion response.
    pub fn last_refresh_response(&self) -> Option<SteamworksServerListResponse> {
        self.last_refresh_response
    }

    /// Returns how many server responded callbacks were observed.
    pub fn server_response_count(&self) -> u64 {
        self.server_response_count
    }

    /// Returns how many server failed callbacks were observed.
    pub fn server_failure_count(&self) -> u64 {
        self.server_failure_count
    }

    /// Returns how many server-list refresh completion callbacks were observed.
    pub fn refresh_complete_count(&self) -> u64 {
        self.refresh_complete_count
    }

    fn record_error(&mut self, error: SteamworksMatchmakingServersError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksMatchmakingServersOperation) {
        match operation {
            SteamworksMatchmakingServersOperation::ServerResponded { server, .. } => {
                self.last_server = Some(server.clone());
                self.server_response_count = self.server_response_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerFailedToRespond { .. } => {
                self.server_failure_count = self.server_failure_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerListRefreshCompleted {
                response, ..
            } => {
                self.last_refresh_response = Some(*response);
                self.refresh_complete_count = self.refresh_complete_count.saturating_add(1);
            }
            SteamworksMatchmakingServersOperation::ServerDetailsRead { server, .. } => {
                self.last_server = Some(server.clone());
            }
            _ => {}
        }
    }

    fn sync_request_count(&mut self, requests: &SteamworksMatchmakingServerListRequests) {
        self.active_server_list_requests = requests.len();
    }

    fn next_request_id(&mut self) -> SteamworksServerListRequestId {
        let request_id = self.next_request_id;
        self.next_request_id = self.next_request_id.wrapping_add(1);
        SteamworksServerListRequestId(request_id)
    }
}

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

    fn as_upstream_map(&self) -> HashMap<&str, &str> {
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

impl SteamworksGameServerItem {
    fn from_steam(server: steamworks::GameServerItem) -> Self {
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

#[derive(Clone, Debug, Default, Resource)]
struct SteamworksMatchmakingServersAsyncResults {
    queue: Arc<Mutex<Vec<SteamworksMatchmakingServersResult>>>,
}

impl SteamworksMatchmakingServersAsyncResults {
    fn push(&self, result: SteamworksMatchmakingServersResult) {
        self.queue
            .lock()
            .expect("Steamworks matchmaking servers async result mutex was poisoned")
            .push(result);
    }

    fn drain(&self) -> Vec<SteamworksMatchmakingServersResult> {
        self.queue
            .lock()
            .expect("Steamworks matchmaking servers async result mutex was poisoned")
            .drain(..)
            .collect()
    }
}

#[derive(Clone, Debug, Default, Resource)]
struct SteamworksMatchmakingServerListRequests {
    storage: Arc<Mutex<SteamworksMatchmakingServerListRequestStorage>>,
}

impl SteamworksMatchmakingServerListRequests {
    fn insert(
        &self,
        request: SteamworksServerListRequestId,
        client: &SteamworksClient,
        handle: Arc<Mutex<steamworks::ServerListRequest>>,
    ) {
        self.storage
            .lock()
            .expect("Steamworks server-list request storage mutex was poisoned")
            .insert(request, client.clone_inner(), handle);
    }

    fn get(
        &self,
        request: SteamworksServerListRequestId,
    ) -> Option<Arc<Mutex<steamworks::ServerListRequest>>> {
        self.storage
            .lock()
            .expect("Steamworks server-list request storage mutex was poisoned")
            .get(request)
    }

    fn remove(
        &self,
        request: SteamworksServerListRequestId,
    ) -> Option<Arc<Mutex<steamworks::ServerListRequest>>> {
        self.storage
            .lock()
            .expect("Steamworks server-list request storage mutex was poisoned")
            .remove(request)
    }

    fn len(&self) -> usize {
        self.storage
            .lock()
            .expect("Steamworks server-list request storage mutex was poisoned")
            .len()
    }
}

#[derive(Default)]
struct SteamworksMatchmakingServerListRequestStorage {
    requests: HashMap<SteamworksServerListRequestId, SteamworksServerListRequestHandle>,
}

impl std::fmt::Debug for SteamworksMatchmakingServerListRequestStorage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SteamworksMatchmakingServerListRequestStorage")
            .field("request_count", &self.requests.len())
            .finish()
    }
}

impl SteamworksMatchmakingServerListRequestStorage {
    fn insert(
        &mut self,
        request: SteamworksServerListRequestId,
        client: steamworks::Client,
        handle: Arc<Mutex<steamworks::ServerListRequest>>,
    ) {
        self.requests.insert(
            request,
            SteamworksServerListRequestHandle {
                handle,
                _client: client,
            },
        );
    }

    fn get(
        &self,
        request: SteamworksServerListRequestId,
    ) -> Option<Arc<Mutex<steamworks::ServerListRequest>>> {
        self.requests
            .get(&request)
            .map(|request| request.handle.clone())
    }

    fn remove(
        &mut self,
        request: SteamworksServerListRequestId,
    ) -> Option<Arc<Mutex<steamworks::ServerListRequest>>> {
        self.requests.remove(&request).map(|request| request.handle)
    }

    fn len(&self) -> usize {
        self.requests.len()
    }
}

impl Drop for SteamworksMatchmakingServerListRequestStorage {
    fn drop(&mut self) {
        for (request_id, request) in self.requests.drain() {
            match request.handle.lock() {
                Ok(mut request) => match request.release() {
                    Ok(()) => {
                        tracing::debug!(
                            target: "bevy_steamworks",
                            request = ?request_id,
                            "released Steamworks server-list request during plugin shutdown"
                        );
                    }
                    Err(source) => {
                        let reason = SteamworksServerListReleaseError::from(source);
                        tracing::error!(
                            target: "bevy_steamworks",
                            request = ?request_id,
                            reason = ?reason,
                            "failed to release Steamworks server-list request during plugin shutdown"
                        );
                    }
                },
                Err(_) => {
                    tracing::error!(
                        target: "bevy_steamworks",
                        request = ?request_id,
                        "failed to lock Steamworks server-list request during plugin shutdown"
                    );
                }
            }
        }
    }
}

#[derive(Clone)]
struct SteamworksServerListRequestHandle {
    handle: Arc<Mutex<steamworks::ServerListRequest>>,
    _client: steamworks::Client,
}

/// A high-level command for Steam Matchmaking Servers workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksMatchmakingServersCommand {
    /// Request a Steam server list.
    RequestServerList {
        /// Steam app ID to query.
        app_id: steamworks::AppId,
        /// Server-list source.
        kind: SteamworksServerListKind,
        /// Filters applied to non-LAN server-list requests.
        filters: SteamworksServerListFilters,
    },
    /// Refresh an existing server-list request.
    RefreshServerList {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
    },
    /// Refresh one server in an existing server-list request.
    RefreshServer {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Server index inside the request.
        server: i32,
    },
    /// Read the number of servers currently known for a request.
    GetServerListCount {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
    },
    /// Read details for one server currently known for a request.
    GetServerDetails {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Server index inside the request.
        server: i32,
    },
    /// Read whether a request is still refreshing.
    IsServerListRefreshing {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
    },
    /// Release a server-list request handle.
    ReleaseServerList {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
    },
}

impl SteamworksMatchmakingServersCommand {
    /// Creates a LAN server-list request command.
    pub fn request_lan_server_list(app_id: impl Into<steamworks::AppId>) -> Self {
        Self::RequestServerList {
            app_id: app_id.into(),
            kind: SteamworksServerListKind::Lan,
            filters: SteamworksServerListFilters::new(),
        }
    }

    /// Creates an Internet server-list request command.
    pub fn request_internet_server_list(
        app_id: impl Into<steamworks::AppId>,
        filters: SteamworksServerListFilters,
    ) -> Self {
        Self::RequestServerList {
            app_id: app_id.into(),
            kind: SteamworksServerListKind::Internet,
            filters,
        }
    }

    /// Creates a favorites server-list request command.
    pub fn request_favorites_server_list(
        app_id: impl Into<steamworks::AppId>,
        filters: SteamworksServerListFilters,
    ) -> Self {
        Self::RequestServerList {
            app_id: app_id.into(),
            kind: SteamworksServerListKind::Favorites,
            filters,
        }
    }

    /// Creates a history server-list request command.
    pub fn request_history_server_list(
        app_id: impl Into<steamworks::AppId>,
        filters: SteamworksServerListFilters,
    ) -> Self {
        Self::RequestServerList {
            app_id: app_id.into(),
            kind: SteamworksServerListKind::History,
            filters,
        }
    }

    /// Creates a friends server-list request command.
    pub fn request_friends_server_list(
        app_id: impl Into<steamworks::AppId>,
        filters: SteamworksServerListFilters,
    ) -> Self {
        Self::RequestServerList {
            app_id: app_id.into(),
            kind: SteamworksServerListKind::Friends,
            filters,
        }
    }

    /// Creates a server-list refresh command.
    pub fn refresh_server_list(request: SteamworksServerListRequestId) -> Self {
        Self::RefreshServerList { request }
    }

    /// Creates a single-server refresh command.
    pub fn refresh_server(request: SteamworksServerListRequestId, server: i32) -> Self {
        Self::RefreshServer { request, server }
    }

    /// Creates a server-list count read command.
    pub fn get_server_list_count(request: SteamworksServerListRequestId) -> Self {
        Self::GetServerListCount { request }
    }

    /// Creates a server details read command.
    pub fn get_server_details(request: SteamworksServerListRequestId, server: i32) -> Self {
        Self::GetServerDetails { request, server }
    }

    /// Creates a server-list refreshing state read command.
    pub fn is_server_list_refreshing(request: SteamworksServerListRequestId) -> Self {
        Self::IsServerListRefreshing { request }
    }

    /// Creates a server-list release command.
    pub fn release_server_list(request: SteamworksServerListRequestId) -> Self {
        Self::ReleaseServerList { request }
    }
}

/// A successfully submitted Matchmaking Servers operation or callback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksMatchmakingServersOperation {
    /// A server-list request was submitted.
    ServerListRequested {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Steam app ID queried.
        app_id: steamworks::AppId,
        /// Server-list source.
        kind: SteamworksServerListKind,
        /// Filters applied to the request.
        filters: SteamworksServerListFilters,
    },
    /// A server responded to a server-list request.
    ServerResponded {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Server index inside the request.
        server_index: i32,
        /// Snapshot of the server.
        server: SteamworksGameServerItem,
    },
    /// A server failed to respond to a server-list request.
    ServerFailedToRespond {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Server index inside the request.
        server_index: i32,
    },
    /// A server-list refresh completed.
    ServerListRefreshCompleted {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Completion response.
        response: SteamworksServerListResponse,
    },
    /// A server-list refresh was submitted.
    ServerListRefreshSubmitted {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
    },
    /// A single-server refresh was submitted.
    ServerRefreshSubmitted {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Server index inside the request.
        server_index: i32,
    },
    /// Server count was read from a request.
    ServerListCountRead {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Server count reported by Steam.
        count: i32,
    },
    /// Server details were read from a request.
    ServerDetailsRead {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Server index inside the request.
        server_index: i32,
        /// Server snapshot.
        server: SteamworksGameServerItem,
    },
    /// Refreshing state was read from a request.
    ServerListRefreshingRead {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Whether the request is currently refreshing.
        refreshing: bool,
    },
    /// A server-list request was released.
    ServerListReleased {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
    },
}

/// Result message emitted by [`SteamworksMatchmakingServersPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksMatchmakingServersResult {
    /// The command was submitted to Steamworks, a value was read, or a callback was observed.
    Ok(SteamworksMatchmakingServersOperation),
    /// The command failed synchronously or callback processing failed.
    Err {
        /// Command that failed.
        command: SteamworksMatchmakingServersCommand,
        /// Failure reason.
        error: SteamworksMatchmakingServersError,
    },
}

/// Synchronous and callback errors from [`SteamworksMatchmakingServersPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksMatchmakingServersError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks matchmaking servers command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A server-list filter key or value is longer than Steam supports.
    #[error(
        "Steamworks server-list filter {field} must be <= {max_supported} bytes, got {requested}"
    )]
    FilterTooLong {
        /// Field that was too long.
        field: &'static str,
        /// Requested byte length.
        requested: usize,
        /// Maximum byte length supported by Steam.
        max_supported: usize,
    },
    /// Filters were provided for a LAN server-list request.
    #[error("Steamworks LAN server-list requests do not support filters")]
    LanFiltersUnsupported,
    /// A server index must be non-negative.
    #[error("Steamworks server index must be non-negative, got {server}")]
    InvalidServerIndex {
        /// Server index supplied by the command.
        server: i32,
    },
    /// A server index was outside the current request result range.
    #[error(
        "Steamworks server-list request {request:?} server index {server} is outside current count {count}"
    )]
    ServerIndexOutOfRange {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Server index supplied by the command.
        server: i32,
        /// Current server count reported by Steam.
        count: i32,
    },
    /// The request ID is not known to this plugin.
    #[error("Steamworks server-list request {request:?} was not found")]
    ServerListRequestNotFound {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
    },
    /// The upstream request was already released.
    #[error("Steamworks server-list request {request:?} was already released")]
    ServerListRequestReleased {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
    },
    /// The upstream server-list request rejected the operation.
    #[error("Steamworks server-list request {request:?} rejected {operation}")]
    ServerListRequestRejected {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Operation name.
        operation: &'static str,
    },
    /// A server-list callback could not read the server snapshot.
    #[error(
        "Steamworks server-list request {request:?} could not read server details for index {server}"
    )]
    ServerDetailsUnavailable {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Server index supplied by Steam callback.
        server: i32,
    },
    /// The upstream server-list query was rejected.
    #[error("Steamworks server-list query {kind:?} was rejected")]
    ServerListQueryRejected {
        /// Server-list source requested.
        kind: SteamworksServerListKind,
    },
    /// Releasing the upstream server-list request failed.
    #[error("Steamworks server-list request {request:?} release failed: {reason:?}")]
    ServerListReleaseFailed {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Failure reason from the upstream wrapper.
        reason: SteamworksServerListReleaseError,
    },
}

impl SteamworksMatchmakingServersError {
    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }
}

fn process_matchmaking_servers_commands(
    client: Option<Res<SteamworksClient>>,
    async_results: Res<SteamworksMatchmakingServersAsyncResults>,
    requests: Res<SteamworksMatchmakingServerListRequests>,
    mut state: ResMut<SteamworksMatchmakingServersState>,
    mut commands: ResMut<Messages<SteamworksMatchmakingServersCommand>>,
    mut results: MessageWriter<SteamworksMatchmakingServersResult>,
) {
    for result in async_results.drain() {
        match &result {
            SteamworksMatchmakingServersResult::Ok(operation) => {
                state.record_operation(operation);
                state.sync_request_count(&requests);
            }
            SteamworksMatchmakingServersResult::Err { error, .. } => {
                state.record_error(error.clone());
                state.sync_request_count(&requests);
            }
        }
        results.write(result);
    }

    let Some(client) = client else {
        let error = SteamworksMatchmakingServersError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks matchmaking servers command failed"
            );
            results.write(SteamworksMatchmakingServersResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        let request = match &command {
            SteamworksMatchmakingServersCommand::RequestServerList { .. } => {
                Some(state.next_request_id())
            }
            _ => None,
        };

        match handle_matchmaking_servers_command(
            &client,
            &async_results,
            &requests,
            command.clone(),
            request,
        ) {
            Ok(operation) => {
                state.record_operation(&operation);
                state.sync_request_count(&requests);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks matchmaking servers command"
                );
                results.write(SteamworksMatchmakingServersResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                state.sync_request_count(&requests);
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks matchmaking servers command failed"
                );
                results.write(SteamworksMatchmakingServersResult::Err { command, error });
            }
        }
    }
}

fn handle_matchmaking_servers_command(
    client: &SteamworksClient,
    async_results: &SteamworksMatchmakingServersAsyncResults,
    requests: &SteamworksMatchmakingServerListRequests,
    command: SteamworksMatchmakingServersCommand,
    request: Option<SteamworksServerListRequestId>,
) -> Result<SteamworksMatchmakingServersOperation, SteamworksMatchmakingServersError> {
    validate_command(&command)?;

    match command {
        SteamworksMatchmakingServersCommand::RequestServerList {
            app_id,
            kind,
            filters,
        } => {
            let request = request.expect("server-list request command missing request id");
            let handle = request_server_list(
                client,
                async_results.clone(),
                request,
                app_id,
                kind,
                &filters,
            )?;
            requests.insert(request, client, handle);
            Ok(SteamworksMatchmakingServersOperation::ServerListRequested {
                request,
                app_id,
                kind,
                filters,
            })
        }
        SteamworksMatchmakingServersCommand::RefreshServerList { request } => {
            let handle = request_handle(requests, request)?;
            handle
                .lock()
                .expect("Steamworks server-list request mutex was poisoned")
                .refresh_query()
                .map_err(
                    |_| SteamworksMatchmakingServersError::ServerListRequestRejected {
                        request,
                        operation: "refresh_query",
                    },
                )?;
            Ok(SteamworksMatchmakingServersOperation::ServerListRefreshSubmitted { request })
        }
        SteamworksMatchmakingServersCommand::RefreshServer { request, server } => {
            let handle = request_handle(requests, request)?;
            validate_server_index_in_request(&handle, request, server)?;
            handle
                .lock()
                .expect("Steamworks server-list request mutex was poisoned")
                .refresh_server(server)
                .map_err(
                    |_| SteamworksMatchmakingServersError::ServerListRequestRejected {
                        request,
                        operation: "refresh_server",
                    },
                )?;
            Ok(
                SteamworksMatchmakingServersOperation::ServerRefreshSubmitted {
                    request,
                    server_index: server,
                },
            )
        }
        SteamworksMatchmakingServersCommand::GetServerListCount { request } => {
            let handle = request_handle(requests, request)?;
            let count = handle
                .lock()
                .expect("Steamworks server-list request mutex was poisoned")
                .get_server_count()
                .map_err(
                    |_| SteamworksMatchmakingServersError::ServerListRequestReleased { request },
                )?;
            Ok(SteamworksMatchmakingServersOperation::ServerListCountRead { request, count })
        }
        SteamworksMatchmakingServersCommand::GetServerDetails { request, server } => {
            let handle = request_handle(requests, request)?;
            validate_server_index_in_request(&handle, request, server)?;
            let item = handle
                .lock()
                .expect("Steamworks server-list request mutex was poisoned")
                .get_server_details(server)
                .map_err(
                    |_| SteamworksMatchmakingServersError::ServerListRequestReleased { request },
                )
                .map(SteamworksGameServerItem::from_steam)?;
            Ok(SteamworksMatchmakingServersOperation::ServerDetailsRead {
                request,
                server_index: server,
                server: item,
            })
        }
        SteamworksMatchmakingServersCommand::IsServerListRefreshing { request } => {
            let handle = request_handle(requests, request)?;
            let refreshing = handle
                .lock()
                .expect("Steamworks server-list request mutex was poisoned")
                .is_refreshing()
                .map_err(
                    |_| SteamworksMatchmakingServersError::ServerListRequestReleased { request },
                )?;
            Ok(
                SteamworksMatchmakingServersOperation::ServerListRefreshingRead {
                    request,
                    refreshing,
                },
            )
        }
        SteamworksMatchmakingServersCommand::ReleaseServerList { request } => {
            let handle = request_handle(requests, request)?;
            handle
                .lock()
                .expect("Steamworks server-list request mutex was poisoned")
                .release()
                .map_err(
                    |source| SteamworksMatchmakingServersError::ServerListReleaseFailed {
                        request,
                        reason: source.into(),
                    },
                )?;
            requests.remove(request);
            Ok(SteamworksMatchmakingServersOperation::ServerListReleased { request })
        }
    }
}

fn request_server_list(
    client: &SteamworksClient,
    async_results: SteamworksMatchmakingServersAsyncResults,
    request: SteamworksServerListRequestId,
    app_id: steamworks::AppId,
    kind: SteamworksServerListKind,
    filters: &SteamworksServerListFilters,
) -> Result<Arc<Mutex<steamworks::ServerListRequest>>, SteamworksMatchmakingServersError> {
    let servers = client.matchmaking_servers();
    let callbacks = server_list_callbacks(request, async_results);
    match kind {
        SteamworksServerListKind::Lan => Ok(servers.lan_server_list(app_id, callbacks)),
        SteamworksServerListKind::Internet => servers
            .internet_server_list(app_id, &filters.as_upstream_map(), callbacks)
            .map_err(|_| SteamworksMatchmakingServersError::ServerListQueryRejected { kind }),
        SteamworksServerListKind::Favorites => servers
            .favorites_server_list(app_id, &filters.as_upstream_map(), callbacks)
            .map_err(|_| SteamworksMatchmakingServersError::ServerListQueryRejected { kind }),
        SteamworksServerListKind::History => servers
            .history_server_list(app_id, &filters.as_upstream_map(), callbacks)
            .map_err(|_| SteamworksMatchmakingServersError::ServerListQueryRejected { kind }),
        SteamworksServerListKind::Friends => servers
            .friends_server_list(app_id, &filters.as_upstream_map(), callbacks)
            .map_err(|_| SteamworksMatchmakingServersError::ServerListQueryRejected { kind }),
    }
}

fn server_list_callbacks(
    request: SteamworksServerListRequestId,
    async_results: SteamworksMatchmakingServersAsyncResults,
) -> steamworks::ServerListCallbacks {
    let responded_results = async_results.clone();
    let failed_results = async_results.clone();

    steamworks::ServerListCallbacks::new(
        Box::new(move |list, server_index| {
            let result = list
                .lock()
                .expect("Steamworks server-list request mutex was poisoned")
                .get_server_details(server_index);
            responded_results.push(match result {
                Ok(server) => SteamworksMatchmakingServersResult::Ok(
                    SteamworksMatchmakingServersOperation::ServerResponded {
                        request,
                        server_index,
                        server: SteamworksGameServerItem::from_steam(server),
                    },
                ),
                Err(()) => SteamworksMatchmakingServersResult::Err {
                    command: SteamworksMatchmakingServersCommand::GetServerDetails {
                        request,
                        server: server_index,
                    },
                    error: SteamworksMatchmakingServersError::ServerDetailsUnavailable {
                        request,
                        server: server_index,
                    },
                },
            });
        }),
        Box::new(move |_list, server_index| {
            failed_results.push(SteamworksMatchmakingServersResult::Ok(
                SteamworksMatchmakingServersOperation::ServerFailedToRespond {
                    request,
                    server_index,
                },
            ));
        }),
        Box::new(move |_list, response| {
            async_results.push(SteamworksMatchmakingServersResult::Ok(
                SteamworksMatchmakingServersOperation::ServerListRefreshCompleted {
                    request,
                    response: response.into(),
                },
            ));
        }),
    )
}

fn request_handle(
    requests: &SteamworksMatchmakingServerListRequests,
    request: SteamworksServerListRequestId,
) -> Result<Arc<Mutex<steamworks::ServerListRequest>>, SteamworksMatchmakingServersError> {
    requests
        .get(request)
        .ok_or(SteamworksMatchmakingServersError::ServerListRequestNotFound { request })
}

fn validate_server_index_in_request(
    handle: &Arc<Mutex<steamworks::ServerListRequest>>,
    request: SteamworksServerListRequestId,
    server: i32,
) -> Result<(), SteamworksMatchmakingServersError> {
    let count = handle
        .lock()
        .expect("Steamworks server-list request mutex was poisoned")
        .get_server_count()
        .map_err(|_| SteamworksMatchmakingServersError::ServerListRequestReleased { request })?;
    if server >= count {
        return Err(SteamworksMatchmakingServersError::ServerIndexOutOfRange {
            request,
            server,
            count,
        });
    }

    Ok(())
}

fn validate_command(
    command: &SteamworksMatchmakingServersCommand,
) -> Result<(), SteamworksMatchmakingServersError> {
    match command {
        SteamworksMatchmakingServersCommand::RequestServerList { kind, filters, .. } => {
            validate_filters(*kind, filters)
        }
        SteamworksMatchmakingServersCommand::RefreshServer { server, .. }
        | SteamworksMatchmakingServersCommand::GetServerDetails { server, .. } => {
            validate_server_index(*server)
        }
        _ => Ok(()),
    }
}

fn validate_filters(
    kind: SteamworksServerListKind,
    filters: &SteamworksServerListFilters,
) -> Result<(), SteamworksMatchmakingServersError> {
    if kind == SteamworksServerListKind::Lan && !filters.is_empty() {
        return Err(SteamworksMatchmakingServersError::LanFiltersUnsupported);
    }

    for (key, value) in filters.entries() {
        validate_filter_text("filter key", key)?;
        validate_filter_text("filter value", value)?;
    }

    Ok(())
}

fn validate_filter_text(
    field: &'static str,
    value: &str,
) -> Result<(), SteamworksMatchmakingServersError> {
    if value.as_bytes().contains(&0) {
        return Err(SteamworksMatchmakingServersError::invalid_string(field));
    }
    if value.len() > STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES {
        return Err(SteamworksMatchmakingServersError::FilterTooLong {
            field,
            requested: value.len(),
            max_supported: STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES,
        });
    }
    Ok(())
}

fn validate_server_index(server: i32) -> Result<(), SteamworksMatchmakingServersError> {
    if server < 0 {
        Err(SteamworksMatchmakingServersError::InvalidServerIndex { server })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    #[test]
    fn matchmaking_servers_plugin_registers_resources_and_messages() {
        let mut app = App::new();
        app.add_plugins(SteamworksMatchmakingServersPlugin::new());

        assert!(app
            .world()
            .contains_resource::<SteamworksMatchmakingServersState>());
        assert!(app
            .world()
            .contains_resource::<SteamworksMatchmakingServersAsyncResults>());
        assert!(app
            .world()
            .contains_resource::<SteamworksMatchmakingServerListRequests>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksMatchmakingServersCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksMatchmakingServersResult>>());
    }

    #[test]
    fn server_list_commands_preserve_inputs() {
        let request = SteamworksServerListRequestId::from_raw(4);
        let filters = SteamworksServerListFilters::new().with("map", "arena");

        assert_eq!(request.raw(), 4);
        assert_eq!(
            SteamworksMatchmakingServersCommand::request_lan_server_list(480),
            SteamworksMatchmakingServersCommand::RequestServerList {
                app_id: steamworks::AppId(480),
                kind: SteamworksServerListKind::Lan,
                filters: SteamworksServerListFilters::new(),
            }
        );
        assert_eq!(
            SteamworksMatchmakingServersCommand::request_internet_server_list(480, filters.clone(),),
            SteamworksMatchmakingServersCommand::RequestServerList {
                app_id: steamworks::AppId(480),
                kind: SteamworksServerListKind::Internet,
                filters: filters.clone(),
            }
        );
        assert_eq!(
            SteamworksMatchmakingServersCommand::request_favorites_server_list(
                480,
                filters.clone(),
            ),
            SteamworksMatchmakingServersCommand::RequestServerList {
                app_id: steamworks::AppId(480),
                kind: SteamworksServerListKind::Favorites,
                filters: filters.clone(),
            }
        );
        assert_eq!(
            SteamworksMatchmakingServersCommand::request_history_server_list(480, filters.clone()),
            SteamworksMatchmakingServersCommand::RequestServerList {
                app_id: steamworks::AppId(480),
                kind: SteamworksServerListKind::History,
                filters: filters.clone(),
            }
        );
        assert_eq!(
            SteamworksMatchmakingServersCommand::request_friends_server_list(480, filters),
            SteamworksMatchmakingServersCommand::RequestServerList {
                app_id: steamworks::AppId(480),
                kind: SteamworksServerListKind::Friends,
                filters: SteamworksServerListFilters::new().with("map", "arena"),
            }
        );
        assert_eq!(
            SteamworksMatchmakingServersCommand::refresh_server_list(request),
            SteamworksMatchmakingServersCommand::RefreshServerList { request }
        );
        assert_eq!(
            SteamworksMatchmakingServersCommand::refresh_server(request, 2),
            SteamworksMatchmakingServersCommand::RefreshServer { request, server: 2 }
        );
        assert_eq!(
            SteamworksMatchmakingServersCommand::get_server_list_count(request),
            SteamworksMatchmakingServersCommand::GetServerListCount { request }
        );
        assert_eq!(
            SteamworksMatchmakingServersCommand::get_server_details(request, 2),
            SteamworksMatchmakingServersCommand::GetServerDetails { request, server: 2 }
        );
        assert_eq!(
            SteamworksMatchmakingServersCommand::is_server_list_refreshing(request),
            SteamworksMatchmakingServersCommand::IsServerListRefreshing { request }
        );
        assert_eq!(
            SteamworksMatchmakingServersCommand::release_server_list(request),
            SteamworksMatchmakingServersCommand::ReleaseServerList { request }
        );
    }

    #[test]
    fn filter_validation_rejects_invalid_inputs() {
        assert_eq!(
            validate_command(&SteamworksMatchmakingServersCommand::RequestServerList {
                app_id: steamworks::AppId(480),
                kind: SteamworksServerListKind::Lan,
                filters: SteamworksServerListFilters::new().with("map", "arena"),
            }),
            Err(SteamworksMatchmakingServersError::LanFiltersUnsupported)
        );
        assert_eq!(
            validate_command(
                &SteamworksMatchmakingServersCommand::request_internet_server_list(
                    480,
                    SteamworksServerListFilters::new().with("bad\0key", "arena"),
                )
            ),
            Err(SteamworksMatchmakingServersError::InvalidString {
                field: "filter key",
            })
        );
        assert_eq!(
            validate_command(
                &SteamworksMatchmakingServersCommand::request_internet_server_list(
                    480,
                    SteamworksServerListFilters::new().with(
                        "map",
                        "x".repeat(STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES + 1),
                    ),
                )
            ),
            Err(SteamworksMatchmakingServersError::FilterTooLong {
                field: "filter value",
                requested: STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES + 1,
                max_supported: STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES,
            })
        );
        assert_eq!(
            validate_command(&SteamworksMatchmakingServersCommand::get_server_details(
                SteamworksServerListRequestId::from_raw(1),
                -1,
            )),
            Err(SteamworksMatchmakingServersError::InvalidServerIndex { server: -1 })
        );
    }

    #[test]
    fn state_records_callback_operations_without_unbounded_history() {
        let mut state = SteamworksMatchmakingServersState::default();
        let request = SteamworksServerListRequestId::from_raw(1);
        let server = SteamworksGameServerItem {
            app_id: 480,
            players: 2,
            do_not_refresh: false,
            successful_response: true,
            have_password: false,
            secure: true,
            bot_players: 0,
            ping: Duration::from_millis(42),
            max_players: 8,
            server_version: 1,
            steam_id: 123,
            last_time_played: Duration::from_secs(0),
            address: Ipv4Addr::LOCALHOST,
            query_port: 27015,
            connection_port: 27016,
            game_description: "Example".to_owned(),
            server_name: "Local".to_owned(),
            game_dir: "example".to_owned(),
            map: "arena".to_owned(),
            tags: "tag".to_owned(),
        };

        state.record_operation(&SteamworksMatchmakingServersOperation::ServerResponded {
            request,
            server_index: 0,
            server: server.clone(),
        });
        state.record_operation(
            &SteamworksMatchmakingServersOperation::ServerFailedToRespond {
                request,
                server_index: 1,
            },
        );
        state.record_operation(
            &SteamworksMatchmakingServersOperation::ServerListRefreshCompleted {
                request,
                response: SteamworksServerListResponse::ServerResponded,
            },
        );

        assert_eq!(state.last_server(), Some(&server));
        assert_eq!(
            state.last_refresh_response(),
            Some(SteamworksServerListResponse::ServerResponded)
        );
        assert_eq!(state.server_response_count(), 1);
        assert_eq!(state.server_failure_count(), 1);
        assert_eq!(state.refresh_complete_count(), 1);
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();
        app.add_plugins(SteamworksMatchmakingServersPlugin::new());
        app.world_mut()
            .write_message(SteamworksMatchmakingServersCommand::request_lan_server_list(480));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksMatchmakingServersResult>>();
        let drained = results.drain().collect::<Vec<_>>();
        assert_eq!(drained.len(), 1);
        assert!(matches!(
            &drained[0],
            SteamworksMatchmakingServersResult::Err {
                error: SteamworksMatchmakingServersError::ClientUnavailable,
                ..
            }
        ));
    }
}
