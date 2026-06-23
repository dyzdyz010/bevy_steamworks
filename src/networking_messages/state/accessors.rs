use super::SteamworksNetworkingMessagesState;
use crate::networking_messages::{
    SteamworksNetworkingMessage, SteamworksNetworkingMessagesConnectionInfo,
    SteamworksNetworkingMessagesError, SteamworksNetworkingMessagesSessionRequestInfo,
};

impl SteamworksNetworkingMessagesState {
    /// Returns the most recent synchronous command error observed by the plugin.
    pub fn last_error(&self) -> Option<&SteamworksNetworkingMessagesError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent batch of received messages.
    pub fn received_messages(&self) -> &[SteamworksNetworkingMessage] {
        &self.received_messages
    }

    /// Returns bounded received message snapshots in observation order across batches.
    pub fn recent_received_messages(&self) -> &[SteamworksNetworkingMessage] {
        &self.recent_received_messages
    }

    /// Returns the most recent received message from the latest receive batch.
    pub fn last_received_message(&self) -> Option<&SteamworksNetworkingMessage> {
        self.received_messages.last()
    }

    /// Returns received messages from the latest receive batch on one channel.
    pub fn received_messages_on_channel(
        &self,
        channel: i32,
    ) -> impl Iterator<Item = &SteamworksNetworkingMessage> + '_ {
        self.received_messages
            .iter()
            .filter(move |message| message.channel == channel)
    }

    /// Returns bounded received message snapshots on one channel across batches.
    pub fn recent_received_messages_on_channel(
        &self,
        channel: i32,
    ) -> impl Iterator<Item = &SteamworksNetworkingMessage> + '_ {
        self.recent_received_messages
            .iter()
            .filter(move |message| message.channel == channel)
    }

    /// Returns received messages from the latest receive batch sent by one peer.
    pub fn received_messages_from_peer<'a>(
        &'a self,
        peer: &'a steamworks::networking_types::NetworkingIdentity,
    ) -> impl Iterator<Item = &'a SteamworksNetworkingMessage> + 'a {
        self.received_messages
            .iter()
            .filter(move |message| &message.peer == peer)
    }

    /// Returns bounded received message snapshots from one peer across batches.
    pub fn recent_received_messages_from_peer<'a>(
        &'a self,
        peer: &'a steamworks::networking_types::NetworkingIdentity,
    ) -> impl Iterator<Item = &'a SteamworksNetworkingMessage> + 'a {
        self.recent_received_messages
            .iter()
            .filter(move |message| &message.peer == peer)
    }

    /// Returns the most recent message from one peer in the latest receive batch.
    pub fn last_received_message_from_peer(
        &self,
        peer: &steamworks::networking_types::NetworkingIdentity,
    ) -> Option<&SteamworksNetworkingMessage> {
        self.received_messages
            .iter()
            .rev()
            .find(|message| &message.peer == peer)
    }

    /// Returns the most recent received message from one peer across retained batches.
    pub fn last_recent_received_message_from_peer(
        &self,
        peer: &steamworks::networking_types::NetworkingIdentity,
    ) -> Option<&SteamworksNetworkingMessage> {
        self.recent_received_messages
            .iter()
            .rev()
            .find(|message| &message.peer == peer)
    }

    /// Returns the most recent connection info snapshot read through the plugin.
    pub fn last_connection_info(&self) -> Option<&SteamworksNetworkingMessagesConnectionInfo> {
        self.last_connection_info.as_ref()
    }

    /// Returns bounded incoming session request callback snapshots.
    pub fn session_requests(&self) -> &[SteamworksNetworkingMessagesSessionRequestInfo] {
        &self.session_requests
    }

    /// Returns the most recent incoming session request observed by the callback.
    pub fn last_session_request(&self) -> Option<&SteamworksNetworkingMessagesSessionRequestInfo> {
        self.last_session_request.as_ref()
    }

    /// Returns the most recent session request from one remote peer.
    pub fn session_request(
        &self,
        remote: &steamworks::networking_types::NetworkingIdentity,
    ) -> Option<&SteamworksNetworkingMessagesSessionRequestInfo> {
        self.session_requests
            .iter()
            .rev()
            .find(|request| &request.remote == remote)
    }

    /// Returns bounded session failure callback snapshots.
    pub fn session_failures(&self) -> &[SteamworksNetworkingMessagesConnectionInfo] {
        &self.session_failures
    }

    /// Returns the most recent session failure observed by the callback.
    pub fn last_session_failure(&self) -> Option<&SteamworksNetworkingMessagesConnectionInfo> {
        self.last_session_failure.as_ref()
    }

    /// Returns the most recent session failure for one remote peer.
    pub fn session_failure(
        &self,
        remote: &steamworks::networking_types::NetworkingIdentity,
    ) -> Option<&SteamworksNetworkingMessagesConnectionInfo> {
        self.session_failures
            .iter()
            .rev()
            .find(|failure| failure.remote.as_ref() == Some(remote))
    }

    /// Returns whether incoming session requests are currently auto-accepted.
    pub fn auto_accept_session_requests(&self) -> bool {
        *self
            .auto_accept_session_requests
            .lock()
            .expect("Steamworks Networking Messages policy mutex was poisoned")
    }

    /// Returns the number of successful send commands observed through the plugin.
    pub fn sent_count(&self) -> u64 {
        self.sent_count
    }

    /// Returns the number of received messages observed through the plugin.
    pub fn received_count(&self) -> u64 {
        self.received_count
    }

    /// Returns the number of incoming session requests observed by the plugin.
    pub fn session_request_count(&self) -> u64 {
        self.session_request_count
    }

    /// Returns the number of session failures observed by the plugin.
    pub fn session_failure_count(&self) -> u64 {
        self.session_failure_count
    }
}
