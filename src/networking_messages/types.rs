use std::net::SocketAddr;

/// Peer identity accepted by the high-level Networking Messages command layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksNetworkingPeer {
    /// A Steam user identity.
    SteamId(steamworks::SteamId),
    /// An IP endpoint identity.
    Ip(SocketAddr),
    /// The local host identity.
    LocalHost,
    /// A prebuilt upstream Steam networking identity.
    Identity(steamworks::networking_types::NetworkingIdentity),
}

impl SteamworksNetworkingPeer {
    /// Creates a peer from a Steam ID.
    pub fn steam_id(id: steamworks::SteamId) -> Self {
        Self::SteamId(id)
    }

    /// Creates a peer from an IP socket address.
    pub fn ip(addr: SocketAddr) -> Self {
        Self::Ip(addr)
    }

    /// Creates a peer for the local host identity.
    pub fn local_host() -> Self {
        Self::LocalHost
    }

    /// Creates a peer from a prebuilt upstream identity.
    pub fn identity(identity: steamworks::networking_types::NetworkingIdentity) -> Self {
        Self::Identity(identity)
    }

    pub(super) fn to_identity(&self) -> steamworks::networking_types::NetworkingIdentity {
        match self {
            Self::SteamId(id) => {
                steamworks::networking_types::NetworkingIdentity::new_steam_id(*id)
            }
            Self::Ip(addr) => steamworks::networking_types::NetworkingIdentity::new_ip(*addr),
            Self::LocalHost => {
                let mut identity = steamworks::networking_types::NetworkingIdentity::new();
                identity.set_local_host();
                identity
            }
            Self::Identity(identity) => identity.clone(),
        }
    }
}

/// Owned snapshot of one received Steam networking message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingMessage {
    /// Identity of the peer that sent the message.
    pub peer: steamworks::networking_types::NetworkingIdentity,
    /// Message payload copied from Steam's message handle.
    pub data: Vec<u8>,
    /// Channel carried by the message.
    pub channel: i32,
    /// Message flags reported by Steam.
    pub send_flags: steamworks::networking_types::SendFlags,
    /// Message number assigned by the sender.
    pub message_number: u64,
    /// Connection user data captured by Steam for this message.
    pub connection_user_data: i64,
}

/// Snapshot of one Networking Messages session request callback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingMessagesSessionRequestInfo {
    /// Remote peer requesting a session.
    pub remote: steamworks::networking_types::NetworkingIdentity,
    /// Whether the plugin accepted the request in the callback.
    pub accepted: bool,
}

/// Snapshot of one Networking Messages session connection state.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksNetworkingMessagesConnectionInfo {
    /// High-level connection state reported by Steam.
    pub state: steamworks::networking_types::NetworkingConnectionState,
    /// Remote peer identity when Steam reports one.
    pub remote: Option<steamworks::networking_types::NetworkingIdentity>,
    /// Connection user data reported by Steam.
    pub user_data: Option<i64>,
    /// End reason reported by Steam when the session has ended.
    pub end_reason: Option<steamworks::networking_types::NetConnectionEnd>,
    /// Real-time status when Steam reports one for the session.
    pub realtime: Option<SteamworksNetworkingMessagesRealtimeInfo>,
}

/// Real-time Networking Messages session status.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksNetworkingMessagesRealtimeInfo {
    /// Connection state in the real-time status block.
    pub connection_state: steamworks::networking_types::NetworkingConnectionState,
    /// Estimated ping in milliseconds.
    pub ping: i32,
    /// Local delivery quality between 0.0 and 1.0.
    pub connection_quality_local: f32,
    /// Remote delivery quality between 0.0 and 1.0.
    pub connection_quality_remote: f32,
    /// Outbound packet rate.
    pub out_packets_per_sec: f32,
    /// Outbound byte rate.
    pub out_bytes_per_sec: f32,
    /// Inbound packet rate.
    pub in_packets_per_sec: f32,
    /// Inbound byte rate.
    pub in_bytes_per_sec: f32,
    /// Estimated send capacity in bytes per second.
    pub send_rate_bytes_per_sec: i32,
    /// Pending unreliable bytes.
    pub pending_unreliable: i32,
    /// Pending reliable bytes.
    pub pending_reliable: i32,
}
