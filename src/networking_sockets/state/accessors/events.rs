use crate::networking_sockets::*;

impl SteamworksNetworkingSocketsState {
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
}
