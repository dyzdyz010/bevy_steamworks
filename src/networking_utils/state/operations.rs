use super::SteamworksNetworkingUtilsState;
use crate::networking_utils::{SteamworksNetworkingUtilsError, SteamworksNetworkingUtilsOperation};

impl SteamworksNetworkingUtilsState {
    pub(in crate::networking_utils) fn record_error(
        &mut self,
        error: SteamworksNetworkingUtilsError,
    ) {
        self.last_error = Some(error);
    }

    pub(in crate::networking_utils) fn record_operation(
        &mut self,
        operation: &SteamworksNetworkingUtilsOperation,
    ) {
        match operation {
            SteamworksNetworkingUtilsOperation::RelayNetworkAccessInitialized => {
                self.relay_network_access_initialized = true;
            }
            SteamworksNetworkingUtilsOperation::RelayNetworkStatusRead { availability } => {
                self.last_relay_network_availability = Some(*availability);
            }
            SteamworksNetworkingUtilsOperation::DetailedRelayNetworkStatusRead { status } => {
                self.last_relay_network_availability = Some(status.availability);
                self.last_relay_ping_measurement_in_progress =
                    Some(status.ping_measurement_in_progress);
                self.last_relay_network_config_availability = Some(status.network_config);
                self.last_any_relay_availability = Some(status.any_relay);
                self.last_relay_debugging_message = Some(status.debugging_message.clone());
                self.last_relay_network_status = Some(status.clone());
            }
            SteamworksNetworkingUtilsOperation::RelayPingMeasurementStateRead { in_progress } => {
                self.last_relay_ping_measurement_in_progress = Some(*in_progress);
            }
            SteamworksNetworkingUtilsOperation::RelayNetworkConfigStatusRead { availability } => {
                self.last_relay_network_config_availability = Some(*availability);
            }
            SteamworksNetworkingUtilsOperation::AnyRelayStatusRead { availability } => {
                self.last_any_relay_availability = Some(*availability);
            }
            SteamworksNetworkingUtilsOperation::RelayDebugMessageRead { message } => {
                self.last_relay_debugging_message = Some(message.clone());
            }
            SteamworksNetworkingUtilsOperation::RelayNetworkStatusChanged { status } => {
                self.last_relay_network_availability = Some(status.availability);
                self.last_relay_ping_measurement_in_progress =
                    Some(status.ping_measurement_in_progress);
                self.last_relay_network_config_availability = Some(status.network_config);
                self.last_any_relay_availability = Some(status.any_relay);
                self.last_relay_debugging_message = Some(status.debugging_message.clone());
                self.last_relay_network_status = Some(status.clone());
                self.relay_network_status_callback_count =
                    self.relay_network_status_callback_count.saturating_add(1);
            }
        }
    }
}
