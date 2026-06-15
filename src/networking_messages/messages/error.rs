use thiserror::Error;

/// Synchronous command errors from [`crate::SteamworksNetworkingMessagesPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksNetworkingMessagesError {
    /// No compatible [`crate::SteamworksClient`] or [`crate::SteamworksServer`] resource exists.
    #[error("Steamworks Networking Messages resource is not available")]
    ClientUnavailable,
    /// A peer identity is invalid.
    #[error("Steam networking peer identity is invalid")]
    InvalidIdentity,
    /// A channel exceeds Steam's signed 32-bit channel range.
    #[error("Steam networking channel {channel} exceeds i32::MAX")]
    InvalidChannel {
        /// Invalid channel.
        channel: u32,
    },
    /// A receive command used a zero batch size.
    #[error("Steam networking receive batch size must be greater than zero")]
    InvalidBatchSize,
    /// A receive command exceeded the per-frame batch cap.
    #[error("Steam networking receive batch size {batch_size} exceeds max {max_batch_size}")]
    BatchSizeTooLarge {
        /// Requested batch size.
        batch_size: usize,
        /// Maximum accepted batch size.
        max_batch_size: usize,
    },
    /// Steam returned an operation error.
    #[error("{operation} failed: {source}")]
    SteamError {
        /// Operation that failed.
        operation: &'static str,
        /// Error returned by Steam.
        source: steamworks::SteamError,
    },
}

impl SteamworksNetworkingMessagesError {
    pub(in crate::networking_messages) fn steam_error(
        operation: &'static str,
        source: steamworks::SteamError,
    ) -> Self {
        Self::SteamError { operation, source }
    }
}
