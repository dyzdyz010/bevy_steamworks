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

    /// Returns the number of cached listen-socket event batches.
    pub fn listen_socket_event_batch_count(&self) -> usize {
        self.listen_socket_events.len()
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

    /// Returns the number of events cached for one listen socket.
    pub fn listen_socket_event_count(
        &self,
        listen_socket: SteamworksListenSocketId,
    ) -> Option<usize> {
        self.listen_socket_event_batch(listen_socket)
            .map(|events| events.events.len())
    }

    /// Returns the most recent connection event batch processed through this plugin.
    pub fn last_connection_events(&self) -> Option<&SteamworksNetworkingSocketsConnectionEvents> {
        self.last_connection_events.as_ref()
    }

    /// Returns bounded connection event batches keyed by connection.
    pub fn connection_events(&self) -> &[SteamworksNetworkingSocketsConnectionEvents] {
        &self.connection_events
    }

    /// Returns the number of cached connection event batches.
    pub fn connection_event_batch_count(&self) -> usize {
        self.connection_events.len()
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

    /// Returns the number of events cached for one connection.
    pub fn connection_event_count(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<usize> {
        self.connection_event_batch(connection)
            .map(|events| events.events.len())
    }

    /// Returns whether the latest cached event batch removed one connection.
    pub fn connection_event_removed(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<bool> {
        self.connection_event_batch(connection)
            .map(|events| events.connection_removed)
    }

    /// Returns the most recent connection info snapshot read through the plugin.
    pub fn last_connection_info(&self) -> Option<&SteamworksNetworkingSocketsConnectionInfo> {
        self.last_connection_info.as_ref()
    }

    /// Returns bounded connection info snapshots keyed by connection.
    pub fn connection_infos(&self) -> &[SteamworksNetworkingSocketsConnectionInfo] {
        &self.connection_infos
    }

    /// Returns the number of cached connection info snapshots.
    pub fn connection_info_count(&self) -> usize {
        self.connection_infos.len()
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

    /// Returns the cached end reason for one connection, preserving a known connection without one as `Some(None)`.
    pub fn connection_end_reason(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<Option<steamworks::networking_types::NetConnectionEnd>> {
        self.connection_info(connection).map(|info| info.end_reason)
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

    /// Returns the number of cached realtime status snapshots.
    pub fn realtime_status_count(&self) -> usize {
        self.realtime_statuses.len()
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

    /// Returns the latest cached send capacity for one connection.
    pub fn connection_send_rate_bytes_per_sec(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<i32> {
        self.realtime_status(connection)
            .map(|status| status.send_rate_bytes_per_sec)
    }

    /// Returns the latest cached pending unreliable bytes for one connection.
    pub fn connection_pending_unreliable(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<i32> {
        self.realtime_status(connection)
            .map(|status| status.pending_unreliable)
    }

    /// Returns the latest cached pending reliable bytes for one connection.
    pub fn connection_pending_reliable(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<i32> {
        self.realtime_status(connection)
            .map(|status| status.pending_reliable)
    }

    /// Returns the number of cached realtime lane snapshots for one connection.
    pub fn connection_lane_count(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<usize> {
        self.realtime_status(connection)
            .map(|status| status.lanes.len())
    }

    /// Returns the most recent sent-message snapshot.
    pub fn last_sent_message(&self) -> Option<&SteamworksNetworkingSocketsSentMessage> {
        self.last_sent_message.as_ref()
    }

    /// Returns the connection for the most recent single-message send.
    pub fn last_sent_message_connection(&self) -> Option<SteamworksNetworkingSocketsConnectionId> {
        self.last_sent_message().map(|message| message.connection)
    }

    /// Returns the message number for the most recent single-message send.
    pub fn last_sent_message_number(&self) -> Option<u64> {
        self.last_sent_message()
            .map(|message| message.message_number)
    }

    /// Returns the byte count for the most recent single-message send.
    pub fn last_sent_message_bytes(&self) -> Option<usize> {
        self.last_sent_message().map(|message| message.bytes)
    }

    /// Returns the most recent batch send outcomes.
    pub fn last_sent_messages(&self) -> &[SteamworksNetworkingSocketsMessageSendResult] {
        &self.last_sent_messages
    }

    /// Returns the number of outcomes in the most recent batch send.
    pub fn last_sent_message_count(&self) -> usize {
        self.last_sent_messages.len()
    }

    /// Returns bounded batch-send outcomes in observation order.
    pub fn recent_sent_messages(&self) -> &[SteamworksNetworkingSocketsMessageSendResult] {
        &self.recent_sent_messages
    }

    /// Returns the number of bounded batch-send outcomes retained across batches.
    pub fn recent_sent_message_count(&self) -> usize {
        self.recent_sent_messages.len()
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

    /// Returns the number of bounded batch-send outcomes for one connection.
    pub fn sent_message_count_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> usize {
        self.sent_messages_for_connection(connection).count()
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

    /// Returns the Steam message number from the most recent successful batch-send outcome for one connection.
    pub fn last_sent_message_number_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<u64> {
        self.last_sent_message_for_connection(connection)
            .and_then(|message| message.result.ok())
    }

    /// Returns whether the most recent batch-send outcome for one connection succeeded.
    pub fn last_sent_message_succeeded_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<bool> {
        self.last_sent_message_for_connection(connection)
            .map(|message| message.result.is_ok())
    }

    /// Returns the byte count from the most recent batch-send outcome for one connection.
    pub fn last_sent_message_bytes_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<usize> {
        self.last_sent_message_for_connection(connection)
            .map(|message| message.bytes)
    }

    /// Returns the most recent batch of received messages.
    pub fn last_received_messages(&self) -> &[SteamworksNetworkingSocketsMessage] {
        &self.last_received_messages
    }

    /// Returns the number of messages in the most recent connection receive batch.
    pub fn last_received_message_count(&self) -> usize {
        self.last_received_messages.len()
    }

    /// Returns bounded received message snapshots in observation order.
    pub fn recent_received_messages(&self) -> &[SteamworksNetworkingSocketsMessage] {
        &self.recent_received_messages
    }

    /// Returns the number of bounded received message snapshots retained across batches.
    pub fn recent_received_message_count(&self) -> usize {
        self.recent_received_messages.len()
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

    /// Returns the number of bounded received message snapshots for one connection.
    pub fn received_message_count_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> usize {
        self.received_messages_for_connection(connection).count()
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

    /// Returns the byte count for the most recent received message for one connection.
    pub fn last_received_message_bytes_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<usize> {
        self.last_received_message_for_connection(connection)
            .map(|message| message.data.len())
    }

    /// Returns the channel for the most recent received message for one connection.
    pub fn last_received_message_channel_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<i32> {
        self.last_received_message_for_connection(connection)
            .map(|message| message.channel)
    }

    /// Returns the payload bytes for the most recent received message for one connection.
    pub fn last_received_message_data_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<&[u8]> {
        self.last_received_message_for_connection(connection)
            .map(|message| message.data.as_slice())
    }

    /// Returns the most recent batch of messages received from a poll group.
    pub fn last_poll_group_messages(&self) -> &[SteamworksNetworkingSocketsPollGroupMessage] {
        &self.last_poll_group_messages
    }

    /// Returns the number of messages in the most recent poll-group receive batch.
    pub fn last_poll_group_message_count(&self) -> usize {
        self.last_poll_group_messages.len()
    }

    /// Returns bounded poll-group message snapshots in observation order.
    pub fn recent_poll_group_messages(&self) -> &[SteamworksNetworkingSocketsPollGroupMessage] {
        &self.recent_poll_group_messages
    }

    /// Returns the number of bounded poll-group message snapshots retained across batches.
    pub fn recent_poll_group_message_count(&self) -> usize {
        self.recent_poll_group_messages.len()
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

    /// Returns the number of bounded poll-group message snapshots for one poll group.
    pub fn poll_group_message_count(
        &self,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) -> usize {
        self.poll_group_messages(poll_group).count()
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

    /// Returns the byte count for the most recent message received from one poll group.
    pub fn last_poll_group_message_bytes(
        &self,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) -> Option<usize> {
        self.last_poll_group_message(poll_group)
            .map(|message| message.data.len())
    }

    /// Returns the channel for the most recent message received from one poll group.
    pub fn last_poll_group_message_channel(
        &self,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) -> Option<i32> {
        self.last_poll_group_message(poll_group)
            .map(|message| message.channel)
    }

    /// Returns the payload bytes for the most recent message received from one poll group.
    pub fn last_poll_group_message_data(
        &self,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) -> Option<&[u8]> {
        self.last_poll_group_message(poll_group)
            .map(|message| message.data.as_slice())
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
