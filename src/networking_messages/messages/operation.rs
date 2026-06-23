use super::super::{
    SteamworksNetworkingMessage, SteamworksNetworkingMessagesConnectionInfo,
    SteamworksNetworkingMessagesSessionDecision, SteamworksNetworkingMessagesSessionRequestInfo,
    SteamworksNetworkingPeer,
};

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
    /// A peer-specific callback-time session request decision was set.
    SessionRequestDecisionSet {
        /// Decision snapshot.
        decision: SteamworksNetworkingMessagesSessionDecision,
    },
    /// A peer-specific callback-time session request decision was cleared.
    SessionRequestDecisionCleared {
        /// Peer whose decision override was removed.
        peer: SteamworksNetworkingPeer,
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
