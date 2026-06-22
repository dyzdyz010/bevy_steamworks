use bevy_ecs::prelude::Resource;

use super::{
    messages::{SteamworksNetworkingUtilsError, SteamworksNetworkingUtilsOperation},
    types::SteamworksRelayNetworkStatus,
};

/// Runtime state for [`crate::SteamworksNetworkingUtilsPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksNetworkingUtilsState {
    last_error: Option<SteamworksNetworkingUtilsError>,
    last_relay_network_availability:
        Option<steamworks::networking_types::NetworkingAvailabilityResult>,
    last_relay_network_status: Option<SteamworksRelayNetworkStatus>,
    last_relay_ping_measurement_in_progress: Option<bool>,
    last_relay_network_config_availability:
        Option<steamworks::networking_types::NetworkingAvailabilityResult>,
    last_any_relay_availability: Option<steamworks::networking_types::NetworkingAvailabilityResult>,
    last_relay_debugging_message: Option<String>,
    relay_network_access_initialized: bool,
    relay_network_status_callback_count: u64,
}

impl SteamworksNetworkingUtilsState {
    /// Returns the most recent synchronous command error observed by the plugin.
    pub fn last_error(&self) -> Option<&SteamworksNetworkingUtilsError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent relay network availability read through the plugin.
    pub fn last_relay_network_availability(
        &self,
    ) -> Option<&steamworks::networking_types::NetworkingAvailabilityResult> {
        self.last_relay_network_availability.as_ref()
    }

    /// Returns the most recent detailed relay network status snapshot.
    pub fn last_relay_network_status(&self) -> Option<&SteamworksRelayNetworkStatus> {
        self.last_relay_network_status.as_ref()
    }

    /// Returns the most recent relay ping-measurement state read through the plugin.
    pub fn relay_ping_measurement_in_progress(&self) -> Option<bool> {
        self.last_relay_ping_measurement_in_progress
    }

    /// Returns the most recent relay network-config prerequisite availability.
    pub fn last_relay_network_config_availability(
        &self,
    ) -> Option<&steamworks::networking_types::NetworkingAvailabilityResult> {
        self.last_relay_network_config_availability.as_ref()
    }

    /// Returns the most recent any-relay availability.
    pub fn last_any_relay_availability(
        &self,
    ) -> Option<&steamworks::networking_types::NetworkingAvailabilityResult> {
        self.last_any_relay_availability.as_ref()
    }

    /// Returns the most recent relay diagnostic debug message.
    pub fn last_relay_debugging_message(&self) -> Option<&str> {
        self.last_relay_debugging_message.as_deref()
    }

    /// Returns whether relay network access was initialized through this plugin.
    pub fn relay_network_access_initialized(&self) -> bool {
        self.relay_network_access_initialized
    }

    /// Returns how many relay network status callbacks this plugin has observed.
    pub fn relay_network_status_callback_count(&self) -> u64 {
        self.relay_network_status_callback_count
    }

    pub(super) fn record_error(&mut self, error: SteamworksNetworkingUtilsError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksNetworkingUtilsOperation) {
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
