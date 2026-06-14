use thiserror::Error;

use super::super::{
    SteamworksServerListKind, SteamworksServerListReleaseError, SteamworksServerListRequestId,
};

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
    pub(in crate::matchmaking_servers) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }
}
