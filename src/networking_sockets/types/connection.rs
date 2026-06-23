use super::*;

/// Connection poll-group assignment snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsPollGroupAssignment {
    /// Connection assigned.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Poll group assigned.
    pub poll_group: SteamworksNetworkingSocketsPollGroupId,
}

/// Connection lane configuration snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsLaneConfiguration {
    /// Connection configured.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Number of lanes configured.
    pub lanes: usize,
}

/// Connection user-data read or mutation snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsConnectionUserData {
    /// Connection inspected or updated.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// User data value.
    pub user_data: i64,
}

/// Connection debug-name mutation snapshot.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsConnectionName {
    /// Connection updated.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Debug name submitted to Steam.
    pub name: String,
}

/// Connection close snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsConnectionClosed {
    /// Connection removed.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Return value from Steam's close call.
    pub close_succeeded: bool,
}

/// Listen socket close snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsListenSocketClosed {
    /// Listen socket removed.
    pub listen_socket: SteamworksListenSocketId,
    /// Accepted child connections removed with the listen socket.
    pub closed_connections: Vec<SteamworksNetworkingSocketsConnectionId>,
}

/// Owned snapshot of one Networking Sockets connection.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksNetworkingSocketsConnectionInfo {
    /// Plugin-owned connection ID.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// High-level connection state reported by Steam.
    pub state: steamworks::networking_types::NetworkingConnectionState,
    /// Remote peer identity when Steam reports one.
    pub remote: Option<steamworks::networking_types::NetworkingIdentity>,
    /// Connection user data reported by Steam.
    pub user_data: i64,
    /// End reason reported by Steam when the connection has ended.
    pub end_reason: Option<steamworks::networking_types::NetConnectionEnd>,
}

/// Owned snapshot of one Networking Sockets realtime lane.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksNetworkingSocketsRealtimeLaneStatus {
    /// Pending unreliable bytes for this lane.
    pub pending_unreliable: i32,
    /// Pending reliable bytes for this lane.
    pub pending_reliable: i32,
    /// Sent reliable bytes awaiting acknowledgement for this lane.
    pub sent_unacked_reliable: i32,
    /// Lane-specific queue time reported by Steam.
    pub queued_send_bytes: i64,
}

/// Owned snapshot of realtime connection status.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksNetworkingSocketsRealtimeStatus {
    /// Plugin-owned connection ID.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Connection state in the realtime status block.
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
    /// Sent reliable bytes awaiting acknowledgement.
    pub sent_unacked_reliable: i32,
    /// Queue time reported by Steam.
    pub queued_send_bytes: i64,
    /// Per-lane status snapshots.
    pub lanes: Vec<SteamworksNetworkingSocketsRealtimeLaneStatus>,
}
