use std::net::SocketAddr;

/// Opaque ID for a listen socket owned by [`crate::SteamworksNetworkingSocketsPlugin`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SteamworksListenSocketId(u64);

impl SteamworksListenSocketId {
    /// Creates an ID from a raw integer.
    pub fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw integer backing this ID.
    pub fn raw(self) -> u64 {
        self.0
    }
}

/// Opaque ID for a connection owned by [`crate::SteamworksNetworkingSocketsPlugin`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SteamworksNetworkingSocketsConnectionId(u64);

impl SteamworksNetworkingSocketsConnectionId {
    /// Creates an ID from a raw integer.
    pub fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw integer backing this ID.
    pub fn raw(self) -> u64 {
        self.0
    }
}

/// Opaque ID for a poll group owned by [`crate::SteamworksNetworkingSocketsPlugin`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SteamworksNetworkingSocketsPollGroupId(u64);

impl SteamworksNetworkingSocketsPollGroupId {
    /// Creates an ID from a raw integer.
    pub fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw integer backing this ID.
    pub fn raw(self) -> u64 {
        self.0
    }
}

/// Policy used when a listen socket receives a connection request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksConnectionRequestPolicy {
    /// Accept incoming connection requests immediately.
    Accept,
    /// Reject incoming connection requests immediately.
    Reject {
        /// End reason sent to the remote peer.
        reason: steamworks::networking_types::NetConnectionEnd,
        /// Optional debug string sent to Steam.
        debug: Option<String>,
    },
}

impl Default for SteamworksConnectionRequestPolicy {
    fn default() -> Self {
        Self::Reject {
            reason: steamworks::networking_types::NetConnectionEnd::App(
                steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
            ),
            debug: Some("connection rejected by bevy_steamworks policy".to_owned()),
        }
    }
}

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

/// Sent-message snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsSentMessage {
    /// Connection sent on.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Assigned message number.
    pub message_number: u64,
    /// Payload size in bytes.
    pub bytes: usize,
}

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

/// Connection user-data mutation snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsConnectionUserData {
    /// Connection updated.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// User data value.
    pub user_data: i64,
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

/// Owned snapshot of one received Networking Sockets message.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsMessage {
    /// Connection that received the message.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Remote peer identity carried by Steam's message.
    pub peer: steamworks::networking_types::NetworkingIdentity,
    /// Message payload copied from Steam's message handle.
    pub data: Vec<u8>,
    /// Message lane/channel.
    pub channel: i32,
    /// Message flags reported by Steam.
    pub send_flags: steamworks::networking_types::SendFlags,
    /// Message number assigned by the sender.
    pub message_number: u64,
    /// Connection user data captured by Steam for this message.
    pub connection_user_data: i64,
}

impl std::fmt::Debug for SteamworksNetworkingSocketsMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SteamworksNetworkingSocketsMessage")
            .field("connection", &self.connection)
            .field("peer", &self.peer)
            .field("data_len", &self.data.len())
            .field("channel", &self.channel)
            .field("send_flags", &self.send_flags)
            .field("message_number", &self.message_number)
            .field("connection_user_data", &self.connection_user_data)
            .finish()
    }
}

/// Owned snapshot of one message received through a poll group.
///
/// The upstream safe wrapper does not expose the raw connection handle carried
/// by poll-group messages. Use [`crate::SteamworksNetworkingSocketsCommand::SetConnectionUserData`]
/// to attach an application-level connection identifier if you need to map
/// poll-group messages back to game state.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsPollGroupMessage {
    /// Poll group that received the message.
    pub poll_group: SteamworksNetworkingSocketsPollGroupId,
    /// Remote peer identity carried by Steam's message.
    pub peer: steamworks::networking_types::NetworkingIdentity,
    /// Message payload copied from Steam's message handle.
    pub data: Vec<u8>,
    /// Message lane/channel.
    pub channel: i32,
    /// Message flags reported by Steam.
    pub send_flags: steamworks::networking_types::SendFlags,
    /// Message number assigned by the sender.
    pub message_number: u64,
    /// Connection user data captured by Steam for this message.
    pub connection_user_data: i64,
}

impl std::fmt::Debug for SteamworksNetworkingSocketsPollGroupMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SteamworksNetworkingSocketsPollGroupMessage")
            .field("poll_group", &self.poll_group)
            .field("peer", &self.peer)
            .field("data_len", &self.data.len())
            .field("channel", &self.channel)
            .field("send_flags", &self.send_flags)
            .field("message_number", &self.message_number)
            .field("connection_user_data", &self.connection_user_data)
            .finish()
    }
}

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
