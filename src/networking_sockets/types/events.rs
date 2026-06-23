use super::*;

/// Snapshot of one listen socket event.
#[derive(Clone, Debug, PartialEq)]
pub enum SteamworksListenSocketEventInfo {
    /// An incoming connection request was accepted.
    ConnectionAccepted {
        /// Listen socket that received the request.
        listen_socket: SteamworksListenSocketId,
        /// Remote peer identity.
        remote: steamworks::networking_types::NetworkingIdentity,
        /// Connection user data reported by Steam.
        user_data: i64,
    },
    /// An incoming connection request was rejected.
    ConnectionRejected {
        /// Listen socket that received the request.
        listen_socket: SteamworksListenSocketId,
        /// Remote peer identity.
        remote: steamworks::networking_types::NetworkingIdentity,
        /// Connection user data reported by Steam.
        user_data: i64,
    },
    /// A connection on a listen socket reached the connected state.
    Connected {
        /// Listen socket that owns the connection.
        listen_socket: SteamworksListenSocketId,
        /// New plugin-owned connection ID.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Remote peer identity.
        remote: steamworks::networking_types::NetworkingIdentity,
        /// Connection user data reported by Steam.
        user_data: i64,
    },
    /// A connection on a listen socket disconnected.
    Disconnected {
        /// Listen socket that observed the disconnect.
        listen_socket: SteamworksListenSocketId,
        /// Plugin-owned connection ID that was removed, if it could be matched unambiguously.
        connection: Option<SteamworksNetworkingSocketsConnectionId>,
        /// Remote peer identity.
        remote: steamworks::networking_types::NetworkingIdentity,
        /// Connection user data reported by Steam.
        user_data: i64,
        /// End reason reported by Steam.
        end_reason: steamworks::networking_types::NetConnectionEnd,
    },
}

/// Snapshot of one connection event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsConnectionEventInfo {
    /// Connection that observed the event.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// New connection state.
    pub new_state: steamworks::networking_types::NetworkingConnectionState,
    /// Previous connection state.
    pub old_state: steamworks::networking_types::NetworkingConnectionState,
}
