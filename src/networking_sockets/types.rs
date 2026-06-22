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

/// Initial Steam Networking Sockets configuration entry for listen/connect commands.
///
/// This owned wrapper keeps command messages comparable and debuggable while
/// converting to upstream [`steamworks::networking_types::NetworkingConfigEntry`]
/// only after validation.
#[derive(Clone, PartialEq)]
pub enum SteamworksNetworkingSocketsConfigEntry {
    /// Signed 32-bit integer config value.
    Int32 {
        /// Upstream config key.
        value: steamworks::networking_types::NetworkingConfigValue,
        /// Config value.
        data: i32,
    },
    /// Signed 64-bit integer config value.
    Int64 {
        /// Upstream config key.
        value: steamworks::networking_types::NetworkingConfigValue,
        /// Config value.
        data: i64,
    },
    /// Floating-point config value.
    Float {
        /// Upstream config key.
        value: steamworks::networking_types::NetworkingConfigValue,
        /// Config value.
        data: f32,
    },
    /// String config value.
    String {
        /// Upstream config key.
        value: steamworks::networking_types::NetworkingConfigValue,
        /// Config value.
        data: String,
    },
}

impl std::fmt::Debug for SteamworksNetworkingSocketsConfigEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int32 { value, data } => f
                .debug_struct("Int32")
                .field("value", value)
                .field("data", data)
                .finish(),
            Self::Int64 { value, data } => f
                .debug_struct("Int64")
                .field("value", value)
                .field("data", data)
                .finish(),
            Self::Float { value, data } => f
                .debug_struct("Float")
                .field("value", value)
                .field("data", data)
                .finish(),
            Self::String { value, data } => f
                .debug_struct("String")
                .field("value", value)
                .field("data_len", &data.len())
                .finish(),
        }
    }
}

impl SteamworksNetworkingSocketsConfigEntry {
    /// Creates an integer config entry.
    pub fn int32(value: steamworks::networking_types::NetworkingConfigValue, data: i32) -> Self {
        Self::Int32 { value, data }
    }

    /// Creates a 64-bit integer config entry.
    pub fn int64(value: steamworks::networking_types::NetworkingConfigValue, data: i64) -> Self {
        Self::Int64 { value, data }
    }

    /// Creates a floating-point config entry.
    pub fn float(value: steamworks::networking_types::NetworkingConfigValue, data: f32) -> Self {
        Self::Float { value, data }
    }

    /// Creates a string config entry.
    pub fn string(
        value: steamworks::networking_types::NetworkingConfigValue,
        data: impl Into<String>,
    ) -> Self {
        Self::String {
            value,
            data: data.into(),
        }
    }

    pub(super) fn to_steam(&self) -> steamworks::networking_types::NetworkingConfigEntry {
        match self {
            Self::Int32 { value, data } => {
                steamworks::networking_types::NetworkingConfigEntry::new_int32(*value, *data)
            }
            Self::Int64 { value, data } => {
                steamworks::networking_types::NetworkingConfigEntry::new_int64(*value, *data)
            }
            Self::Float { value, data } => {
                steamworks::networking_types::NetworkingConfigEntry::new_float(*value, *data)
            }
            Self::String { value, data } => {
                steamworks::networking_types::NetworkingConfigEntry::new_string(*value, data)
            }
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

/// Owned outbound Networking Sockets message submitted by a batch send command.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsOutboundMessage {
    /// Connection to send on.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Delivery flags.
    pub send_flags: steamworks::networking_types::SendFlags,
    /// Message lane/channel.
    pub channel: i32,
    /// Payload copied into an upstream message before sending.
    pub data: Vec<u8>,
    /// Application user data attached to the outgoing message.
    pub user_data: i64,
}

impl SteamworksNetworkingSocketsOutboundMessage {
    /// Creates an outbound message for channel `0` with no user data.
    pub fn new(
        connection: SteamworksNetworkingSocketsConnectionId,
        send_flags: steamworks::networking_types::SendFlags,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            connection,
            send_flags,
            channel: 0,
            data: data.into(),
            user_data: 0,
        }
    }

    /// Sets the message lane/channel.
    pub fn with_channel(mut self, channel: i32) -> Self {
        self.channel = channel;
        self
    }

    /// Sets application user data on the outgoing message.
    pub fn with_user_data(mut self, user_data: i64) -> Self {
        self.user_data = user_data;
        self
    }
}

impl std::fmt::Debug for SteamworksNetworkingSocketsOutboundMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SteamworksNetworkingSocketsOutboundMessage")
            .field("connection", &self.connection)
            .field("send_flags", &self.send_flags)
            .field("channel", &self.channel)
            .field("data_len", &self.data.len())
            .field("user_data", &self.user_data)
            .finish()
    }
}

/// Per-message result returned by a batch send command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsMessageSendResult {
    /// Connection that this message targeted.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Delivery flags used by this message.
    pub send_flags: steamworks::networking_types::SendFlags,
    /// Message lane/channel.
    pub channel: i32,
    /// Payload size in bytes.
    pub bytes: usize,
    /// Application user data that was attached to the outgoing message.
    pub user_data: i64,
    /// Message number assigned by Steam, or the per-message Steam send error.
    pub result: Result<u64, steamworks::SteamError>,
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

impl std::fmt::Debug for SteamworksNetworkingSocketsConnectionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SteamworksNetworkingSocketsConnectionName")
            .field("connection", &self.connection)
            .field("name_len", &self.name.len())
            .finish()
    }
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

/// Message batch received from one connection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsConnectionMessages {
    /// Connection that was received from.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Owned message snapshots.
    pub messages: Vec<SteamworksNetworkingSocketsMessage>,
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

/// Message batch received from one poll group.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsPollGroupMessages {
    /// Poll group that was received from.
    pub poll_group: SteamworksNetworkingSocketsPollGroupId,
    /// Owned message snapshots.
    pub messages: Vec<SteamworksNetworkingSocketsPollGroupMessage>,
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
