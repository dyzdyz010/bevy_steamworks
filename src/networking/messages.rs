use bevy_ecs::message::Message;
use thiserror::Error;

use super::*;

/// A high-level command for Steam's legacy P2P Networking workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksNetworkingCommand {
    /// Accept a legacy P2P session from a remote Steam user.
    AcceptP2pSession {
        /// Remote Steam user.
        user: steamworks::SteamId,
    },
    /// Close a legacy P2P session with a remote Steam user.
    CloseP2pSession {
        /// Remote Steam user.
        user: steamworks::SteamId,
    },
    /// Read the legacy P2P session state for a remote Steam user.
    GetP2pSessionState {
        /// Remote Steam user.
        user: steamworks::SteamId,
    },
    /// Send a legacy P2P packet to a remote Steam user.
    SendP2pPacket {
        /// Remote Steam user.
        remote: steamworks::SteamId,
        /// Delivery mode.
        send_type: SteamworksP2pSendType,
        /// Channel to send on.
        channel: u32,
        /// Payload to send.
        data: Vec<u8>,
    },
    /// Read whether a legacy P2P packet is queued on one channel.
    GetAvailablePacketSize {
        /// Channel to inspect.
        channel: u32,
    },
    /// Read one legacy P2P packet from one channel.
    ReadP2pPacket {
        /// Channel to read from.
        channel: u32,
        /// Receive buffer size to allocate for this read.
        max_bytes: usize,
    },
}

impl SteamworksNetworkingCommand {
    /// Creates a [`SteamworksNetworkingCommand::AcceptP2pSession`] command.
    pub fn accept_p2p_session(user: steamworks::SteamId) -> Self {
        Self::AcceptP2pSession { user }
    }

    /// Creates a [`SteamworksNetworkingCommand::CloseP2pSession`] command.
    pub fn close_p2p_session(user: steamworks::SteamId) -> Self {
        Self::CloseP2pSession { user }
    }

    /// Creates a [`SteamworksNetworkingCommand::GetP2pSessionState`] command.
    pub fn get_p2p_session_state(user: steamworks::SteamId) -> Self {
        Self::GetP2pSessionState { user }
    }

    /// Creates a [`SteamworksNetworkingCommand::SendP2pPacket`] command.
    pub fn send_p2p_packet(
        remote: steamworks::SteamId,
        send_type: SteamworksP2pSendType,
        channel: u32,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self::SendP2pPacket {
            remote,
            send_type,
            channel,
            data: data.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingCommand::GetAvailablePacketSize`] command.
    pub fn get_available_packet_size(channel: u32) -> Self {
        Self::GetAvailablePacketSize { channel }
    }

    /// Creates a [`SteamworksNetworkingCommand::ReadP2pPacket`] command.
    pub fn read_p2p_packet(channel: u32, max_bytes: usize) -> Self {
        Self::ReadP2pPacket { channel, max_bytes }
    }
}

/// A successfully submitted legacy P2P operation, read, or callback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksNetworkingOperation {
    /// A remote legacy P2P session was accepted.
    SessionAccepted {
        /// Remote Steam user.
        user: steamworks::SteamId,
    },
    /// A remote legacy P2P session was closed.
    SessionClosed {
        /// Remote Steam user.
        user: steamworks::SteamId,
    },
    /// Session state was read for one remote Steam user.
    SessionStateRead {
        /// Session state result.
        state: SteamworksP2pSessionStateResult,
    },
    /// A packet was submitted to Steam.
    PacketSent {
        /// Remote Steam user.
        remote: steamworks::SteamId,
        /// Delivery mode.
        send_type: SteamworksP2pSendType,
        /// Channel sent on.
        channel: u32,
        /// Payload size in bytes.
        bytes: usize,
    },
    /// Packet availability was read for one channel.
    PacketAvailabilityRead {
        /// Availability snapshot.
        availability: SteamworksP2pPacketAvailability,
    },
    /// One read command completed.
    PacketRead {
        /// Channel read from.
        channel: u32,
        /// Packet snapshot, or `None` when no packet was available.
        packet: Option<SteamworksP2pPacket>,
    },
    /// A legacy P2P session request callback was observed.
    SessionRequestReceived {
        /// Remote Steam user requesting a session.
        remote: steamworks::SteamId,
    },
    /// A legacy P2P session connection failure callback was observed.
    SessionConnectFailed {
        /// Remote Steam user.
        remote: steamworks::SteamId,
        /// Session error decoded from Steam's callback.
        error: steamworks::P2PSessionError,
    },
}

/// Result message emitted by [`crate::SteamworksNetworkingPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksNetworkingResult {
    /// The command, read operation, or callback succeeded.
    Ok(SteamworksNetworkingOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksNetworkingCommand,
        /// Failure reason.
        error: SteamworksNetworkingError,
    },
}

/// Synchronous command errors from [`crate::SteamworksNetworkingPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksNetworkingError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
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
    pub(super) fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }
}
