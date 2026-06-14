//! High-level Bevy ECS integration for Steam Networking Sockets.
//!
//! This module builds on top of the upstream
//! [`steamworks::networking_sockets::NetworkingSockets`] API. It keeps listen
//! sockets and connections owned by a private Bevy resource, while exposing
//! stable integer IDs, owned snapshots, and command/result messages to game
//! systems.

use std::{collections::HashMap, net::SocketAddr, sync::Mutex};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

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

/// Opaque ID for a listen socket owned by [`SteamworksNetworkingSocketsPlugin`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SteamworksListenSocketId(u64);

impl SteamworksListenSocketId {
    /// Creates an ID from a raw integer.
    pub fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw integer backing this ID.
    pub fn raw(self) -> u64 {
        self.0
    }
}

/// Opaque ID for a connection owned by [`SteamworksNetworkingSocketsPlugin`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SteamworksNetworkingSocketsConnectionId(u64);

impl SteamworksNetworkingSocketsConnectionId {
    /// Creates an ID from a raw integer.
    pub fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw integer backing this ID.
    pub fn raw(self) -> u64 {
        self.0
    }
}

/// Opaque ID for a poll group owned by [`SteamworksNetworkingSocketsPlugin`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SteamworksNetworkingSocketsPollGroupId(u64);

impl SteamworksNetworkingSocketsPollGroupId {
    /// Creates an ID from a raw integer.
    pub fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw integer backing this ID.
    pub fn raw(self) -> u64 {
        self.0
    }
}

/// Runtime state for [`SteamworksNetworkingSocketsPlugin`].
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
    last_received_messages: Vec<SteamworksNetworkingSocketsMessage>,
    last_poll_group_messages: Vec<SteamworksNetworkingSocketsPollGroupMessage>,
    last_flushed_connection: Option<SteamworksNetworkingSocketsConnectionId>,
    last_connection_poll_group_set: Option<SteamworksNetworkingSocketsPollGroupAssignment>,
    last_connection_poll_group_cleared: Option<SteamworksNetworkingSocketsConnectionId>,
    last_connection_lanes_configured: Option<SteamworksNetworkingSocketsLaneConfiguration>,
    last_connection_user_data: Option<SteamworksNetworkingSocketsConnectionUserData>,
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

    /// Returns the most recent connection user data set through this plugin.
    pub fn last_connection_user_data(
        &self,
    ) -> Option<&SteamworksNetworkingSocketsConnectionUserData> {
        self.last_connection_user_data.as_ref()
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

    /// Returns the number of successful send commands observed through the plugin.
    pub fn sent_count(&self) -> u64 {
        self.sent_count
    }

    /// Returns the number of messages received through the plugin.
    pub fn received_count(&self) -> u64 {
        self.received_count
    }

    fn sync_handle_counts(&mut self, handles: &SteamworksNetworkingSocketsHandleStorage) {
        self.listen_socket_count = handles.listen_sockets.len();
        self.connection_count = handles.connections.len();
        self.poll_group_count = handles.poll_groups.len();
    }

    fn record_error(&mut self, error: SteamworksNetworkingSocketsError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksNetworkingSocketsOperation) {
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
            SteamworksNetworkingSocketsOperation::ConnectionUserDataSet {
                connection,
                user_data,
            } => {
                self.last_connection_user_data =
                    Some(SteamworksNetworkingSocketsConnectionUserData {
                        connection: *connection,
                        user_data: *user_data,
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
        let id = SteamworksListenSocketId(self.next_listen_socket_id);
        self.next_listen_socket_id = self.next_listen_socket_id.saturating_add(1).max(1);
        self.listen_sockets.insert(id, socket);
        id
    }

    fn insert_connection(
        &mut self,
        connection: steamworks::networking_sockets::NetConnection,
        metadata: SteamworksNetworkingSocketsConnectionMetadata,
    ) -> SteamworksNetworkingSocketsConnectionId {
        let id = SteamworksNetworkingSocketsConnectionId(self.next_connection_id);
        self.next_connection_id = self.next_connection_id.saturating_add(1).max(1);
        self.connections.insert(id, connection);
        self.connection_metadata.insert(id, metadata);
        id
    }

    fn insert_poll_group(
        &mut self,
        poll_group: steamworks::networking_sockets::NetPollGroup,
    ) -> SteamworksNetworkingSocketsPollGroupId {
        let id = SteamworksNetworkingSocketsPollGroupId(self.next_poll_group_id);
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

/// Policy used when a listen socket receives a connection request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksConnectionRequestPolicy {
    /// Accept incoming connection requests immediately.
    Accept,
    /// Reject incoming connection requests immediately.
    Reject {
        /// End reason sent to the remote peer.
        reason: steamworks::networking_types::NetConnectionEnd,
        /// Optional debug string sent to Steam.
        debug: Option<String>,
    },
}

impl Default for SteamworksConnectionRequestPolicy {
    fn default() -> Self {
        Self::Reject {
            reason: steamworks::networking_types::NetConnectionEnd::App(
                steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
            ),
            debug: Some("connection rejected by bevy_steamworks policy".to_owned()),
        }
    }
}

/// Listen socket creation snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsListenSocketCreated {
    /// New plugin-owned listen socket ID.
    pub listen_socket: SteamworksListenSocketId,
    /// Bound local address or virtual-port descriptor.
    pub endpoint: SteamworksNetworkingSocketsListenEndpoint,
}

/// Connection creation snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsConnectionCreated {
    /// New plugin-owned connection ID.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Connection target.
    pub target: SteamworksNetworkingSocketsConnectionTarget,
}

/// Listen socket event batch snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksNetworkingSocketsListenSocketEvents {
    /// Listen socket that was polled.
    pub listen_socket: SteamworksListenSocketId,
    /// Events observed.
    pub events: Vec<SteamworksListenSocketEventInfo>,
}

/// Connection event batch snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsConnectionEvents {
    /// Connection that was polled.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Events observed.
    pub events: Vec<SteamworksNetworkingSocketsConnectionEventInfo>,
    /// Whether a terminal event caused the plugin to remove this connection.
    pub connection_removed: bool,
}

/// Sent-message snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsSentMessage {
    /// Connection sent on.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Assigned message number.
    pub message_number: u64,
    /// Payload size in bytes.
    pub bytes: usize,
}

/// Connection poll-group assignment snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsPollGroupAssignment {
    /// Connection assigned.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Poll group assigned.
    pub poll_group: SteamworksNetworkingSocketsPollGroupId,
}

/// Connection lane configuration snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsLaneConfiguration {
    /// Connection configured.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Number of lanes configured.
    pub lanes: usize,
}

/// Connection user-data mutation snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsConnectionUserData {
    /// Connection updated.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// User data value.
    pub user_data: i64,
}

/// Connection close snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsConnectionClosed {
    /// Connection removed.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Return value from Steam's close call.
    pub close_succeeded: bool,
}

/// Listen socket close snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsListenSocketClosed {
    /// Listen socket removed.
    pub listen_socket: SteamworksListenSocketId,
    /// Accepted child connections removed with the listen socket.
    pub closed_connections: Vec<SteamworksNetworkingSocketsConnectionId>,
}

/// Owned snapshot of one Networking Sockets connection.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksNetworkingSocketsConnectionInfo {
    /// Plugin-owned connection ID.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// High-level connection state reported by Steam.
    pub state: steamworks::networking_types::NetworkingConnectionState,
    /// Remote peer identity when Steam reports one.
    pub remote: Option<steamworks::networking_types::NetworkingIdentity>,
    /// Connection user data reported by Steam.
    pub user_data: i64,
    /// End reason reported by Steam when the connection has ended.
    pub end_reason: Option<steamworks::networking_types::NetConnectionEnd>,
}

/// Owned snapshot of one Networking Sockets realtime lane.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksNetworkingSocketsRealtimeLaneStatus {
    /// Pending unreliable bytes for this lane.
    pub pending_unreliable: i32,
    /// Pending reliable bytes for this lane.
    pub pending_reliable: i32,
    /// Sent reliable bytes awaiting acknowledgement for this lane.
    pub sent_unacked_reliable: i32,
    /// Lane-specific queue time reported by Steam.
    pub queued_send_bytes: i64,
}

/// Owned snapshot of realtime connection status.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksNetworkingSocketsRealtimeStatus {
    /// Plugin-owned connection ID.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Connection state in the realtime status block.
    pub connection_state: steamworks::networking_types::NetworkingConnectionState,
    /// Estimated ping in milliseconds.
    pub ping: i32,
    /// Local delivery quality between 0.0 and 1.0.
    pub connection_quality_local: f32,
    /// Remote delivery quality between 0.0 and 1.0.
    pub connection_quality_remote: f32,
    /// Outbound packet rate.
    pub out_packets_per_sec: f32,
    /// Outbound byte rate.
    pub out_bytes_per_sec: f32,
    /// Inbound packet rate.
    pub in_packets_per_sec: f32,
    /// Inbound byte rate.
    pub in_bytes_per_sec: f32,
    /// Estimated send capacity in bytes per second.
    pub send_rate_bytes_per_sec: i32,
    /// Pending unreliable bytes.
    pub pending_unreliable: i32,
    /// Pending reliable bytes.
    pub pending_reliable: i32,
    /// Sent reliable bytes awaiting acknowledgement.
    pub sent_unacked_reliable: i32,
    /// Queue time reported by Steam.
    pub queued_send_bytes: i64,
    /// Per-lane status snapshots.
    pub lanes: Vec<SteamworksNetworkingSocketsRealtimeLaneStatus>,
}

/// Owned snapshot of one received Networking Sockets message.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsMessage {
    /// Connection that received the message.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Remote peer identity carried by Steam's message.
    pub peer: steamworks::networking_types::NetworkingIdentity,
    /// Message payload copied from Steam's message handle.
    pub data: Vec<u8>,
    /// Message lane/channel.
    pub channel: i32,
    /// Message flags reported by Steam.
    pub send_flags: steamworks::networking_types::SendFlags,
    /// Message number assigned by the sender.
    pub message_number: u64,
    /// Connection user data captured by Steam for this message.
    pub connection_user_data: i64,
}

impl std::fmt::Debug for SteamworksNetworkingSocketsMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SteamworksNetworkingSocketsMessage")
            .field("connection", &self.connection)
            .field("peer", &self.peer)
            .field("data_len", &self.data.len())
            .field("channel", &self.channel)
            .field("send_flags", &self.send_flags)
            .field("message_number", &self.message_number)
            .field("connection_user_data", &self.connection_user_data)
            .finish()
    }
}

/// Owned snapshot of one message received through a poll group.
///
/// The upstream safe wrapper does not expose the raw connection handle carried
/// by poll-group messages. Use [`SteamworksNetworkingSocketsCommand::SetConnectionUserData`]
/// to attach an application-level connection identifier if you need to map
/// poll-group messages back to game state.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsPollGroupMessage {
    /// Poll group that received the message.
    pub poll_group: SteamworksNetworkingSocketsPollGroupId,
    /// Remote peer identity carried by Steam's message.
    pub peer: steamworks::networking_types::NetworkingIdentity,
    /// Message payload copied from Steam's message handle.
    pub data: Vec<u8>,
    /// Message lane/channel.
    pub channel: i32,
    /// Message flags reported by Steam.
    pub send_flags: steamworks::networking_types::SendFlags,
    /// Message number assigned by the sender.
    pub message_number: u64,
    /// Connection user data captured by Steam for this message.
    pub connection_user_data: i64,
}

impl std::fmt::Debug for SteamworksNetworkingSocketsPollGroupMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SteamworksNetworkingSocketsPollGroupMessage")
            .field("poll_group", &self.poll_group)
            .field("peer", &self.peer)
            .field("data_len", &self.data.len())
            .field("channel", &self.channel)
            .field("send_flags", &self.send_flags)
            .field("message_number", &self.message_number)
            .field("connection_user_data", &self.connection_user_data)
            .finish()
    }
}

/// Snapshot of one listen socket event.
#[derive(Clone, Debug, PartialEq)]
pub enum SteamworksListenSocketEventInfo {
    /// An incoming connection request was accepted.
    ConnectionAccepted {
        /// Listen socket that received the request.
        listen_socket: SteamworksListenSocketId,
        /// Remote peer identity.
        remote: steamworks::networking_types::NetworkingIdentity,
        /// Connection user data reported by Steam.
        user_data: i64,
    },
    /// An incoming connection request was rejected.
    ConnectionRejected {
        /// Listen socket that received the request.
        listen_socket: SteamworksListenSocketId,
        /// Remote peer identity.
        remote: steamworks::networking_types::NetworkingIdentity,
        /// Connection user data reported by Steam.
        user_data: i64,
    },
    /// A connection on a listen socket reached the connected state.
    Connected {
        /// Listen socket that owns the connection.
        listen_socket: SteamworksListenSocketId,
        /// New plugin-owned connection ID.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Remote peer identity.
        remote: steamworks::networking_types::NetworkingIdentity,
        /// Connection user data reported by Steam.
        user_data: i64,
    },
    /// A connection on a listen socket disconnected.
    Disconnected {
        /// Listen socket that observed the disconnect.
        listen_socket: SteamworksListenSocketId,
        /// Plugin-owned connection ID that was removed, if it could be matched unambiguously.
        connection: Option<SteamworksNetworkingSocketsConnectionId>,
        /// Remote peer identity.
        remote: steamworks::networking_types::NetworkingIdentity,
        /// Connection user data reported by Steam.
        user_data: i64,
        /// End reason reported by Steam.
        end_reason: steamworks::networking_types::NetConnectionEnd,
    },
}

/// Snapshot of one connection event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsConnectionEventInfo {
    /// Connection that observed the event.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// New connection state.
    pub new_state: steamworks::networking_types::NetworkingConnectionState,
    /// Previous connection state.
    pub old_state: steamworks::networking_types::NetworkingConnectionState,
}

/// A high-level command for Steam Networking Sockets workflows.
#[derive(Clone, Message, PartialEq)]
pub enum SteamworksNetworkingSocketsCommand {
    /// Initialize Steam Networking Sockets authentication resources.
    InitAuthentication,
    /// Read Steam Networking Sockets authentication status.
    GetAuthenticationStatus,
    /// Create an IP listen socket.
    CreateListenSocketIp {
        /// Local socket address to bind.
        local_address: SocketAddr,
    },
    /// Create a P2P listen socket.
    CreateListenSocketP2p {
        /// Local virtual port.
        local_virtual_port: i32,
    },
    /// Connect to an IP endpoint.
    ConnectByIpAddress {
        /// Remote socket address.
        address: SocketAddr,
    },
    /// Connect to a Steam networking identity using P2P.
    ConnectP2p {
        /// Remote identity.
        identity: steamworks::networking_types::NetworkingIdentity,
        /// Remote virtual port.
        remote_virtual_port: i32,
    },
    /// Create a poll group for receiving messages from many connections.
    CreatePollGroup,
    /// Poll events from one listen socket.
    PollListenSocketEvents {
        /// Listen socket to poll.
        listen_socket: SteamworksListenSocketId,
        /// Maximum events to process.
        max_events: usize,
        /// Policy for incoming connection requests.
        request_policy: SteamworksConnectionRequestPolicy,
    },
    /// Poll events from one connection.
    PollConnectionEvents {
        /// Connection to poll.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Maximum events to process.
        max_events: usize,
    },
    /// Read connection info.
    GetConnectionInfo {
        /// Connection to inspect.
        connection: SteamworksNetworkingSocketsConnectionId,
    },
    /// Read realtime connection status.
    GetRealtimeConnectionStatus {
        /// Connection to inspect.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Number of lane statuses to request.
        lanes: u32,
    },
    /// Send one message on a connection.
    SendMessage {
        /// Connection to send on.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Delivery flags.
        send_flags: steamworks::networking_types::SendFlags,
        /// Payload.
        data: Vec<u8>,
    },
    /// Receive messages from one connection.
    ReceiveMessages {
        /// Connection to receive from.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Maximum number of messages to receive.
        batch_size: usize,
    },
    /// Receive messages from one poll group.
    ReceivePollGroupMessages {
        /// Poll group to receive from.
        poll_group: SteamworksNetworkingSocketsPollGroupId,
        /// Maximum number of messages to receive.
        batch_size: usize,
    },
    /// Flush pending messages on one connection.
    FlushMessages {
        /// Connection to flush.
        connection: SteamworksNetworkingSocketsConnectionId,
    },
    /// Assign a connection to a poll group.
    SetConnectionPollGroup {
        /// Connection to assign.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Poll group that should receive messages for the connection.
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    },
    /// Remove a connection from its current poll group.
    ClearConnectionPollGroup {
        /// Connection to update.
        connection: SteamworksNetworkingSocketsConnectionId,
    },
    /// Configure outbound message lanes for one connection.
    ConfigureConnectionLanes {
        /// Connection to configure.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Lane priorities in Steam order.
        lane_priorities: Vec<i32>,
        /// Lane weights in Steam order.
        lane_weights: Vec<u16>,
    },
    /// Set connection user data.
    SetConnectionUserData {
        /// Connection to update.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// User data value.
        user_data: i64,
    },
    /// Close and drop one connection.
    CloseConnection {
        /// Connection to close.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// End reason sent to Steam.
        reason: steamworks::networking_types::NetConnectionEnd,
        /// Optional debug string.
        debug: Option<String>,
        /// Whether Steam should try to flush remaining reliable data.
        enable_linger: bool,
    },
    /// Drop one listen socket.
    CloseListenSocket {
        /// Listen socket to drop.
        listen_socket: SteamworksListenSocketId,
    },
    /// Drop one poll group.
    ClosePollGroup {
        /// Poll group to drop.
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    },
}

impl std::fmt::Debug for SteamworksNetworkingSocketsCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitAuthentication => f.write_str("InitAuthentication"),
            Self::GetAuthenticationStatus => f.write_str("GetAuthenticationStatus"),
            Self::CreateListenSocketIp { local_address } => f
                .debug_struct("CreateListenSocketIp")
                .field("local_address", local_address)
                .finish(),
            Self::CreateListenSocketP2p { local_virtual_port } => f
                .debug_struct("CreateListenSocketP2p")
                .field("local_virtual_port", local_virtual_port)
                .finish(),
            Self::ConnectByIpAddress { address } => f
                .debug_struct("ConnectByIpAddress")
                .field("address", address)
                .finish(),
            Self::ConnectP2p {
                identity,
                remote_virtual_port,
            } => f
                .debug_struct("ConnectP2p")
                .field("identity", identity)
                .field("remote_virtual_port", remote_virtual_port)
                .finish(),
            Self::CreatePollGroup => f.write_str("CreatePollGroup"),
            Self::PollListenSocketEvents {
                listen_socket,
                max_events,
                request_policy,
            } => f
                .debug_struct("PollListenSocketEvents")
                .field("listen_socket", listen_socket)
                .field("max_events", max_events)
                .field("request_policy", request_policy)
                .finish(),
            Self::PollConnectionEvents {
                connection,
                max_events,
            } => f
                .debug_struct("PollConnectionEvents")
                .field("connection", connection)
                .field("max_events", max_events)
                .finish(),
            Self::GetConnectionInfo { connection } => f
                .debug_struct("GetConnectionInfo")
                .field("connection", connection)
                .finish(),
            Self::GetRealtimeConnectionStatus { connection, lanes } => f
                .debug_struct("GetRealtimeConnectionStatus")
                .field("connection", connection)
                .field("lanes", lanes)
                .finish(),
            Self::SendMessage {
                connection,
                send_flags,
                data,
            } => f
                .debug_struct("SendMessage")
                .field("connection", connection)
                .field("send_flags", send_flags)
                .field("data_len", &data.len())
                .finish(),
            Self::ReceiveMessages {
                connection,
                batch_size,
            } => f
                .debug_struct("ReceiveMessages")
                .field("connection", connection)
                .field("batch_size", batch_size)
                .finish(),
            Self::ReceivePollGroupMessages {
                poll_group,
                batch_size,
            } => f
                .debug_struct("ReceivePollGroupMessages")
                .field("poll_group", poll_group)
                .field("batch_size", batch_size)
                .finish(),
            Self::FlushMessages { connection } => f
                .debug_struct("FlushMessages")
                .field("connection", connection)
                .finish(),
            Self::SetConnectionPollGroup {
                connection,
                poll_group,
            } => f
                .debug_struct("SetConnectionPollGroup")
                .field("connection", connection)
                .field("poll_group", poll_group)
                .finish(),
            Self::ClearConnectionPollGroup { connection } => f
                .debug_struct("ClearConnectionPollGroup")
                .field("connection", connection)
                .finish(),
            Self::ConfigureConnectionLanes {
                connection,
                lane_priorities,
                lane_weights,
            } => f
                .debug_struct("ConfigureConnectionLanes")
                .field("connection", connection)
                .field("lane_priorities", lane_priorities)
                .field("lane_weights", lane_weights)
                .finish(),
            Self::SetConnectionUserData {
                connection,
                user_data,
            } => f
                .debug_struct("SetConnectionUserData")
                .field("connection", connection)
                .field("user_data", user_data)
                .finish(),
            Self::CloseConnection {
                connection,
                reason,
                debug,
                enable_linger,
            } => f
                .debug_struct("CloseConnection")
                .field("connection", connection)
                .field("reason", reason)
                .field("debug", debug)
                .field("enable_linger", enable_linger)
                .finish(),
            Self::CloseListenSocket { listen_socket } => f
                .debug_struct("CloseListenSocket")
                .field("listen_socket", listen_socket)
                .finish(),
            Self::ClosePollGroup { poll_group } => f
                .debug_struct("ClosePollGroup")
                .field("poll_group", poll_group)
                .finish(),
        }
    }
}

impl SteamworksNetworkingSocketsCommand {
    /// Creates a [`SteamworksNetworkingSocketsCommand::CreateListenSocketIp`] command.
    pub fn create_listen_socket_ip(local_address: SocketAddr) -> Self {
        Self::CreateListenSocketIp { local_address }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CreateListenSocketP2p`] command.
    pub fn create_listen_socket_p2p(local_virtual_port: i32) -> Self {
        Self::CreateListenSocketP2p { local_virtual_port }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ConnectByIpAddress`] command.
    pub fn connect_by_ip_address(address: SocketAddr) -> Self {
        Self::ConnectByIpAddress { address }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ConnectP2p`] command.
    pub fn connect_p2p(
        identity: steamworks::networking_types::NetworkingIdentity,
        remote_virtual_port: i32,
    ) -> Self {
        Self::ConnectP2p {
            identity,
            remote_virtual_port,
        }
    }

    /// Creates a P2P connect command for a Steam user.
    pub fn connect_p2p_steam_id(steam_id: steamworks::SteamId, remote_virtual_port: i32) -> Self {
        Self::connect_p2p(
            steamworks::networking_types::NetworkingIdentity::new_steam_id(steam_id),
            remote_virtual_port,
        )
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CreatePollGroup`] command.
    pub fn create_poll_group() -> Self {
        Self::CreatePollGroup
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::PollListenSocketEvents`] command.
    pub fn poll_listen_socket_events(
        listen_socket: SteamworksListenSocketId,
        max_events: usize,
        request_policy: SteamworksConnectionRequestPolicy,
    ) -> Self {
        Self::PollListenSocketEvents {
            listen_socket,
            max_events,
            request_policy,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::PollConnectionEvents`] command.
    pub fn poll_connection_events(
        connection: SteamworksNetworkingSocketsConnectionId,
        max_events: usize,
    ) -> Self {
        Self::PollConnectionEvents {
            connection,
            max_events,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::GetConnectionInfo`] command.
    pub fn get_connection_info(connection: SteamworksNetworkingSocketsConnectionId) -> Self {
        Self::GetConnectionInfo { connection }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::GetRealtimeConnectionStatus`] command.
    pub fn get_realtime_connection_status(
        connection: SteamworksNetworkingSocketsConnectionId,
        lanes: u32,
    ) -> Self {
        Self::GetRealtimeConnectionStatus { connection, lanes }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::SendMessage`] command.
    pub fn send_message(
        connection: SteamworksNetworkingSocketsConnectionId,
        send_flags: steamworks::networking_types::SendFlags,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self::SendMessage {
            connection,
            send_flags,
            data: data.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ReceiveMessages`] command.
    pub fn receive_messages(
        connection: SteamworksNetworkingSocketsConnectionId,
        batch_size: usize,
    ) -> Self {
        Self::ReceiveMessages {
            connection,
            batch_size,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ReceivePollGroupMessages`] command.
    pub fn receive_poll_group_messages(
        poll_group: SteamworksNetworkingSocketsPollGroupId,
        batch_size: usize,
    ) -> Self {
        Self::ReceivePollGroupMessages {
            poll_group,
            batch_size,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::FlushMessages`] command.
    pub fn flush_messages(connection: SteamworksNetworkingSocketsConnectionId) -> Self {
        Self::FlushMessages { connection }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::SetConnectionPollGroup`] command.
    pub fn set_connection_poll_group(
        connection: SteamworksNetworkingSocketsConnectionId,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) -> Self {
        Self::SetConnectionPollGroup {
            connection,
            poll_group,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ClearConnectionPollGroup`] command.
    pub fn clear_connection_poll_group(
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Self {
        Self::ClearConnectionPollGroup { connection }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ConfigureConnectionLanes`] command.
    pub fn configure_connection_lanes(
        connection: SteamworksNetworkingSocketsConnectionId,
        lane_priorities: impl Into<Vec<i32>>,
        lane_weights: impl Into<Vec<u16>>,
    ) -> Self {
        Self::ConfigureConnectionLanes {
            connection,
            lane_priorities: lane_priorities.into(),
            lane_weights: lane_weights.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::SetConnectionUserData`] command.
    pub fn set_connection_user_data(
        connection: SteamworksNetworkingSocketsConnectionId,
        user_data: i64,
    ) -> Self {
        Self::SetConnectionUserData {
            connection,
            user_data,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CloseConnection`] command.
    pub fn close_connection(connection: SteamworksNetworkingSocketsConnectionId) -> Self {
        Self::close_connection_with_reason(
            connection,
            steamworks::networking_types::NetConnectionEnd::App(
                steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
            ),
            None,
            false,
        )
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CloseConnection`] command with options.
    pub fn close_connection_with_reason(
        connection: SteamworksNetworkingSocketsConnectionId,
        reason: steamworks::networking_types::NetConnectionEnd,
        debug: Option<String>,
        enable_linger: bool,
    ) -> Self {
        Self::CloseConnection {
            connection,
            reason,
            debug,
            enable_linger,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CloseListenSocket`] command.
    pub fn close_listen_socket(listen_socket: SteamworksListenSocketId) -> Self {
        Self::CloseListenSocket { listen_socket }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ClosePollGroup`] command.
    pub fn close_poll_group(poll_group: SteamworksNetworkingSocketsPollGroupId) -> Self {
        Self::ClosePollGroup { poll_group }
    }
}

/// A successfully submitted Networking Sockets operation, read, or event.
#[derive(Clone, Debug, PartialEq)]
pub enum SteamworksNetworkingSocketsOperation {
    /// Authentication initialization was submitted.
    AuthenticationInitialized {
        /// Current authentication availability.
        availability: steamworks::networking_types::NetworkingAvailabilityResult,
    },
    /// Authentication status was read.
    AuthenticationStatusRead {
        /// Current authentication availability.
        availability: steamworks::networking_types::NetworkingAvailabilityResult,
    },
    /// A listen socket was created.
    ListenSocketCreated {
        /// New plugin-owned listen socket ID.
        listen_socket: SteamworksListenSocketId,
        /// Bound local address or virtual-port descriptor.
        endpoint: SteamworksNetworkingSocketsListenEndpoint,
    },
    /// A connection was created.
    ConnectionCreated {
        /// New plugin-owned connection ID.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Connection target.
        target: SteamworksNetworkingSocketsConnectionTarget,
    },
    /// A poll group was created.
    PollGroupCreated {
        /// New plugin-owned poll group ID.
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    },
    /// Listen socket events were processed.
    ListenSocketEventsPolled {
        /// Listen socket that was polled.
        listen_socket: SteamworksListenSocketId,
        /// Events observed.
        events: Vec<SteamworksListenSocketEventInfo>,
    },
    /// Connection events were processed.
    ConnectionEventsPolled {
        /// Connection that was polled.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Events observed.
        events: Vec<SteamworksNetworkingSocketsConnectionEventInfo>,
        /// Whether a terminal event caused the plugin to remove this connection.
        connection_removed: bool,
    },
    /// Connection info was read.
    ConnectionInfoRead {
        /// Connection info snapshot.
        info: SteamworksNetworkingSocketsConnectionInfo,
    },
    /// Realtime connection status was read.
    RealtimeConnectionStatusRead {
        /// Realtime status snapshot.
        status: SteamworksNetworkingSocketsRealtimeStatus,
    },
    /// One message was sent.
    MessageSent {
        /// Connection sent on.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Assigned message number.
        message_number: u64,
        /// Payload size in bytes.
        bytes: usize,
    },
    /// Messages were received.
    MessagesReceived {
        /// Connection received from.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Owned message snapshots.
        messages: Vec<SteamworksNetworkingSocketsMessage>,
    },
    /// Messages were received from a poll group.
    PollGroupMessagesReceived {
        /// Poll group received from.
        poll_group: SteamworksNetworkingSocketsPollGroupId,
        /// Owned message snapshots.
        messages: Vec<SteamworksNetworkingSocketsPollGroupMessage>,
    },
    /// Pending messages were flushed.
    MessagesFlushed {
        /// Connection flushed.
        connection: SteamworksNetworkingSocketsConnectionId,
    },
    /// A connection was assigned to a poll group.
    ConnectionPollGroupSet {
        /// Connection assigned.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Poll group assigned.
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    },
    /// A connection was removed from its current poll group.
    ConnectionPollGroupCleared {
        /// Connection updated.
        connection: SteamworksNetworkingSocketsConnectionId,
    },
    /// Outbound message lanes were configured for a connection.
    ConnectionLanesConfigured {
        /// Connection configured.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Number of lanes configured.
        lanes: usize,
    },
    /// Connection user data was set.
    ConnectionUserDataSet {
        /// Connection updated.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// User data value.
        user_data: i64,
    },
    /// A connection was closed and removed.
    ConnectionClosed {
        /// Connection removed.
        connection: SteamworksNetworkingSocketsConnectionId,
        /// Return value from Steam's close call.
        close_succeeded: bool,
    },
    /// A listen socket was closed and removed.
    ListenSocketClosed {
        /// Listen socket removed.
        listen_socket: SteamworksListenSocketId,
        /// Accepted child connections removed with the listen socket.
        closed_connections: Vec<SteamworksNetworkingSocketsConnectionId>,
    },
    /// A poll group was closed and removed.
    PollGroupClosed {
        /// Poll group removed.
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    },
}

/// Listen socket endpoint created by a command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksNetworkingSocketsListenEndpoint {
    /// IP listen endpoint.
    Ip(SocketAddr),
    /// P2P virtual port.
    P2p {
        /// Local virtual port.
        local_virtual_port: i32,
    },
}

/// Connection target created by a command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksNetworkingSocketsConnectionTarget {
    /// IP target.
    Ip(SocketAddr),
    /// P2P identity target.
    P2p {
        /// Remote identity.
        identity: steamworks::networking_types::NetworkingIdentity,
        /// Remote virtual port.
        remote_virtual_port: i32,
    },
}

/// Result message emitted by [`SteamworksNetworkingSocketsPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksNetworkingSocketsResult {
    /// The command or event succeeded.
    Ok(SteamworksNetworkingSocketsOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksNetworkingSocketsCommand,
        /// Failure reason.
        error: SteamworksNetworkingSocketsError,
    },
}

/// Synchronous command errors from [`SteamworksNetworkingSocketsPlugin`].
#[derive(Clone, Debug, Error, PartialEq)]
pub enum SteamworksNetworkingSocketsError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A listen socket ID is not owned by this plugin.
    #[error("Steam Networking Sockets listen socket {id:?} was not found")]
    ListenSocketNotFound {
        /// Missing listen socket ID.
        id: SteamworksListenSocketId,
    },
    /// A connection ID is not owned by this plugin.
    #[error("Steam Networking Sockets connection {id:?} was not found")]
    ConnectionNotFound {
        /// Missing connection ID.
        id: SteamworksNetworkingSocketsConnectionId,
    },
    /// A poll group ID is not owned by this plugin.
    #[error("Steam Networking Sockets poll group {id:?} was not found")]
    PollGroupNotFound {
        /// Missing poll group ID.
        id: SteamworksNetworkingSocketsPollGroupId,
    },
    /// A max-events value was zero.
    #[error("Steam Networking Sockets max_events must be greater than zero")]
    InvalidEventLimit,
    /// A max-events value exceeded this crate's per-command cap.
    #[error("Steam Networking Sockets max_events {requested} exceeds max {max_supported}")]
    TooManyEvents {
        /// Requested event count.
        requested: usize,
        /// Maximum accepted event count.
        max_supported: usize,
    },
    /// A message receive batch size was zero.
    #[error("Steam Networking Sockets receive batch size must be greater than zero")]
    InvalidBatchSize,
    /// A message receive batch size exceeded this crate's per-command cap.
    #[error("Steam Networking Sockets receive batch size {requested} exceeds max {max_supported}")]
    BatchSizeTooLarge {
        /// Requested batch size.
        requested: usize,
        /// Maximum accepted batch size.
        max_supported: usize,
    },
    /// A message payload exceeded this crate's per-message cap.
    #[error("Steam Networking Sockets message size {bytes} exceeds max {max_supported}")]
    MessageTooLarge {
        /// Requested payload size.
        bytes: usize,
        /// Maximum accepted payload size.
        max_supported: usize,
    },
    /// A lane count exceeded this crate's per-command cap.
    #[error("Steam Networking Sockets lane count {lanes} exceeds max {max_supported}")]
    InvalidLaneCount {
        /// Invalid lane count.
        lanes: u32,
        /// Maximum accepted lane count.
        max_supported: u32,
    },
    /// A lane configuration has mismatched priority and weight lengths or no lanes.
    #[error(
        "Steam Networking Sockets lane configuration requires matching nonzero priorities and weights, got {priorities} priorities and {weights} weights"
    )]
    InvalidLaneConfiguration {
        /// Number of priority entries.
        priorities: usize,
        /// Number of weight entries.
        weights: usize,
    },
    /// A lane configuration exceeded this crate's per-command cap.
    #[error(
        "Steam Networking Sockets configured lane count {requested} exceeds max {max_supported}"
    )]
    TooManyConfiguredLanes {
        /// Requested lane count.
        requested: usize,
        /// Maximum accepted lane count.
        max_supported: usize,
    },
    /// A virtual port was negative.
    #[error("Steam Networking Sockets virtual port {port} must not be negative")]
    InvalidVirtualPort {
        /// Invalid virtual port.
        port: i32,
    },
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steam Networking Sockets command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// Steam returned an invalid handle.
    #[error("{operation} returned an invalid handle")]
    InvalidHandle {
        /// Operation that failed.
        operation: &'static str,
    },
    /// Steam returned an operation error.
    #[error("{operation} failed: {source}")]
    SteamError {
        /// Operation that failed.
        operation: &'static str,
        /// Error returned by Steam.
        source: steamworks::SteamError,
    },
    /// Steam returned `false` for a boolean operation.
    #[error("{operation} failed")]
    OperationFailed {
        /// Operation that failed.
        operation: &'static str,
    },
}

impl SteamworksNetworkingSocketsError {
    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    fn invalid_handle(operation: &'static str) -> Self {
        Self::InvalidHandle { operation }
    }

    fn steam_error(operation: &'static str, source: steamworks::SteamError) -> Self {
        Self::SteamError { operation, source }
    }

    fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
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
mod tests {
    use std::{net::Ipv4Addr, str::FromStr};

    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    fn connection_id() -> SteamworksNetworkingSocketsConnectionId {
        SteamworksNetworkingSocketsConnectionId::from_raw(42)
    }

    fn listen_socket_id() -> SteamworksListenSocketId {
        SteamworksListenSocketId::from_raw(7)
    }

    fn poll_group_id() -> SteamworksNetworkingSocketsPollGroupId {
        SteamworksNetworkingSocketsPollGroupId::from_raw(9)
    }

    fn localhost() -> SocketAddr {
        SocketAddr::from((Ipv4Addr::LOCALHOST, 27015))
    }

    #[test]
    fn networking_sockets_plugin_registers_resources_and_messages() {
        let mut app = App::new();

        app.add_plugins(SteamworksNetworkingSocketsPlugin::new());

        assert!(app
            .world()
            .contains_resource::<SteamworksNetworkingSocketsState>());
        assert!(app
            .world()
            .contains_resource::<SteamworksNetworkingSocketsHandles>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksNetworkingSocketsCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksNetworkingSocketsResult>>());
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksNetworkingSocketsPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksNetworkingSocketsCommand>>()
            .write(SteamworksNetworkingSocketsCommand::GetAuthenticationStatus);

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksNetworkingSocketsResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksNetworkingSocketsResult::Err {
                command: SteamworksNetworkingSocketsCommand::GetAuthenticationStatus,
                error: SteamworksNetworkingSocketsError::ClientUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksNetworkingSocketsState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksNetworkingSocketsError::ClientUnavailable)
        );
    }

    #[test]
    fn validation_rejects_invalid_inputs() {
        assert_eq!(
            validate_command(&SteamworksNetworkingSocketsCommand::poll_connection_events(
                connection_id(),
                0,
            )),
            Err(SteamworksNetworkingSocketsError::InvalidEventLimit)
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingSocketsCommand::poll_connection_events(
                connection_id(),
                STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND + 1,
            )),
            Err(SteamworksNetworkingSocketsError::TooManyEvents {
                requested: STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND + 1,
                max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND,
            })
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingSocketsCommand::receive_messages(
                connection_id(),
                0,
            )),
            Err(SteamworksNetworkingSocketsError::InvalidBatchSize)
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingSocketsCommand::receive_messages(
                connection_id(),
                STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND + 1,
            )),
            Err(SteamworksNetworkingSocketsError::BatchSizeTooLarge {
                requested: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND + 1,
                max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND,
            })
        );
        assert_eq!(
            validate_command(
                &SteamworksNetworkingSocketsCommand::receive_poll_group_messages(
                    poll_group_id(),
                    0,
                ),
            ),
            Err(SteamworksNetworkingSocketsError::InvalidBatchSize)
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingSocketsCommand::send_message(
                connection_id(),
                steamworks::networking_types::SendFlags::RELIABLE,
                vec![0; STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES + 1],
            )),
            Err(SteamworksNetworkingSocketsError::MessageTooLarge {
                bytes: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES + 1,
                max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES,
            })
        );
        assert_eq!(
            validate_command(
                &SteamworksNetworkingSocketsCommand::GetRealtimeConnectionStatus {
                    connection: connection_id(),
                    lanes: STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES + 1,
                },
            ),
            Err(SteamworksNetworkingSocketsError::InvalidLaneCount {
                lanes: STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES + 1,
                max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES,
            })
        );
        assert_eq!(
            validate_command(
                &SteamworksNetworkingSocketsCommand::configure_connection_lanes(
                    connection_id(),
                    vec![0, 10],
                    vec![100],
                )
            ),
            Err(SteamworksNetworkingSocketsError::InvalidLaneConfiguration {
                priorities: 2,
                weights: 1,
            })
        );
        assert_eq!(
            validate_command(
                &SteamworksNetworkingSocketsCommand::configure_connection_lanes(
                    connection_id(),
                    Vec::<i32>::new(),
                    Vec::<u16>::new(),
                )
            ),
            Err(SteamworksNetworkingSocketsError::InvalidLaneConfiguration {
                priorities: 0,
                weights: 0,
            })
        );
        assert_eq!(
            validate_command(
                &SteamworksNetworkingSocketsCommand::configure_connection_lanes(
                    connection_id(),
                    vec![0; STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES + 1],
                    vec![100; STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES + 1],
                )
            ),
            Err(SteamworksNetworkingSocketsError::TooManyConfiguredLanes {
                requested: STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES + 1,
                max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES,
            })
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingSocketsCommand::create_listen_socket_p2p(-1,)),
            Err(SteamworksNetworkingSocketsError::InvalidVirtualPort { port: -1 })
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingSocketsCommand::CloseConnection {
                connection: connection_id(),
                reason: steamworks::networking_types::NetConnectionEnd::App(
                    steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
                ),
                debug: Some("bad\0debug".to_owned()),
                enable_linger: false,
            }),
            Err(SteamworksNetworkingSocketsError::InvalidString { field: "debug" })
        );
    }

    #[test]
    fn constructors_preserve_inputs() {
        let address = localhost();
        assert_eq!(
            SteamworksNetworkingSocketsCommand::create_listen_socket_ip(address),
            SteamworksNetworkingSocketsCommand::CreateListenSocketIp {
                local_address: address,
            }
        );
        assert_eq!(
            SteamworksNetworkingSocketsCommand::connect_by_ip_address(address),
            SteamworksNetworkingSocketsCommand::ConnectByIpAddress { address }
        );
        assert_eq!(
            SteamworksNetworkingSocketsCommand::poll_listen_socket_events(
                listen_socket_id(),
                8,
                SteamworksConnectionRequestPolicy::Accept,
            ),
            SteamworksNetworkingSocketsCommand::PollListenSocketEvents {
                listen_socket: listen_socket_id(),
                max_events: 8,
                request_policy: SteamworksConnectionRequestPolicy::Accept,
            }
        );
        assert_eq!(
            SteamworksNetworkingSocketsCommand::create_poll_group(),
            SteamworksNetworkingSocketsCommand::CreatePollGroup
        );
        assert_eq!(
            SteamworksNetworkingSocketsCommand::get_realtime_connection_status(connection_id(), 4),
            SteamworksNetworkingSocketsCommand::GetRealtimeConnectionStatus {
                connection: connection_id(),
                lanes: 4,
            }
        );
        assert_eq!(
            SteamworksNetworkingSocketsCommand::receive_poll_group_messages(poll_group_id(), 16),
            SteamworksNetworkingSocketsCommand::ReceivePollGroupMessages {
                poll_group: poll_group_id(),
                batch_size: 16,
            }
        );
        assert_eq!(
            SteamworksNetworkingSocketsCommand::flush_messages(connection_id()),
            SteamworksNetworkingSocketsCommand::FlushMessages {
                connection: connection_id(),
            }
        );
        assert_eq!(
            SteamworksNetworkingSocketsCommand::set_connection_poll_group(
                connection_id(),
                poll_group_id(),
            ),
            SteamworksNetworkingSocketsCommand::SetConnectionPollGroup {
                connection: connection_id(),
                poll_group: poll_group_id(),
            }
        );
        assert_eq!(
            SteamworksNetworkingSocketsCommand::clear_connection_poll_group(connection_id()),
            SteamworksNetworkingSocketsCommand::ClearConnectionPollGroup {
                connection: connection_id(),
            }
        );
        assert_eq!(
            SteamworksNetworkingSocketsCommand::configure_connection_lanes(
                connection_id(),
                vec![0, 10],
                vec![100, 20],
            ),
            SteamworksNetworkingSocketsCommand::ConfigureConnectionLanes {
                connection: connection_id(),
                lane_priorities: vec![0, 10],
                lane_weights: vec![100, 20],
            }
        );
        assert_eq!(
            SteamworksNetworkingSocketsCommand::set_connection_user_data(connection_id(), 11),
            SteamworksNetworkingSocketsCommand::SetConnectionUserData {
                connection: connection_id(),
                user_data: 11,
            }
        );
        assert_eq!(
            SteamworksNetworkingSocketsCommand::close_listen_socket(listen_socket_id()),
            SteamworksNetworkingSocketsCommand::CloseListenSocket {
                listen_socket: listen_socket_id(),
            }
        );
        assert_eq!(
            SteamworksNetworkingSocketsCommand::close_poll_group(poll_group_id()),
            SteamworksNetworkingSocketsCommand::ClosePollGroup {
                poll_group: poll_group_id(),
            }
        );
        assert_eq!(
            SteamworksNetworkingSocketsCommand::close_connection(connection_id()),
            SteamworksNetworkingSocketsCommand::CloseConnection {
                connection: connection_id(),
                reason: steamworks::networking_types::NetConnectionEnd::App(
                    steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
                ),
                debug: None,
                enable_linger: false,
            }
        );
    }

    #[test]
    fn debug_redacts_message_payload_bytes() {
        let peer = steamworks::networking_types::NetworkingIdentity::new_ip(localhost());
        let command = SteamworksNetworkingSocketsCommand::send_message(
            connection_id(),
            steamworks::networking_types::SendFlags::RELIABLE,
            vec![1, 2, 3],
        );
        let message = SteamworksNetworkingSocketsMessage {
            connection: connection_id(),
            peer: peer.clone(),
            data: vec![4, 5, 6],
            channel: 0,
            send_flags: steamworks::networking_types::SendFlags::RELIABLE,
            message_number: 1,
            connection_user_data: 0,
        };
        let poll_group_message = SteamworksNetworkingSocketsPollGroupMessage {
            poll_group: poll_group_id(),
            peer,
            data: vec![7, 8, 9],
            channel: 1,
            send_flags: steamworks::networking_types::SendFlags::UNRELIABLE,
            message_number: 2,
            connection_user_data: 99,
        };
        let operation = SteamworksNetworkingSocketsOperation::MessagesReceived {
            connection: connection_id(),
            messages: vec![message.clone()],
        };
        let result = SteamworksNetworkingSocketsResult::Ok(
            SteamworksNetworkingSocketsOperation::PollGroupMessagesReceived {
                poll_group: poll_group_id(),
                messages: vec![poll_group_message.clone()],
            },
        );

        let command_debug = format!("{command:?}");
        let message_debug = format!("{message:?}");
        let poll_group_debug = format!("{poll_group_message:?}");
        let operation_debug = format!("{operation:?}");
        let result_debug = format!("{result:?}");

        assert!(command_debug.contains("data_len: 3"));
        assert!(!command_debug.contains("[1, 2, 3]"));
        assert!(message_debug.contains("data_len: 3"));
        assert!(!message_debug.contains("[4, 5, 6]"));
        assert!(poll_group_debug.contains("data_len: 3"));
        assert!(!poll_group_debug.contains("[7, 8, 9]"));
        assert!(operation_debug.contains("data_len: 3"));
        assert!(!operation_debug.contains("[4, 5, 6]"));
        assert!(result_debug.contains("data_len: 3"));
        assert!(!result_debug.contains("[7, 8, 9]"));
    }

    #[test]
    fn state_records_operations_without_unbounded_message_history() {
        let mut state = SteamworksNetworkingSocketsState::default();
        let endpoint = SteamworksNetworkingSocketsListenEndpoint::Ip(localhost());
        let target = SteamworksNetworkingSocketsConnectionTarget::Ip(localhost());
        let peer = steamworks::networking_types::NetworkingIdentity::new_ip(localhost());
        let listen_event = SteamworksListenSocketEventInfo::ConnectionRejected {
            listen_socket: listen_socket_id(),
            remote: peer.clone(),
            user_data: 17,
        };
        let connection_event = SteamworksNetworkingSocketsConnectionEventInfo {
            connection: connection_id(),
            new_state: steamworks::networking_types::NetworkingConnectionState::ClosedByPeer,
            old_state: steamworks::networking_types::NetworkingConnectionState::Connecting,
        };
        let info = SteamworksNetworkingSocketsConnectionInfo {
            connection: connection_id(),
            state: steamworks::networking_types::NetworkingConnectionState::Connected,
            remote: Some(peer.clone()),
            user_data: 21,
            end_reason: None,
        };
        let realtime_status = SteamworksNetworkingSocketsRealtimeStatus {
            connection: connection_id(),
            connection_state: steamworks::networking_types::NetworkingConnectionState::Connected,
            ping: 42,
            connection_quality_local: 0.95,
            connection_quality_remote: 0.9,
            out_packets_per_sec: 10.0,
            out_bytes_per_sec: 1000.0,
            in_packets_per_sec: 11.0,
            in_bytes_per_sec: 1100.0,
            send_rate_bytes_per_sec: 1200,
            pending_unreliable: 1,
            pending_reliable: 2,
            sent_unacked_reliable: 3,
            queued_send_bytes: 4,
            lanes: vec![SteamworksNetworkingSocketsRealtimeLaneStatus {
                pending_unreliable: 5,
                pending_reliable: 6,
                sent_unacked_reliable: 7,
                queued_send_bytes: 8,
            }],
        };

        state.record_operation(
            &SteamworksNetworkingSocketsOperation::AuthenticationInitialized {
                availability: Ok(steamworks::networking_types::NetworkingAvailability::Attempting),
            },
        );
        state.record_operation(
            &SteamworksNetworkingSocketsOperation::AuthenticationStatusRead {
                availability: Ok(steamworks::networking_types::NetworkingAvailability::Current),
            },
        );
        state.record_operation(&SteamworksNetworkingSocketsOperation::ListenSocketCreated {
            listen_socket: listen_socket_id(),
            endpoint: endpoint.clone(),
        });
        state.record_operation(&SteamworksNetworkingSocketsOperation::ConnectionCreated {
            connection: connection_id(),
            target: target.clone(),
        });
        state.record_operation(&SteamworksNetworkingSocketsOperation::PollGroupCreated {
            poll_group: poll_group_id(),
        });
        state.record_operation(
            &SteamworksNetworkingSocketsOperation::ListenSocketEventsPolled {
                listen_socket: listen_socket_id(),
                events: vec![listen_event.clone()],
            },
        );
        state.record_operation(
            &SteamworksNetworkingSocketsOperation::ConnectionEventsPolled {
                connection: connection_id(),
                events: vec![connection_event.clone()],
                connection_removed: true,
            },
        );
        state.record_operation(&SteamworksNetworkingSocketsOperation::ConnectionInfoRead {
            info: info.clone(),
        });
        state.record_operation(
            &SteamworksNetworkingSocketsOperation::RealtimeConnectionStatusRead {
                status: realtime_status.clone(),
            },
        );
        let first = SteamworksNetworkingSocketsMessage {
            connection: connection_id(),
            peer: peer.clone(),
            data: vec![1],
            channel: 0,
            send_flags: steamworks::networking_types::SendFlags::RELIABLE,
            message_number: 1,
            connection_user_data: 0,
        };
        let second = SteamworksNetworkingSocketsMessage {
            data: vec![2, 3],
            message_number: 2,
            ..first.clone()
        };

        state.record_operation(&SteamworksNetworkingSocketsOperation::MessagesReceived {
            connection: connection_id(),
            messages: vec![first],
        });
        state.record_operation(&SteamworksNetworkingSocketsOperation::MessagesReceived {
            connection: connection_id(),
            messages: vec![second.clone()],
        });
        let poll_group_message = SteamworksNetworkingSocketsPollGroupMessage {
            poll_group: poll_group_id(),
            peer,
            data: vec![9],
            channel: 1,
            send_flags: steamworks::networking_types::SendFlags::UNRELIABLE,
            message_number: 4,
            connection_user_data: 99,
        };
        state.record_operation(
            &SteamworksNetworkingSocketsOperation::PollGroupMessagesReceived {
                poll_group: poll_group_id(),
                messages: vec![poll_group_message.clone()],
            },
        );
        state.record_operation(&SteamworksNetworkingSocketsOperation::MessageSent {
            connection: connection_id(),
            message_number: 3,
            bytes: 2,
        });
        state.record_operation(&SteamworksNetworkingSocketsOperation::MessagesFlushed {
            connection: connection_id(),
        });
        state.record_operation(
            &SteamworksNetworkingSocketsOperation::ConnectionPollGroupSet {
                connection: connection_id(),
                poll_group: poll_group_id(),
            },
        );
        state.record_operation(
            &SteamworksNetworkingSocketsOperation::ConnectionPollGroupCleared {
                connection: connection_id(),
            },
        );
        state.record_operation(
            &SteamworksNetworkingSocketsOperation::ConnectionLanesConfigured {
                connection: connection_id(),
                lanes: 2,
            },
        );
        state.record_operation(
            &SteamworksNetworkingSocketsOperation::ConnectionUserDataSet {
                connection: connection_id(),
                user_data: 123,
            },
        );
        state.record_operation(&SteamworksNetworkingSocketsOperation::ConnectionClosed {
            connection: connection_id(),
            close_succeeded: false,
        });
        state.record_operation(&SteamworksNetworkingSocketsOperation::ListenSocketClosed {
            listen_socket: listen_socket_id(),
            closed_connections: vec![connection_id()],
        });
        state.record_operation(&SteamworksNetworkingSocketsOperation::PollGroupClosed {
            poll_group: poll_group_id(),
        });

        assert_eq!(
            state.last_authentication_status(),
            Some(&Ok(
                steamworks::networking_types::NetworkingAvailability::Current
            ))
        );
        assert_eq!(
            state.last_created_listen_socket(),
            Some(&SteamworksNetworkingSocketsListenSocketCreated {
                listen_socket: listen_socket_id(),
                endpoint,
            })
        );
        assert_eq!(
            state.last_created_connection(),
            Some(&SteamworksNetworkingSocketsConnectionCreated {
                connection: connection_id(),
                target,
            })
        );
        assert_eq!(state.last_created_poll_group(), Some(poll_group_id()));
        assert_eq!(
            state.last_listen_socket_events(),
            Some(&SteamworksNetworkingSocketsListenSocketEvents {
                listen_socket: listen_socket_id(),
                events: vec![listen_event],
            })
        );
        assert_eq!(
            state.last_connection_events(),
            Some(&SteamworksNetworkingSocketsConnectionEvents {
                connection: connection_id(),
                events: vec![connection_event],
                connection_removed: true,
            })
        );
        assert_eq!(state.last_connection_info(), Some(&info));
        assert_eq!(state.last_realtime_status(), Some(&realtime_status));
        assert_eq!(state.received_count(), 3);
        assert_eq!(state.sent_count(), 1);
        assert_eq!(
            state.last_sent_message(),
            Some(&SteamworksNetworkingSocketsSentMessage {
                connection: connection_id(),
                message_number: 3,
                bytes: 2,
            })
        );
        assert_eq!(state.last_received_messages(), &[second]);
        assert_eq!(state.last_poll_group_messages(), &[poll_group_message]);
        assert_eq!(state.last_flushed_connection(), Some(connection_id()));
        assert_eq!(
            state.last_connection_poll_group_set(),
            Some(&SteamworksNetworkingSocketsPollGroupAssignment {
                connection: connection_id(),
                poll_group: poll_group_id(),
            })
        );
        assert_eq!(
            state.last_connection_poll_group_cleared(),
            Some(connection_id())
        );
        assert_eq!(
            state.last_connection_lanes_configured(),
            Some(&SteamworksNetworkingSocketsLaneConfiguration {
                connection: connection_id(),
                lanes: 2,
            })
        );
        assert_eq!(
            state.last_connection_user_data(),
            Some(&SteamworksNetworkingSocketsConnectionUserData {
                connection: connection_id(),
                user_data: 123,
            })
        );
        assert_eq!(
            state.last_closed_connection(),
            Some(&SteamworksNetworkingSocketsConnectionClosed {
                connection: connection_id(),
                close_succeeded: false,
            })
        );
        assert_eq!(
            state.last_closed_listen_socket(),
            Some(&SteamworksNetworkingSocketsListenSocketClosed {
                listen_socket: listen_socket_id(),
                closed_connections: vec![connection_id()],
            })
        );
        assert_eq!(state.last_closed_poll_group(), Some(poll_group_id()));
    }

    #[test]
    fn id_round_trips_raw_values() {
        assert_eq!(SteamworksListenSocketId::from_raw(5).raw(), 5);
        assert_eq!(
            SteamworksNetworkingSocketsConnectionId::from_raw(6).raw(),
            6
        );
        assert_eq!(SteamworksNetworkingSocketsPollGroupId::from_raw(8).raw(), 8);
    }

    #[test]
    fn handle_storage_starts_ids_at_one() {
        let storage = SteamworksNetworkingSocketsHandleStorage::default();

        assert_eq!(storage.next_listen_socket_id, 1);
        assert_eq!(storage.next_connection_id, 1);
        assert_eq!(storage.next_poll_group_id, 1);
    }

    #[test]
    fn connection_metadata_tracks_poll_group_membership() {
        let mut storage = SteamworksNetworkingSocketsHandleStorage::default();

        storage.connection_metadata.insert(
            connection_id(),
            SteamworksNetworkingSocketsConnectionMetadata::independent(),
        );

        storage.set_connection_poll_group(connection_id(), poll_group_id());
        assert_eq!(
            storage
                .connection_metadata
                .get(&connection_id())
                .and_then(|metadata| metadata.poll_group),
            Some(poll_group_id())
        );

        storage.clear_connection_poll_group(connection_id());
        assert_eq!(
            storage
                .connection_metadata
                .get(&connection_id())
                .and_then(|metadata| metadata.poll_group),
            None
        );
    }

    #[test]
    fn poll_group_metadata_cleanup_clears_all_matching_connections() {
        let mut storage = SteamworksNetworkingSocketsHandleStorage::default();
        let first = SteamworksNetworkingSocketsConnectionId::from_raw(1);
        let second = SteamworksNetworkingSocketsConnectionId::from_raw(2);
        let other = SteamworksNetworkingSocketsConnectionId::from_raw(3);
        let other_poll_group = SteamworksNetworkingSocketsPollGroupId::from_raw(10);

        for connection in [first, second, other] {
            storage.connection_metadata.insert(
                connection,
                SteamworksNetworkingSocketsConnectionMetadata::independent(),
            );
        }
        storage.set_connection_poll_group(first, poll_group_id());
        storage.set_connection_poll_group(second, poll_group_id());
        storage.set_connection_poll_group(other, other_poll_group);

        assert_eq!(storage.clear_poll_group_metadata(poll_group_id()), 2);
        assert_eq!(
            storage
                .connection_metadata
                .get(&first)
                .and_then(|metadata| metadata.poll_group),
            None
        );
        assert_eq!(
            storage
                .connection_metadata
                .get(&second)
                .and_then(|metadata| metadata.poll_group),
            None
        );
        assert_eq!(
            storage
                .connection_metadata
                .get(&other)
                .and_then(|metadata| metadata.poll_group),
            Some(other_poll_group)
        );
    }

    #[test]
    fn missing_poll_group_remove_does_not_clear_connection_metadata() {
        let mut storage = SteamworksNetworkingSocketsHandleStorage::default();

        storage.connection_metadata.insert(
            connection_id(),
            SteamworksNetworkingSocketsConnectionMetadata::independent(),
        );
        storage.set_connection_poll_group(connection_id(), poll_group_id());

        assert!(storage.remove_poll_group(&poll_group_id()).is_none());
        assert_eq!(
            storage
                .connection_metadata
                .get(&connection_id())
                .and_then(|metadata| metadata.poll_group),
            Some(poll_group_id())
        );
    }

    #[test]
    fn connection_metadata_matches_listen_socket_disconnects() {
        let mut storage = SteamworksNetworkingSocketsHandleStorage::default();
        let remote = steamworks::networking_types::NetworkingIdentity::new_ip(localhost());
        let end_reason = steamworks::networking_types::NetConnectionEnd::App(
            steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
        );
        let first = SteamworksNetworkingSocketsConnectionId::from_raw(1);
        let second = SteamworksNetworkingSocketsConnectionId::from_raw(2);

        storage.connection_metadata.insert(
            first,
            SteamworksNetworkingSocketsConnectionMetadata::listen_socket(
                listen_socket_id(),
                remote.clone(),
                10,
            ),
        );
        storage.connection_metadata.insert(
            second,
            SteamworksNetworkingSocketsConnectionMetadata::listen_socket(
                listen_socket_id(),
                remote.clone(),
                20,
            ),
        );

        assert_eq!(
            storage.remove_listen_connection_by_event(listen_socket_id(), &remote, 20, end_reason),
            Some(second)
        );
        assert!(storage.connection_metadata.contains_key(&first));
        assert!(!storage.connection_metadata.contains_key(&second));
    }

    #[test]
    fn duplicate_listen_socket_disconnect_metadata_is_not_removed_without_terminal_info() {
        let mut storage = SteamworksNetworkingSocketsHandleStorage::default();
        let remote = steamworks::networking_types::NetworkingIdentity::new_ip(localhost());
        let end_reason = steamworks::networking_types::NetConnectionEnd::App(
            steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
        );
        let first = SteamworksNetworkingSocketsConnectionId::from_raw(1);
        let second = SteamworksNetworkingSocketsConnectionId::from_raw(2);

        for connection in [first, second] {
            storage.connection_metadata.insert(
                connection,
                SteamworksNetworkingSocketsConnectionMetadata::listen_socket(
                    listen_socket_id(),
                    remote.clone(),
                    10,
                ),
            );
        }

        assert_eq!(
            storage.remove_listen_connection_by_event(listen_socket_id(), &remote, 10, end_reason),
            None
        );
        assert!(storage.connection_metadata.contains_key(&first));
        assert!(storage.connection_metadata.contains_key(&second));
    }

    #[test]
    fn p2p_connect_constructor_accepts_steam_id() {
        let command = SteamworksNetworkingSocketsCommand::connect_p2p_steam_id(
            steamworks::SteamId::from_raw(123),
            0,
        );

        let SteamworksNetworkingSocketsCommand::ConnectP2p {
            identity,
            remote_virtual_port,
        } = command
        else {
            panic!("expected ConnectP2p command");
        };

        assert_eq!(identity.debug_string(), "steamid:123");
        assert_eq!(remote_virtual_port, 0);
    }

    #[test]
    fn request_policy_default_rejects() {
        assert!(matches!(
            SteamworksConnectionRequestPolicy::default(),
            SteamworksConnectionRequestPolicy::Reject { .. }
        ));
    }

    #[test]
    fn socket_addr_from_str_is_tested_for_ipv6_coverage() {
        assert!(SocketAddr::from_str("[::1]:27015").is_ok());
    }
}
