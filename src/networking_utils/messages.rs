use bevy_ecs::message::Message;
use thiserror::Error;

use super::types::SteamworksRelayNetworkStatus;

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

/// Result message emitted by [`crate::SteamworksNetworkingUtilsPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksNetworkingUtilsResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksNetworkingUtilsOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksNetworkingUtilsCommand,
        /// Failure reason.
        error: SteamworksNetworkingUtilsError,
    },
}

crate::result_ext::impl_steamworks_result_helpers!(
    SteamworksNetworkingUtilsResult,
    SteamworksNetworkingUtilsOperation,
    SteamworksNetworkingUtilsCommand,
    SteamworksNetworkingUtilsError
);

/// Synchronous errors from [`crate::SteamworksNetworkingUtilsPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksNetworkingUtilsError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
}
