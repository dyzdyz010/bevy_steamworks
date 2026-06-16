use bevy_ecs::prelude::Resource;

use super::handles::SteamworksNetworkingSocketsHandleStorage;
use super::*;

/// Runtime state for [`crate::SteamworksNetworkingSocketsPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksNetworkingSocketsState {
    last_error: Option<SteamworksNetworkingSocketsError>,
    last_authentication_status: Option<steamworks::networking_types::NetworkingAvailabilityResult>,
    last_created_listen_socket: Option<SteamworksNetworkingSocketsListenSocketCreated>,
    last_created_connection: Option<SteamworksNetworkingSocketsConnectionCreated>,
    last_created_poll_group: Option<SteamworksNetworkingSocketsPollGroupId>,
    last_listen_socket_events: Option<SteamworksNetworkingSocketsListenSocketEvents>,
    last_connection_events: Option<SteamworksNetworkingSocketsConnectionEvents>,
    last_connection_info: Option<SteamworksNetworkingSocketsConnectionInfo>,
    last_realtime_status: Option<SteamworksNetworkingSocketsRealtimeStatus>,
    last_sent_message: Option<SteamworksNetworkingSocketsSentMessage>,
    last_sent_messages: Vec<SteamworksNetworkingSocketsMessageSendResult>,
    last_received_messages: Vec<SteamworksNetworkingSocketsMessage>,
    last_poll_group_messages: Vec<SteamworksNetworkingSocketsPollGroupMessage>,
    last_flushed_connection: Option<SteamworksNetworkingSocketsConnectionId>,
    last_connection_poll_group_set: Option<SteamworksNetworkingSocketsPollGroupAssignment>,
    last_connection_poll_group_cleared: Option<SteamworksNetworkingSocketsConnectionId>,
    last_connection_lanes_configured: Option<SteamworksNetworkingSocketsLaneConfiguration>,
    last_connection_user_data: Option<SteamworksNetworkingSocketsConnectionUserData>,
    last_connection_name: Option<SteamworksNetworkingSocketsConnectionName>,
    last_closed_connection: Option<SteamworksNetworkingSocketsConnectionClosed>,
    last_closed_listen_socket: Option<SteamworksNetworkingSocketsListenSocketClosed>,
    last_closed_poll_group: Option<SteamworksNetworkingSocketsPollGroupId>,
    listen_socket_count: usize,
    connection_count: usize,
    poll_group_count: usize,
    sent_count: u64,
    received_count: u64,
}

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

    pub(super) fn sync_handle_counts(
        &mut self,
        handles: &SteamworksNetworkingSocketsHandleStorage,
    ) {
        self.listen_socket_count = handles.listen_sockets.len();
        self.connection_count = handles.connections.len();
        self.poll_group_count = handles.poll_groups.len();
    }

    pub(super) fn record_error(&mut self, error: SteamworksNetworkingSocketsError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksNetworkingSocketsOperation) {
        match operation {
            SteamworksNetworkingSocketsOperation::AuthenticationInitialized { availability }
            | SteamworksNetworkingSocketsOperation::AuthenticationStatusRead { availability } => {
                self.last_authentication_status = Some(*availability);
            }
            SteamworksNetworkingSocketsOperation::ListenSocketCreated {
                listen_socket,
                endpoint,
            } => {
                self.last_created_listen_socket =
                    Some(SteamworksNetworkingSocketsListenSocketCreated {
                        listen_socket: *listen_socket,
                        endpoint: endpoint.clone(),
                    });
            }
            SteamworksNetworkingSocketsOperation::ConnectionCreated { connection, target } => {
                self.last_created_connection = Some(SteamworksNetworkingSocketsConnectionCreated {
                    connection: *connection,
                    target: target.clone(),
                });
            }
            SteamworksNetworkingSocketsOperation::PollGroupCreated { poll_group } => {
                self.last_created_poll_group = Some(*poll_group);
            }
            SteamworksNetworkingSocketsOperation::ListenSocketEventsPolled {
                listen_socket,
                events,
            } => {
                self.last_listen_socket_events =
                    Some(SteamworksNetworkingSocketsListenSocketEvents {
                        listen_socket: *listen_socket,
                        events: events.clone(),
                    });
            }
            SteamworksNetworkingSocketsOperation::ConnectionEventsPolled {
                connection,
                events,
                connection_removed,
            } => {
                self.last_connection_events = Some(SteamworksNetworkingSocketsConnectionEvents {
                    connection: *connection,
                    events: events.clone(),
                    connection_removed: *connection_removed,
                });
            }
            SteamworksNetworkingSocketsOperation::ConnectionInfoRead { info } => {
                self.last_connection_info = Some(info.clone());
            }
            SteamworksNetworkingSocketsOperation::RealtimeConnectionStatusRead { status } => {
                self.last_realtime_status = Some(status.clone());
            }
            SteamworksNetworkingSocketsOperation::MessageSent {
                connection,
                message_number,
                bytes,
            } => {
                self.sent_count = self.sent_count.saturating_add(1);
                self.last_sent_message = Some(SteamworksNetworkingSocketsSentMessage {
                    connection: *connection,
                    message_number: *message_number,
                    bytes: *bytes,
                });
            }
            SteamworksNetworkingSocketsOperation::MessagesSent { messages } => {
                let successful = messages
                    .iter()
                    .filter(|message| message.result.is_ok())
                    .count();
                self.sent_count = self
                    .sent_count
                    .saturating_add(successful.try_into().unwrap_or(u64::MAX));
                self.last_sent_messages.clone_from(messages);
            }
            SteamworksNetworkingSocketsOperation::MessagesReceived { messages, .. } => {
                self.received_count = self
                    .received_count
                    .saturating_add(messages.len().try_into().unwrap_or(u64::MAX));
                self.last_received_messages.clone_from(messages);
            }
            SteamworksNetworkingSocketsOperation::PollGroupMessagesReceived {
                messages, ..
            } => {
                self.received_count = self
                    .received_count
                    .saturating_add(messages.len().try_into().unwrap_or(u64::MAX));
                self.last_poll_group_messages.clone_from(messages);
            }
            SteamworksNetworkingSocketsOperation::MessagesFlushed { connection } => {
                self.last_flushed_connection = Some(*connection);
            }
            SteamworksNetworkingSocketsOperation::ConnectionPollGroupSet {
                connection,
                poll_group,
            } => {
                self.last_connection_poll_group_set =
                    Some(SteamworksNetworkingSocketsPollGroupAssignment {
                        connection: *connection,
                        poll_group: *poll_group,
                    });
            }
            SteamworksNetworkingSocketsOperation::ConnectionPollGroupCleared { connection } => {
                self.last_connection_poll_group_cleared = Some(*connection);
            }
            SteamworksNetworkingSocketsOperation::ConnectionLanesConfigured {
                connection,
                lanes,
            } => {
                self.last_connection_lanes_configured =
                    Some(SteamworksNetworkingSocketsLaneConfiguration {
                        connection: *connection,
                        lanes: *lanes,
                    });
            }
            SteamworksNetworkingSocketsOperation::ConnectionUserDataRead {
                connection,
                user_data,
            }
            | SteamworksNetworkingSocketsOperation::ConnectionUserDataSet {
                connection,
                user_data,
            } => {
                self.last_connection_user_data =
                    Some(SteamworksNetworkingSocketsConnectionUserData {
                        connection: *connection,
                        user_data: *user_data,
                    });
            }
            SteamworksNetworkingSocketsOperation::ConnectionNameSet { connection, name } => {
                self.last_connection_name = Some(SteamworksNetworkingSocketsConnectionName {
                    connection: *connection,
                    name: name.clone(),
                });
            }
            SteamworksNetworkingSocketsOperation::ConnectionClosed {
                connection,
                close_succeeded,
            } => {
                self.last_closed_connection = Some(SteamworksNetworkingSocketsConnectionClosed {
                    connection: *connection,
                    close_succeeded: *close_succeeded,
                });
            }
            SteamworksNetworkingSocketsOperation::ListenSocketClosed {
                listen_socket,
                closed_connections,
            } => {
                self.last_closed_listen_socket =
                    Some(SteamworksNetworkingSocketsListenSocketClosed {
                        listen_socket: *listen_socket,
                        closed_connections: closed_connections.clone(),
                    });
            }
            SteamworksNetworkingSocketsOperation::PollGroupClosed { poll_group } => {
                self.last_closed_poll_group = Some(*poll_group);
            }
        }
    }
}
