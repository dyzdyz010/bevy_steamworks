use super::super::{
    SteamworksListenSocketEventInfo, SteamworksListenSocketId,
    SteamworksNetworkingSocketsConnectionEventInfo, SteamworksNetworkingSocketsConnectionId,
    SteamworksNetworkingSocketsConnectionInfo, SteamworksNetworkingSocketsConnectionTarget,
    SteamworksNetworkingSocketsListenEndpoint, SteamworksNetworkingSocketsMessage,
    SteamworksNetworkingSocketsPollGroupId, SteamworksNetworkingSocketsPollGroupMessage,
    SteamworksNetworkingSocketsRealtimeStatus,
};

/// A successfully submitted Networking Sockets operation, read, or event.
#[derive(Clone, Debug, PartialEq)]
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
