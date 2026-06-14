use bevy_ecs::prelude::Resource;

use super::*;

/// Runtime state for [`crate::SteamworksNetworkingPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksNetworkingState {
    last_error: Option<SteamworksNetworkingError>,
    last_accepted_session: Option<steamworks::SteamId>,
    last_closed_session: Option<steamworks::SteamId>,
    last_session_state: Option<SteamworksP2pSessionStateResult>,
    last_packet_availability: Option<SteamworksP2pPacketAvailability>,
    last_sent_packet: Option<SteamworksP2pPacketSent>,
    last_packet: Option<SteamworksP2pPacket>,
    sent_count: u64,
    received_count: u64,
    empty_read_count: u64,
    last_empty_read_channel: Option<u32>,
    session_request_count: u64,
    last_session_request: Option<steamworks::SteamId>,
    session_connect_failure_count: u64,
    last_session_connect_failure: Option<SteamworksP2pSessionConnectFailure>,
}

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

    /// Returns the most recent packet availability read through the plugin.
    pub fn last_packet_availability(&self) -> Option<&SteamworksP2pPacketAvailability> {
        self.last_packet_availability.as_ref()
    }

    /// Returns the most recent packet send submitted through the plugin.
    pub fn last_sent_packet(&self) -> Option<SteamworksP2pPacketSent> {
        self.last_sent_packet
    }

    /// Returns the most recent packet read through the plugin.
    pub fn last_packet(&self) -> Option<&SteamworksP2pPacket> {
        self.last_packet.as_ref()
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

    /// Returns the number of legacy P2P session connection failures observed.
    pub fn session_connect_failure_count(&self) -> u64 {
        self.session_connect_failure_count
    }

    /// Returns the most recent legacy P2P session connection failure callback snapshot.
    pub fn last_session_connect_failure(&self) -> Option<SteamworksP2pSessionConnectFailure> {
        self.last_session_connect_failure
    }

    pub(super) fn record_error(&mut self, error: SteamworksNetworkingError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksNetworkingOperation) {
        match operation {
            SteamworksNetworkingOperation::SessionAccepted { user } => {
                self.last_accepted_session = Some(*user);
            }
            SteamworksNetworkingOperation::SessionClosed { user } => {
                self.last_closed_session = Some(*user);
                if self
                    .last_session_state
                    .as_ref()
                    .is_some_and(|state| state.user == *user)
                {
                    self.last_session_state = None;
                }
            }
            SteamworksNetworkingOperation::SessionStateRead { state } => {
                self.last_session_state = Some(state.clone());
            }
            SteamworksNetworkingOperation::PacketSent {
                remote,
                send_type,
                channel,
                bytes,
            } => {
                self.sent_count = self.sent_count.saturating_add(1);
                self.last_sent_packet = Some(SteamworksP2pPacketSent {
                    remote: *remote,
                    send_type: *send_type,
                    channel: *channel,
                    bytes: *bytes,
                });
            }
            SteamworksNetworkingOperation::PacketRead {
                packet: Some(packet),
                ..
            } => {
                self.received_count = self.received_count.saturating_add(1);
                self.last_packet = Some(packet.clone());
            }
            SteamworksNetworkingOperation::PacketRead {
                channel,
                packet: None,
            } => {
                self.empty_read_count = self.empty_read_count.saturating_add(1);
                self.last_empty_read_channel = Some(*channel);
            }
            SteamworksNetworkingOperation::PacketAvailabilityRead { availability } => {
                self.last_packet_availability = Some(availability.clone());
            }
            SteamworksNetworkingOperation::SessionRequestReceived { remote } => {
                self.session_request_count = self.session_request_count.saturating_add(1);
                self.last_session_request = Some(*remote);
            }
            SteamworksNetworkingOperation::SessionConnectFailed { remote, error } => {
                self.session_connect_failure_count =
                    self.session_connect_failure_count.saturating_add(1);
                self.last_session_connect_failure = Some(SteamworksP2pSessionConnectFailure {
                    remote: *remote,
                    error: *error,
                });
            }
        }
    }
}
