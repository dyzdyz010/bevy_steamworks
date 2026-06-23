use std::net::SocketAddr;

/// Listen socket endpoint created by a command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksNetworkingSocketsListenEndpoint {
    /// IP listen endpoint.
    Ip(SocketAddr),
    /// P2P virtual port.
    P2p {
        /// Local virtual port.
        local_virtual_port: i32,
    },
    /// Hosted dedicated-server virtual port.
    HostedDedicatedServer {
        /// Local virtual port.
        local_virtual_port: u32,
    },
}

/// Connection target created by a command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksNetworkingSocketsConnectionTarget {
    /// IP target.
    Ip(SocketAddr),
    /// P2P identity target.
    P2p {
        /// Remote identity.
        identity: steamworks::networking_types::NetworkingIdentity,
        /// Remote virtual port.
        remote_virtual_port: i32,
    },
}
