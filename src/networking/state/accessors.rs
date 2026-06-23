use super::{
    SteamworksNetworkingError, SteamworksNetworkingState, SteamworksP2pPacket,
    SteamworksP2pPacketAvailability, SteamworksP2pPacketSent, SteamworksP2pSendType,
    SteamworksP2pSessionConnectFailure, SteamworksP2pSessionState, SteamworksP2pSessionStateResult,
};

impl SteamworksNetworkingState {
    /// Returns the most recent synchronous command error observed by the plugin.
    pub fn last_error(&self) -> Option<&SteamworksNetworkingError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent accepted legacy P2P session remote.
    pub fn last_accepted_session(&self) -> Option<steamworks::SteamId> {
        self.last_accepted_session
    }

    /// Returns the most recent closed legacy P2P session remote.
    pub fn last_closed_session(&self) -> Option<steamworks::SteamId> {
        self.last_closed_session
    }

    /// Returns the most recent P2P session state read through the plugin.
    pub fn last_session_state(&self) -> Option<&SteamworksP2pSessionStateResult> {
        self.last_session_state.as_ref()
    }

    /// Returns bounded session state snapshots keyed by remote Steam user.
    pub fn session_states(&self) -> &[SteamworksP2pSessionStateResult] {
        &self.session_states
    }

    /// Returns the number of cached session state snapshots.
    pub fn session_state_count(&self) -> usize {
        self.session_states.len()
    }

    /// Returns the cached P2P session state for one remote Steam user.
    pub fn session_state(
        &self,
        user: steamworks::SteamId,
    ) -> Option<&SteamworksP2pSessionStateResult> {
        self.session_states.iter().find(|state| state.user == user)
    }

    /// Returns whether a session state read has been cached for one remote Steam user.
    pub fn has_session_state(&self, user: steamworks::SteamId) -> bool {
        self.session_state(user).is_some()
    }

    /// Returns the cached concrete session state for one remote Steam user.
    ///
    /// The outer `Option` distinguishes an unread user from a completed read.
    /// The inner `Option` is `None` when Steam reported no session.
    pub fn p2p_session_state(
        &self,
        user: steamworks::SteamId,
    ) -> Option<Option<&SteamworksP2pSessionState>> {
        self.session_state(user).map(|result| result.state.as_ref())
    }

    /// Returns whether Steam reports an active open P2P connection.
    pub fn session_active(&self, user: steamworks::SteamId) -> Option<bool> {
        self.p2p_session_state(user)
            .and_then(|state| state.map(|state| state.connection_active))
    }

    /// Returns whether Steam is currently trying to establish a P2P connection.
    pub fn session_connecting(&self, user: steamworks::SteamId) -> Option<bool> {
        self.p2p_session_state(user)
            .and_then(|state| state.map(|state| state.connecting))
    }

    /// Returns whether the P2P session is using a Steam relay.
    pub fn session_using_relay(&self, user: steamworks::SteamId) -> Option<bool> {
        self.p2p_session_state(user)
            .and_then(|state| state.map(|state| state.using_relay))
    }

    /// Returns the last P2P session error cached for a remote user.
    pub fn session_error(&self, user: steamworks::SteamId) -> Option<steamworks::P2PSessionError> {
        self.p2p_session_state(user)
            .and_then(|state| state.map(|state| state.error))
    }

    /// Returns the cached bytes queued for sending to a remote user.
    pub fn session_bytes_queued_for_send(&self, user: steamworks::SteamId) -> Option<i32> {
        self.p2p_session_state(user)
            .and_then(|state| state.map(|state| state.bytes_queued_for_send))
    }

    /// Returns the cached packets queued for sending to a remote user.
    pub fn session_packets_queued_for_send(&self, user: steamworks::SteamId) -> Option<i32> {
        self.p2p_session_state(user)
            .and_then(|state| state.map(|state| state.packets_queued_for_send))
    }

    /// Returns the cached remote IP for a P2P session, preserving a cached session without an IP as `Some(None)`.
    pub fn session_remote_ip(
        &self,
        user: steamworks::SteamId,
    ) -> Option<Option<std::net::Ipv4Addr>> {
        self.p2p_session_state(user)
            .and_then(|state| state.map(|state| state.remote_ip))
    }

    /// Returns the cached remote port for a P2P session, preserving a cached session without a port as `Some(None)`.
    pub fn session_remote_port(&self, user: steamworks::SteamId) -> Option<Option<u16>> {
        self.p2p_session_state(user)
            .and_then(|state| state.map(|state| state.remote_port))
    }

    /// Returns the most recent packet availability read through the plugin.
    pub fn last_packet_availability(&self) -> Option<&SteamworksP2pPacketAvailability> {
        self.last_packet_availability.as_ref()
    }

    /// Returns bounded packet availability snapshots keyed by channel.
    pub fn packet_availabilities(&self) -> &[SteamworksP2pPacketAvailability] {
        &self.packet_availabilities
    }

    /// Returns the number of cached packet availability snapshots.
    pub fn packet_availability_count(&self) -> usize {
        self.packet_availabilities.len()
    }

    /// Returns the cached packet availability for one channel.
    pub fn packet_availability(&self, channel: u32) -> Option<&SteamworksP2pPacketAvailability> {
        self.packet_availabilities
            .iter()
            .find(|availability| availability.channel == channel)
    }

    /// Returns available packet bytes for one channel, preserving a completed read with no packet as `Some(None)`.
    pub fn packet_available_bytes(&self, channel: u32) -> Option<Option<usize>> {
        self.packet_availability(channel)
            .map(|availability| availability.bytes)
    }

    /// Returns whether a packet is currently cached as available on a channel.
    pub fn packet_available(&self, channel: u32) -> Option<bool> {
        self.packet_available_bytes(channel)
            .map(|bytes| bytes.is_some())
    }

    /// Returns the most recent packet send submitted through the plugin.
    pub fn last_sent_packet(&self) -> Option<SteamworksP2pPacketSent> {
        self.last_sent_packet
    }

    /// Returns the remote user targeted by the most recent packet send.
    pub fn last_sent_packet_remote(&self) -> Option<steamworks::SteamId> {
        self.last_sent_packet.map(|packet| packet.remote)
    }

    /// Returns the delivery mode used by the most recent packet send.
    pub fn last_sent_packet_send_type(&self) -> Option<SteamworksP2pSendType> {
        self.last_sent_packet.map(|packet| packet.send_type)
    }

    /// Returns the channel used by the most recent packet send.
    pub fn last_sent_packet_channel(&self) -> Option<u32> {
        self.last_sent_packet.map(|packet| packet.channel)
    }

    /// Returns the payload byte count of the most recent packet send.
    pub fn last_sent_packet_bytes(&self) -> Option<usize> {
        self.last_sent_packet.map(|packet| packet.bytes)
    }

    /// Returns the most recent packet read through the plugin.
    pub fn last_packet(&self) -> Option<&SteamworksP2pPacket> {
        self.last_packet.as_ref()
    }

    /// Returns the remote user for the most recent received packet.
    pub fn last_packet_remote(&self) -> Option<steamworks::SteamId> {
        self.last_packet.as_ref().map(|packet| packet.remote)
    }

    /// Returns the channel for the most recent received packet.
    pub fn last_packet_channel(&self) -> Option<u32> {
        self.last_packet.as_ref().map(|packet| packet.channel)
    }

    /// Returns the payload byte count for the most recent received packet.
    pub fn last_packet_bytes(&self) -> Option<usize> {
        self.last_packet.as_ref().map(|packet| packet.data.len())
    }

    /// Returns the payload bytes for the most recent received packet.
    pub fn last_packet_data(&self) -> Option<&[u8]> {
        self.last_packet
            .as_ref()
            .map(|packet| packet.data.as_slice())
    }

    /// Returns bounded received packet snapshots.
    pub fn received_packets(&self) -> &[SteamworksP2pPacket] {
        &self.received_packets
    }

    /// Returns the number of received packet snapshots currently cached.
    pub fn cached_received_packet_count(&self) -> usize {
        self.received_packets.len()
    }

    /// Returns received packets from one remote Steam user.
    pub fn received_packets_from(
        &self,
        remote: steamworks::SteamId,
    ) -> impl Iterator<Item = &SteamworksP2pPacket> + '_ {
        self.received_packets
            .iter()
            .filter(move |packet| packet.remote == remote)
    }

    /// Returns the number of cached received packets from one remote Steam user.
    pub fn received_packet_count_from(&self, remote: steamworks::SteamId) -> usize {
        self.received_packets_from(remote).count()
    }

    /// Returns received packets read from one channel.
    pub fn received_packets_on_channel(
        &self,
        channel: u32,
    ) -> impl Iterator<Item = &SteamworksP2pPacket> + '_ {
        self.received_packets
            .iter()
            .filter(move |packet| packet.channel == channel)
    }

    /// Returns the number of cached received packets read from one channel.
    pub fn received_packet_count_on_channel(&self, channel: u32) -> usize {
        self.received_packets_on_channel(channel).count()
    }

    /// Returns the most recent packet from one remote Steam user.
    pub fn last_packet_from(&self, remote: steamworks::SteamId) -> Option<&SteamworksP2pPacket> {
        self.received_packets
            .iter()
            .rev()
            .find(|packet| packet.remote == remote)
    }

    /// Returns the byte count of the most recent packet from one remote Steam user.
    pub fn last_packet_bytes_from(&self, remote: steamworks::SteamId) -> Option<usize> {
        self.last_packet_from(remote)
            .map(|packet| packet.data.len())
    }

    /// Returns the most recent packet read from one channel.
    pub fn last_packet_on_channel(&self, channel: u32) -> Option<&SteamworksP2pPacket> {
        self.received_packets
            .iter()
            .rev()
            .find(|packet| packet.channel == channel)
    }

    /// Returns the byte count of the most recent packet read from one channel.
    pub fn last_packet_bytes_on_channel(&self, channel: u32) -> Option<usize> {
        self.last_packet_on_channel(channel)
            .map(|packet| packet.data.len())
    }

    /// Returns the number of successful send commands observed through the plugin.
    pub fn sent_count(&self) -> u64 {
        self.sent_count
    }

    /// Returns the number of packets read through the plugin.
    pub fn received_count(&self) -> u64 {
        self.received_count
    }

    /// Returns the number of read commands that found no queued packet.
    pub fn empty_read_count(&self) -> u64 {
        self.empty_read_count
    }

    /// Returns the most recent channel where a read command found no queued packet.
    pub fn last_empty_read_channel(&self) -> Option<u32> {
        self.last_empty_read_channel
    }

    /// Returns the number of incoming legacy P2P session requests observed.
    pub fn session_request_count(&self) -> u64 {
        self.session_request_count
    }

    /// Returns the most recent incoming legacy P2P session request remote.
    pub fn last_session_request(&self) -> Option<steamworks::SteamId> {
        self.last_session_request
    }

    /// Returns bounded incoming legacy P2P session request remotes.
    pub fn session_requests(&self) -> &[steamworks::SteamId] {
        &self.session_requests
    }

    /// Returns the number of cached incoming legacy P2P session request remotes.
    pub fn cached_session_request_count(&self) -> usize {
        self.session_requests.len()
    }

    /// Returns whether a remote has a cached incoming legacy P2P session request.
    pub fn has_session_request(&self, remote: steamworks::SteamId) -> bool {
        self.session_requests.contains(&remote)
    }

    /// Returns the number of legacy P2P session connection failures observed.
    pub fn session_connect_failure_count(&self) -> u64 {
        self.session_connect_failure_count
    }

    /// Returns the most recent legacy P2P session connection failure callback snapshot.
    pub fn last_session_connect_failure(&self) -> Option<SteamworksP2pSessionConnectFailure> {
        self.last_session_connect_failure
    }

    /// Returns bounded legacy P2P session connection failure callback snapshots.
    pub fn session_connect_failures(&self) -> &[SteamworksP2pSessionConnectFailure] {
        &self.session_connect_failures
    }

    /// Returns the number of cached legacy P2P session connection failure snapshots.
    pub fn cached_session_connect_failure_count(&self) -> usize {
        self.session_connect_failures.len()
    }

    /// Returns the cached legacy P2P session connection failure for one remote.
    pub fn session_connect_failure(
        &self,
        remote: steamworks::SteamId,
    ) -> Option<SteamworksP2pSessionConnectFailure> {
        self.session_connect_failures
            .iter()
            .find(|failure| failure.remote == remote)
            .copied()
    }

    /// Returns whether a remote has a cached legacy P2P session connection failure.
    pub fn has_session_connect_failure(&self, remote: steamworks::SteamId) -> bool {
        self.session_connect_failure(remote).is_some()
    }

    /// Returns the cached legacy P2P session connection failure error for one remote.
    pub fn session_connect_failure_error(
        &self,
        remote: steamworks::SteamId,
    ) -> Option<steamworks::P2PSessionError> {
        self.session_connect_failure(remote)
            .map(|failure| failure.error)
    }
}
