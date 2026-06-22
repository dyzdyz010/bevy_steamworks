use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::Messages,
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};

use crate::SteamworksSystem;

use super::*;

#[test]
fn plugin_name_matches_networking_messages_type_path_for_bevy_tracking() {
    let plugin = SteamworksNetworkingMessagesPlugin::new();

    assert_eq!(
        plugin.name(),
        std::any::type_name::<SteamworksNetworkingMessagesPlugin>()
    );
    assert_eq!(
        plugin.name(),
        "bevy_steamworks::networking_messages::SteamworksNetworkingMessagesPlugin"
    );
}

#[test]
fn configuration_accessors_expose_builder_settings() {
    let plugin = SteamworksNetworkingMessagesPlugin::new();
    assert!(plugin.auto_accepts_session_requests());

    let plugin = SteamworksNetworkingMessagesPlugin::new().auto_accept_session_requests(false);
    assert!(!plugin.auto_accepts_session_requests());
}

#[test]
fn networking_messages_plugin_registers_resources_and_messages() {
    let mut app = App::new();

    app.add_plugins(SteamworksNetworkingMessagesPlugin::new());

    assert!(app
        .world()
        .contains_resource::<SteamworksNetworkingMessagesState>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingMessagesCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingMessagesResult>>());
    assert!(app
        .world()
        .resource::<SteamworksNetworkingMessagesState>()
        .auto_accept_session_requests());
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksNetworkingMessagesPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksNetworkingMessagesCommand>>()
        .write(SteamworksNetworkingMessagesCommand::receive_messages(0, 1));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksNetworkingMessagesResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksNetworkingMessagesResult::Err {
            command: SteamworksNetworkingMessagesCommand::receive_messages(0, 1),
            error: SteamworksNetworkingMessagesError::ClientUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksNetworkingMessagesState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksNetworkingMessagesError::ClientUnavailable)
    );
}

#[test]
fn local_auto_accept_command_updates_without_client() {
    let mut app = App::new();

    app.add_plugins(SteamworksNetworkingMessagesPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksNetworkingMessagesCommand>>()
        .write(SteamworksNetworkingMessagesCommand::set_auto_accept_session_requests(false));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksNetworkingMessagesResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksNetworkingMessagesResult::Ok(
            SteamworksNetworkingMessagesOperation::AutoAcceptSessionRequestsSet { enabled: false }
        )]
    );
    assert!(!app
        .world()
        .resource::<SteamworksNetworkingMessagesState>()
        .auto_accept_session_requests());
}

#[derive(Default, Resource)]
struct ObservedAutoAccept(bool);

fn observe_auto_accept_policy(
    state: Res<SteamworksNetworkingMessagesState>,
    mut observed: ResMut<ObservedAutoAccept>,
) {
    observed.0 = state.auto_accept_session_requests();
}

#[test]
fn auto_accept_command_applies_before_run_callbacks_set() {
    let mut app = App::new();

    app.insert_resource(ObservedAutoAccept(true));
    app.add_plugins(SteamworksNetworkingMessagesPlugin::new());
    app.add_systems(
        First,
        observe_auto_accept_policy
            .after(SteamworksSystem::RunCallbacks)
            .before(SteamworksSystem::ProcessNetworkingMessagesCommands),
    );
    app.world_mut()
        .resource_mut::<Messages<SteamworksNetworkingMessagesCommand>>()
        .write(SteamworksNetworkingMessagesCommand::set_auto_accept_session_requests(false));

    app.update();

    assert!(!app.world().resource::<ObservedAutoAccept>().0);
}

#[test]
fn constructors_preserve_inputs() {
    let steam_id = steamworks::SteamId::from_raw(42);
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 27015));
    let identity = steamworks::networking_types::NetworkingIdentity::new_steam_id(steam_id);
    let peer = SteamworksNetworkingPeer::steam_id(steam_id);

    assert_eq!(peer, SteamworksNetworkingPeer::SteamId(steam_id));
    assert_eq!(
        SteamworksNetworkingPeer::ip(addr),
        SteamworksNetworkingPeer::Ip(addr)
    );
    assert_eq!(
        SteamworksNetworkingPeer::local_host(),
        SteamworksNetworkingPeer::LocalHost
    );
    assert_eq!(
        SteamworksNetworkingPeer::identity(identity.clone()),
        SteamworksNetworkingPeer::Identity(identity.clone())
    );
    assert_eq!(
        SteamworksNetworkingPeer::from(steam_id),
        SteamworksNetworkingPeer::SteamId(steam_id)
    );
    assert_eq!(
        SteamworksNetworkingPeer::from(addr),
        SteamworksNetworkingPeer::Ip(addr)
    );
    assert_eq!(
        SteamworksNetworkingPeer::from(identity.clone()),
        SteamworksNetworkingPeer::Identity(identity.clone())
    );
    assert_eq!(
        SteamworksNetworkingPeer::from(steam_id).to_identity(),
        steamworks::networking_types::NetworkingIdentity::new_steam_id(steam_id)
    );
    assert_eq!(
        SteamworksNetworkingPeer::from(addr).to_identity(),
        steamworks::networking_types::NetworkingIdentity::new_ip(addr)
    );
    assert_eq!(
        steamworks::networking_types::NetworkingIdentity::from(SteamworksNetworkingPeer::from(
            steam_id
        )),
        steamworks::networking_types::NetworkingIdentity::new_steam_id(steam_id)
    );

    assert_eq!(
        SteamworksNetworkingMessagesCommand::send_message(
            steam_id,
            steamworks::networking_types::SendFlags::UNRELIABLE,
            7,
            [1, 2, 3],
        ),
        SteamworksNetworkingMessagesCommand::SendMessage {
            peer: SteamworksNetworkingPeer::SteamId(steam_id),
            send_flags: steamworks::networking_types::SendFlags::UNRELIABLE,
            channel: 7,
            data: vec![1, 2, 3],
        }
    );
    assert_eq!(
        SteamworksNetworkingMessagesCommand::send_message_to_steam_id(
            steam_id,
            steamworks::networking_types::SendFlags::RELIABLE,
            3,
            [4, 5],
        ),
        SteamworksNetworkingMessagesCommand::SendMessage {
            peer: SteamworksNetworkingPeer::SteamId(steam_id),
            send_flags: steamworks::networking_types::SendFlags::RELIABLE,
            channel: 3,
            data: vec![4, 5],
        }
    );
    assert_eq!(
        SteamworksNetworkingMessagesCommand::receive_messages(2, 16),
        SteamworksNetworkingMessagesCommand::ReceiveMessages {
            channel: 2,
            batch_size: 16,
        }
    );
    assert_eq!(
        SteamworksNetworkingMessagesCommand::get_session_connection_info(addr),
        SteamworksNetworkingMessagesCommand::GetSessionConnectionInfo {
            peer: SteamworksNetworkingPeer::Ip(addr)
        }
    );
    assert_eq!(
        SteamworksNetworkingMessagesCommand::set_auto_accept_session_requests(false),
        SteamworksNetworkingMessagesCommand::SetAutoAcceptSessionRequests { enabled: false }
    );
}

#[test]
fn state_records_operations_without_unbounded_message_history() {
    let mut state = SteamworksNetworkingMessagesState::new(true);
    let peer = steamworks::networking_types::NetworkingIdentity::new_steam_id(
        steamworks::SteamId::from_raw(42),
    );
    let first = SteamworksNetworkingMessage {
        peer: peer.clone(),
        data: vec![1],
        channel: 0,
        send_flags: steamworks::networking_types::SendFlags::RELIABLE,
        message_number: 1,
        connection_user_data: 0,
    };
    let second = SteamworksNetworkingMessage {
        peer: peer.clone(),
        data: vec![2],
        channel: 0,
        send_flags: steamworks::networking_types::SendFlags::RELIABLE,
        message_number: 2,
        connection_user_data: 0,
    };

    state.record_operation(&SteamworksNetworkingMessagesOperation::MessagesReceived {
        channel: 0,
        messages: vec![first],
    });
    state.record_operation(&SteamworksNetworkingMessagesOperation::MessagesReceived {
        channel: 0,
        messages: vec![second.clone()],
    });
    state.record_operation(&SteamworksNetworkingMessagesOperation::MessageSent {
        peer: SteamworksNetworkingPeer::identity(peer.clone()),
        channel: 0,
        send_flags: steamworks::networking_types::SendFlags::RELIABLE,
        bytes: 3,
    });
    state.record_operation(
        &SteamworksNetworkingMessagesOperation::SessionRequestReceived {
            request: SteamworksNetworkingMessagesSessionRequestInfo {
                remote: peer,
                accepted: true,
            },
        },
    );
    state.record_operation(
        &SteamworksNetworkingMessagesOperation::AutoAcceptSessionRequestsSet { enabled: false },
    );

    assert_eq!(state.received_messages(), &[second]);
    assert_eq!(state.received_count(), 2);
    assert_eq!(state.sent_count(), 1);
    assert_eq!(state.session_request_count(), 1);
    assert!(!state.auto_accept_session_requests());
}
