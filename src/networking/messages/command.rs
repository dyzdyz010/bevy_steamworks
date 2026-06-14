use bevy_ecs::message::Message;

use super::super::SteamworksP2pSendType;

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
    /// Creates a [`crate::SteamworksNetworkingCommand::AcceptP2pSession`] command.
    pub fn accept_p2p_session(user: steamworks::SteamId) -> Self {
        Self::AcceptP2pSession { user }
    }

    /// Creates a [`crate::SteamworksNetworkingCommand::CloseP2pSession`] command.
    pub fn close_p2p_session(user: steamworks::SteamId) -> Self {
        Self::CloseP2pSession { user }
    }

    /// Creates a [`crate::SteamworksNetworkingCommand::GetP2pSessionState`] command.
    pub fn get_p2p_session_state(user: steamworks::SteamId) -> Self {
        Self::GetP2pSessionState { user }
    }

    /// Creates a [`crate::SteamworksNetworkingCommand::SendP2pPacket`] command.
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

    /// Creates a [`crate::SteamworksNetworkingCommand::GetAvailablePacketSize`] command.
    pub fn get_available_packet_size(channel: u32) -> Self {
        Self::GetAvailablePacketSize { channel }
    }

    /// Creates a [`crate::SteamworksNetworkingCommand::ReadP2pPacket`] command.
    pub fn read_p2p_packet(channel: u32, max_bytes: usize) -> Self {
        Self::ReadP2pPacket { channel, max_bytes }
    }
}
