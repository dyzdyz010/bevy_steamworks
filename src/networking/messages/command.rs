use bevy_ecs::message::Message;

use super::super::SteamworksP2pSendType;

mod constructors;

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
