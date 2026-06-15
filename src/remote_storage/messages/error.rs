use thiserror::Error;

/// Synchronous and async errors from [`crate::SteamworksRemoteStoragePlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksRemoteStorageError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks Remote Storage command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// The upstream Steamworks API returned an explicit Steam error.
    #[error("Steamworks Remote Storage operation {operation} failed: {source}")]
    SteamError {
        /// Operation that failed.
        operation: &'static str,
        /// Plugin request ID for async operations.
        request_id: Option<u64>,
        /// Steamworks error.
        source: steamworks::SteamError,
    },
    /// The requested file is not available in Steam Remote Storage.
    #[error("Steamworks Remote Storage file is not available: {name}")]
    FileUnavailable {
        /// File name submitted.
        name: String,
    },
    /// The requested file is not available for an async Steam Remote Storage request.
    #[error("Steamworks Remote Storage file is not available for request {request_id}: {name}")]
    FileUnavailableForRequest {
        /// Plugin request ID for the background operation.
        request_id: u64,
        /// File name submitted.
        name: String,
    },
    /// The upstream Steamworks API rejected the operation.
    #[error("Steamworks Remote Storage operation failed: {operation}")]
    OperationFailed {
        /// Operation that failed.
        operation: &'static str,
    },
    /// A background file IO worker failed.
    #[error("Steamworks Remote Storage file IO operation {operation} failed: {message}")]
    FileIo {
        /// Operation that failed.
        operation: &'static str,
        /// Plugin request ID for the background operation.
        request_id: u64,
        /// File name submitted.
        name: String,
        /// IO error text.
        message: String,
    },
}

impl SteamworksRemoteStorageError {
    pub(in crate::remote_storage) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    pub(in crate::remote_storage) fn steam_error(
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

    pub(in crate::remote_storage) fn file_unavailable(name: impl Into<String>) -> Self {
        Self::FileUnavailable { name: name.into() }
    }

    pub(in crate::remote_storage) fn file_unavailable_for_request(
        request_id: u64,
        name: impl Into<String>,
    ) -> Self {
        Self::FileUnavailableForRequest {
            request_id,
            name: name.into(),
        }
    }

    pub(in crate::remote_storage) fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }

    pub(in crate::remote_storage) fn file_io(
        operation: &'static str,
        request_id: u64,
        name: impl Into<String>,
        source: impl ToString,
    ) -> Self {
        Self::FileIo {
            operation,
            request_id,
            name: name.into(),
            message: source.to_string(),
        }
    }
}
