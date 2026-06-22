use super::{
    SteamworksNetworkingError, SteamworksNetworkingState, SteamworksP2pPacket,
    SteamworksP2pPacketAvailability, SteamworksP2pPacketSent, SteamworksP2pSessionConnectFailure,
    SteamworksP2pSessionStateResult,
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

    /// Returns the cached P2P session state for one remote Steam user.
    pub fn session_state(
        &self,
        user: steamworks::SteamId,
    ) -> Option<&SteamworksP2pSessionStateResult> {
        self.session_states.iter().find(|state| state.user == user)
    }

    /// Returns the most recent packet availability read through the plugin.
    pub fn last_packet_availability(&self) -> Option<&SteamworksP2pPacketAvailability> {
        self.last_packet_availability.as_ref()
    }

    /// Returns bounded packet availability snapshots keyed by channel.
    pub fn packet_availabilities(&self) -> &[SteamworksP2pPacketAvailability] {
        &self.packet_availabilities
    }

    /// Returns the cached packet availability for one channel.
    pub fn packet_availability(&self, channel: u32) -> Option<&SteamworksP2pPacketAvailability> {
        self.packet_availabilities
            .iter()
            .find(|availability| availability.channel == channel)
    }

    /// Returns the most recent packet send submitted through the plugin.
    pub fn last_sent_packet(&self) -> Option<SteamworksP2pPacketSent> {
        self.last_sent_packet
    }

    /// Returns the most recent packet read through the plugin.
    pub fn last_packet(&self) -> Option<&SteamworksP2pPacket> {
        self.last_packet.as_ref()
    }

    /// Returns bounded received packet snapshots.
    pub fn received_packets(&self) -> &[SteamworksP2pPacket] {
        &self.received_packets
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

    /// Returns received packets read from one channel.
    pub fn received_packets_on_channel(
        &self,
        channel: u32,
    ) -> impl Iterator<Item = &SteamworksP2pPacket> + '_ {
        self.received_packets
            .iter()
            .filter(move |packet| packet.channel == channel)
    }

    /// Returns the most recent packet from one remote Steam user.
    pub fn last_packet_from(&self, remote: steamworks::SteamId) -> Option<&SteamworksP2pPacket> {
        self.received_packets
            .iter()
            .rev()
            .find(|packet| packet.remote == remote)
    }

    /// Returns the most recent packet read from one channel.
    pub fn last_packet_on_channel(&self, channel: u32) -> Option<&SteamworksP2pPacket> {
        self.received_packets
            .iter()
            .rev()
            .find(|packet| packet.channel == channel)
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
}
