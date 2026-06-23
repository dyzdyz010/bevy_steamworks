use super::SteamworksNetworkingUtilsState;
use crate::networking_utils::{SteamworksNetworkingUtilsError, SteamworksRelayNetworkStatus};
use steamworks::networking_types::{
    NetworkingAvailability, NetworkingAvailabilityError, NetworkingAvailabilityResult,
};

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

    /// Returns the most recent successful summary relay availability value.
    pub fn relay_network_availability(&self) -> Option<NetworkingAvailability> {
        availability_value(self.last_relay_network_availability.as_ref())
    }

    /// Returns the most recent summary relay availability error.
    pub fn relay_network_availability_error(&self) -> Option<NetworkingAvailabilityError> {
        availability_error(self.last_relay_network_availability.as_ref())
    }

    /// Returns whether the most recent summary relay availability is current.
    pub fn relay_network_available(&self) -> Option<bool> {
        availability_current(self.last_relay_network_availability.as_ref())
    }

    /// Returns whether the most recent summary relay availability is still pending.
    pub fn relay_network_pending(&self) -> Option<bool> {
        availability_pending(self.last_relay_network_availability.as_ref())
    }

    /// Returns whether the most recent summary relay availability reported an error.
    pub fn relay_network_unavailable(&self) -> Option<bool> {
        self.last_relay_network_availability
            .as_ref()
            .map(Result::is_err)
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

    /// Returns whether the most recent relay network-config availability is current.
    pub fn relay_network_config_available(&self) -> Option<bool> {
        availability_current(self.last_relay_network_config_availability.as_ref())
    }

    /// Returns the most recent any-relay availability.
    pub fn last_any_relay_availability(
        &self,
    ) -> Option<&steamworks::networking_types::NetworkingAvailabilityResult> {
        self.last_any_relay_availability.as_ref()
    }

    /// Returns whether the most recent any-relay availability is current.
    pub fn any_relay_available(&self) -> Option<bool> {
        availability_current(self.last_any_relay_availability.as_ref())
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
}

fn availability_value(
    availability: Option<&NetworkingAvailabilityResult>,
) -> Option<NetworkingAvailability> {
    match availability {
        Some(Ok(value)) => Some(*value),
        _ => None,
    }
}

fn availability_error(
    availability: Option<&NetworkingAvailabilityResult>,
) -> Option<NetworkingAvailabilityError> {
    match availability {
        Some(Err(error)) => Some(*error),
        _ => None,
    }
}

fn availability_current(availability: Option<&NetworkingAvailabilityResult>) -> Option<bool> {
    availability.map(|availability| matches!(availability, Ok(NetworkingAvailability::Current)))
}

fn availability_pending(availability: Option<&NetworkingAvailabilityResult>) -> Option<bool> {
    availability.map(|availability| {
        matches!(
            availability,
            Ok(NetworkingAvailability::Waiting | NetworkingAvailability::Attempting)
                | Err(NetworkingAvailabilityError::Retrying)
        )
    })
}
