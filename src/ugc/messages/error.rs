use std::path::PathBuf;

use thiserror::Error;

/// Synchronous and async errors from [`crate::SteamworksUgcPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksUgcError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// No [`crate::SteamworksServer`] resource exists.
    #[error("SteamworksServer resource is not available")]
    ServerUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks UGC command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A Workshop item ID was zero.
    #[error("Steamworks UGC item id must be non-zero")]
    InvalidItemId,
    /// An item list was empty.
    #[error("Steamworks UGC item list must not be empty")]
    EmptyItemList,
    /// A Workshop depot ID was zero.
    #[error("Steamworks UGC Workshop depot id must be non-zero")]
    InvalidWorkshopDepot,
    /// An item list exceeded the supported per-command cap.
    #[error("Steamworks UGC item list length {requested} exceeds max {max_supported}")]
    TooManyItems {
        /// Requested item count.
        requested: usize,
        /// Maximum accepted item count.
        max_supported: usize,
    },
    /// UGC query pages are one-based.
    #[error("Steamworks UGC query page must be greater than zero")]
    InvalidPage,
    /// A Workshop item update had no fields to apply.
    #[error("Steamworks UGC item update must include at least one field or change note")]
    EmptyItemUpdate,
    /// A Workshop item update field exceeded a Steam size limit.
    #[error(
        "Steamworks UGC update field {field} must be <= {max_supported} bytes, got {requested}"
    )]
    StringTooLong {
        /// Field that was too long.
        field: &'static str,
        /// Requested byte length.
        requested: usize,
        /// Maximum byte length supported by Steam.
        max_supported: usize,
    },
    /// A Workshop item tag contained unsupported text.
    #[error("Steamworks UGC update tag contains unsupported text: {tag}")]
    InvalidTagText {
        /// Tag that was rejected.
        tag: String,
    },
    /// A Workshop item key/value tag key contained unsupported text.
    #[error("Steamworks UGC update key/value tag key contains unsupported text: {key}")]
    InvalidKeyValueTagKey {
        /// Key that was rejected.
        key: String,
    },
    /// A Workshop item update path could not be canonicalized before calling Steam.
    #[error("Steamworks UGC update path field {field} could not be canonicalized: {path}")]
    InvalidPath {
        /// Field that contained the invalid path.
        field: &'static str,
        /// Path that failed canonicalization.
        path: PathBuf,
    },
    /// Too many key/value tag removals were requested in one update.
    #[error(
        "Steamworks UGC update key/value tag removal count {requested} exceeds max {max_supported}"
    )]
    TooManyKeyValueTagRemovals {
        /// Requested removal count.
        requested: usize,
        /// Maximum accepted removal count.
        max_supported: usize,
    },
    /// Too many key/value tag additions were requested in one update.
    #[error("Steamworks UGC update key/value tag addition count {requested} exceeds max {max_supported}")]
    TooManyKeyValueTags {
        /// Requested addition count.
        requested: usize,
        /// Maximum accepted addition count.
        max_supported: usize,
    },
    /// No item update watch exists for the request ID.
    #[error("Steamworks UGC item update request {request_id} was not found")]
    ItemUpdateNotFound {
        /// Plugin request ID.
        request_id: u64,
    },
    /// Steam failed to create a UGC query handle.
    #[error("Steamworks UGC failed to create query handle")]
    CreateQueryFailed,
    /// Steam rejected a synchronous operation.
    #[error("Steamworks UGC operation failed: {operation}")]
    OperationFailed {
        /// Operation that failed.
        operation: &'static str,
    },
    /// The upstream Steamworks API returned an explicit Steam error.
    #[error("Steamworks UGC operation {operation} failed: {source}")]
    SteamError {
        /// Operation that failed.
        operation: &'static str,
        /// Plugin request ID for async operations.
        request_id: Option<u64>,
        /// Steamworks error.
        source: steamworks::SteamError,
    },
}

impl SteamworksUgcError {
    pub(in crate::ugc) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    pub(in crate::ugc) fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }

    pub(in crate::ugc) fn steam_error(
        operation: &'static str,
        request_id: Option<u64>,
        source: steamworks::SteamError,
    ) -> Self {
        Self::SteamError {
            operation,
            request_id,
            source,
        }
    }

    pub(in crate::ugc) fn async_request_id(&self) -> Option<u64> {
        match self {
            Self::SteamError { request_id, .. } => *request_id,
            _ => None,
        }
    }
}
