use crate::networking_sockets::*;

impl SteamworksNetworkingSocketsState {
    /// Returns the most recent batch of messages received from a poll group.
    pub fn last_poll_group_messages(&self) -> &[SteamworksNetworkingSocketsPollGroupMessage] {
        &self.last_poll_group_messages
    }

    /// Returns the number of messages in the most recent poll-group receive batch.
    pub fn last_poll_group_message_count(&self) -> usize {
        self.last_poll_group_messages.len()
    }

    /// Returns bounded poll-group message snapshots in observation order.
    pub fn recent_poll_group_messages(&self) -> &[SteamworksNetworkingSocketsPollGroupMessage] {
        &self.recent_poll_group_messages
    }

    /// Returns the number of bounded poll-group message snapshots retained across batches.
    pub fn recent_poll_group_message_count(&self) -> usize {
        self.recent_poll_group_messages.len()
    }

    /// Returns bounded poll-group message snapshots for one poll group.
    pub fn poll_group_messages(
        &self,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) -> impl Iterator<Item = &SteamworksNetworkingSocketsPollGroupMessage> + '_ {
        self.recent_poll_group_messages
            .iter()
            .filter(move |message| message.poll_group == poll_group)
    }

    /// Returns the number of bounded poll-group message snapshots for one poll group.
    pub fn poll_group_message_count(
        &self,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) -> usize {
        self.poll_group_messages(poll_group).count()
    }

    /// Returns the most recent message received from one poll group.
    pub fn last_poll_group_message(
        &self,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) -> Option<&SteamworksNetworkingSocketsPollGroupMessage> {
        self.recent_poll_group_messages
            .iter()
            .rev()
            .find(|message| message.poll_group == poll_group)
    }

    /// Returns the byte count for the most recent message received from one poll group.
    pub fn last_poll_group_message_bytes(
        &self,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) -> Option<usize> {
        self.last_poll_group_message(poll_group)
            .map(|message| message.data.len())
    }

    /// Returns the channel for the most recent message received from one poll group.
    pub fn last_poll_group_message_channel(
        &self,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) -> Option<i32> {
        self.last_poll_group_message(poll_group)
            .map(|message| message.channel)
    }

    /// Returns the payload bytes for the most recent message received from one poll group.
    pub fn last_poll_group_message_data(
        &self,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) -> Option<&[u8]> {
        self.last_poll_group_message(poll_group)
            .map(|message| message.data.as_slice())
    }
}
