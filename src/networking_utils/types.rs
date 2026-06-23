/// Owned snapshot of Steam relay network status.
///
/// The upstream status object exposes its debug message by reference. This type
/// copies that string so apps can store the snapshot in ECS resources.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksRelayNetworkStatus {
    /// Summary status. `Ok(Current)` means relay initialization is complete.
    pub availability: steamworks::networking_types::NetworkingAvailabilityResult,
    /// Whether Steam is currently measuring relay latency.
    pub ping_measurement_in_progress: bool,
    /// Availability of the network config prerequisite.
    pub network_config: steamworks::networking_types::NetworkingAvailabilityResult,
    /// Availability of at least one Steam Datagram Relay.
    pub any_relay: steamworks::networking_types::NetworkingAvailabilityResult,
    /// Non-localized diagnostic text from Steam.
    pub debugging_message: String,
}

impl SteamworksRelayNetworkStatus {
    /// Returns whether the summary relay status is current and usable.
    pub fn relay_network_available(&self) -> bool {
        availability_is_current(&self.availability)
    }

    /// Returns whether the summary relay status is still being acquired.
    pub fn relay_network_pending(&self) -> bool {
        availability_is_pending(&self.availability)
    }

    /// Returns whether the summary relay status currently reports an error.
    pub fn relay_network_unavailable(&self) -> bool {
        self.availability.is_err()
    }

    /// Returns whether the relay network-config prerequisite is current.
    pub fn network_config_available(&self) -> bool {
        availability_is_current(&self.network_config)
    }

    /// Returns whether at least one Steam Datagram Relay is current.
    pub fn any_relay_available(&self) -> bool {
        availability_is_current(&self.any_relay)
    }

    pub(super) fn from_steam(status: steamworks::networking_utils::RelayNetworkStatus) -> Self {
        Self {
            availability: status.availability(),
            ping_measurement_in_progress: status.is_ping_measurement_in_progress(),
            network_config: status.network_config(),
            any_relay: status.any_relay(),
            debugging_message: status.debugging_message().to_owned(),
        }
    }
}

fn availability_is_current(
    availability: &steamworks::networking_types::NetworkingAvailabilityResult,
) -> bool {
    matches!(
        availability,
        Ok(steamworks::networking_types::NetworkingAvailability::Current)
    )
}

fn availability_is_pending(
    availability: &steamworks::networking_types::NetworkingAvailabilityResult,
) -> bool {
    matches!(
        availability,
        Ok(
            steamworks::networking_types::NetworkingAvailability::Waiting
                | steamworks::networking_types::NetworkingAvailability::Attempting
        ) | Err(steamworks::networking_types::NetworkingAvailabilityError::Retrying)
    )
}
