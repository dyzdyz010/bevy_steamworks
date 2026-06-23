use crate::networking_sockets::*;

impl SteamworksNetworkingSocketsState {
    /// Returns the most recent sent-message snapshot.
    pub fn last_sent_message(&self) -> Option<&SteamworksNetworkingSocketsSentMessage> {
        self.last_sent_message.as_ref()
    }

    /// Returns the connection for the most recent single-message send.
    pub fn last_sent_message_connection(&self) -> Option<SteamworksNetworkingSocketsConnectionId> {
        self.last_sent_message().map(|message| message.connection)
    }

    /// Returns the message number for the most recent single-message send.
    pub fn last_sent_message_number(&self) -> Option<u64> {
        self.last_sent_message()
            .map(|message| message.message_number)
    }

    /// Returns the byte count for the most recent single-message send.
    pub fn last_sent_message_bytes(&self) -> Option<usize> {
        self.last_sent_message().map(|message| message.bytes)
    }

    /// Returns the most recent batch send outcomes.
    pub fn last_sent_messages(&self) -> &[SteamworksNetworkingSocketsMessageSendResult] {
        &self.last_sent_messages
    }

    /// Returns the number of outcomes in the most recent batch send.
    pub fn last_sent_message_count(&self) -> usize {
        self.last_sent_messages.len()
    }

    /// Returns bounded batch-send outcomes in observation order.
    pub fn recent_sent_messages(&self) -> &[SteamworksNetworkingSocketsMessageSendResult] {
        &self.recent_sent_messages
    }

    /// Returns the number of bounded batch-send outcomes retained across batches.
    pub fn recent_sent_message_count(&self) -> usize {
        self.recent_sent_messages.len()
    }

    /// Returns bounded batch-send outcomes for one connection.
    pub fn sent_messages_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> impl Iterator<Item = &SteamworksNetworkingSocketsMessageSendResult> + '_ {
        self.recent_sent_messages
            .iter()
            .filter(move |message| message.connection == connection)
    }

    /// Returns the number of bounded batch-send outcomes for one connection.
    pub fn sent_message_count_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> usize {
        self.sent_messages_for_connection(connection).count()
    }

    /// Returns the most recent batch-send outcome for one connection.
    pub fn last_sent_message_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<&SteamworksNetworkingSocketsMessageSendResult> {
        self.recent_sent_messages
            .iter()
            .rev()
            .find(|message| message.connection == connection)
    }

    /// Returns the Steam message number from the most recent successful batch-send outcome for one connection.
    pub fn last_sent_message_number_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<u64> {
        self.last_sent_message_for_connection(connection)
            .and_then(|message| message.result.ok())
    }

    /// Returns whether the most recent batch-send outcome for one connection succeeded.
    pub fn last_sent_message_succeeded_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<bool> {
        self.last_sent_message_for_connection(connection)
            .map(|message| message.result.is_ok())
    }

    /// Returns the byte count from the most recent batch-send outcome for one connection.
    pub fn last_sent_message_bytes_for_connection(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<usize> {
        self.last_sent_message_for_connection(connection)
            .map(|message| message.bytes)
    }
}
