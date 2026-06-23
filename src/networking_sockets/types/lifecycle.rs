use super::*;

/// Listen socket creation snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsListenSocketCreated {
    /// New plugin-owned listen socket ID.
    pub listen_socket: SteamworksListenSocketId,
    /// Bound local address or virtual-port descriptor.
    pub endpoint: SteamworksNetworkingSocketsListenEndpoint,
}

/// Connection creation snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsConnectionCreated {
    /// New plugin-owned connection ID.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Connection target.
    pub target: SteamworksNetworkingSocketsConnectionTarget,
}

/// Listen socket event batch snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksNetworkingSocketsListenSocketEvents {
    /// Listen socket that was polled.
    pub listen_socket: SteamworksListenSocketId,
    /// Events observed.
    pub events: Vec<SteamworksListenSocketEventInfo>,
}

/// Connection event batch snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsConnectionEvents {
    /// Connection that was polled.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Events observed.
    pub events: Vec<SteamworksNetworkingSocketsConnectionEventInfo>,
    /// Whether a terminal event caused the plugin to remove this connection.
    pub connection_removed: bool,
}
