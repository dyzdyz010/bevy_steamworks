use super::{
    handles::{
        is_terminal_connection_state, SteamworksNetworkingSocketsConnectionMetadata,
        SteamworksNetworkingSocketsHandleStorage,
    },
    SteamworksConnectionRequestPolicy, SteamworksListenSocketEventInfo, SteamworksListenSocketId,
    SteamworksNetworkingSocketsConnectionEventInfo, SteamworksNetworkingSocketsConnectionEvents,
    SteamworksNetworkingSocketsConnectionId, SteamworksNetworkingSocketsError,
    SteamworksNetworkingSocketsListenSocketEvents, SteamworksNetworkingSocketsOperation,
};

pub(super) fn poll_listen_socket_events(
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    listen_socket: SteamworksListenSocketId,
    max_events: usize,
    request_policy: &SteamworksConnectionRequestPolicy,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let mut events = Vec::new();
    for _ in 0..max_events {
        let event = {
            let socket = handles.listen_sockets.get(&listen_socket).ok_or(
                SteamworksNetworkingSocketsError::ListenSocketNotFound { id: listen_socket },
            )?;
            socket.try_receive_event()
        };

        let Some(event) = event else {
            break;
        };

        match event {
            steamworks::networking_types::ListenSocketEvent::Connecting(request) => {
                let remote = request.remote();
                let user_data = request.user_data();
                match request_policy {
                    SteamworksConnectionRequestPolicy::Accept => {
                        request.accept().map_err(|source| {
                            SteamworksNetworkingSocketsError::steam_error(
                                "listen_socket_event.connection_request.accept",
                                source,
                            )
                        })?;
                        events.push(SteamworksListenSocketEventInfo::ConnectionAccepted {
                            listen_socket,
                            remote,
                            user_data,
                        });
                    }
                    SteamworksConnectionRequestPolicy::Reject { reason, debug } => {
                        if !request.reject(*reason, debug.as_deref()) {
                            return Err(SteamworksNetworkingSocketsError::operation_failed(
                                "listen_socket_event.connection_request.reject",
                            ));
                        }
                        events.push(SteamworksListenSocketEventInfo::ConnectionRejected {
                            listen_socket,
                            remote,
                            user_data,
                        });
                    }
                }
            }
            steamworks::networking_types::ListenSocketEvent::Connected(event) => {
                let remote = event.remote();
                let user_data = event.user_data();
                let owner = handles.listen_socket_owner(listen_socket).ok_or(
                    SteamworksNetworkingSocketsError::ListenSocketNotFound { id: listen_socket },
                )?;
                let connection = handles.insert_connection(
                    event.take_connection(),
                    SteamworksNetworkingSocketsConnectionMetadata::listen_socket(
                        listen_socket,
                        remote.clone(),
                        user_data,
                    ),
                    owner,
                );
                events.push(SteamworksListenSocketEventInfo::Connected {
                    listen_socket,
                    connection,
                    remote,
                    user_data,
                });
            }
            steamworks::networking_types::ListenSocketEvent::Disconnected(event) => {
                let remote = event.remote();
                let user_data = event.user_data();
                let end_reason = event.end_reason();
                let connection = handles.remove_listen_connection_by_event(
                    listen_socket,
                    &remote,
                    user_data,
                    end_reason,
                );
                events.push(SteamworksListenSocketEventInfo::Disconnected {
                    listen_socket,
                    connection,
                    remote,
                    user_data,
                    end_reason,
                });
            }
        }
    }

    Ok(
        SteamworksNetworkingSocketsOperation::ListenSocketEventsPolled {
            listen_socket,
            events,
        },
    )
}

pub(super) fn poll_all_listen_socket_events(
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    max_events_per_socket: usize,
    request_policy: &SteamworksConnectionRequestPolicy,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let mut listen_sockets = handles.listen_sockets.keys().copied().collect::<Vec<_>>();
    listen_sockets.sort_by_key(|listen_socket| listen_socket.raw());

    let mut batches = Vec::with_capacity(listen_sockets.len());
    for listen_socket in listen_sockets {
        if !handles.listen_sockets.contains_key(&listen_socket) {
            continue;
        }
        let SteamworksNetworkingSocketsOperation::ListenSocketEventsPolled {
            listen_socket,
            events,
        } = poll_listen_socket_events(
            handles,
            listen_socket,
            max_events_per_socket,
            request_policy,
        )?
        else {
            unreachable!("poll_listen_socket_events returns ListenSocketEventsPolled");
        };
        batches.push(SteamworksNetworkingSocketsListenSocketEvents {
            listen_socket,
            events,
        });
    }

    Ok(
        SteamworksNetworkingSocketsOperation::AllListenSocketEventsPolled {
            listen_sockets: batches,
        },
    )
}

pub(super) fn poll_connection_events(
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    connection: SteamworksNetworkingSocketsConnectionId,
    max_events: usize,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let mut events = Vec::new();
    let mut should_remove = false;
    for _ in 0..max_events {
        let event = {
            let connection_ref = handles
                .connections
                .get(&connection)
                .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: connection })?;
            connection_ref.try_receive_event()
        };

        let Some(event) = event else {
            break;
        };

        let terminal_event = is_terminal_connection_state(event.new_state);
        events.push(SteamworksNetworkingSocketsConnectionEventInfo {
            connection,
            new_state: event.new_state,
            old_state: event.old_state,
        });

        if terminal_event {
            should_remove = true;
            break;
        }
    }

    if should_remove {
        handles.remove_connection(&connection);
    }

    Ok(
        SteamworksNetworkingSocketsOperation::ConnectionEventsPolled {
            connection,
            events,
            connection_removed: should_remove,
        },
    )
}

pub(super) fn poll_all_connection_events(
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    max_events_per_connection: usize,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let mut connections = handles.connections.keys().copied().collect::<Vec<_>>();
    connections.sort_by_key(|connection| connection.raw());

    let mut batches = Vec::with_capacity(connections.len());
    for connection in connections {
        if !handles.connections.contains_key(&connection) {
            continue;
        }
        let SteamworksNetworkingSocketsOperation::ConnectionEventsPolled {
            connection,
            events,
            connection_removed,
        } = poll_connection_events(handles, connection, max_events_per_connection)?
        else {
            unreachable!("poll_connection_events returns ConnectionEventsPolled");
        };
        batches.push(SteamworksNetworkingSocketsConnectionEvents {
            connection,
            events,
            connection_removed,
        });
    }

    Ok(
        SteamworksNetworkingSocketsOperation::AllConnectionEventsPolled {
            connections: batches,
        },
    )
}
