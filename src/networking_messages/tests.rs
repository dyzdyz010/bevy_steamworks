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
    let other_peer = steamworks::networking_types::NetworkingIdentity::new_steam_id(
        steamworks::SteamId::from_raw(43),
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
    let third = SteamworksNetworkingMessage {
        peer: other_peer.clone(),
        data: vec![3],
        channel: 1,
        send_flags: steamworks::networking_types::SendFlags::UNRELIABLE,
        message_number: 3,
        connection_user_data: 0,
    };

    state.record_operation(&SteamworksNetworkingMessagesOperation::MessagesReceived {
        channel: 0,
        messages: vec![first.clone()],
    });
    state.record_operation(&SteamworksNetworkingMessagesOperation::MessagesReceived {
        channel: 0,
        messages: vec![second.clone(), third.clone()],
    });
    state.record_operation(&SteamworksNetworkingMessagesOperation::MessageSent {
        peer: SteamworksNetworkingPeer::identity(peer.clone()),
        channel: 0,
        send_flags: steamworks::networking_types::SendFlags::RELIABLE,
        bytes: 3,
    });
    state.record_operation(
        &SteamworksNetworkingMessagesOperation::SessionConnectionInfoRead {
            peer: SteamworksNetworkingPeer::identity(peer.clone()),
            info: SteamworksNetworkingMessagesConnectionInfo {
                state: steamworks::networking_types::NetworkingConnectionState::Connected,
                remote: Some(peer.clone()),
                user_data: Some(9),
                end_reason: None,
                realtime: Some(SteamworksNetworkingMessagesRealtimeInfo {
                    connection_state:
                        steamworks::networking_types::NetworkingConnectionState::Connected,
                    ping: 42,
                    connection_quality_local: 0.9,
                    connection_quality_remote: 0.8,
                    out_packets_per_sec: 1.0,
                    out_bytes_per_sec: 2.0,
                    in_packets_per_sec: 3.0,
                    in_bytes_per_sec: 4.0,
                    send_rate_bytes_per_sec: 1024,
                    pending_unreliable: 5,
                    pending_reliable: 6,
                }),
            },
        },
    );
    state.record_operation(
        &SteamworksNetworkingMessagesOperation::SessionRequestReceived {
            request: SteamworksNetworkingMessagesSessionRequestInfo {
                remote: peer.clone(),
                accepted: true,
            },
        },
    );
    state.record_operation(&SteamworksNetworkingMessagesOperation::SessionFailed {
        info: SteamworksNetworkingMessagesConnectionInfo {
            state: steamworks::networking_types::NetworkingConnectionState::ProblemDetectedLocally,
            remote: Some(peer.clone()),
            user_data: Some(7),
            end_reason: None,
            realtime: None,
        },
    });
    state.record_operation(
        &SteamworksNetworkingMessagesOperation::AutoAcceptSessionRequestsSet { enabled: false },
    );

    assert_eq!(state.received_messages(), &[second.clone(), third.clone()]);
    assert_eq!(state.received_message_count(), 2);
    assert_eq!(
        state.recent_received_messages(),
        &[first.clone(), second.clone(), third.clone()]
    );
    assert_eq!(state.recent_received_message_count(), 3);
    assert_eq!(state.last_received_message(), Some(&third));
    assert_eq!(state.last_received_message_peer(), Some(&other_peer));
    assert_eq!(state.last_received_message_channel(), Some(1));
    assert_eq!(state.last_received_message_bytes(), Some(1));
    assert_eq!(state.last_received_message_data(), Some([3].as_slice()));
    assert_eq!(
        state
            .received_messages_on_channel(0)
            .cloned()
            .collect::<Vec<_>>(),
        vec![second.clone()]
    );
    assert_eq!(state.received_message_count_on_channel(0), 1);
    assert_eq!(
        state
            .recent_received_messages_on_channel(0)
            .cloned()
            .collect::<Vec<_>>(),
        vec![first.clone(), second.clone()]
    );
    assert_eq!(state.recent_received_message_count_on_channel(0), 2);
    assert_eq!(state.last_received_message_on_channel(1), Some(&third));
    assert_eq!(
        state.last_recent_received_message_on_channel(0),
        Some(&second)
    );
    assert_eq!(
        state
            .received_messages_from_peer(&peer)
            .cloned()
            .collect::<Vec<_>>(),
        vec![second.clone()]
    );
    assert_eq!(state.received_message_count_from_peer(&peer), 1);
    assert_eq!(
        state
            .recent_received_messages_from_peer(&peer)
            .cloned()
            .collect::<Vec<_>>(),
        vec![first.clone(), second.clone()]
    );
    assert_eq!(state.recent_received_message_count_from_peer(&peer), 2);
    assert_eq!(state.last_received_message_from_peer(&peer), Some(&second));
    assert_eq!(
        state.last_recent_received_message_from_peer(&peer),
        Some(&second)
    );
    assert_eq!(
        state.last_recent_received_message_bytes_from_peer(&peer),
        Some(1)
    );
    assert_eq!(
        state.last_connection_state(),
        Some(steamworks::networking_types::NetworkingConnectionState::Connected)
    );
    assert_eq!(state.last_connection_remote(), Some(Some(&peer)));
    assert_eq!(state.last_connection_user_data(), Some(Some(9)));
    assert_eq!(state.last_connection_end_reason(), Some(None));
    assert_eq!(state.last_connection_ping(), Some(42));
    assert_eq!(state.last_connection_quality(), Some((0.9, 0.8)));
    assert_eq!(state.received_count(), 3);
    assert_eq!(state.sent_count(), 1);
    assert_eq!(state.session_request_count(), 1);
    assert_eq!(state.session_requests().len(), 1);
    assert_eq!(state.cached_session_request_count(), 1);
    assert_eq!(
        state.session_request(&peer),
        Some(&SteamworksNetworkingMessagesSessionRequestInfo {
            remote: peer.clone(),
            accepted: true,
        })
    );
    assert!(state.has_session_request(&peer));
    assert_eq!(state.session_request_accepted(&peer), Some(true));
    assert_eq!(state.session_failure_count(), 1);
    assert_eq!(state.session_failures().len(), 1);
    assert_eq!(state.cached_session_failure_count(), 1);
    assert_eq!(
        state.session_failure(&peer),
        Some(&SteamworksNetworkingMessagesConnectionInfo {
            state: steamworks::networking_types::NetworkingConnectionState::ProblemDetectedLocally,
            remote: Some(peer.clone()),
            user_data: Some(7),
            end_reason: None,
            realtime: None,
        })
    );
    assert!(state.has_session_failure(&peer));
    assert_eq!(
        state.session_failure_state(&peer),
        Some(steamworks::networking_types::NetworkingConnectionState::ProblemDetectedLocally)
    );
    assert_eq!(state.session_failure_end_reason(&peer), Some(None));
    assert!(!state.auto_accept_session_requests());
}

#[test]
fn session_callback_caches_are_bounded() {
    let mut state = SteamworksNetworkingMessagesState::new(true);

    for raw in 1..=(super::state::STEAMWORKS_NETWORKING_MESSAGES_STATE_CACHE_LIMIT as u64 + 1) {
        let remote = steamworks::networking_types::NetworkingIdentity::new_steam_id(
            steamworks::SteamId::from_raw(raw),
        );
        state.record_operation(
            &SteamworksNetworkingMessagesOperation::SessionRequestReceived {
                request: SteamworksNetworkingMessagesSessionRequestInfo {
                    remote: remote.clone(),
                    accepted: raw % 2 == 0,
                },
            },
        );
        state.record_operation(&SteamworksNetworkingMessagesOperation::SessionFailed {
            info: SteamworksNetworkingMessagesConnectionInfo {
                state: steamworks::networking_types::NetworkingConnectionState::ClosedByPeer,
                remote: Some(remote),
                user_data: None,
                end_reason: None,
                realtime: None,
            },
        });
    }

    let first_remote = steamworks::networking_types::NetworkingIdentity::new_steam_id(
        steamworks::SteamId::from_raw(1),
    );
    let second_remote = steamworks::networking_types::NetworkingIdentity::new_steam_id(
        steamworks::SteamId::from_raw(2),
    );

    assert_eq!(
        state.session_requests().len(),
        super::state::STEAMWORKS_NETWORKING_MESSAGES_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.session_failures().len(),
        super::state::STEAMWORKS_NETWORKING_MESSAGES_STATE_CACHE_LIMIT
    );
    assert_eq!(state.session_request(&first_remote), None);
    assert_eq!(state.session_failure(&first_remote), None);
    assert_eq!(
        state.session_request(&second_remote),
        Some(&SteamworksNetworkingMessagesSessionRequestInfo {
            remote: second_remote.clone(),
            accepted: true,
        })
    );
    assert!(state.session_failure(&second_remote).is_some());
}

#[test]
fn recent_received_message_cache_is_bounded() {
    let mut state = SteamworksNetworkingMessagesState::new(true);

    for raw in 1..=(super::state::STEAMWORKS_NETWORKING_MESSAGES_STATE_CACHE_LIMIT as u64 + 1) {
        state.record_operation(&SteamworksNetworkingMessagesOperation::MessagesReceived {
            channel: raw as u32,
            messages: vec![SteamworksNetworkingMessage {
                peer: steamworks::networking_types::NetworkingIdentity::new_steam_id(
                    steamworks::SteamId::from_raw(raw),
                ),
                data: vec![raw as u8],
                channel: raw as i32,
                send_flags: steamworks::networking_types::SendFlags::RELIABLE,
                message_number: raw,
                connection_user_data: raw as i64,
            }],
        });
    }

    assert_eq!(
        state.recent_received_messages().len(),
        super::state::STEAMWORKS_NETWORKING_MESSAGES_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state
            .recent_received_messages()
            .first()
            .map(|message| message.message_number),
        Some(2)
    );
    assert_eq!(
        state
            .received_messages()
            .first()
            .map(|message| message.message_number),
        Some(super::state::STEAMWORKS_NETWORKING_MESSAGES_STATE_CACHE_LIMIT as u64 + 1)
    );
}
