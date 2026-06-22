use bevy_ecs::prelude::Resource;

use super::{messages::SteamworksNetworkingUtilsError, types::SteamworksRelayNetworkStatus};

mod accessors;
mod operations;

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
