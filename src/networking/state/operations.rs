use super::{
    SteamworksNetworkingError, SteamworksNetworkingOperation, SteamworksNetworkingState,
    SteamworksP2pPacketSent, SteamworksP2pSessionConnectFailure,
};

impl SteamworksNetworkingState {
    pub(in crate::networking) fn record_error(&mut self, error: SteamworksNetworkingError) {
        self.last_error = Some(error);
    }

    pub(in crate::networking) fn record_operation(
        &mut self,
        operation: &SteamworksNetworkingOperation,
    ) {
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
