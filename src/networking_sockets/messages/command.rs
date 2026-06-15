use std::net::SocketAddr;

use bevy_ecs::message::Message;

use super::super::{
    SteamworksConnectionRequestPolicy, SteamworksListenSocketId,
    SteamworksNetworkingSocketsConfigEntry, SteamworksNetworkingSocketsConnectionId,
    SteamworksNetworkingSocketsOutboundMessage, SteamworksNetworkingSocketsPollGroupId,
};

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

impl std::fmt::Debug for SteamworksNetworkingSocketsCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitAuthentication => f.write_str("InitAuthentication"),
            Self::GetAuthenticationStatus => f.write_str("GetAuthenticationStatus"),
            Self::CreateListenSocketIp {
                local_address,
                options,
            } => f
                .debug_struct("CreateListenSocketIp")
                .field("local_address", local_address)
                .field("options", options)
                .finish(),
            Self::CreateListenSocketP2p {
                local_virtual_port,
                options,
            } => f
                .debug_struct("CreateListenSocketP2p")
                .field("local_virtual_port", local_virtual_port)
                .field("options", options)
                .finish(),
            Self::ConnectByIpAddress { address, options } => f
                .debug_struct("ConnectByIpAddress")
                .field("address", address)
                .field("options", options)
                .finish(),
            Self::ConnectP2p {
                identity,
                remote_virtual_port,
                options,
            } => f
                .debug_struct("ConnectP2p")
                .field("identity", identity)
                .field("remote_virtual_port", remote_virtual_port)
                .field("options", options)
                .finish(),
            Self::CreatePollGroup => f.write_str("CreatePollGroup"),
            Self::PollListenSocketEvents {
                listen_socket,
                max_events,
                request_policy,
            } => f
                .debug_struct("PollListenSocketEvents")
                .field("listen_socket", listen_socket)
                .field("max_events", max_events)
                .field("request_policy", request_policy)
                .finish(),
            Self::PollConnectionEvents {
                connection,
                max_events,
            } => f
                .debug_struct("PollConnectionEvents")
                .field("connection", connection)
                .field("max_events", max_events)
                .finish(),
            Self::GetConnectionInfo { connection } => f
                .debug_struct("GetConnectionInfo")
                .field("connection", connection)
                .finish(),
            Self::GetRealtimeConnectionStatus { connection, lanes } => f
                .debug_struct("GetRealtimeConnectionStatus")
                .field("connection", connection)
                .field("lanes", lanes)
                .finish(),
            Self::SendMessage {
                connection,
                send_flags,
                data,
            } => f
                .debug_struct("SendMessage")
                .field("connection", connection)
                .field("send_flags", send_flags)
                .field("data_len", &data.len())
                .finish(),
            Self::SendMessages { messages } => f
                .debug_struct("SendMessages")
                .field("messages", messages)
                .finish(),
            Self::ReceiveMessages {
                connection,
                batch_size,
            } => f
                .debug_struct("ReceiveMessages")
                .field("connection", connection)
                .field("batch_size", batch_size)
                .finish(),
            Self::ReceivePollGroupMessages {
                poll_group,
                batch_size,
            } => f
                .debug_struct("ReceivePollGroupMessages")
                .field("poll_group", poll_group)
                .field("batch_size", batch_size)
                .finish(),
            Self::FlushMessages { connection } => f
                .debug_struct("FlushMessages")
                .field("connection", connection)
                .finish(),
            Self::SetConnectionPollGroup {
                connection,
                poll_group,
            } => f
                .debug_struct("SetConnectionPollGroup")
                .field("connection", connection)
                .field("poll_group", poll_group)
                .finish(),
            Self::ClearConnectionPollGroup { connection } => f
                .debug_struct("ClearConnectionPollGroup")
                .field("connection", connection)
                .finish(),
            Self::ConfigureConnectionLanes {
                connection,
                lane_priorities,
                lane_weights,
            } => f
                .debug_struct("ConfigureConnectionLanes")
                .field("connection", connection)
                .field("lane_priorities", lane_priorities)
                .field("lane_weights", lane_weights)
                .finish(),
            Self::SetConnectionUserData {
                connection,
                user_data,
            } => f
                .debug_struct("SetConnectionUserData")
                .field("connection", connection)
                .field("user_data", user_data)
                .finish(),
            Self::CloseConnection {
                connection,
                reason,
                debug,
                enable_linger,
            } => f
                .debug_struct("CloseConnection")
                .field("connection", connection)
                .field("reason", reason)
                .field("debug", debug)
                .field("enable_linger", enable_linger)
                .finish(),
            Self::CloseListenSocket { listen_socket } => f
                .debug_struct("CloseListenSocket")
                .field("listen_socket", listen_socket)
                .finish(),
            Self::ClosePollGroup { poll_group } => f
                .debug_struct("ClosePollGroup")
                .field("poll_group", poll_group)
                .finish(),
        }
    }
}

impl SteamworksNetworkingSocketsCommand {
    /// Creates a [`SteamworksNetworkingSocketsCommand::InitAuthentication`] command.
    pub fn init_authentication() -> Self {
        Self::InitAuthentication
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::GetAuthenticationStatus`] command.
    pub fn get_authentication_status() -> Self {
        Self::GetAuthenticationStatus
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CreateListenSocketIp`] command.
    pub fn create_listen_socket_ip(local_address: SocketAddr) -> Self {
        Self::create_listen_socket_ip_with_options(local_address, [])
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CreateListenSocketIp`] command with config.
    pub fn create_listen_socket_ip_with_options(
        local_address: SocketAddr,
        options: impl Into<Vec<SteamworksNetworkingSocketsConfigEntry>>,
    ) -> Self {
        Self::CreateListenSocketIp {
            local_address,
            options: options.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CreateListenSocketP2p`] command.
    pub fn create_listen_socket_p2p(local_virtual_port: i32) -> Self {
        Self::create_listen_socket_p2p_with_options(local_virtual_port, [])
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CreateListenSocketP2p`] command with config.
    pub fn create_listen_socket_p2p_with_options(
        local_virtual_port: i32,
        options: impl Into<Vec<SteamworksNetworkingSocketsConfigEntry>>,
    ) -> Self {
        Self::CreateListenSocketP2p {
            local_virtual_port,
            options: options.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ConnectByIpAddress`] command.
    pub fn connect_by_ip_address(address: SocketAddr) -> Self {
        Self::connect_by_ip_address_with_options(address, [])
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ConnectByIpAddress`] command with config.
    pub fn connect_by_ip_address_with_options(
        address: SocketAddr,
        options: impl Into<Vec<SteamworksNetworkingSocketsConfigEntry>>,
    ) -> Self {
        Self::ConnectByIpAddress {
            address,
            options: options.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ConnectP2p`] command.
    pub fn connect_p2p(
        identity: steamworks::networking_types::NetworkingIdentity,
        remote_virtual_port: i32,
    ) -> Self {
        Self::connect_p2p_with_options(identity, remote_virtual_port, [])
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ConnectP2p`] command with config.
    pub fn connect_p2p_with_options(
        identity: steamworks::networking_types::NetworkingIdentity,
        remote_virtual_port: i32,
        options: impl Into<Vec<SteamworksNetworkingSocketsConfigEntry>>,
    ) -> Self {
        Self::ConnectP2p {
            identity,
            remote_virtual_port,
            options: options.into(),
        }
    }

    /// Creates a P2P connect command for a Steam user.
    pub fn connect_p2p_steam_id(steam_id: steamworks::SteamId, remote_virtual_port: i32) -> Self {
        Self::connect_p2p_steam_id_with_options(steam_id, remote_virtual_port, [])
    }

    /// Creates a P2P connect command for a Steam user with config.
    pub fn connect_p2p_steam_id_with_options(
        steam_id: steamworks::SteamId,
        remote_virtual_port: i32,
        options: impl Into<Vec<SteamworksNetworkingSocketsConfigEntry>>,
    ) -> Self {
        Self::connect_p2p_with_options(
            steamworks::networking_types::NetworkingIdentity::new_steam_id(steam_id),
            remote_virtual_port,
            options,
        )
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CreatePollGroup`] command.
    pub fn create_poll_group() -> Self {
        Self::CreatePollGroup
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::PollListenSocketEvents`] command.
    pub fn poll_listen_socket_events(
        listen_socket: SteamworksListenSocketId,
        max_events: usize,
        request_policy: SteamworksConnectionRequestPolicy,
    ) -> Self {
        Self::PollListenSocketEvents {
            listen_socket,
            max_events,
            request_policy,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::PollConnectionEvents`] command.
    pub fn poll_connection_events(
        connection: SteamworksNetworkingSocketsConnectionId,
        max_events: usize,
    ) -> Self {
        Self::PollConnectionEvents {
            connection,
            max_events,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::GetConnectionInfo`] command.
    pub fn get_connection_info(connection: SteamworksNetworkingSocketsConnectionId) -> Self {
        Self::GetConnectionInfo { connection }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::GetRealtimeConnectionStatus`] command.
    pub fn get_realtime_connection_status(
        connection: SteamworksNetworkingSocketsConnectionId,
        lanes: u32,
    ) -> Self {
        Self::GetRealtimeConnectionStatus { connection, lanes }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::SendMessage`] command.
    pub fn send_message(
        connection: SteamworksNetworkingSocketsConnectionId,
        send_flags: steamworks::networking_types::SendFlags,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self::SendMessage {
            connection,
            send_flags,
            data: data.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::SendMessages`] command.
    pub fn send_messages(
        messages: impl Into<Vec<SteamworksNetworkingSocketsOutboundMessage>>,
    ) -> Self {
        Self::SendMessages {
            messages: messages.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ReceiveMessages`] command.
    pub fn receive_messages(
        connection: SteamworksNetworkingSocketsConnectionId,
        batch_size: usize,
    ) -> Self {
        Self::ReceiveMessages {
            connection,
            batch_size,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ReceivePollGroupMessages`] command.
    pub fn receive_poll_group_messages(
        poll_group: SteamworksNetworkingSocketsPollGroupId,
        batch_size: usize,
    ) -> Self {
        Self::ReceivePollGroupMessages {
            poll_group,
            batch_size,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::FlushMessages`] command.
    pub fn flush_messages(connection: SteamworksNetworkingSocketsConnectionId) -> Self {
        Self::FlushMessages { connection }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::SetConnectionPollGroup`] command.
    pub fn set_connection_poll_group(
        connection: SteamworksNetworkingSocketsConnectionId,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) -> Self {
        Self::SetConnectionPollGroup {
            connection,
            poll_group,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ClearConnectionPollGroup`] command.
    pub fn clear_connection_poll_group(
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Self {
        Self::ClearConnectionPollGroup { connection }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ConfigureConnectionLanes`] command.
    pub fn configure_connection_lanes(
        connection: SteamworksNetworkingSocketsConnectionId,
        lane_priorities: impl Into<Vec<i32>>,
        lane_weights: impl Into<Vec<u16>>,
    ) -> Self {
        Self::ConfigureConnectionLanes {
            connection,
            lane_priorities: lane_priorities.into(),
            lane_weights: lane_weights.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::SetConnectionUserData`] command.
    pub fn set_connection_user_data(
        connection: SteamworksNetworkingSocketsConnectionId,
        user_data: i64,
    ) -> Self {
        Self::SetConnectionUserData {
            connection,
            user_data,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CloseConnection`] command.
    pub fn close_connection(connection: SteamworksNetworkingSocketsConnectionId) -> Self {
        Self::close_connection_with_reason(
            connection,
            steamworks::networking_types::NetConnectionEnd::App(
                steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
            ),
            None,
            false,
        )
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CloseConnection`] command with options.
    pub fn close_connection_with_reason(
        connection: SteamworksNetworkingSocketsConnectionId,
        reason: steamworks::networking_types::NetConnectionEnd,
        debug: Option<String>,
        enable_linger: bool,
    ) -> Self {
        Self::CloseConnection {
            connection,
            reason,
            debug,
            enable_linger,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CloseListenSocket`] command.
    pub fn close_listen_socket(listen_socket: SteamworksListenSocketId) -> Self {
        Self::CloseListenSocket { listen_socket }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ClosePollGroup`] command.
    pub fn close_poll_group(poll_group: SteamworksNetworkingSocketsPollGroupId) -> Self {
        Self::ClosePollGroup { poll_group }
    }
}
