use std::net::SocketAddr;

use bevy_ecs::message::Message;

use super::super::{
    SteamworksConnectionRequestPolicy, SteamworksListenSocketId,
    SteamworksNetworkingSocketsConfigEntry, SteamworksNetworkingSocketsConnectionId,
    SteamworksNetworkingSocketsOutboundMessage, SteamworksNetworkingSocketsPollGroupId,
};

mod constructors;
mod debug;

/// A high-level command for Steam Networking Sockets workflows.
#[derive(Clone, Message, PartialEq)]
pub enum SteamworksNetworkingSocketsCommand {
    /// Initialize Steam Networking Sockets authentication resources.
    InitAuthentication,
    /// Read Steam Networking Sockets authentication status.
    GetAuthenticationStatus,
    /// Create an IP listen socket.
    CreateListenSocketIp {
        /// Local socket address to bind.
        local_address: SocketAddr,
        /// Initial Steam Networking Sockets config entries.
        options: Vec<SteamworksNetworkingSocketsConfigEntry>,
    },
    /// Create a P2P listen socket.
    CreateListenSocketP2p {
        /// Local virtual port.
        local_virtual_port: i32,
        /// Initial Steam Networking Sockets config entries.
        options: Vec<SteamworksNetworkingSocketsConfigEntry>,
    },
    /// Create a hosted dedicated-server listen socket.
    CreateHostedDedicatedServerListenSocket {
        /// Local virtual port.
        local_virtual_port: u32,
        /// Initial Steam Networking Sockets config entries.
        options: Vec<SteamworksNetworkingSocketsConfigEntry>,
    },
    /// Connect to an IP endpoint.
    ConnectByIpAddress {
        /// Remote socket address.
        address: SocketAddr,
        /// Initial Steam Networking Sockets config entries.
        options: Vec<SteamworksNetworkingSocketsConfigEntry>,
    },
    /// Connect to a Steam networking identity using P2P.
    ConnectP2p {
        /// Remote identity.
        identity: steamworks::networking_types::NetworkingIdentity,
        /// Remote virtual port.
        remote_virtual_port: i32,
        /// Initial Steam Networking Sockets config entries.
        options: Vec<SteamworksNetworkingSocketsConfigEntry>,
    },
    /// Create a poll group for receiving messages from many connections.
    CreatePollGroup,
    /// Create a server-owned poll group for receiving messages from many server connections.
    CreateServerPollGroup,
    /// Poll events from one listen socket.
    PollListenSocketEvents {
        /// Listen socket to poll.
        listen_socket: SteamworksListenSocketId,
        /// Maximum events to process.
        max_events: usize,
        /// Policy for incoming connection requests.
        request_policy: SteamworksConnectionRequestPolicy,
    },
    /// Poll events from one connection.
    PollConnectionEvents {
        /// Connection to poll.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Maximum events to process.
        max_events: usize,
    },
    /// Read connection info.
    GetConnectionInfo {
        /// Connection to inspect.
        connection: SteamworksNetworkingSocketsConnectionId,
    },
    /// Read connection user data.
    GetConnectionUserData {
        /// Connection to inspect.
        connection: SteamworksNetworkingSocketsConnectionId,
    },
    /// Read realtime connection status.
    GetRealtimeConnectionStatus {
        /// Connection to inspect.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Number of lane statuses to request.
        lanes: u32,
    },
    /// Send one message on a connection.
    SendMessage {
        /// Connection to send on.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Delivery flags.
        send_flags: steamworks::networking_types::SendFlags,
        /// Payload.
        data: Vec<u8>,
    },
    /// Send multiple allocated messages with per-message lane/channel support.
    SendMessages {
        /// Outbound messages to submit.
        messages: Vec<SteamworksNetworkingSocketsOutboundMessage>,
    },
    /// Receive messages from one connection.
    ReceiveMessages {
        /// Connection to receive from.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Maximum number of messages to receive.
        batch_size: usize,
    },
    /// Receive messages from one poll group.
    ReceivePollGroupMessages {
        /// Poll group to receive from.
        poll_group: SteamworksNetworkingSocketsPollGroupId,
        /// Maximum number of messages to receive.
        batch_size: usize,
    },
    /// Flush pending messages on one connection.
    FlushMessages {
        /// Connection to flush.
        connection: SteamworksNetworkingSocketsConnectionId,
    },
    /// Assign a connection to a poll group.
    SetConnectionPollGroup {
        /// Connection to assign.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Poll group that should receive messages for the connection.
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    },
    /// Remove a connection from its current poll group.
    ClearConnectionPollGroup {
        /// Connection to update.
        connection: SteamworksNetworkingSocketsConnectionId,
    },
    /// Configure outbound message lanes for one connection.
    ConfigureConnectionLanes {
        /// Connection to configure.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Lane priorities in Steam order.
        lane_priorities: Vec<i32>,
        /// Lane weights in Steam order.
        lane_weights: Vec<u16>,
    },
    /// Set connection user data.
    SetConnectionUserData {
        /// Connection to update.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// User data value.
        user_data: i64,
    },
    /// Set a connection debug name.
    SetConnectionName {
        /// Connection to update.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Debug name submitted to Steam.
        name: String,
    },
    /// Close and drop one connection.
    CloseConnection {
        /// Connection to close.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// End reason sent to Steam.
        reason: steamworks::networking_types::NetConnectionEnd,
        /// Optional debug string.
        debug: Option<String>,
        /// Whether Steam should try to flush remaining reliable data.
        enable_linger: bool,
    },
    /// Drop one listen socket.
    CloseListenSocket {
        /// Listen socket to drop.
        listen_socket: SteamworksListenSocketId,
    },
    /// Drop one poll group.
    ClosePollGroup {
        /// Poll group to drop.
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    },
}
