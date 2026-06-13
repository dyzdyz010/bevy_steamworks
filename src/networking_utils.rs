//! High-level Bevy ECS integration for Steam Networking Utils.
//!
//! This module builds on top of the upstream
//! [`steamworks::networking_utils::NetworkingUtils`] API. It exposes Steam
//! Datagram Relay initialization and relay status diagnostics through Bevy
//! commands/results, and turns relay status callbacks into owned Bevy messages.

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

/// Bevy plugin for high-level Steam Networking Utils commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksNetworkingUtilsCommand`] and
/// [`SteamworksNetworkingUtilsResult`] messages and processes commands in
/// [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksNetworkingUtilsPlugin;

impl SteamworksNetworkingUtilsPlugin {
    /// Creates a Networking Utils plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksNetworkingUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksNetworkingUtilsState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksNetworkingUtilsCommand>()
            .add_message::<SteamworksNetworkingUtilsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessNetworkingUtilsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_networking_utils_commands
                    .in_set(SteamworksSystem::ProcessNetworkingUtilsCommands),
            );
    }
}

/// Runtime state for [`SteamworksNetworkingUtilsPlugin`].
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

    fn record_error(&mut self, error: SteamworksNetworkingUtilsError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksNetworkingUtilsOperation) {
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
    fn from_steam(status: steamworks::networking_utils::RelayNetworkStatus) -> Self {
        Self {
            availability: status.availability(),
            ping_measurement_in_progress: status.is_ping_measurement_in_progress(),
            network_config: status.network_config(),
            any_relay: status.any_relay(),
            debugging_message: status.debugging_message().to_owned(),
        }
    }
}

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
    /// Creates a [`SteamworksNetworkingUtilsCommand::InitRelayNetworkAccess`] command.
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

/// Result message emitted by [`SteamworksNetworkingUtilsPlugin`].
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

/// Synchronous errors from [`SteamworksNetworkingUtilsPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksNetworkingUtilsError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
}

fn process_networking_utils_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksNetworkingUtilsState>,
    mut commands: ResMut<Messages<SteamworksNetworkingUtilsCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksNetworkingUtilsResult>,
) {
    let Some(client) = client else {
        let error = SteamworksNetworkingUtilsError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks networking utils command failed"
            );
            results.write(SteamworksNetworkingUtilsResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for event in steam_events.read() {
        if let SteamworksEvent::RelayNetworkStatusCallback(_) = event {
            let operation = SteamworksNetworkingUtilsOperation::RelayNetworkStatusChanged {
                status: SteamworksRelayNetworkStatus::from_steam(
                    client.networking_utils().detailed_relay_network_status(),
                ),
            };
            state.record_operation(&operation);
            tracing::debug!(
                target: "bevy_steamworks",
                operation = ?operation,
                "processed Steamworks relay network status callback"
            );
            results.write(SteamworksNetworkingUtilsResult::Ok(operation));
        }
    }

    for command in commands.drain() {
        match handle_networking_utils_command(&client, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks networking utils command"
                );
                results.write(SteamworksNetworkingUtilsResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks networking utils command failed"
                );
                results.write(SteamworksNetworkingUtilsResult::Err { command, error });
            }
        }
    }
}

fn handle_networking_utils_command(
    client: &SteamworksClient,
    command: &SteamworksNetworkingUtilsCommand,
) -> Result<SteamworksNetworkingUtilsOperation, SteamworksNetworkingUtilsError> {
    validate_command(command)?;

    let networking_utils = client.networking_utils();
    Ok(match command {
        SteamworksNetworkingUtilsCommand::InitRelayNetworkAccess => {
            networking_utils.init_relay_network_access();
            SteamworksNetworkingUtilsOperation::RelayNetworkAccessInitialized
        }
        SteamworksNetworkingUtilsCommand::GetRelayNetworkStatus => {
            SteamworksNetworkingUtilsOperation::RelayNetworkStatusRead {
                availability: networking_utils.relay_network_status(),
            }
        }
        SteamworksNetworkingUtilsCommand::GetDetailedRelayNetworkStatus => {
            SteamworksNetworkingUtilsOperation::DetailedRelayNetworkStatusRead {
                status: SteamworksRelayNetworkStatus::from_steam(
                    networking_utils.detailed_relay_network_status(),
                ),
            }
        }
    })
}

fn validate_command(
    _command: &SteamworksNetworkingUtilsCommand,
) -> Result<(), SteamworksNetworkingUtilsError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    fn current_status() -> SteamworksRelayNetworkStatus {
        SteamworksRelayNetworkStatus {
            availability: Ok(steamworks::networking_types::NetworkingAvailability::Current),
            ping_measurement_in_progress: false,
            network_config: Ok(steamworks::networking_types::NetworkingAvailability::Current),
            any_relay: Ok(steamworks::networking_types::NetworkingAvailability::Current),
            debugging_message: "ready".to_owned(),
        }
    }

    #[test]
    fn networking_utils_plugin_registers_resources_and_messages() {
        let mut app = App::new();

        app.add_plugins(SteamworksNetworkingUtilsPlugin::new());

        assert!(app
            .world()
            .contains_resource::<SteamworksNetworkingUtilsState>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksNetworkingUtilsCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksNetworkingUtilsResult>>());
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksNetworkingUtilsPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksNetworkingUtilsCommand>>()
            .write(SteamworksNetworkingUtilsCommand::GetDetailedRelayNetworkStatus);

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksNetworkingUtilsResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksNetworkingUtilsResult::Err {
                command: SteamworksNetworkingUtilsCommand::GetDetailedRelayNetworkStatus,
                error: SteamworksNetworkingUtilsError::ClientUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksNetworkingUtilsState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksNetworkingUtilsError::ClientUnavailable)
        );
    }

    #[test]
    fn state_records_operations_without_unbounded_callback_history() {
        let mut state = SteamworksNetworkingUtilsState::default();
        let status = current_status();

        state.record_operation(&SteamworksNetworkingUtilsOperation::RelayNetworkAccessInitialized);
        state.record_operation(
            &SteamworksNetworkingUtilsOperation::RelayNetworkStatusRead {
                availability: Ok(steamworks::networking_types::NetworkingAvailability::Waiting),
            },
        );
        state.record_operation(
            &SteamworksNetworkingUtilsOperation::DetailedRelayNetworkStatusRead {
                status: status.clone(),
            },
        );
        state.record_operation(
            &SteamworksNetworkingUtilsOperation::RelayNetworkStatusChanged {
                status: status.clone(),
            },
        );

        assert!(state.relay_network_access_initialized());
        assert_eq!(
            state.last_relay_network_availability(),
            Some(&Ok(
                steamworks::networking_types::NetworkingAvailability::Current
            ))
        );
        assert_eq!(state.last_relay_network_status(), Some(&status));
        assert_eq!(state.relay_network_status_callback_count(), 1);
    }

    #[test]
    fn relay_status_callback_operation_updates_state() {
        let mut state = SteamworksNetworkingUtilsState::default();
        let status = current_status();

        state.record_operation(
            &SteamworksNetworkingUtilsOperation::RelayNetworkStatusChanged {
                status: status.clone(),
            },
        );
        assert_eq!(state.last_relay_network_status(), Some(&status));
        assert_eq!(state.relay_network_status_callback_count(), 1);
    }

    #[test]
    fn constructors_and_validation_cover_current_commands() {
        assert_eq!(
            SteamworksNetworkingUtilsCommand::init_relay_network_access(),
            SteamworksNetworkingUtilsCommand::InitRelayNetworkAccess
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingUtilsCommand::InitRelayNetworkAccess),
            Ok(())
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingUtilsCommand::GetRelayNetworkStatus),
            Ok(())
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingUtilsCommand::GetDetailedRelayNetworkStatus),
            Ok(())
        );
    }
}
