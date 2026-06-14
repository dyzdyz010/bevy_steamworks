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
