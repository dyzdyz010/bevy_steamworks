use bevy_ecs::{
    message::{MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::{SteamworksClient, SteamworksServer};

use super::{
    handles::{
        SteamworksNetworkingSocketsConnectionMetadata, SteamworksNetworkingSocketsHandleStorage,
        SteamworksNetworkingSocketsHandles,
    },
    polling::{poll_connection_events, poll_listen_socket_events},
    snapshots::{
        snapshot_connection_info, snapshot_message, snapshot_poll_group_message,
        snapshot_realtime_status,
    },
    validation::validate_command,
    SteamworksNetworkingSocketsCommand, SteamworksNetworkingSocketsConnectionTarget,
    SteamworksNetworkingSocketsError, SteamworksNetworkingSocketsListenEndpoint,
    SteamworksNetworkingSocketsMessageSendResult, SteamworksNetworkingSocketsOperation,
    SteamworksNetworkingSocketsOutboundMessage, SteamworksNetworkingSocketsResult,
    SteamworksNetworkingSocketsState,
};

pub(super) fn process_networking_sockets_commands(
    client: Option<Res<SteamworksClient>>,
    server: Option<Res<SteamworksServer>>,
    mut state: ResMut<SteamworksNetworkingSocketsState>,
    handles: Res<SteamworksNetworkingSocketsHandles>,
    mut commands: ResMut<Messages<SteamworksNetworkingSocketsCommand>>,
    mut results: MessageWriter<SteamworksNetworkingSocketsResult>,
) {
    let mut handles = handles
        .storage
        .lock()
        .expect("Steamworks Networking Sockets handle storage mutex was poisoned");

    for command in commands.drain() {
        match handle_networking_sockets_command(
            client.as_deref(),
            server.as_deref(),
            &mut handles,
            &command,
        ) {
            Ok(operation) => {
                state.record_operation(&operation);
                state.sync_handle_counts(&handles);
                if matches!(
                    &operation,
                    SteamworksNetworkingSocketsOperation::ConnectionClosed {
                        close_succeeded: false,
                        ..
                    }
                ) {
                    tracing::warn!(
                        target: "bevy_steamworks",
                        operation = ?operation,
                        "Steamworks networking sockets connection was removed after close returned false"
                    );
                } else {
                    tracing::debug!(
                        target: "bevy_steamworks",
                        operation = ?operation,
                        "processed Steamworks networking sockets command"
                    );
                }
                results.write(SteamworksNetworkingSocketsResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                state.sync_handle_counts(&handles);
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks networking sockets command failed"
                );
                results.write(SteamworksNetworkingSocketsResult::Err { command, error });
            }
        }
    }
}

fn handle_networking_sockets_command(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    command: &SteamworksNetworkingSocketsCommand,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    validate_command(command)?;

    if matches!(
        command,
        SteamworksNetworkingSocketsCommand::SendMessages { .. }
    ) && client.is_none()
    {
        return Err(SteamworksNetworkingSocketsError::ClientUnavailable);
    }

    let sockets = networking_sockets(client, server)?;
    Ok(match command {
        SteamworksNetworkingSocketsCommand::InitAuthentication => {
            SteamworksNetworkingSocketsOperation::AuthenticationInitialized {
                availability: sockets.init_authentication(),
            }
        }
        SteamworksNetworkingSocketsCommand::GetAuthenticationStatus => {
            SteamworksNetworkingSocketsOperation::AuthenticationStatusRead {
                availability: sockets.get_authentication_status(),
            }
        }
        SteamworksNetworkingSocketsCommand::CreateListenSocketIp { local_address } => {
            let socket = sockets
                .create_listen_socket_ip(*local_address, [])
                .map_err(|_| {
                    SteamworksNetworkingSocketsError::invalid_handle(
                        "networking_sockets.create_listen_socket_ip",
                    )
                })?;
            let listen_socket = handles.insert_listen_socket(socket);
            SteamworksNetworkingSocketsOperation::ListenSocketCreated {
                listen_socket,
                endpoint: SteamworksNetworkingSocketsListenEndpoint::Ip(*local_address),
            }
        }
        SteamworksNetworkingSocketsCommand::CreateListenSocketP2p { local_virtual_port } => {
            let socket = sockets
                .create_listen_socket_p2p(*local_virtual_port, [])
                .map_err(|_| {
                    SteamworksNetworkingSocketsError::invalid_handle(
                        "networking_sockets.create_listen_socket_p2p",
                    )
                })?;
            let listen_socket = handles.insert_listen_socket(socket);
            SteamworksNetworkingSocketsOperation::ListenSocketCreated {
                listen_socket,
                endpoint: SteamworksNetworkingSocketsListenEndpoint::P2p {
                    local_virtual_port: *local_virtual_port,
                },
            }
        }
        SteamworksNetworkingSocketsCommand::ConnectByIpAddress { address } => {
            let connection = sockets.connect_by_ip_address(*address, []).map_err(|_| {
                SteamworksNetworkingSocketsError::invalid_handle(
                    "networking_sockets.connect_by_ip_address",
                )
            })?;
            let connection = handles.insert_connection(
                connection,
                SteamworksNetworkingSocketsConnectionMetadata::independent(),
            );
            SteamworksNetworkingSocketsOperation::ConnectionCreated {
                connection,
                target: SteamworksNetworkingSocketsConnectionTarget::Ip(*address),
            }
        }
        SteamworksNetworkingSocketsCommand::ConnectP2p {
            identity,
            remote_virtual_port,
        } => {
            let connection = sockets
                .connect_p2p(identity.clone(), *remote_virtual_port, [])
                .map_err(|_| {
                    SteamworksNetworkingSocketsError::invalid_handle(
                        "networking_sockets.connect_p2p",
                    )
                })?;
            let connection = handles.insert_connection(
                connection,
                SteamworksNetworkingSocketsConnectionMetadata::independent(),
            );
            SteamworksNetworkingSocketsOperation::ConnectionCreated {
                connection,
                target: SteamworksNetworkingSocketsConnectionTarget::P2p {
                    identity: identity.clone(),
                    remote_virtual_port: *remote_virtual_port,
                },
            }
        }
        SteamworksNetworkingSocketsCommand::CreatePollGroup => {
            let poll_group = handles.insert_poll_group(sockets.create_poll_group());
            SteamworksNetworkingSocketsOperation::PollGroupCreated { poll_group }
        }
        SteamworksNetworkingSocketsCommand::PollListenSocketEvents {
            listen_socket,
            max_events,
            request_policy,
        } => poll_listen_socket_events(handles, *listen_socket, *max_events, request_policy)?,
        SteamworksNetworkingSocketsCommand::PollConnectionEvents {
            connection,
            max_events,
        } => poll_connection_events(handles, *connection, *max_events)?,
        SteamworksNetworkingSocketsCommand::GetConnectionInfo { connection } => {
            let connection_ref = handles
                .connections
                .get(connection)
                .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: *connection })?;
            let info = connection_ref.info().map_err(|_| {
                SteamworksNetworkingSocketsError::invalid_handle("net_connection.info")
            })?;
            SteamworksNetworkingSocketsOperation::ConnectionInfoRead {
                info: snapshot_connection_info(*connection, info),
            }
        }
        SteamworksNetworkingSocketsCommand::GetRealtimeConnectionStatus { connection, lanes } => {
            let connection_ref = handles
                .connections
                .get(connection)
                .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: *connection })?;
            let (info, lanes) = sockets
                .get_realtime_connection_status(connection_ref, *lanes as i32)
                .map_err(|source| {
                    SteamworksNetworkingSocketsError::steam_error(
                        "networking_sockets.get_realtime_connection_status",
                        source,
                    )
                })?;
            SteamworksNetworkingSocketsOperation::RealtimeConnectionStatusRead {
                status: snapshot_realtime_status(*connection, info, lanes),
            }
        }
        SteamworksNetworkingSocketsCommand::SendMessage {
            connection,
            send_flags,
            data,
        } => {
            let connection_ref = handles
                .connections
                .get(connection)
                .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: *connection })?;
            let message_number =
                connection_ref
                    .send_message(data, *send_flags)
                    .map_err(|source| {
                        SteamworksNetworkingSocketsError::steam_error(
                            "net_connection.send_message",
                            source,
                        )
                    })?;
            SteamworksNetworkingSocketsOperation::MessageSent {
                connection: *connection,
                message_number: u64::from(message_number),
                bytes: data.len(),
            }
        }
        SteamworksNetworkingSocketsCommand::SendMessages { messages } => {
            let client = client.expect("SendMessages requires SteamworksClient");
            let mut outbound = Vec::with_capacity(messages.len());
            for message in messages {
                let connection_ref = handles.connections.get(&message.connection).ok_or(
                    SteamworksNetworkingSocketsError::ConnectionNotFound {
                        id: message.connection,
                    },
                )?;
                outbound.push(allocate_outbound_message(client, connection_ref, message)?);
            }

            let send_results = sockets.send_messages(outbound);
            let messages: Vec<SteamworksNetworkingSocketsMessageSendResult> = messages
                .iter()
                .zip(send_results)
                .map(
                    |(message, result)| SteamworksNetworkingSocketsMessageSendResult {
                        connection: message.connection,
                        send_flags: message.send_flags,
                        channel: message.channel,
                        bytes: message.data.len(),
                        user_data: message.user_data,
                        result: result.map(u64::from),
                    },
                )
                .collect();
            if messages
                .iter()
                .any(|message: &SteamworksNetworkingSocketsMessageSendResult| {
                    message.result.is_err()
                })
            {
                tracing::warn!(
                    target: "bevy_steamworks",
                    messages = ?messages,
                    "Steamworks networking sockets batch send had per-message failures"
                );
            }
            SteamworksNetworkingSocketsOperation::MessagesSent { messages }
        }
        SteamworksNetworkingSocketsCommand::ReceiveMessages {
            connection,
            batch_size,
        } => {
            let connection_ref = handles
                .connections
                .get_mut(connection)
                .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: *connection })?;
            let messages = connection_ref.receive_messages(*batch_size).map_err(|_| {
                SteamworksNetworkingSocketsError::invalid_handle("net_connection.receive_messages")
            })?;
            let messages = messages
                .into_iter()
                .map(|message| snapshot_message(*connection, message))
                .collect();
            SteamworksNetworkingSocketsOperation::MessagesReceived {
                connection: *connection,
                messages,
            }
        }
        SteamworksNetworkingSocketsCommand::ReceivePollGroupMessages {
            poll_group,
            batch_size,
        } => {
            let poll_group_ref = handles
                .poll_groups
                .get_mut(poll_group)
                .ok_or(SteamworksNetworkingSocketsError::PollGroupNotFound { id: *poll_group })?;
            let messages = poll_group_ref
                .receive_messages(*batch_size)
                .into_iter()
                .map(|message| snapshot_poll_group_message(*poll_group, message))
                .collect();
            SteamworksNetworkingSocketsOperation::PollGroupMessagesReceived {
                poll_group: *poll_group,
                messages,
            }
        }
        SteamworksNetworkingSocketsCommand::FlushMessages { connection } => {
            let connection_ref = handles
                .connections
                .get(connection)
                .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: *connection })?;
            connection_ref.flush_messages().map_err(|source| {
                SteamworksNetworkingSocketsError::steam_error(
                    "net_connection.flush_messages",
                    source,
                )
            })?;
            SteamworksNetworkingSocketsOperation::MessagesFlushed {
                connection: *connection,
            }
        }
        SteamworksNetworkingSocketsCommand::SetConnectionPollGroup {
            connection,
            poll_group,
        } => {
            let connection_ref = handles
                .connections
                .get(connection)
                .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: *connection })?;
            let poll_group_ref = handles
                .poll_groups
                .get(poll_group)
                .ok_or(SteamworksNetworkingSocketsError::PollGroupNotFound { id: *poll_group })?;
            connection_ref.set_poll_group(poll_group_ref);
            handles.set_connection_poll_group(*connection, *poll_group);
            SteamworksNetworkingSocketsOperation::ConnectionPollGroupSet {
                connection: *connection,
                poll_group: *poll_group,
            }
        }
        SteamworksNetworkingSocketsCommand::ClearConnectionPollGroup { connection } => {
            let connection_ref = handles
                .connections
                .get(connection)
                .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: *connection })?;
            connection_ref.clear_poll_group().map_err(|_| {
                SteamworksNetworkingSocketsError::invalid_handle("net_connection.clear_poll_group")
            })?;
            handles.clear_connection_poll_group(*connection);
            SteamworksNetworkingSocketsOperation::ConnectionPollGroupCleared {
                connection: *connection,
            }
        }
        SteamworksNetworkingSocketsCommand::ConfigureConnectionLanes {
            connection,
            lane_priorities,
            lane_weights,
        } => {
            let connection_ref = handles
                .connections
                .get(connection)
                .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: *connection })?;
            sockets
                .configure_connection_lanes(
                    connection_ref,
                    lane_priorities.len() as i32,
                    lane_priorities,
                    lane_weights,
                )
                .map_err(|source| {
                    SteamworksNetworkingSocketsError::steam_error(
                        "networking_sockets.configure_connection_lanes",
                        source,
                    )
                })?;
            SteamworksNetworkingSocketsOperation::ConnectionLanesConfigured {
                connection: *connection,
                lanes: lane_priorities.len(),
            }
        }
        SteamworksNetworkingSocketsCommand::SetConnectionUserData {
            connection,
            user_data,
        } => {
            let connection_ref = handles
                .connections
                .get(connection)
                .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: *connection })?;
            connection_ref
                .set_connection_user_data(*user_data)
                .map_err(|_| {
                    SteamworksNetworkingSocketsError::invalid_handle(
                        "net_connection.set_connection_user_data",
                    )
                })?;
            handles.update_connection_user_data(*connection, *user_data);
            SteamworksNetworkingSocketsOperation::ConnectionUserDataSet {
                connection: *connection,
                user_data: *user_data,
            }
        }
        SteamworksNetworkingSocketsCommand::CloseConnection {
            connection,
            reason,
            debug,
            enable_linger,
        } => {
            let connection_handle = handles
                .remove_connection(connection)
                .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: *connection })?;
            let close_succeeded =
                connection_handle.close(*reason, debug.as_deref(), *enable_linger);
            SteamworksNetworkingSocketsOperation::ConnectionClosed {
                connection: *connection,
                close_succeeded,
            }
        }
        SteamworksNetworkingSocketsCommand::CloseListenSocket { listen_socket } => {
            if !handles.listen_sockets.contains_key(listen_socket) {
                return Err(SteamworksNetworkingSocketsError::ListenSocketNotFound {
                    id: *listen_socket,
                });
            }
            let closed_connections = handles.remove_connections_for_listen_socket(*listen_socket);
            handles.listen_sockets.remove(listen_socket);
            SteamworksNetworkingSocketsOperation::ListenSocketClosed {
                listen_socket: *listen_socket,
                closed_connections,
            }
        }
        SteamworksNetworkingSocketsCommand::ClosePollGroup { poll_group } => {
            handles
                .remove_poll_group(poll_group)
                .ok_or(SteamworksNetworkingSocketsError::PollGroupNotFound { id: *poll_group })?;
            SteamworksNetworkingSocketsOperation::PollGroupClosed {
                poll_group: *poll_group,
            }
        }
    })
}

fn networking_sockets(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
) -> Result<steamworks::networking_sockets::NetworkingSockets, SteamworksNetworkingSocketsError> {
    if let Some(client) = client {
        Ok(client.networking_sockets())
    } else if let Some(server) = server {
        Ok(server.networking_sockets())
    } else {
        Err(SteamworksNetworkingSocketsError::ClientUnavailable)
    }
}

fn allocate_outbound_message(
    client: &SteamworksClient,
    connection: &steamworks::networking_sockets::NetConnection,
    message: &SteamworksNetworkingSocketsOutboundMessage,
) -> Result<steamworks::networking_types::NetworkingMessage, SteamworksNetworkingSocketsError> {
    let mut outbound = client
        .networking_utils()
        .allocate_message(message.data.len());
    outbound.set_connection(connection);
    outbound.set_send_flags(message.send_flags);
    outbound.set_channel(message.channel);
    outbound.set_user_data(message.user_data);

    if !message.data.is_empty() {
        outbound
            .copy_data_into_buffer(&message.data)
            .map_err(|source| {
                SteamworksNetworkingSocketsError::message_error(
                    "networking_utils.allocate_message.copy_data_into_buffer",
                    source,
                )
            })?;
    }

    Ok(outbound)
}
