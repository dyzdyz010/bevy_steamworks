use crate::networking_sockets::*;

impl SteamworksNetworkingSocketsState {
    /// Returns the most recent batch of received messages.
    pub fn last_received_messages(&self) -> &[SteamworksNetworkingSocketsMessage] {
        &self.last_received_messages
    }

    /// Returns the number of messages in the most recent connection receive batch.
    pub fn last_received_message_count(&self) -> usize {
        self.last_received_messages.len()
    }

    /// Returns bounded received message snapshots in observation order.
    pub fn recent_received_messages(&self) -> &[SteamworksNetworkingSocketsMessage] {
        &self.recent_received_messages
    }

    /// Returns the number of bounded received message snapshots retained across batches.
    pub fn recent_received_message_count(&self) -> usize {
        self.recent_received_messages.len()
    }

    /// Returns bounded received message snapshots for one connection.
    pub fn received_messages_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> impl Iterator<Item = &SteamworksNetworkingSocketsMessage> + '_ {
        self.recent_received_messages
            .iter()
            .filter(move |message| message.connection == connection)
    }

    /// Returns the number of bounded received message snapshots for one connection.
    pub fn received_message_count_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> usize {
        self.received_messages_for_connection(connection).count()
    }

    /// Returns the most recent received message for one connection.
    pub fn last_received_message_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<&SteamworksNetworkingSocketsMessage> {
        self.recent_received_messages
            .iter()
            .rev()
            .find(|message| message.connection == connection)
    }

    /// Returns the byte count for the most recent received message for one connection.
    pub fn last_received_message_bytes_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<usize> {
        self.last_received_message_for_connection(connection)
            .map(|message| message.data.len())
    }

    /// Returns the channel for the most recent received message for one connection.
    pub fn last_received_message_channel_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<i32> {
        self.last_received_message_for_connection(connection)
            .map(|message| message.channel)
    }

    /// Returns the payload bytes for the most recent received message for one connection.
    pub fn last_received_message_data_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<&[u8]> {
        self.last_received_message_for_connection(connection)
            .map(|message| message.data.as_slice())
    }
}
