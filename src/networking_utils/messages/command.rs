use bevy_ecs::message::Message;

/// A high-level command for Steam Networking Utils relay diagnostics.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksNetworkingUtilsCommand {
    /// Initialize Steam Datagram Relay network access early.
    InitRelayNetworkAccess,
    /// Read the summary relay network availability.
    GetRelayNetworkStatus,
    /// Read the detailed relay network status snapshot.
    GetDetailedRelayNetworkStatus,
    /// Read whether Steam is currently measuring relay latency.
    IsRelayPingMeasurementInProgress,
    /// Read the relay network-config prerequisite availability.
    GetRelayNetworkConfigStatus,
    /// Read whether any Steam Datagram Relay can be reached.
    GetAnyRelayStatus,
    /// Read Steam's non-localized relay diagnostic message.
    GetRelayDebugMessage,
}

impl SteamworksNetworkingUtilsCommand {
    /// Creates a [`crate::SteamworksNetworkingUtilsCommand::InitRelayNetworkAccess`] command.
    pub fn init_relay_network_access() -> Self {
        Self::InitRelayNetworkAccess
    }

    /// Creates a [`crate::SteamworksNetworkingUtilsCommand::GetRelayNetworkStatus`] command.
    pub fn get_relay_network_status() -> Self {
        Self::GetRelayNetworkStatus
    }

    /// Creates a [`crate::SteamworksNetworkingUtilsCommand::GetDetailedRelayNetworkStatus`] command.
    pub fn get_detailed_relay_network_status() -> Self {
        Self::GetDetailedRelayNetworkStatus
    }

    /// Creates a [`crate::SteamworksNetworkingUtilsCommand::IsRelayPingMeasurementInProgress`] command.
    pub fn is_relay_ping_measurement_in_progress() -> Self {
        Self::IsRelayPingMeasurementInProgress
    }

    /// Creates a [`crate::SteamworksNetworkingUtilsCommand::GetRelayNetworkConfigStatus`] command.
    pub fn get_relay_network_config_status() -> Self {
        Self::GetRelayNetworkConfigStatus
    }

    /// Creates a [`crate::SteamworksNetworkingUtilsCommand::GetAnyRelayStatus`] command.
    pub fn get_any_relay_status() -> Self {
        Self::GetAnyRelayStatus
    }

    /// Creates a [`crate::SteamworksNetworkingUtilsCommand::GetRelayDebugMessage`] command.
    pub fn get_relay_debug_message() -> Self {
        Self::GetRelayDebugMessage
    }
}
