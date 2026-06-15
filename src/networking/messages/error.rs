use thiserror::Error;

/// Synchronous command errors from [`crate::SteamworksNetworkingPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksNetworkingError {
    /// No compatible [`crate::SteamworksClient`] or [`crate::SteamworksServer`] resource exists.
    #[error("Steamworks networking resource is not available")]
    ClientUnavailable,
    /// A Steam ID was zero.
    #[error("Steam networking command requires a non-zero Steam ID")]
    InvalidSteamId,
    /// A channel exceeds Steam's signed 32-bit channel range.
    #[error("Steam networking channel {channel} exceeds i32::MAX")]
    InvalidChannel {
        /// Invalid channel.
        channel: u32,
    },
    /// A send payload exceeded Steam's limit for the selected send type.
    #[error("Steam networking packet size {bytes} exceeds max {max_bytes}")]
    PacketTooLarge {
        /// Requested packet size.
        bytes: usize,
        /// Maximum accepted packet size.
        max_bytes: usize,
    },
    /// A read command used a zero buffer size.
    #[error("Steam networking read buffer size must be greater than zero")]
    InvalidReadBufferSize,
    /// A read command exceeded the per-frame allocation cap.
    #[error("Steam networking read buffer size {max_bytes} exceeds max {max_supported}")]
    ReadBufferTooLarge {
        /// Requested read buffer size.
        max_bytes: usize,
        /// Maximum accepted read buffer size.
        max_supported: usize,
    },
    /// A queued packet is larger than the requested read buffer.
    #[error(
        "Steam networking queued packet size {available_bytes} exceeds read buffer {max_bytes}"
    )]
    PacketExceedsReadBuffer {
        /// Queued packet size reported by Steam.
        available_bytes: usize,
        /// Requested read buffer size.
        max_bytes: usize,
    },
    /// Steam returned `false` for a boolean operation.
    #[error("{operation} failed")]
    OperationFailed {
        /// Operation that failed.
        operation: &'static str,
    },
}

impl SteamworksNetworkingError {
    pub(in crate::networking) fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }
}
