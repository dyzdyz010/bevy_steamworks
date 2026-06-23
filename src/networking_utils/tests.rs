use bevy_app::{App, Plugin};
use bevy_ecs::message::Messages;

use crate::SteamworksEvent;

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
    assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingUtilsCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingUtilsResult>>());
}

#[test]
fn plugin_name_matches_networking_utils_type_path_for_bevy_tracking() {
    let plugin = SteamworksNetworkingUtilsPlugin::new();

    assert_eq!(
        plugin.name(),
        std::any::type_name::<SteamworksNetworkingUtilsPlugin>()
    );
    assert_eq!(
        plugin.name(),
        "bevy_steamworks::networking_utils::SteamworksNetworkingUtilsPlugin"
    );
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksNetworkingUtilsPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksNetworkingUtilsCommand>>()
        .write(SteamworksNetworkingUtilsCommand::get_detailed_relay_network_status());

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
fn constructors_preserve_inputs() {
    assert_eq!(
        SteamworksNetworkingUtilsCommand::init_relay_network_access(),
        SteamworksNetworkingUtilsCommand::InitRelayNetworkAccess
    );
    assert_eq!(
        SteamworksNetworkingUtilsCommand::get_relay_network_status(),
        SteamworksNetworkingUtilsCommand::GetRelayNetworkStatus
    );
    assert_eq!(
        SteamworksNetworkingUtilsCommand::get_detailed_relay_network_status(),
        SteamworksNetworkingUtilsCommand::GetDetailedRelayNetworkStatus
    );
    assert_eq!(
        SteamworksNetworkingUtilsCommand::is_relay_ping_measurement_in_progress(),
        SteamworksNetworkingUtilsCommand::IsRelayPingMeasurementInProgress
    );
    assert_eq!(
        SteamworksNetworkingUtilsCommand::get_relay_network_config_status(),
        SteamworksNetworkingUtilsCommand::GetRelayNetworkConfigStatus
    );
    assert_eq!(
        SteamworksNetworkingUtilsCommand::get_any_relay_status(),
        SteamworksNetworkingUtilsCommand::GetAnyRelayStatus
    );
    assert_eq!(
        SteamworksNetworkingUtilsCommand::get_relay_debug_message(),
        SteamworksNetworkingUtilsCommand::GetRelayDebugMessage
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
    assert_eq!(
        state.relay_network_availability(),
        Some(steamworks::networking_types::NetworkingAvailability::Waiting)
    );
    assert_eq!(state.relay_network_availability_error(), None);
    assert_eq!(state.relay_network_available(), Some(false));
    assert_eq!(state.relay_network_pending(), Some(true));
    assert_eq!(state.relay_network_unavailable(), Some(false));

    state.record_operation(
        &SteamworksNetworkingUtilsOperation::DetailedRelayNetworkStatusRead {
            status: status.clone(),
        },
    );
    assert!(status.relay_network_available());
    assert!(!status.relay_network_pending());
    assert!(!status.relay_network_unavailable());
    assert!(status.network_config_available());
    assert!(status.any_relay_available());

    state.record_operation(
        &SteamworksNetworkingUtilsOperation::RelayPingMeasurementStateRead { in_progress: true },
    );
    state.record_operation(
        &SteamworksNetworkingUtilsOperation::RelayNetworkConfigStatusRead {
            availability: Ok(steamworks::networking_types::NetworkingAvailability::Attempting),
        },
    );
    state.record_operation(&SteamworksNetworkingUtilsOperation::AnyRelayStatusRead {
        availability: Ok(steamworks::networking_types::NetworkingAvailability::Current),
    });
    state.record_operation(&SteamworksNetworkingUtilsOperation::RelayDebugMessageRead {
        message: "measuring".to_owned(),
    });

    assert_eq!(state.relay_ping_measurement_in_progress(), Some(true));
    assert_eq!(
        state.last_relay_network_config_availability(),
        Some(&Ok(
            steamworks::networking_types::NetworkingAvailability::Attempting
        ))
    );
    assert_eq!(
        state.relay_network_config_availability(),
        Some(steamworks::networking_types::NetworkingAvailability::Attempting)
    );
    assert_eq!(state.relay_network_config_availability_error(), None);
    assert_eq!(state.relay_network_config_available(), Some(false));
    assert_eq!(state.relay_network_config_pending(), Some(true));
    assert_eq!(state.relay_network_config_unavailable(), Some(false));
    assert_eq!(
        state.last_any_relay_availability(),
        Some(&Ok(
            steamworks::networking_types::NetworkingAvailability::Current
        ))
    );
    assert_eq!(
        state.any_relay_availability(),
        Some(steamworks::networking_types::NetworkingAvailability::Current)
    );
    assert_eq!(state.any_relay_availability_error(), None);
    assert_eq!(state.any_relay_available(), Some(true));
    assert_eq!(state.any_relay_pending(), Some(false));
    assert_eq!(state.any_relay_unavailable(), Some(false));
    assert_eq!(state.last_relay_debugging_message(), Some("measuring"));

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
    assert_eq!(state.relay_ping_measurement_in_progress(), Some(false));
    assert_eq!(
        state.last_relay_network_config_availability(),
        Some(&Ok(
            steamworks::networking_types::NetworkingAvailability::Current
        ))
    );
    assert_eq!(
        state.last_any_relay_availability(),
        Some(&Ok(
            steamworks::networking_types::NetworkingAvailability::Current
        ))
    );
    assert_eq!(
        state.relay_network_availability(),
        Some(steamworks::networking_types::NetworkingAvailability::Current)
    );
    assert_eq!(state.relay_network_available(), Some(true));
    assert_eq!(state.relay_network_pending(), Some(false));
    assert_eq!(state.relay_network_unavailable(), Some(false));
    assert_eq!(state.relay_network_config_available(), Some(true));
    assert_eq!(state.relay_network_config_pending(), Some(false));
    assert_eq!(state.relay_network_config_unavailable(), Some(false));
    assert_eq!(state.any_relay_available(), Some(true));
    assert_eq!(state.any_relay_pending(), Some(false));
    assert_eq!(state.any_relay_unavailable(), Some(false));
    assert_eq!(state.last_relay_debugging_message(), Some("ready"));
    assert_eq!(state.relay_network_status_callback_count(), 1);

    state.record_operation(
        &SteamworksNetworkingUtilsOperation::RelayNetworkStatusRead {
            availability: Err(steamworks::networking_types::NetworkingAvailabilityError::Retrying),
        },
    );
    assert_eq!(
        state.relay_network_availability_error(),
        Some(steamworks::networking_types::NetworkingAvailabilityError::Retrying)
    );
    assert_eq!(state.relay_network_available(), Some(false));
    assert_eq!(state.relay_network_pending(), Some(true));
    assert_eq!(state.relay_network_unavailable(), Some(true));

    state.record_operation(
        &SteamworksNetworkingUtilsOperation::RelayNetworkConfigStatusRead {
            availability: Err(steamworks::networking_types::NetworkingAvailabilityError::Unknown),
        },
    );
    assert_eq!(state.relay_network_config_availability(), None);
    assert_eq!(
        state.relay_network_config_availability_error(),
        Some(steamworks::networking_types::NetworkingAvailabilityError::Unknown)
    );
    assert_eq!(state.relay_network_config_available(), Some(false));
    assert_eq!(state.relay_network_config_pending(), Some(false));
    assert_eq!(state.relay_network_config_unavailable(), Some(true));

    state.record_operation(&SteamworksNetworkingUtilsOperation::AnyRelayStatusRead {
        availability: Err(steamworks::networking_types::NetworkingAvailabilityError::Retrying),
    });
    assert_eq!(state.any_relay_availability(), None);
    assert_eq!(
        state.any_relay_availability_error(),
        Some(steamworks::networking_types::NetworkingAvailabilityError::Retrying)
    );
    assert_eq!(state.any_relay_available(), Some(false));
    assert_eq!(state.any_relay_pending(), Some(true));
    assert_eq!(state.any_relay_unavailable(), Some(true));
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
    assert_eq!(state.relay_ping_measurement_in_progress(), Some(false));
    assert_eq!(
        state.last_relay_network_config_availability(),
        Some(&Ok(
            steamworks::networking_types::NetworkingAvailability::Current
        ))
    );
    assert_eq!(
        state.last_any_relay_availability(),
        Some(&Ok(
            steamworks::networking_types::NetworkingAvailability::Current
        ))
    );
    assert_eq!(state.last_relay_debugging_message(), Some("ready"));
    assert_eq!(state.relay_network_status_callback_count(), 1);
}
