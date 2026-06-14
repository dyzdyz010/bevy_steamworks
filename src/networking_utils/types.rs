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
