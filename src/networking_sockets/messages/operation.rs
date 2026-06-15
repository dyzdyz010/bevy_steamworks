use super::super::{
    SteamworksListenSocketEventInfo, SteamworksListenSocketId,
    SteamworksNetworkingSocketsConnectionEventInfo, SteamworksNetworkingSocketsConnectionId,
    SteamworksNetworkingSocketsConnectionInfo, SteamworksNetworkingSocketsConnectionTarget,
    SteamworksNetworkingSocketsListenEndpoint, SteamworksNetworkingSocketsMessage,
    SteamworksNetworkingSocketsMessageSendResult, SteamworksNetworkingSocketsPollGroupId,
    SteamworksNetworkingSocketsPollGroupMessage, SteamworksNetworkingSocketsRealtimeStatus,
};

/// A successfully submitted Networking Sockets operation, read, or event.
#[derive(Clone, PartialEq)]
pub enum SteamworksNetworkingSocketsOperation {
    /// Authentication initialization was submitted.
    AuthenticationInitialized {
        /// Current authentication availability.
        availability: steamworks::networking_types::NetworkingAvailabilityResult,
    },
    /// Authentication status was read.
    AuthenticationStatusRead {
        /// Current authentication availability.
        availability: steamworks::networking_types::NetworkingAvailabilityResult,
    },
    /// A listen socket was created.
    ListenSocketCreated {
        /// New plugin-owned listen socket ID.
        listen_socket: SteamworksListenSocketId,
        /// Bound local address or virtual-port descriptor.
        endpoint: SteamworksNetworkingSocketsListenEndpoint,
    },
    /// A connection was created.
    ConnectionCreated {
        /// New plugin-owned connection ID.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Connection target.
        target: SteamworksNetworkingSocketsConnectionTarget,
    },
    /// A poll group was created.
    PollGroupCreated {
        /// New plugin-owned poll group ID.
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    },
    /// Listen socket events were processed.
    ListenSocketEventsPolled {
        /// Listen socket that was polled.
        listen_socket: SteamworksListenSocketId,
        /// Events observed.
        events: Vec<SteamworksListenSocketEventInfo>,
    },
    /// Connection events were processed.
    ConnectionEventsPolled {
        /// Connection that was polled.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Events observed.
        events: Vec<SteamworksNetworkingSocketsConnectionEventInfo>,
        /// Whether a terminal event caused the plugin to remove this connection.
        connection_removed: bool,
    },
    /// Connection info was read.
    ConnectionInfoRead {
        /// Connection info snapshot.
        info: SteamworksNetworkingSocketsConnectionInfo,
    },
    /// Realtime connection status was read.
    RealtimeConnectionStatusRead {
        /// Realtime status snapshot.
        status: SteamworksNetworkingSocketsRealtimeStatus,
    },
    /// One message was sent.
    MessageSent {
        /// Connection sent on.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Assigned message number.
        message_number: u64,
        /// Payload size in bytes.
        bytes: usize,
    },
    /// Multiple allocated messages were submitted.
    MessagesSent {
        /// Per-message send outcomes in command order.
        messages: Vec<SteamworksNetworkingSocketsMessageSendResult>,
    },
    /// Messages were received.
    MessagesReceived {
        /// Connection received from.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Owned message snapshots.
        messages: Vec<SteamworksNetworkingSocketsMessage>,
    },
    /// Messages were received from a poll group.
    PollGroupMessagesReceived {
        /// Poll group received from.
        poll_group: SteamworksNetworkingSocketsPollGroupId,
        /// Owned message snapshots.
        messages: Vec<SteamworksNetworkingSocketsPollGroupMessage>,
    },
    /// Pending messages were flushed.
    MessagesFlushed {
        /// Connection flushed.
        connection: SteamworksNetworkingSocketsConnectionId,
    },
    /// A connection was assigned to a poll group.
    ConnectionPollGroupSet {
        /// Connection assigned.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Poll group assigned.
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    },
    /// A connection was removed from its current poll group.
    ConnectionPollGroupCleared {
        /// Connection updated.
        connection: SteamworksNetworkingSocketsConnectionId,
    },
    /// Outbound message lanes were configured for a connection.
    ConnectionLanesConfigured {
        /// Connection configured.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Number of lanes configured.
        lanes: usize,
    },
    /// Connection user data was set.
    ConnectionUserDataSet {
        /// Connection updated.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// User data value.
        user_data: i64,
    },
    /// Connection debug name was set.
    ConnectionNameSet {
        /// Connection updated.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Debug name submitted to Steam.
        name: String,
    },
    /// A connection was closed and removed.
    ConnectionClosed {
        /// Connection removed.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Return value from Steam's close call.
        close_succeeded: bool,
    },
    /// A listen socket was closed and removed.
    ListenSocketClosed {
        /// Listen socket removed.
        listen_socket: SteamworksListenSocketId,
        /// Accepted child connections removed with the listen socket.
        closed_connections: Vec<SteamworksNetworkingSocketsConnectionId>,
    },
    /// A poll group was closed and removed.
    PollGroupClosed {
        /// Poll group removed.
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    },
}

impl std::fmt::Debug for SteamworksNetworkingSocketsOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthenticationInitialized { availability } => f
                .debug_struct("AuthenticationInitialized")
                .field("availability", availability)
                .finish(),
            Self::AuthenticationStatusRead { availability } => f
                .debug_struct("AuthenticationStatusRead")
                .field("availability", availability)
                .finish(),
            Self::ListenSocketCreated {
                listen_socket,
                endpoint,
            } => f
                .debug_struct("ListenSocketCreated")
                .field("listen_socket", listen_socket)
                .field("endpoint", endpoint)
                .finish(),
            Self::ConnectionCreated { connection, target } => f
                .debug_struct("ConnectionCreated")
                .field("connection", connection)
                .field("target", target)
                .finish(),
            Self::PollGroupCreated { poll_group } => f
                .debug_struct("PollGroupCreated")
                .field("poll_group", poll_group)
                .finish(),
            Self::ListenSocketEventsPolled {
                listen_socket,
                events,
            } => f
                .debug_struct("ListenSocketEventsPolled")
                .field("listen_socket", listen_socket)
                .field("events", events)
                .finish(),
            Self::ConnectionEventsPolled {
                connection,
                events,
                connection_removed,
            } => f
                .debug_struct("ConnectionEventsPolled")
                .field("connection", connection)
                .field("events", events)
                .field("connection_removed", connection_removed)
                .finish(),
            Self::ConnectionInfoRead { info } => f
                .debug_struct("ConnectionInfoRead")
                .field("info", info)
                .finish(),
            Self::RealtimeConnectionStatusRead { status } => f
                .debug_struct("RealtimeConnectionStatusRead")
                .field("status", status)
                .finish(),
            Self::MessageSent {
                connection,
                message_number,
                bytes,
            } => f
                .debug_struct("MessageSent")
                .field("connection", connection)
                .field("message_number", message_number)
                .field("bytes", bytes)
                .finish(),
            Self::MessagesSent { messages } => f
                .debug_struct("MessagesSent")
                .field("messages", messages)
                .finish(),
            Self::MessagesReceived {
                connection,
                messages,
            } => f
                .debug_struct("MessagesReceived")
                .field("connection", connection)
                .field("messages", messages)
                .finish(),
            Self::PollGroupMessagesReceived {
                poll_group,
                messages,
            } => f
                .debug_struct("PollGroupMessagesReceived")
                .field("poll_group", poll_group)
                .field("messages", messages)
                .finish(),
            Self::MessagesFlushed { connection } => f
                .debug_struct("MessagesFlushed")
                .field("connection", connection)
                .finish(),
            Self::ConnectionPollGroupSet {
                connection,
                poll_group,
            } => f
                .debug_struct("ConnectionPollGroupSet")
                .field("connection", connection)
                .field("poll_group", poll_group)
                .finish(),
            Self::ConnectionPollGroupCleared { connection } => f
                .debug_struct("ConnectionPollGroupCleared")
                .field("connection", connection)
                .finish(),
            Self::ConnectionLanesConfigured { connection, lanes } => f
                .debug_struct("ConnectionLanesConfigured")
                .field("connection", connection)
                .field("lanes", lanes)
                .finish(),
            Self::ConnectionUserDataSet {
                connection,
                user_data,
            } => f
                .debug_struct("ConnectionUserDataSet")
                .field("connection", connection)
                .field("user_data", user_data)
                .finish(),
            Self::ConnectionNameSet { connection, name } => f
                .debug_struct("ConnectionNameSet")
                .field("connection", connection)
                .field("name_len", &name.len())
                .finish(),
            Self::ConnectionClosed {
                connection,
                close_succeeded,
            } => f
                .debug_struct("ConnectionClosed")
                .field("connection", connection)
                .field("close_succeeded", close_succeeded)
                .finish(),
            Self::ListenSocketClosed {
                listen_socket,
                closed_connections,
            } => f
                .debug_struct("ListenSocketClosed")
                .field("listen_socket", listen_socket)
                .field("closed_connections", closed_connections)
                .finish(),
            Self::PollGroupClosed { poll_group } => f
                .debug_struct("PollGroupClosed")
                .field("poll_group", poll_group)
                .finish(),
        }
    }
}
