use super::super::{
    SteamworksP2pPacket, SteamworksP2pPacketAvailability, SteamworksP2pSendType,
    SteamworksP2pSessionStateResult,
};

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
