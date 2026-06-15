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
}

impl SteamworksNetworkingUtilsCommand {
    /// Creates a [`crate::SteamworksNetworkingUtilsCommand::InitRelayNetworkAccess`] command.
    pub fn init_relay_network_access() -> Self {
        Self::InitRelayNetworkAccess
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
