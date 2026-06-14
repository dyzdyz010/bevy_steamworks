use std::net::Ipv4Addr;

use super::{STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES, STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES};

/// Delivery mode for a legacy P2P packet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksP2pSendType {
    /// Send directly over UDP. Payloads must be at most 1200 bytes.
    Unreliable,
    /// Like [`Self::Unreliable`], but does not buffer packets before connection start.
    UnreliableNoDelay,
    /// Reliable packet sending. Payloads must be at most 1 MiB.
    Reliable,
    /// Reliable sending with Steam's buffering behavior.
    ReliableWithBuffering,
}

impl SteamworksP2pSendType {
    pub(super) fn to_steam(self) -> steamworks::SendType {
        match self {
            Self::Unreliable => steamworks::SendType::Unreliable,
            Self::UnreliableNoDelay => steamworks::SendType::UnreliableNoDelay,
            Self::Reliable => steamworks::SendType::Reliable,
            Self::ReliableWithBuffering => steamworks::SendType::ReliableWithBuffering,
        }
    }

    pub(super) fn max_packet_bytes(self) -> usize {
        match self {
            Self::Unreliable | Self::UnreliableNoDelay => {
                STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES
            }
            Self::Reliable | Self::ReliableWithBuffering => {
                STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES
            }
        }
    }
}

/// Owned snapshot of a legacy P2P session state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksP2pSessionState {
    /// Whether Steam reports an active open connection.
    pub connection_active: bool,
    /// Whether Steam is currently trying to establish the connection.
    pub connecting: bool,
    /// Last session error reported by Steam.
    pub error: steamworks::P2PSessionError,
    /// Whether the session is routed through a Steam relay server.
    pub using_relay: bool,
    /// Bytes queued for sending.
    pub bytes_queued_for_send: i32,
    /// Packets queued for sending.
    pub packets_queued_for_send: i32,
    /// Potential remote IP address, when Steam exposes one.
    pub remote_ip: Option<Ipv4Addr>,
    /// Potential remote port, when Steam exposes one.
    pub remote_port: Option<u16>,
}

impl From<steamworks::P2PSessionState> for SteamworksP2pSessionState {
    fn from(state: steamworks::P2PSessionState) -> Self {
        Self {
            connection_active: state.connection_active,
            connecting: state.connecting,
            error: state.error,
            using_relay: state.using_relay,
            bytes_queued_for_send: state.bytes_queued_for_send,
            packets_queued_for_send: state.packets_queued_for_send,
            remote_ip: state.remote_ip,
            remote_port: state.remote_port,
        }
    }
}

/// Result of reading legacy P2P session state for one user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksP2pSessionStateResult {
    /// Remote Steam user inspected.
    pub user: steamworks::SteamId,
    /// Session state, or `None` when Steam reports no session for the user.
    pub state: Option<SteamworksP2pSessionState>,
}

/// Owned snapshot of one received legacy P2P packet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksP2pPacket {
    /// Remote Steam user that sent the packet.
    pub remote: steamworks::SteamId,
    /// Channel the packet was read from.
    pub channel: u32,
    /// Packet payload.
    pub data: Vec<u8>,
}

/// Snapshot of one submitted legacy P2P packet send.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SteamworksP2pPacketSent {
    /// Remote Steam user targeted by the packet.
    pub remote: steamworks::SteamId,
    /// Delivery mode used for the send.
    pub send_type: SteamworksP2pSendType,
    /// Channel sent on.
    pub channel: u32,
    /// Payload size in bytes.
    pub bytes: usize,
}

/// Packet availability snapshot for one legacy P2P channel.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksP2pPacketAvailability {
    /// Channel that was inspected.
    pub channel: u32,
    /// Available packet size in bytes, if a packet is queued.
    pub bytes: Option<usize>,
}

/// Legacy P2P session connection failure callback snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SteamworksP2pSessionConnectFailure {
    /// Remote Steam user.
    pub remote: steamworks::SteamId,
    /// Session error decoded from Steam's callback.
    pub error: steamworks::P2PSessionError,
}
