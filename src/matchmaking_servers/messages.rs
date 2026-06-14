use bevy_ecs::message::Message;
use thiserror::Error;

use super::*;

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

/// Result message emitted by [`crate::SteamworksMatchmakingServersPlugin`].
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

/// Synchronous and callback errors from [`crate::SteamworksMatchmakingServersPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksMatchmakingServersError {
    /// No [`crate::SteamworksClient`] resource exists.
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
    pub(super) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }
}
