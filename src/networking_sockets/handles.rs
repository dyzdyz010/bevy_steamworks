use std::{collections::HashMap, sync::Mutex};

use bevy_ecs::prelude::Resource;

use super::{
    SteamworksListenSocketId, SteamworksNetworkingSocketsConnectionId,
    SteamworksNetworkingSocketsPollGroupId,
};

#[derive(Default, Resource)]
pub(super) struct SteamworksNetworkingSocketsHandles {
    pub(super) storage: Mutex<SteamworksNetworkingSocketsHandleStorage>,
}

pub(super) struct SteamworksNetworkingSocketsHandleStorage {
    pub(super) next_listen_socket_id: u64,
    pub(super) next_connection_id: u64,
    pub(super) next_poll_group_id: u64,
    pub(super) listen_sockets:
        HashMap<SteamworksListenSocketId, steamworks::networking_sockets::ListenSocket>,
    pub(super) connections: HashMap<
        SteamworksNetworkingSocketsConnectionId,
        steamworks::networking_sockets::NetConnection,
    >,
    pub(super) poll_groups: HashMap<
        SteamworksNetworkingSocketsPollGroupId,
        steamworks::networking_sockets::NetPollGroup,
    >,
    pub(super) connection_metadata: HashMap<
        SteamworksNetworkingSocketsConnectionId,
        SteamworksNetworkingSocketsConnectionMetadata,
    >,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SteamworksNetworkingSocketsConnectionMetadata {
    pub(super) listen_socket: Option<SteamworksListenSocketId>,
    pub(super) poll_group: Option<SteamworksNetworkingSocketsPollGroupId>,
    pub(super) remote: Option<steamworks::networking_types::NetworkingIdentity>,
    pub(super) user_data: i64,
}

impl SteamworksNetworkingSocketsConnectionMetadata {
    pub(super) fn independent() -> Self {
        Self {
            listen_socket: None,
            poll_group: None,
            remote: None,
            user_data: 0,
        }
    }

    pub(super) fn listen_socket(
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
    pub(super) fn insert_listen_socket(
        &mut self,
        socket: steamworks::networking_sockets::ListenSocket,
    ) -> SteamworksListenSocketId {
        let id = SteamworksListenSocketId::from_raw(self.next_listen_socket_id);
        self.next_listen_socket_id = self.next_listen_socket_id.saturating_add(1).max(1);
        self.listen_sockets.insert(id, socket);
        id
    }

    pub(super) fn insert_connection(
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

    pub(super) fn insert_poll_group(
        &mut self,
        poll_group: steamworks::networking_sockets::NetPollGroup,
    ) -> SteamworksNetworkingSocketsPollGroupId {
        let id = SteamworksNetworkingSocketsPollGroupId::from_raw(self.next_poll_group_id);
        self.next_poll_group_id = self.next_poll_group_id.saturating_add(1).max(1);
        self.poll_groups.insert(id, poll_group);
        id
    }

    pub(super) fn remove_connection(
        &mut self,
        connection: &SteamworksNetworkingSocketsConnectionId,
    ) -> Option<steamworks::networking_sockets::NetConnection> {
        self.connection_metadata.remove(connection);
        self.connections.remove(connection)
    }

    pub(super) fn remove_poll_group(
        &mut self,
        poll_group: &SteamworksNetworkingSocketsPollGroupId,
    ) -> Option<steamworks::networking_sockets::NetPollGroup> {
        let removed = self.poll_groups.remove(poll_group)?;
        self.clear_poll_group_metadata(*poll_group);
        Some(removed)
    }

    pub(super) fn clear_poll_group_metadata(
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

    pub(super) fn remove_connections_for_listen_socket(
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

    pub(super) fn remove_listen_connection_by_event(
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

    pub(super) fn update_connection_user_data(
        &mut self,
        connection: SteamworksNetworkingSocketsConnectionId,
        user_data: i64,
    ) {
        if let Some(metadata) = self.connection_metadata.get_mut(&connection) {
            metadata.user_data = user_data;
        }
    }

    pub(super) fn clear_connection_poll_group(
        &mut self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) {
        if let Some(metadata) = self.connection_metadata.get_mut(&connection) {
            metadata.poll_group = None;
        }
    }

    pub(super) fn set_connection_poll_group(
        &mut self,
        connection: SteamworksNetworkingSocketsConnectionId,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) {
        if let Some(metadata) = self.connection_metadata.get_mut(&connection) {
            metadata.poll_group = Some(poll_group);
        }
    }
}

pub(super) fn is_terminal_connection_state(
    state: steamworks::networking_types::NetworkingConnectionState,
) -> bool {
    matches!(
        state,
        steamworks::networking_types::NetworkingConnectionState::ClosedByPeer
            | steamworks::networking_types::NetworkingConnectionState::ProblemDetectedLocally
    )
}
