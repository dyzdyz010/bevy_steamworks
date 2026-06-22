use super::{
    push_poll_group_messages, push_received_messages, push_sent_message_results,
    remove_connection_state, upsert_connection_events, upsert_connection_info,
    upsert_listen_socket_events, upsert_realtime_status,
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
                let batch = SteamworksNetworkingSocketsListenSocketEvents {
                    listen_socket: *listen_socket,
                    events: events.clone(),
                };
                upsert_listen_socket_events(&mut self.listen_socket_events, batch.clone());
                self.last_listen_socket_events = Some(batch);
            }
            SteamworksNetworkingSocketsOperation::AllListenSocketEventsPolled {
                listen_sockets,
            } => {
                for batch in listen_sockets {
                    upsert_listen_socket_events(&mut self.listen_socket_events, batch.clone());
                }
                self.last_listen_socket_events = listen_sockets.last().cloned();
            }
            SteamworksNetworkingSocketsOperation::ConnectionEventsPolled {
                connection,
                events,
                connection_removed,
            } => {
                let batch = SteamworksNetworkingSocketsConnectionEvents {
                    connection: *connection,
                    events: events.clone(),
                    connection_removed: *connection_removed,
                };
                upsert_connection_events(&mut self.connection_events, batch.clone());
                if *connection_removed {
                    remove_connection_state(self, *connection);
                }
                self.last_connection_events = Some(batch);
            }
            SteamworksNetworkingSocketsOperation::AllConnectionEventsPolled { connections } => {
                for batch in connections {
                    upsert_connection_events(&mut self.connection_events, batch.clone());
                    if batch.connection_removed {
                        remove_connection_state(self, batch.connection);
                    }
                }
                self.last_connection_events = connections.last().cloned();
            }
            SteamworksNetworkingSocketsOperation::ConnectionInfoRead { info } => {
                upsert_connection_info(&mut self.connection_infos, info.clone());
                self.last_connection_info = Some(info.clone());
            }
            SteamworksNetworkingSocketsOperation::RealtimeConnectionStatusRead { status } => {
                upsert_realtime_status(&mut self.realtime_statuses, status.clone());
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
                push_sent_message_results(&mut self.recent_sent_messages, messages);
                self.last_sent_messages.clone_from(messages);
            }
            SteamworksNetworkingSocketsOperation::MessagesReceived { messages, .. } => {
                self.received_count = self
                    .received_count
                    .saturating_add(messages.len().try_into().unwrap_or(u64::MAX));
                push_received_messages(&mut self.recent_received_messages, messages);
                self.last_received_messages.clone_from(messages);
            }
            SteamworksNetworkingSocketsOperation::AllMessagesReceived { connections } => {
                let messages = connections
                    .iter()
                    .flat_map(|batch| batch.messages.iter().cloned())
                    .collect::<Vec<_>>();
                self.received_count = self
                    .received_count
                    .saturating_add(messages.len().try_into().unwrap_or(u64::MAX));
                push_received_messages(&mut self.recent_received_messages, &messages);
                self.last_received_messages = messages;
            }
            SteamworksNetworkingSocketsOperation::PollGroupMessagesReceived {
                messages, ..
            } => {
                self.received_count = self
                    .received_count
                    .saturating_add(messages.len().try_into().unwrap_or(u64::MAX));
                push_poll_group_messages(&mut self.recent_poll_group_messages, messages);
                self.last_poll_group_messages.clone_from(messages);
            }
            SteamworksNetworkingSocketsOperation::AllPollGroupMessagesReceived { poll_groups } => {
                let messages = poll_groups
                    .iter()
                    .flat_map(|batch| batch.messages.iter().cloned())
                    .collect::<Vec<_>>();
                self.received_count = self
                    .received_count
                    .saturating_add(messages.len().try_into().unwrap_or(u64::MAX));
                push_poll_group_messages(&mut self.recent_poll_group_messages, &messages);
                self.last_poll_group_messages = messages;
            }
            SteamworksNetworkingSocketsOperation::MessagesFlushed { connection } => {
                self.last_flushed_connection = Some(*connection);
            }
            SteamworksNetworkingSocketsOperation::AllMessagesFlushed { connections } => {
                self.last_flushed_connection = connections.last().copied();
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
                remove_connection_state(self, *connection);
                self.last_closed_connection = Some(SteamworksNetworkingSocketsConnectionClosed {
                    connection: *connection,
                    close_succeeded: *close_succeeded,
                });
            }
            SteamworksNetworkingSocketsOperation::AllConnectionsClosed { connections } => {
                for connection in connections {
                    remove_connection_state(self, connection.connection);
                }
                self.last_closed_connection = connections.last().cloned();
            }
            SteamworksNetworkingSocketsOperation::ListenSocketClosed {
                listen_socket,
                closed_connections,
            } => {
                self.listen_socket_events
                    .retain(|events| events.listen_socket != *listen_socket);
                self.last_closed_listen_socket =
                    Some(SteamworksNetworkingSocketsListenSocketClosed {
                        listen_socket: *listen_socket,
                        closed_connections: closed_connections.clone(),
                    });
            }
            SteamworksNetworkingSocketsOperation::AllListenSocketsClosed { listen_sockets } => {
                for listen_socket in listen_sockets {
                    self.listen_socket_events
                        .retain(|events| events.listen_socket != listen_socket.listen_socket);
                }
                self.last_closed_listen_socket = listen_sockets.last().cloned();
            }
            SteamworksNetworkingSocketsOperation::PollGroupClosed { poll_group } => {
                self.recent_poll_group_messages
                    .retain(|message| message.poll_group != *poll_group);
                self.last_closed_poll_group = Some(*poll_group);
            }
            SteamworksNetworkingSocketsOperation::AllPollGroupsClosed { poll_groups } => {
                for poll_group in poll_groups {
                    self.recent_poll_group_messages
                        .retain(|message| message.poll_group != *poll_group);
                }
                self.last_closed_poll_group = poll_groups.last().copied();
            }
        }
    }
}
