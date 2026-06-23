use super::{
    SteamworksListenSocketId, SteamworksNetworkingSocketsConnectionClosed,
    SteamworksNetworkingSocketsConnectionCreated, SteamworksNetworkingSocketsConnectionEvents,
    SteamworksNetworkingSocketsConnectionId, SteamworksNetworkingSocketsConnectionInfo,
    SteamworksNetworkingSocketsConnectionName, SteamworksNetworkingSocketsConnectionUserData,
    SteamworksNetworkingSocketsError, SteamworksNetworkingSocketsLaneConfiguration,
    SteamworksNetworkingSocketsListenSocketClosed, SteamworksNetworkingSocketsListenSocketCreated,
    SteamworksNetworkingSocketsListenSocketEvents, SteamworksNetworkingSocketsMessage,
    SteamworksNetworkingSocketsMessageSendResult, SteamworksNetworkingSocketsPollGroupAssignment,
    SteamworksNetworkingSocketsPollGroupId, SteamworksNetworkingSocketsPollGroupMessage,
    SteamworksNetworkingSocketsRealtimeStatus, SteamworksNetworkingSocketsSentMessage,
    SteamworksNetworkingSocketsState,
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

    /// Returns bounded listen-socket event batches keyed by listen socket.
    pub fn listen_socket_events(&self) -> &[SteamworksNetworkingSocketsListenSocketEvents] {
        &self.listen_socket_events
    }

    /// Returns the cached event batch for one listen socket.
    pub fn listen_socket_event_batch(
        &self,
        listen_socket: SteamworksListenSocketId,
    ) -> Option<&SteamworksNetworkingSocketsListenSocketEvents> {
        self.listen_socket_events
            .iter()
            .find(|events| events.listen_socket == listen_socket)
    }

    /// Returns the most recent connection event batch processed through this plugin.
    pub fn last_connection_events(&self) -> Option<&SteamworksNetworkingSocketsConnectionEvents> {
        self.last_connection_events.as_ref()
    }

    /// Returns bounded connection event batches keyed by connection.
    pub fn connection_events(&self) -> &[SteamworksNetworkingSocketsConnectionEvents] {
        &self.connection_events
    }

    /// Returns the cached event batch for one connection.
    pub fn connection_event_batch(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<&SteamworksNetworkingSocketsConnectionEvents> {
        self.connection_events
            .iter()
            .find(|events| events.connection == connection)
    }

    /// Returns the most recent connection info snapshot read through the plugin.
    pub fn last_connection_info(&self) -> Option<&SteamworksNetworkingSocketsConnectionInfo> {
        self.last_connection_info.as_ref()
    }

    /// Returns bounded connection info snapshots keyed by connection.
    pub fn connection_infos(&self) -> &[SteamworksNetworkingSocketsConnectionInfo] {
        &self.connection_infos
    }

    /// Returns the cached connection info for one connection.
    pub fn connection_info(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<&SteamworksNetworkingSocketsConnectionInfo> {
        self.connection_infos
            .iter()
            .find(|info| info.connection == connection)
    }

    /// Returns the latest known high-level state for one connection.
    pub fn connection_state(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<steamworks::networking_types::NetworkingConnectionState> {
        self.connection_info(connection)
            .map(|info| info.state)
            .or_else(|| {
                self.realtime_status(connection)
                    .map(|status| status.connection_state)
            })
    }

    /// Returns the latest known remote identity for one connection, preserving a known connection without identity as `Some(None)`.
    pub fn connection_remote(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<Option<&steamworks::networking_types::NetworkingIdentity>> {
        self.connection_info(connection)
            .map(|info| info.remote.as_ref())
    }

    /// Returns the latest known connection user data.
    pub fn connection_user_data(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<i64> {
        if let Some(user_data) = self
            .last_connection_user_data
            .as_ref()
            .filter(|user_data| user_data.connection == connection)
            .map(|user_data| user_data.user_data)
        {
            return Some(user_data);
        }

        self.connection_info(connection).map(|info| info.user_data)
    }

    /// Returns the most recent debug name submitted for one connection.
    pub fn connection_name(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<&str> {
        self.last_connection_name
            .as_ref()
            .filter(|name| name.connection == connection)
            .map(|name| name.name.as_str())
    }

    /// Returns the most recent realtime connection status snapshot.
    pub fn last_realtime_status(&self) -> Option<&SteamworksNetworkingSocketsRealtimeStatus> {
        self.last_realtime_status.as_ref()
    }

    /// Returns bounded realtime connection status snapshots keyed by connection.
    pub fn realtime_statuses(&self) -> &[SteamworksNetworkingSocketsRealtimeStatus] {
        &self.realtime_statuses
    }

    /// Returns the cached realtime status for one connection.
    pub fn realtime_status(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<&SteamworksNetworkingSocketsRealtimeStatus> {
        self.realtime_statuses
            .iter()
            .find(|status| status.connection == connection)
    }

    /// Returns the latest cached ping estimate for one connection.
    pub fn connection_ping(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<i32> {
        self.realtime_status(connection).map(|status| status.ping)
    }

    /// Returns the latest cached local and remote connection quality for one connection.
    pub fn connection_quality(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<(f32, f32)> {
        self.realtime_status(connection).map(|status| {
            (
                status.connection_quality_local,
                status.connection_quality_remote,
            )
        })
    }

    /// Returns the most recent sent-message snapshot.
    pub fn last_sent_message(&self) -> Option<&SteamworksNetworkingSocketsSentMessage> {
        self.last_sent_message.as_ref()
    }

    /// Returns the most recent batch send outcomes.
    pub fn last_sent_messages(&self) -> &[SteamworksNetworkingSocketsMessageSendResult] {
        &self.last_sent_messages
    }

    /// Returns bounded batch-send outcomes in observation order.
    pub fn recent_sent_messages(&self) -> &[SteamworksNetworkingSocketsMessageSendResult] {
        &self.recent_sent_messages
    }

    /// Returns bounded batch-send outcomes for one connection.
    pub fn sent_messages_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> impl Iterator<Item = &SteamworksNetworkingSocketsMessageSendResult> + '_ {
        self.recent_sent_messages
            .iter()
            .filter(move |message| message.connection == connection)
    }

    /// Returns the most recent batch-send outcome for one connection.
    pub fn last_sent_message_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<&SteamworksNetworkingSocketsMessageSendResult> {
        self.recent_sent_messages
            .iter()
            .rev()
            .find(|message| message.connection == connection)
    }

    /// Returns the most recent batch of received messages.
    pub fn last_received_messages(&self) -> &[SteamworksNetworkingSocketsMessage] {
        &self.last_received_messages
    }

    /// Returns bounded received message snapshots in observation order.
    pub fn recent_received_messages(&self) -> &[SteamworksNetworkingSocketsMessage] {
        &self.recent_received_messages
    }

    /// Returns bounded received message snapshots for one connection.
    pub fn received_messages_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> impl Iterator<Item = &SteamworksNetworkingSocketsMessage> + '_ {
        self.recent_received_messages
            .iter()
            .filter(move |message| message.connection == connection)
    }

    /// Returns the most recent received message for one connection.
    pub fn last_received_message_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<&SteamworksNetworkingSocketsMessage> {
        self.recent_received_messages
            .iter()
            .rev()
            .find(|message| message.connection == connection)
    }

    /// Returns the most recent batch of messages received from a poll group.
    pub fn last_poll_group_messages(&self) -> &[SteamworksNetworkingSocketsPollGroupMessage] {
        &self.last_poll_group_messages
    }

    /// Returns bounded poll-group message snapshots in observation order.
    pub fn recent_poll_group_messages(&self) -> &[SteamworksNetworkingSocketsPollGroupMessage] {
        &self.recent_poll_group_messages
    }

    /// Returns bounded poll-group message snapshots for one poll group.
    pub fn poll_group_messages(
        &self,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) -> impl Iterator<Item = &SteamworksNetworkingSocketsPollGroupMessage> + '_ {
        self.recent_poll_group_messages
            .iter()
            .filter(move |message| message.poll_group == poll_group)
    }

    /// Returns the most recent message received from one poll group.
    pub fn last_poll_group_message(
        &self,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) -> Option<&SteamworksNetworkingSocketsPollGroupMessage> {
        self.recent_poll_group_messages
            .iter()
            .rev()
            .find(|message| message.poll_group == poll_group)
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
