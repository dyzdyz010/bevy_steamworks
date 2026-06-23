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

    /// Returns the number of messages in the most recent receive batch.
    pub fn received_message_count(&self) -> usize {
        self.received_messages.len()
    }

    /// Returns bounded received message snapshots in observation order across batches.
    pub fn recent_received_messages(&self) -> &[SteamworksNetworkingMessage] {
        &self.recent_received_messages
    }

    /// Returns the number of bounded received message snapshots retained across batches.
    pub fn recent_received_message_count(&self) -> usize {
        self.recent_received_messages.len()
    }

    /// Returns the most recent received message from the latest receive batch.
    pub fn last_received_message(&self) -> Option<&SteamworksNetworkingMessage> {
        self.received_messages.last()
    }

    /// Returns the peer for the most recent received message from the latest receive batch.
    pub fn last_received_message_peer(
        &self,
    ) -> Option<&steamworks::networking_types::NetworkingIdentity> {
        self.last_received_message().map(|message| &message.peer)
    }

    /// Returns the channel for the most recent received message from the latest receive batch.
    pub fn last_received_message_channel(&self) -> Option<i32> {
        self.last_received_message().map(|message| message.channel)
    }

    /// Returns the payload byte count for the most recent received message from the latest receive batch.
    pub fn last_received_message_bytes(&self) -> Option<usize> {
        self.last_received_message()
            .map(|message| message.data.len())
    }

    /// Returns the payload bytes for the most recent received message from the latest receive batch.
    pub fn last_received_message_data(&self) -> Option<&[u8]> {
        self.last_received_message()
            .map(|message| message.data.as_slice())
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

    /// Returns the number of messages in the latest receive batch on one channel.
    pub fn received_message_count_on_channel(&self, channel: i32) -> usize {
        self.received_messages_on_channel(channel).count()
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

    /// Returns the number of retained received messages on one channel across batches.
    pub fn recent_received_message_count_on_channel(&self, channel: i32) -> usize {
        self.recent_received_messages_on_channel(channel).count()
    }

    /// Returns the most recent received message on one channel from the latest receive batch.
    pub fn last_received_message_on_channel(
        &self,
        channel: i32,
    ) -> Option<&SteamworksNetworkingMessage> {
        self.received_messages
            .iter()
            .rev()
            .find(|message| message.channel == channel)
    }

    /// Returns the most recent retained received message on one channel across batches.
    pub fn last_recent_received_message_on_channel(
        &self,
        channel: i32,
    ) -> Option<&SteamworksNetworkingMessage> {
        self.recent_received_messages
            .iter()
            .rev()
            .find(|message| message.channel == channel)
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

    /// Returns the number of messages in the latest receive batch from one peer.
    pub fn received_message_count_from_peer(
        &self,
        peer: &steamworks::networking_types::NetworkingIdentity,
    ) -> usize {
        self.received_messages_from_peer(peer).count()
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

    /// Returns the number of retained received messages from one peer across batches.
    pub fn recent_received_message_count_from_peer(
        &self,
        peer: &steamworks::networking_types::NetworkingIdentity,
    ) -> usize {
        self.recent_received_messages_from_peer(peer).count()
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

    /// Returns the payload byte count for the most recent retained message from one peer.
    pub fn last_recent_received_message_bytes_from_peer(
        &self,
        peer: &steamworks::networking_types::NetworkingIdentity,
    ) -> Option<usize> {
        self.last_recent_received_message_from_peer(peer)
            .map(|message| message.data.len())
    }

    /// Returns the most recent connection info snapshot read through the plugin.
    pub fn last_connection_info(&self) -> Option<&SteamworksNetworkingMessagesConnectionInfo> {
        self.last_connection_info.as_ref()
    }

    /// Returns the state from the most recent connection info snapshot.
    pub fn last_connection_state(
        &self,
    ) -> Option<steamworks::networking_types::NetworkingConnectionState> {
        self.last_connection_info().map(|info| info.state)
    }

    /// Returns the remote peer from the most recent connection info snapshot, preserving a read with no remote as `Some(None)`.
    pub fn last_connection_remote(
        &self,
    ) -> Option<Option<&steamworks::networking_types::NetworkingIdentity>> {
        self.last_connection_info().map(|info| info.remote.as_ref())
    }

    /// Returns user data from the most recent connection info snapshot, preserving a read with no value as `Some(None)`.
    pub fn last_connection_user_data(&self) -> Option<Option<i64>> {
        self.last_connection_info().map(|info| info.user_data)
    }

    /// Returns the end reason from the most recent connection info snapshot, preserving a read with no value as `Some(None)`.
    pub fn last_connection_end_reason(
        &self,
    ) -> Option<Option<steamworks::networking_types::NetConnectionEnd>> {
        self.last_connection_info().map(|info| info.end_reason)
    }

    /// Returns the estimated ping from the most recent realtime connection info snapshot.
    pub fn last_connection_ping(&self) -> Option<i32> {
        self.last_connection_info()
            .and_then(|info| info.realtime.as_ref().map(|realtime| realtime.ping))
    }

    /// Returns local and remote quality from the most recent realtime connection info snapshot.
    pub fn last_connection_quality(&self) -> Option<(f32, f32)> {
        self.last_connection_info().and_then(|info| {
            info.realtime.as_ref().map(|realtime| {
                (
                    realtime.connection_quality_local,
                    realtime.connection_quality_remote,
                )
            })
        })
    }

    /// Returns bounded incoming session request callback snapshots.
    pub fn session_requests(&self) -> &[SteamworksNetworkingMessagesSessionRequestInfo] {
        &self.session_requests
    }

    /// Returns the number of cached incoming session request callback snapshots.
    pub fn cached_session_request_count(&self) -> usize {
        self.session_requests.len()
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

    /// Returns whether a session request callback snapshot is cached for one remote peer.
    pub fn has_session_request(
        &self,
        remote: &steamworks::networking_types::NetworkingIdentity,
    ) -> bool {
        self.session_request(remote).is_some()
    }

    /// Returns whether the latest cached session request for a remote peer was accepted.
    pub fn session_request_accepted(
        &self,
        remote: &steamworks::networking_types::NetworkingIdentity,
    ) -> Option<bool> {
        self.session_request(remote).map(|request| request.accepted)
    }

    /// Returns bounded session failure callback snapshots.
    pub fn session_failures(&self) -> &[SteamworksNetworkingMessagesConnectionInfo] {
        &self.session_failures
    }

    /// Returns the number of cached session failure callback snapshots.
    pub fn cached_session_failure_count(&self) -> usize {
        self.session_failures.len()
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

    /// Returns whether a session failure callback snapshot is cached for one remote peer.
    pub fn has_session_failure(
        &self,
        remote: &steamworks::networking_types::NetworkingIdentity,
    ) -> bool {
        self.session_failure(remote).is_some()
    }

    /// Returns the latest cached session failure state for one remote peer.
    pub fn session_failure_state(
        &self,
        remote: &steamworks::networking_types::NetworkingIdentity,
    ) -> Option<steamworks::networking_types::NetworkingConnectionState> {
        self.session_failure(remote).map(|failure| failure.state)
    }

    /// Returns the latest cached session failure end reason for one remote peer, preserving a failure without one as `Some(None)`.
    pub fn session_failure_end_reason(
        &self,
        remote: &steamworks::networking_types::NetworkingIdentity,
    ) -> Option<Option<steamworks::networking_types::NetConnectionEnd>> {
        self.session_failure(remote)
            .map(|failure| failure.end_reason)
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
