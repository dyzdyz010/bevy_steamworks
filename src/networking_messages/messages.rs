use bevy_ecs::message::Message;
use thiserror::Error;

use super::types::{
    SteamworksNetworkingMessage, SteamworksNetworkingMessagesConnectionInfo,
    SteamworksNetworkingMessagesSessionRequestInfo, SteamworksNetworkingPeer,
};

/// A high-level command for Steam Networking Messages workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksNetworkingMessagesCommand {
    /// Send one payload to a peer on a channel.
    SendMessage {
        /// Remote peer.
        peer: SteamworksNetworkingPeer,
        /// Delivery flags.
        send_flags: steamworks::networking_types::SendFlags,
        /// Routing channel.
        channel: u32,
        /// Payload to send.
        data: Vec<u8>,
    },
    /// Receive up to `batch_size` messages from one channel.
    ReceiveMessages {
        /// Routing channel.
        channel: u32,
        /// Maximum number of messages to receive.
        batch_size: usize,
    },
    /// Read connection information for one peer.
    GetSessionConnectionInfo {
        /// Peer to inspect.
        peer: SteamworksNetworkingPeer,
    },
    /// Set whether future incoming session requests are accepted in the callback.
    SetAutoAcceptSessionRequests {
        /// Whether session requests should be accepted.
        enabled: bool,
    },
}

impl SteamworksNetworkingMessagesCommand {
    /// Creates a [`SteamworksNetworkingMessagesCommand::SendMessage`] command.
    pub fn send_message(
        peer: SteamworksNetworkingPeer,
        send_flags: steamworks::networking_types::SendFlags,
        channel: u32,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self::SendMessage {
            peer,
            send_flags,
            channel,
            data: data.into(),
        }
    }

    /// Creates a command that sends a payload to a Steam user.
    pub fn send_message_to_steam_id(
        steam_id: steamworks::SteamId,
        send_flags: steamworks::networking_types::SendFlags,
        channel: u32,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self::send_message(
            SteamworksNetworkingPeer::steam_id(steam_id),
            send_flags,
            channel,
            data,
        )
    }

    /// Creates a [`SteamworksNetworkingMessagesCommand::ReceiveMessages`] command.
    pub fn receive_messages(channel: u32, batch_size: usize) -> Self {
        Self::ReceiveMessages {
            channel,
            batch_size,
        }
    }

    /// Creates a [`SteamworksNetworkingMessagesCommand::GetSessionConnectionInfo`] command.
    pub fn get_session_connection_info(peer: SteamworksNetworkingPeer) -> Self {
        Self::GetSessionConnectionInfo { peer }
    }

    /// Creates a [`SteamworksNetworkingMessagesCommand::SetAutoAcceptSessionRequests`] command.
    pub fn set_auto_accept_session_requests(enabled: bool) -> Self {
        Self::SetAutoAcceptSessionRequests { enabled }
    }
}

/// A successfully submitted Steam Networking Messages operation or callback.
#[derive(Clone, Debug, PartialEq)]
pub enum SteamworksNetworkingMessagesOperation {
    /// A payload was sent to Steam.
    MessageSent {
        /// Remote peer.
        peer: SteamworksNetworkingPeer,
        /// Routing channel.
        channel: u32,
        /// Delivery flags.
        send_flags: steamworks::networking_types::SendFlags,
        /// Payload size in bytes.
        bytes: usize,
    },
    /// One receive command completed.
    MessagesReceived {
        /// Routing channel read.
        channel: u32,
        /// Owned message snapshots.
        messages: Vec<SteamworksNetworkingMessage>,
    },
    /// Connection info was read for a peer.
    SessionConnectionInfoRead {
        /// Peer inspected.
        peer: SteamworksNetworkingPeer,
        /// Connection snapshot.
        info: SteamworksNetworkingMessagesConnectionInfo,
    },
    /// The auto-accept session request policy was changed.
    AutoAcceptSessionRequestsSet {
        /// Whether future session requests are accepted.
        enabled: bool,
    },
    /// A session request callback was observed.
    SessionRequestReceived {
        /// Request snapshot.
        request: SteamworksNetworkingMessagesSessionRequestInfo,
    },
    /// A session failure callback was observed.
    SessionFailed {
        /// Failure connection snapshot.
        info: SteamworksNetworkingMessagesConnectionInfo,
    },
}

/// Result message emitted by [`crate::SteamworksNetworkingMessagesPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksNetworkingMessagesResult {
    /// The command, receive operation, or callback succeeded.
    Ok(SteamworksNetworkingMessagesOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksNetworkingMessagesCommand,
        /// Failure reason.
        error: SteamworksNetworkingMessagesError,
    },
}

/// Synchronous command errors from [`crate::SteamworksNetworkingMessagesPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksNetworkingMessagesError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
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
    pub(super) fn steam_error(operation: &'static str, source: steamworks::SteamError) -> Self {
        Self::SteamError { operation, source }
    }
}
