use super::{
    SteamworksNetworkingSocketsConnectionClosed, SteamworksNetworkingSocketsConnectionCreated,
    SteamworksNetworkingSocketsConnectionEvents, SteamworksNetworkingSocketsConnectionId,
    SteamworksNetworkingSocketsConnectionInfo, SteamworksNetworkingSocketsConnectionName,
    SteamworksNetworkingSocketsConnectionUserData, SteamworksNetworkingSocketsError,
    SteamworksNetworkingSocketsLaneConfiguration, SteamworksNetworkingSocketsListenSocketClosed,
    SteamworksNetworkingSocketsListenSocketCreated, SteamworksNetworkingSocketsListenSocketEvents,
    SteamworksNetworkingSocketsMessage, SteamworksNetworkingSocketsMessageSendResult,
    SteamworksNetworkingSocketsPollGroupAssignment, SteamworksNetworkingSocketsPollGroupId,
    SteamworksNetworkingSocketsPollGroupMessage, SteamworksNetworkingSocketsRealtimeStatus,
    SteamworksNetworkingSocketsSentMessage, SteamworksNetworkingSocketsState,
};

impl SteamworksNetworkingSocketsState {
    /// Returns the most recent synchronous command error observed by the plugin.
    pub fn last_error(&self) -> Option<&SteamworksNetworkingSocketsError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent authentication availability read through the plugin.
    pub fn last_authentication_status(
        &self,
    ) -> Option<&steamworks::networking_types::NetworkingAvailabilityResult> {
        self.last_authentication_status.as_ref()
    }

    /// Returns the most recent listen socket created through this plugin.
    pub fn last_created_listen_socket(
        &self,
    ) -> Option<&SteamworksNetworkingSocketsListenSocketCreated> {
        self.last_created_listen_socket.as_ref()
    }

    /// Returns the most recent connection created through this plugin.
    pub fn last_created_connection(&self) -> Option<&SteamworksNetworkingSocketsConnectionCreated> {
        self.last_created_connection.as_ref()
    }

    /// Returns the most recent poll group created through this plugin.
    pub fn last_created_poll_group(&self) -> Option<SteamworksNetworkingSocketsPollGroupId> {
        self.last_created_poll_group
    }

    /// Returns the most recent listen-socket event batch processed through this plugin.
    pub fn last_listen_socket_events(
        &self,
    ) -> Option<&SteamworksNetworkingSocketsListenSocketEvents> {
        self.last_listen_socket_events.as_ref()
    }

    /// Returns the most recent connection event batch processed through this plugin.
    pub fn last_connection_events(&self) -> Option<&SteamworksNetworkingSocketsConnectionEvents> {
        self.last_connection_events.as_ref()
    }

    /// Returns the most recent connection info snapshot read through the plugin.
    pub fn last_connection_info(&self) -> Option<&SteamworksNetworkingSocketsConnectionInfo> {
        self.last_connection_info.as_ref()
    }

    /// Returns the most recent realtime connection status snapshot.
    pub fn last_realtime_status(&self) -> Option<&SteamworksNetworkingSocketsRealtimeStatus> {
        self.last_realtime_status.as_ref()
    }

    /// Returns the most recent sent-message snapshot.
    pub fn last_sent_message(&self) -> Option<&SteamworksNetworkingSocketsSentMessage> {
        self.last_sent_message.as_ref()
    }

    /// Returns the most recent batch send outcomes.
    pub fn last_sent_messages(&self) -> &[SteamworksNetworkingSocketsMessageSendResult] {
        &self.last_sent_messages
    }

    /// Returns the most recent batch of received messages.
    pub fn last_received_messages(&self) -> &[SteamworksNetworkingSocketsMessage] {
        &self.last_received_messages
    }

    /// Returns the most recent batch of messages received from a poll group.
    pub fn last_poll_group_messages(&self) -> &[SteamworksNetworkingSocketsPollGroupMessage] {
        &self.last_poll_group_messages
    }

    /// Returns the most recent connection flushed through this plugin.
    pub fn last_flushed_connection(&self) -> Option<SteamworksNetworkingSocketsConnectionId> {
        self.last_flushed_connection
    }

    /// Returns the most recent connection-to-poll-group assignment.
    pub fn last_connection_poll_group_set(
        &self,
    ) -> Option<&SteamworksNetworkingSocketsPollGroupAssignment> {
        self.last_connection_poll_group_set.as_ref()
    }

    /// Returns the most recent connection removed from a poll group.
    pub fn last_connection_poll_group_cleared(
        &self,
    ) -> Option<SteamworksNetworkingSocketsConnectionId> {
        self.last_connection_poll_group_cleared
    }

    /// Returns the most recent lane configuration submitted through this plugin.
    pub fn last_connection_lanes_configured(
        &self,
    ) -> Option<&SteamworksNetworkingSocketsLaneConfiguration> {
        self.last_connection_lanes_configured.as_ref()
    }

    /// Returns the most recent connection user data read or set through this plugin.
    pub fn last_connection_user_data(
        &self,
    ) -> Option<&SteamworksNetworkingSocketsConnectionUserData> {
        self.last_connection_user_data.as_ref()
    }

    /// Returns the most recent connection debug name submitted through this plugin.
    pub fn last_connection_name(&self) -> Option<&SteamworksNetworkingSocketsConnectionName> {
        self.last_connection_name.as_ref()
    }

    /// Returns the most recent connection closed through this plugin.
    pub fn last_closed_connection(&self) -> Option<&SteamworksNetworkingSocketsConnectionClosed> {
        self.last_closed_connection.as_ref()
    }

    /// Returns the most recent listen socket closed through this plugin.
    pub fn last_closed_listen_socket(
        &self,
    ) -> Option<&SteamworksNetworkingSocketsListenSocketClosed> {
        self.last_closed_listen_socket.as_ref()
    }

    /// Returns the most recent poll group closed through this plugin.
    pub fn last_closed_poll_group(&self) -> Option<SteamworksNetworkingSocketsPollGroupId> {
        self.last_closed_poll_group
    }

    /// Returns the number of listen sockets currently owned by this plugin.
    pub fn listen_socket_count(&self) -> usize {
        self.listen_socket_count
    }

    /// Returns the number of connections currently owned by this plugin.
    pub fn connection_count(&self) -> usize {
        self.connection_count
    }

    /// Returns the number of poll groups currently owned by this plugin.
    pub fn poll_group_count(&self) -> usize {
        self.poll_group_count
    }

    /// Returns the number of successful messages submitted through the plugin.
    pub fn sent_count(&self) -> u64 {
        self.sent_count
    }

    /// Returns the number of messages received through the plugin.
    pub fn received_count(&self) -> u64 {
        self.received_count
    }
}
