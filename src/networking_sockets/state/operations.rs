use super::{
    SteamworksNetworkingSocketsConnectionClosed, SteamworksNetworkingSocketsConnectionCreated,
    SteamworksNetworkingSocketsConnectionEvents, SteamworksNetworkingSocketsConnectionName,
    SteamworksNetworkingSocketsConnectionUserData, SteamworksNetworkingSocketsError,
    SteamworksNetworkingSocketsLaneConfiguration, SteamworksNetworkingSocketsListenSocketClosed,
    SteamworksNetworkingSocketsListenSocketCreated, SteamworksNetworkingSocketsListenSocketEvents,
    SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsPollGroupAssignment,
    SteamworksNetworkingSocketsSentMessage, SteamworksNetworkingSocketsState,
};

use crate::networking_sockets::handles::SteamworksNetworkingSocketsHandleStorage;

impl SteamworksNetworkingSocketsState {
    pub(in crate::networking_sockets) fn sync_handle_counts(
        &mut self,
        handles: &SteamworksNetworkingSocketsHandleStorage,
    ) {
        self.listen_socket_count = handles.listen_sockets.len();
        self.connection_count = handles.connections.len();
        self.poll_group_count = handles.poll_groups.len();
    }

    pub(in crate::networking_sockets) fn record_error(
        &mut self,
        error: SteamworksNetworkingSocketsError,
    ) {
        self.last_error = Some(error);
    }

    pub(in crate::networking_sockets) fn record_operation(
        &mut self,
        operation: &SteamworksNetworkingSocketsOperation,
    ) {
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
            SteamworksNetworkingSocketsOperation::AllListenSocketEventsPolled {
                listen_sockets,
            } => {
                self.last_listen_socket_events = listen_sockets.last().cloned();
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
            SteamworksNetworkingSocketsOperation::AllConnectionEventsPolled { connections } => {
                self.last_connection_events = connections.last().cloned();
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
