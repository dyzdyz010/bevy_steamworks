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
                self.last_relay_network_status = Some(status.clone());
            }
            SteamworksNetworkingUtilsOperation::RelayNetworkStatusChanged { status } => {
                self.last_relay_network_availability = Some(status.availability);
                self.last_relay_network_status = Some(status.clone());
                self.relay_network_status_callback_count =
                    self.relay_network_status_callback_count.saturating_add(1);
            }
        }
    }
}
