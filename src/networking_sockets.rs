//! High-level Bevy ECS integration for Steam Networking Sockets.
//!
//! This module builds on top of the upstream
//! [`steamworks::networking_sockets::NetworkingSockets`] API. It keeps listen
//! sockets and connections owned by a private Bevy resource, while exposing
//! stable integer IDs, owned snapshots, and command/result messages to game
//! systems.

use std::{collections::HashMap, sync::Mutex};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};

use crate::{SteamworksClient, SteamworksSystem};

/// Maximum number of socket/listen events processed by one poll command.
pub const STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND: usize = 256;

/// Maximum number of messages received by one socket receive command.
pub const STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND: usize = 1024;

/// Maximum realtime lane statuses requested by one status command.
///
/// The Steamworks API accepts a signed lane count and returns one status per
/// requested lane. This cap prevents a single ECS command from allocating an
/// unbounded status vector.
pub const STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES: u32 = 64;

/// Maximum lanes configured by one lane configuration command.
pub const STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES: usize = 64;

/// Conservative maximum message payload size accepted by this command layer.
///
/// The upstream Steamworks API can reject oversize messages at send time. This
/// cap keeps one ECS command from allocating or attempting to submit unbounded
/// payloads in a frame.
pub const STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES: usize = 1_048_576;

/// Bevy plugin for high-level Steam Networking Sockets commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksNetworkingSocketsCommand`] and
/// [`SteamworksNetworkingSocketsResult`] messages and processes commands in
/// [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksNetworkingSocketsPlugin;

impl SteamworksNetworkingSocketsPlugin {
    /// Creates a Networking Sockets plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksNetworkingSocketsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksNetworkingSocketsState>()
            .init_resource::<SteamworksNetworkingSocketsHandles>()
            .add_message::<SteamworksNetworkingSocketsCommand>()
            .add_message::<SteamworksNetworkingSocketsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessNetworkingSocketsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_networking_sockets_commands
                    .in_set(SteamworksSystem::ProcessNetworkingSocketsCommands),
            );
    }
}

mod messages;
mod state;
mod types;

pub use messages::*;
pub use state::SteamworksNetworkingSocketsState;
pub use types::*;

#[derive(Default, Resource)]
struct SteamworksNetworkingSocketsHandles {
    storage: Mutex<SteamworksNetworkingSocketsHandleStorage>,
}

struct SteamworksNetworkingSocketsHandleStorage {
    next_listen_socket_id: u64,
    next_connection_id: u64,
    next_poll_group_id: u64,
    listen_sockets: HashMap<SteamworksListenSocketId, steamworks::networking_sockets::ListenSocket>,
    connections: HashMap<
        SteamworksNetworkingSocketsConnectionId,
        steamworks::networking_sockets::NetConnection,
    >,
    poll_groups: HashMap<
        SteamworksNetworkingSocketsPollGroupId,
        steamworks::networking_sockets::NetPollGroup,
    >,
    connection_metadata: HashMap<
        SteamworksNetworkingSocketsConnectionId,
        SteamworksNetworkingSocketsConnectionMetadata,
    >,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SteamworksNetworkingSocketsConnectionMetadata {
    listen_socket: Option<SteamworksListenSocketId>,
    poll_group: Option<SteamworksNetworkingSocketsPollGroupId>,
    remote: Option<steamworks::networking_types::NetworkingIdentity>,
    user_data: i64,
}

impl SteamworksNetworkingSocketsConnectionMetadata {
    fn independent() -> Self {
        Self {
            listen_socket: None,
            poll_group: None,
            remote: None,
            user_data: 0,
        }
    }

    fn listen_socket(
        listen_socket: SteamworksListenSocketId,
        remote: steamworks::networking_types::NetworkingIdentity,
        user_data: i64,
    ) -> Self {
        Self {
            listen_socket: Some(listen_socket),
            poll_group: None,
            remote: Some(remote),
            user_data,
        }
    }
}

impl Default for SteamworksNetworkingSocketsHandleStorage {
    fn default() -> Self {
        Self {
            next_listen_socket_id: 1,
            next_connection_id: 1,
            next_poll_group_id: 1,
            listen_sockets: HashMap::default(),
            connections: HashMap::default(),
            poll_groups: HashMap::default(),
            connection_metadata: HashMap::default(),
        }
    }
}

impl SteamworksNetworkingSocketsHandleStorage {
    fn insert_listen_socket(
        &mut self,
        socket: steamworks::networking_sockets::ListenSocket,
    ) -> SteamworksListenSocketId {
        let id = SteamworksListenSocketId::from_raw(self.next_listen_socket_id);
        self.next_listen_socket_id = self.next_listen_socket_id.saturating_add(1).max(1);
        self.listen_sockets.insert(id, socket);
        id
    }

    fn insert_connection(
        &mut self,
        connection: steamworks::networking_sockets::NetConnection,
        metadata: SteamworksNetworkingSocketsConnectionMetadata,
    ) -> SteamworksNetworkingSocketsConnectionId {
        let id = SteamworksNetworkingSocketsConnectionId::from_raw(self.next_connection_id);
        self.next_connection_id = self.next_connection_id.saturating_add(1).max(1);
        self.connections.insert(id, connection);
        self.connection_metadata.insert(id, metadata);
        id
    }

    fn insert_poll_group(
        &mut self,
        poll_group: steamworks::networking_sockets::NetPollGroup,
    ) -> SteamworksNetworkingSocketsPollGroupId {
        let id = SteamworksNetworkingSocketsPollGroupId::from_raw(self.next_poll_group_id);
        self.next_poll_group_id = self.next_poll_group_id.saturating_add(1).max(1);
        self.poll_groups.insert(id, poll_group);
        id
    }

    fn remove_connection(
        &mut self,
        connection: &SteamworksNetworkingSocketsConnectionId,
    ) -> Option<steamworks::networking_sockets::NetConnection> {
        self.connection_metadata.remove(connection);
        self.connections.remove(connection)
    }

    fn remove_poll_group(
        &mut self,
        poll_group: &SteamworksNetworkingSocketsPollGroupId,
    ) -> Option<steamworks::networking_sockets::NetPollGroup> {
        let removed = self.poll_groups.remove(poll_group)?;
        self.clear_poll_group_metadata(*poll_group);
        Some(removed)
    }

    fn clear_poll_group_metadata(
        &mut self,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) -> usize {
        let mut cleared = 0;
        for metadata in self.connection_metadata.values_mut() {
            if metadata.poll_group == Some(poll_group) {
                metadata.poll_group = None;
                cleared += 1;
            }
        }
        cleared
    }

    fn remove_connections_for_listen_socket(
        &mut self,
        listen_socket: SteamworksListenSocketId,
    ) -> Vec<SteamworksNetworkingSocketsConnectionId> {
        let connections = self
            .connection_metadata
            .iter()
            .filter_map(|(connection, metadata)| {
                (metadata.listen_socket == Some(listen_socket)).then_some(*connection)
            })
            .collect::<Vec<_>>();

        for connection in &connections {
            self.remove_connection(connection);
        }

        connections
    }

    fn remove_listen_connection_by_event(
        &mut self,
        listen_socket: SteamworksListenSocketId,
        remote: &steamworks::networking_types::NetworkingIdentity,
        user_data: i64,
        end_reason: steamworks::networking_types::NetConnectionEnd,
    ) -> Option<SteamworksNetworkingSocketsConnectionId> {
        let candidates = self
            .connection_metadata
            .iter()
            .filter_map(|(connection, metadata)| {
                let matches_listen_socket = metadata.listen_socket == Some(listen_socket);
                let matches_remote = metadata
                    .remote
                    .as_ref()
                    .is_some_and(|known| known == remote);
                let matches_user_data = metadata.user_data == user_data;

                (matches_listen_socket && matches_remote && matches_user_data)
                    .then_some(*connection)
            })
            .collect::<Vec<_>>();

        let connection = match candidates.as_slice() {
            [] => return None,
            [connection] => *connection,
            _ => {
                let terminal_candidates = candidates
                    .iter()
                    .copied()
                    .filter(|connection| {
                        self.connections
                            .get(connection)
                            .and_then(|connection| connection.info().ok())
                            .is_some_and(|info| {
                                info.state().is_ok_and(is_terminal_connection_state)
                                    && info.end_reason() == Some(end_reason)
                            })
                    })
                    .collect::<Vec<_>>();

                match terminal_candidates.as_slice() {
                    [connection] => *connection,
                    _ => return None,
                }
            }
        };

        self.remove_connection(&connection);
        Some(connection)
    }

    fn update_connection_user_data(
        &mut self,
        connection: SteamworksNetworkingSocketsConnectionId,
        user_data: i64,
    ) {
        if let Some(metadata) = self.connection_metadata.get_mut(&connection) {
            metadata.user_data = user_data;
        }
    }

    fn clear_connection_poll_group(&mut self, connection: SteamworksNetworkingSocketsConnectionId) {
        if let Some(metadata) = self.connection_metadata.get_mut(&connection) {
            metadata.poll_group = None;
        }
    }

    fn set_connection_poll_group(
        &mut self,
        connection: SteamworksNetworkingSocketsConnectionId,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) {
        if let Some(metadata) = self.connection_metadata.get_mut(&connection) {
            metadata.poll_group = Some(poll_group);
        }
    }
}

fn process_networking_sockets_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksNetworkingSocketsState>,
    handles: Res<SteamworksNetworkingSocketsHandles>,
    mut commands: ResMut<Messages<SteamworksNetworkingSocketsCommand>>,
    mut results: MessageWriter<SteamworksNetworkingSocketsResult>,
) {
    let Some(client) = client else {
        let error = SteamworksNetworkingSocketsError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks networking sockets command failed"
            );
            results.write(SteamworksNetworkingSocketsResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    let mut handles = handles
        .storage
        .lock()
        .expect("Steamworks Networking Sockets handle storage mutex was poisoned");

    for command in commands.drain() {
        match handle_networking_sockets_command(&client, &mut handles, &command) {
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
    client: &SteamworksClient,
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    command: &SteamworksNetworkingSocketsCommand,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    validate_command(command)?;

    let sockets = client.networking_sockets();
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

fn poll_listen_socket_events(
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
                let connection = handles.insert_connection(
                    event.take_connection(),
                    SteamworksNetworkingSocketsConnectionMetadata::listen_socket(
                        listen_socket,
                        remote.clone(),
                        user_data,
                    ),
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

fn poll_connection_events(
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

fn is_terminal_connection_state(
    state: steamworks::networking_types::NetworkingConnectionState,
) -> bool {
    matches!(
        state,
        steamworks::networking_types::NetworkingConnectionState::ClosedByPeer
            | steamworks::networking_types::NetworkingConnectionState::ProblemDetectedLocally
    )
}

fn snapshot_connection_info(
    connection: SteamworksNetworkingSocketsConnectionId,
    info: steamworks::networking_types::NetConnectionInfo,
) -> SteamworksNetworkingSocketsConnectionInfo {
    SteamworksNetworkingSocketsConnectionInfo {
        connection,
        state: info
            .state()
            .unwrap_or(steamworks::networking_types::NetworkingConnectionState::None),
        remote: info.identity_remote(),
        user_data: info.user_data(),
        end_reason: info.end_reason(),
    }
}

fn snapshot_realtime_status(
    connection: SteamworksNetworkingSocketsConnectionId,
    info: steamworks::networking_types::NetConnectionRealTimeInfo,
    lanes: Vec<steamworks::networking_types::NetConnectionRealTimeLaneStatus>,
) -> SteamworksNetworkingSocketsRealtimeStatus {
    SteamworksNetworkingSocketsRealtimeStatus {
        connection,
        connection_state: info
            .connection_state()
            .unwrap_or(steamworks::networking_types::NetworkingConnectionState::None),
        ping: info.ping(),
        connection_quality_local: info.connection_quality_local(),
        connection_quality_remote: info.connection_quality_remote(),
        out_packets_per_sec: info.out_packets_per_sec(),
        out_bytes_per_sec: info.out_bytes_per_sec(),
        in_packets_per_sec: info.in_packets_per_sec(),
        in_bytes_per_sec: info.in_bytes_per_sec(),
        send_rate_bytes_per_sec: info.send_rate_bytes_per_sec(),
        pending_unreliable: info.pending_unreliable(),
        pending_reliable: info.pending_reliable(),
        sent_unacked_reliable: info.sent_unacked_reliable(),
        queued_send_bytes: info.queued_send_bytes(),
        lanes: lanes
            .into_iter()
            .map(|lane| SteamworksNetworkingSocketsRealtimeLaneStatus {
                pending_unreliable: lane.pending_unreliable(),
                pending_reliable: lane.pending_reliable(),
                sent_unacked_reliable: lane.sent_unacked_reliable(),
                queued_send_bytes: lane.queued_send_bytes(),
            })
            .collect(),
    }
}

fn snapshot_message(
    connection: SteamworksNetworkingSocketsConnectionId,
    message: steamworks::networking_types::NetworkingMessage,
) -> SteamworksNetworkingSocketsMessage {
    SteamworksNetworkingSocketsMessage {
        connection,
        peer: message.identity_peer(),
        data: message.data().to_vec(),
        channel: message.channel(),
        send_flags: message.send_flags(),
        message_number: u64::from(message.message_number()),
        connection_user_data: message.connection_user_data(),
    }
}

fn snapshot_poll_group_message(
    poll_group: SteamworksNetworkingSocketsPollGroupId,
    message: steamworks::networking_types::NetworkingMessage,
) -> SteamworksNetworkingSocketsPollGroupMessage {
    SteamworksNetworkingSocketsPollGroupMessage {
        poll_group,
        peer: message.identity_peer(),
        data: message.data().to_vec(),
        channel: message.channel(),
        send_flags: message.send_flags(),
        message_number: u64::from(message.message_number()),
        connection_user_data: message.connection_user_data(),
    }
}

fn validate_command(
    command: &SteamworksNetworkingSocketsCommand,
) -> Result<(), SteamworksNetworkingSocketsError> {
    match command {
        SteamworksNetworkingSocketsCommand::CreateListenSocketP2p { local_virtual_port } => {
            validate_virtual_port(*local_virtual_port)
        }
        SteamworksNetworkingSocketsCommand::ConnectP2p {
            remote_virtual_port,
            ..
        } => validate_virtual_port(*remote_virtual_port),
        SteamworksNetworkingSocketsCommand::PollListenSocketEvents {
            max_events,
            request_policy,
            ..
        } => {
            validate_event_limit(*max_events)?;
            validate_request_policy(request_policy)
        }
        SteamworksNetworkingSocketsCommand::PollConnectionEvents { max_events, .. } => {
            validate_event_limit(*max_events)
        }
        SteamworksNetworkingSocketsCommand::ReceiveMessages { batch_size, .. } => {
            validate_message_batch_size(*batch_size)
        }
        SteamworksNetworkingSocketsCommand::ReceivePollGroupMessages { batch_size, .. } => {
            validate_message_batch_size(*batch_size)
        }
        SteamworksNetworkingSocketsCommand::SendMessage { data, .. } => {
            if data.len() > STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES {
                return Err(SteamworksNetworkingSocketsError::MessageTooLarge {
                    bytes: data.len(),
                    max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES,
                });
            }
            Ok(())
        }
        SteamworksNetworkingSocketsCommand::GetRealtimeConnectionStatus { lanes, .. } => {
            if *lanes > STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES {
                return Err(SteamworksNetworkingSocketsError::InvalidLaneCount {
                    lanes: *lanes,
                    max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES,
                });
            }
            Ok(())
        }
        SteamworksNetworkingSocketsCommand::ConfigureConnectionLanes {
            lane_priorities,
            lane_weights,
            ..
        } => validate_lane_configuration(lane_priorities, lane_weights),
        SteamworksNetworkingSocketsCommand::CloseConnection { debug, .. } => {
            if debug
                .as_ref()
                .is_some_and(|value| value.as_bytes().contains(&0))
            {
                return Err(SteamworksNetworkingSocketsError::invalid_string("debug"));
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn validate_event_limit(max_events: usize) -> Result<(), SteamworksNetworkingSocketsError> {
    if max_events == 0 {
        return Err(SteamworksNetworkingSocketsError::InvalidEventLimit);
    }
    if max_events > STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND {
        return Err(SteamworksNetworkingSocketsError::TooManyEvents {
            requested: max_events,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND,
        });
    }
    Ok(())
}

fn validate_message_batch_size(batch_size: usize) -> Result<(), SteamworksNetworkingSocketsError> {
    if batch_size == 0 {
        return Err(SteamworksNetworkingSocketsError::InvalidBatchSize);
    }
    if batch_size > STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND {
        return Err(SteamworksNetworkingSocketsError::BatchSizeTooLarge {
            requested: batch_size,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND,
        });
    }
    Ok(())
}

fn validate_lane_configuration(
    lane_priorities: &[i32],
    lane_weights: &[u16],
) -> Result<(), SteamworksNetworkingSocketsError> {
    if lane_priorities.is_empty() || lane_priorities.len() != lane_weights.len() {
        return Err(SteamworksNetworkingSocketsError::InvalidLaneConfiguration {
            priorities: lane_priorities.len(),
            weights: lane_weights.len(),
        });
    }
    if lane_priorities.len() > STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES {
        return Err(SteamworksNetworkingSocketsError::TooManyConfiguredLanes {
            requested: lane_priorities.len(),
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES,
        });
    }
    Ok(())
}

fn validate_virtual_port(port: i32) -> Result<(), SteamworksNetworkingSocketsError> {
    if port < 0 {
        return Err(SteamworksNetworkingSocketsError::InvalidVirtualPort { port });
    }
    Ok(())
}

fn validate_request_policy(
    policy: &SteamworksConnectionRequestPolicy,
) -> Result<(), SteamworksNetworkingSocketsError> {
    match policy {
        SteamworksConnectionRequestPolicy::Reject { debug, .. }
            if debug
                .as_ref()
                .is_some_and(|value| value.as_bytes().contains(&0)) =>
        {
            Err(SteamworksNetworkingSocketsError::invalid_string(
                "request_policy.debug",
            ))
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests;
