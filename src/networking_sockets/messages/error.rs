use thiserror::Error;

use super::super::{
    SteamworksListenSocketId, SteamworksNetworkingSocketsConnectionId,
    SteamworksNetworkingSocketsPollGroupId,
};

/// Synchronous command errors from [`crate::SteamworksNetworkingSocketsPlugin`].
#[derive(Clone, Debug, Error, PartialEq)]
pub enum SteamworksNetworkingSocketsError {
    /// No compatible [`crate::SteamworksClient`] or [`crate::SteamworksServer`] resource exists.
    ///
    /// [`crate::SteamworksNetworkingSocketsCommand::SendMessages`] still
    /// requires a client because the upstream safe message allocator is
    /// client-only.
    #[error("Steamworks Networking Sockets resource is not available")]
    ClientUnavailable,
    /// A listen socket ID is not owned by this plugin.
    #[error("Steam Networking Sockets listen socket {id:?} was not found")]
    ListenSocketNotFound {
        /// Missing listen socket ID.
        id: SteamworksListenSocketId,
    },
    /// A connection ID is not owned by this plugin.
    #[error("Steam Networking Sockets connection {id:?} was not found")]
    ConnectionNotFound {
        /// Missing connection ID.
        id: SteamworksNetworkingSocketsConnectionId,
    },
    /// A poll group ID is not owned by this plugin.
    #[error("Steam Networking Sockets poll group {id:?} was not found")]
    PollGroupNotFound {
        /// Missing poll group ID.
        id: SteamworksNetworkingSocketsPollGroupId,
    },
    /// A max-events value was zero.
    #[error("Steam Networking Sockets max_events must be greater than zero")]
    InvalidEventLimit,
    /// A max-events value exceeded this crate's per-command cap.
    #[error("Steam Networking Sockets max_events {requested} exceeds max {max_supported}")]
    TooManyEvents {
        /// Requested event count.
        requested: usize,
        /// Maximum accepted event count.
        max_supported: usize,
    },
    /// A message receive batch size was zero.
    #[error("Steam Networking Sockets receive batch size must be greater than zero")]
    InvalidBatchSize,
    /// A message receive batch size exceeded this crate's per-command cap.
    #[error("Steam Networking Sockets receive batch size {requested} exceeds max {max_supported}")]
    BatchSizeTooLarge {
        /// Requested batch size.
        requested: usize,
        /// Maximum accepted batch size.
        max_supported: usize,
    },
    /// A send-message batch was empty.
    #[error("Steam Networking Sockets send message batch must not be empty")]
    EmptyMessageBatch,
    /// A send-message batch exceeded this crate's per-command cap.
    #[error(
        "Steam Networking Sockets send message batch size {requested} exceeds max {max_supported}"
    )]
    SendBatchTooLarge {
        /// Requested batch size.
        requested: usize,
        /// Maximum accepted batch size.
        max_supported: usize,
    },
    /// A message payload exceeded this crate's per-message cap.
    #[error("Steam Networking Sockets message size {bytes} exceeds max {max_supported}")]
    MessageTooLarge {
        /// Requested payload size.
        bytes: usize,
        /// Maximum accepted payload size.
        max_supported: usize,
    },
    /// A message lane/channel was negative.
    #[error("Steam Networking Sockets message channel {channel} must not be negative")]
    InvalidMessageChannel {
        /// Invalid message channel.
        channel: i32,
    },
    /// A lane count exceeded this crate's per-command cap.
    #[error("Steam Networking Sockets lane count {lanes} exceeds max {max_supported}")]
    InvalidLaneCount {
        /// Invalid lane count.
        lanes: u32,
        /// Maximum accepted lane count.
        max_supported: u32,
    },
    /// A lane configuration has mismatched priority and weight lengths or no lanes.
    #[error(
        "Steam Networking Sockets lane configuration requires matching nonzero priorities and weights, got {priorities} priorities and {weights} weights"
    )]
    InvalidLaneConfiguration {
        /// Number of priority entries.
        priorities: usize,
        /// Number of weight entries.
        weights: usize,
    },
    /// A lane configuration exceeded this crate's per-command cap.
    #[error(
        "Steam Networking Sockets configured lane count {requested} exceeds max {max_supported}"
    )]
    TooManyConfiguredLanes {
        /// Requested lane count.
        requested: usize,
        /// Maximum accepted lane count.
        max_supported: usize,
    },
    /// A virtual port was negative.
    #[error("Steam Networking Sockets virtual port {port} must not be negative")]
    InvalidVirtualPort {
        /// Invalid virtual port.
        port: i32,
    },
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steam Networking Sockets command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// Steam returned an invalid handle.
    #[error("{operation} returned an invalid handle")]
    InvalidHandle {
        /// Operation that failed.
        operation: &'static str,
    },
    /// Steam returned an operation error.
    #[error("{operation} failed: {source}")]
    SteamError {
        /// Operation that failed.
        operation: &'static str,
        /// Error returned by Steam.
        source: steamworks::SteamError,
    },
    /// The upstream message allocation wrapper rejected a payload buffer.
    #[error("{operation} failed: {message}")]
    MessageError {
        /// Operation that failed.
        operation: &'static str,
        /// Error returned by the upstream message wrapper.
        message: String,
    },
    /// Steam returned `false` for a boolean operation.
    #[error("{operation} failed")]
    OperationFailed {
        /// Operation that failed.
        operation: &'static str,
    },
}

impl SteamworksNetworkingSocketsError {
    pub(in crate::networking_sockets) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    pub(in crate::networking_sockets) fn invalid_handle(operation: &'static str) -> Self {
        Self::InvalidHandle { operation }
    }

    pub(in crate::networking_sockets) fn steam_error(
        operation: &'static str,
        source: steamworks::SteamError,
    ) -> Self {
        Self::SteamError { operation, source }
    }

    pub(in crate::networking_sockets) fn message_error(
        operation: &'static str,
        source: impl std::error::Error,
    ) -> Self {
        Self::MessageError {
            operation,
            message: source.to_string(),
        }
    }

    pub(in crate::networking_sockets) fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }
}
