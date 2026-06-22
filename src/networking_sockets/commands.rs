use bevy_ecs::{
    message::{MessageWriter, Messages},
    prelude::{Res, ResMut},
};

pub(super) mod helpers;

mod auth_commands;
mod connection_commands;
mod listen_socket_commands;
mod message_commands;
mod poll_group_commands;

use crate::{SteamworksClient, SteamworksServer};

use super::{
    handles::{SteamworksNetworkingSocketsHandleStorage, SteamworksNetworkingSocketsHandles},
    polling::{
        poll_all_connection_events, poll_all_listen_socket_events, poll_connection_events,
        poll_listen_socket_events,
    },
    validation::validate_command,
    SteamworksNetworkingSocketsCommand, SteamworksNetworkingSocketsError,
    SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsResult,
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

pub(super) fn handle_networking_sockets_command(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    command: &SteamworksNetworkingSocketsCommand,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    validate_command(command)?;

    Ok(match command {
        SteamworksNetworkingSocketsCommand::InitAuthentication => {
            auth_commands::init_authentication(client, server)?
        }
        SteamworksNetworkingSocketsCommand::GetAuthenticationStatus => {
            auth_commands::get_authentication_status(client, server)?
        }
        SteamworksNetworkingSocketsCommand::CreateListenSocketIp {
            local_address,
            options,
        } => listen_socket_commands::create_listen_socket_ip(
            client,
            server,
            handles,
            *local_address,
            options,
        )?,
        SteamworksNetworkingSocketsCommand::CreateListenSocketP2p {
            local_virtual_port,
            options,
        } => listen_socket_commands::create_listen_socket_p2p(
            client,
            server,
            handles,
            *local_virtual_port,
            options,
        )?,
        SteamworksNetworkingSocketsCommand::CreateHostedDedicatedServerListenSocket {
            local_virtual_port,
            options,
        } => listen_socket_commands::create_hosted_dedicated_server_listen_socket(
            server,
            handles,
            *local_virtual_port,
            options,
        )?,
        SteamworksNetworkingSocketsCommand::ConnectByIpAddress { address, options } => {
            connection_commands::connect_by_ip_address(client, server, handles, *address, options)?
        }
        SteamworksNetworkingSocketsCommand::ConnectP2p {
            identity,
            remote_virtual_port,
            options,
        } => connection_commands::connect_p2p(
            client,
            server,
            handles,
            identity,
            *remote_virtual_port,
            options,
        )?,
        SteamworksNetworkingSocketsCommand::CreatePollGroup => {
            poll_group_commands::create_poll_group(client, server, handles)?
        }
        SteamworksNetworkingSocketsCommand::CreateServerPollGroup => {
            poll_group_commands::create_server_poll_group(server, handles)?
        }
        SteamworksNetworkingSocketsCommand::PollListenSocketEvents {
            listen_socket,
            max_events,
            request_policy,
        } => poll_listen_socket_events(handles, *listen_socket, *max_events, request_policy)?,
        SteamworksNetworkingSocketsCommand::PollAllListenSocketEvents {
            max_events_per_socket,
            request_policy,
        } => poll_all_listen_socket_events(handles, *max_events_per_socket, request_policy)?,
        SteamworksNetworkingSocketsCommand::PollConnectionEvents {
            connection,
            max_events,
        } => poll_connection_events(handles, *connection, *max_events)?,
        SteamworksNetworkingSocketsCommand::PollAllConnectionEvents {
            max_events_per_connection,
        } => poll_all_connection_events(handles, *max_events_per_connection)?,
        SteamworksNetworkingSocketsCommand::GetConnectionInfo { connection } => {
            connection_commands::get_connection_info(handles, *connection)?
        }
        SteamworksNetworkingSocketsCommand::GetConnectionUserData { connection } => {
            connection_commands::get_connection_user_data(handles, *connection)?
        }
        SteamworksNetworkingSocketsCommand::GetRealtimeConnectionStatus { connection, lanes } => {
            connection_commands::get_realtime_connection_status(
                client,
                server,
                handles,
                *connection,
                *lanes,
            )?
        }
        SteamworksNetworkingSocketsCommand::SendMessage {
            connection,
            send_flags,
            data,
        } => message_commands::send_message(handles, *connection, *send_flags, data)?,
        SteamworksNetworkingSocketsCommand::SendMessages { messages } => {
            message_commands::send_messages(client, server, handles, messages)?
        }
        SteamworksNetworkingSocketsCommand::ReceiveMessages {
            connection,
            batch_size,
        } => message_commands::receive_messages(handles, *connection, *batch_size)?,
        SteamworksNetworkingSocketsCommand::ReceiveAllMessages {
            batch_size_per_connection,
        } => message_commands::receive_all_messages(handles, *batch_size_per_connection)?,
        SteamworksNetworkingSocketsCommand::ReceivePollGroupMessages {
            poll_group,
            batch_size,
        } => poll_group_commands::receive_poll_group_messages(handles, *poll_group, *batch_size)?,
        SteamworksNetworkingSocketsCommand::ReceiveAllPollGroupMessages {
            batch_size_per_poll_group,
        } => poll_group_commands::receive_all_poll_group_messages(
            handles,
            *batch_size_per_poll_group,
        )?,
        SteamworksNetworkingSocketsCommand::FlushMessages { connection } => {
            message_commands::flush_messages(handles, *connection)?
        }
        SteamworksNetworkingSocketsCommand::SetConnectionPollGroup {
            connection,
            poll_group,
        } => connection_commands::set_connection_poll_group(handles, *connection, *poll_group)?,
        SteamworksNetworkingSocketsCommand::ClearConnectionPollGroup { connection } => {
            connection_commands::clear_connection_poll_group(handles, *connection)?
        }
        SteamworksNetworkingSocketsCommand::ConfigureConnectionLanes {
            connection,
            lane_priorities,
            lane_weights,
        } => connection_commands::configure_connection_lanes(
            client,
            server,
            handles,
            *connection,
            lane_priorities,
            lane_weights,
        )?,
        SteamworksNetworkingSocketsCommand::SetConnectionUserData {
            connection,
            user_data,
        } => connection_commands::set_connection_user_data(handles, *connection, *user_data)?,
        SteamworksNetworkingSocketsCommand::SetConnectionName { connection, name } => {
            connection_commands::set_connection_name(handles, *connection, name)?
        }
        SteamworksNetworkingSocketsCommand::CloseConnection {
            connection,
            reason,
            debug,
            enable_linger,
        } => connection_commands::close_connection(
            handles,
            *connection,
            *reason,
            debug.as_deref(),
            *enable_linger,
        )?,
        SteamworksNetworkingSocketsCommand::CloseListenSocket { listen_socket } => {
            listen_socket_commands::close_listen_socket(handles, *listen_socket)?
        }
        SteamworksNetworkingSocketsCommand::ClosePollGroup { poll_group } => {
            poll_group_commands::close_poll_group(handles, *poll_group)?
        }
    })
}
