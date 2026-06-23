use super::super::types::SteamworksRelayNetworkStatus;

/// A successfully submitted Steam Networking Utils operation or synchronous read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksNetworkingUtilsOperation {
    /// Relay network access initialization was submitted to Steam.
    RelayNetworkAccessInitialized,
    /// Summary relay network availability was read.
    RelayNetworkStatusRead {
        /// Current relay network availability.
        availability: steamworks::networking_types::NetworkingAvailabilityResult,
    },
    /// Detailed relay network status was read.
    DetailedRelayNetworkStatusRead {
        /// Owned relay network status snapshot.
        status: SteamworksRelayNetworkStatus,
    },
    /// Relay ping measurement state was read.
    RelayPingMeasurementStateRead {
        /// Whether Steam is currently measuring relay latency.
        in_progress: bool,
    },
    /// Relay network-config prerequisite availability was read.
    RelayNetworkConfigStatusRead {
        /// Network-config prerequisite availability.
        availability: steamworks::networking_types::NetworkingAvailabilityResult,
    },
    /// Any-relay availability was read.
    AnyRelayStatusRead {
        /// Availability of at least one Steam Datagram Relay.
        availability: steamworks::networking_types::NetworkingAvailabilityResult,
    },
    /// Relay diagnostic debug message was read.
    RelayDebugMessageRead {
        /// Non-localized diagnostic text from Steam.
        message: String,
    },
    /// A relay network status callback was observed after Steam callbacks were pumped.
    RelayNetworkStatusChanged {
        /// Owned relay network status snapshot read when the callback was observed.
        status: SteamworksRelayNetworkStatus,
    },
}
