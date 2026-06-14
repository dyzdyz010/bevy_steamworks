use thiserror::Error;

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
    pub(in crate::matchmaking) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    pub(in crate::matchmaking) fn lobby_key_too_long(key: impl Into<String>) -> Self {
        Self::LobbyKeyTooLong { key: key.into() }
    }

    pub(in crate::matchmaking) fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }

    pub(in crate::matchmaking) fn steam_error(
        operation: &'static str,
        source: steamworks::SteamError,
    ) -> Self {
        Self::SteamError { operation, source }
    }
}
