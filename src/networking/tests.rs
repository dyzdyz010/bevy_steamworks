use bevy_app::{App, Plugin};
use bevy_ecs::message::Messages;

use crate::SteamworksEvent;

use super::*;

fn user() -> steamworks::SteamId {
    steamworks::SteamId::from_raw(42)
}

fn other_user() -> steamworks::SteamId {
    steamworks::SteamId::from_raw(43)
}

fn packet(data: &[u8]) -> SteamworksP2pPacket {
    SteamworksP2pPacket {
        remote: user(),
        channel: 0,
        data: data.to_vec(),
    }
}

#[test]
fn networking_plugin_registers_resources_and_messages() {
    let mut app = App::new();

    app.add_plugins(SteamworksNetworkingPlugin::new());

    assert!(app.world().contains_resource::<SteamworksNetworkingState>());
    assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingResult>>());
}

#[test]
fn plugin_name_matches_networking_type_path_for_bevy_tracking() {
    let plugin = SteamworksNetworkingPlugin::new();

    assert_eq!(
        plugin.name(),
        std::any::type_name::<SteamworksNetworkingPlugin>()
    );
    assert_eq!(
        plugin.name(),
        "bevy_steamworks::networking::SteamworksNetworkingPlugin"
    );
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksNetworkingPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksNetworkingCommand>>()
        .write(SteamworksNetworkingCommand::get_available_packet_size(0));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksNetworkingResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksNetworkingResult::Err {
            command: SteamworksNetworkingCommand::get_available_packet_size(0),
            error: SteamworksNetworkingError::ClientUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksNetworkingState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksNetworkingError::ClientUnavailable)
    );
}

#[test]
fn p2p_callback_events_are_bridged_without_client() {
    let mut app = App::new();

    app.add_plugins(SteamworksNetworkingPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::P2PSessionRequest(
            steamworks::P2PSessionRequest { remote: user() },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::P2PSessionConnectFail(
            steamworks::P2PSessionConnectFail {
                remote: user(),
                error: 4,
            },
        ));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksNetworkingResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![
            SteamworksNetworkingResult::Ok(SteamworksNetworkingOperation::SessionRequestReceived {
                remote: user()
            },),
            SteamworksNetworkingResult::Ok(SteamworksNetworkingOperation::SessionConnectFailed {
                remote: user(),
                error: steamworks::P2PSessionError::Timeout,
            },),
        ]
    );

    let state = app.world().resource::<SteamworksNetworkingState>();
    assert_eq!(state.session_request_count(), 1);
    assert_eq!(state.last_session_request(), Some(user()));
    assert_eq!(state.session_connect_failure_count(), 1);
    assert_eq!(
        state.last_session_connect_failure(),
        Some(SteamworksP2pSessionConnectFailure {
            remote: user(),
            error: steamworks::P2PSessionError::Timeout,
        })
    );
}

#[test]
fn packet_exceeds_read_buffer_error_is_cloneable_and_comparable() {
    assert_eq!(
        SteamworksNetworkingError::PacketExceedsReadBuffer {
            available_bytes: 4097,
            max_bytes: 4096,
        }
        .clone(),
        SteamworksNetworkingError::PacketExceedsReadBuffer {
            available_bytes: 4097,
            max_bytes: 4096,
        }
    );
}

#[test]
fn constructors_preserve_inputs() {
    assert_eq!(
        SteamworksNetworkingCommand::send_p2p_packet(
            user(),
            SteamworksP2pSendType::ReliableWithBuffering,
            7,
            vec![1, 2, 3],
        ),
        SteamworksNetworkingCommand::SendP2pPacket {
            remote: user(),
            send_type: SteamworksP2pSendType::ReliableWithBuffering,
            channel: 7,
            data: vec![1, 2, 3],
        }
    );
    assert_eq!(
        SteamworksNetworkingCommand::read_p2p_packet(3, 1024),
        SteamworksNetworkingCommand::ReadP2pPacket {
            channel: 3,
            max_bytes: 1024,
        }
    );
}

#[test]
fn state_records_operations_without_unbounded_packet_history() {
    let mut state = SteamworksNetworkingState::default();
    let first = packet(&[1]);
    let second = packet(&[2, 3]);
    let third = SteamworksP2pPacket {
        remote: other_user(),
        channel: 1,
        data: vec![4],
    };
    let session_state = SteamworksP2pSessionStateResult {
        user: user(),
        state: None,
    };

    state.record_operation(&SteamworksNetworkingOperation::SessionAccepted { user: user() });
    state.record_operation(&SteamworksNetworkingOperation::SessionStateRead {
        state: session_state.clone(),
    });
    state.record_operation(&SteamworksNetworkingOperation::PacketRead {
        channel: 0,
        packet: Some(first.clone()),
    });
    state.record_operation(&SteamworksNetworkingOperation::PacketRead {
        channel: 0,
        packet: Some(second.clone()),
    });
    state.record_operation(&SteamworksNetworkingOperation::PacketRead {
        channel: 1,
        packet: Some(third.clone()),
    });
    state.record_operation(&SteamworksNetworkingOperation::PacketRead {
        channel: 7,
        packet: None,
    });
    state.record_operation(&SteamworksNetworkingOperation::PacketSent {
        remote: user(),
        send_type: SteamworksP2pSendType::Reliable,
        channel: 0,
        bytes: 3,
    });
    state.record_operation(&SteamworksNetworkingOperation::PacketAvailabilityRead {
        availability: SteamworksP2pPacketAvailability {
            channel: 0,
            bytes: Some(2),
        },
    });
    state.record_operation(&SteamworksNetworkingOperation::SessionRequestReceived {
        remote: user(),
    });
    state.record_operation(&SteamworksNetworkingOperation::SessionConnectFailed {
        remote: user(),
        error: steamworks::P2PSessionError::NoRightsToApp,
    });

    assert_eq!(state.last_accepted_session(), Some(user()));
    assert_eq!(state.last_session_state(), Some(&session_state));
    assert_eq!(state.session_states(), &[session_state.clone()]);
    assert_eq!(state.session_state(user()), Some(&session_state));
    assert_eq!(state.received_count(), 3);
    assert_eq!(
        state.received_packets(),
        &[first.clone(), second.clone(), third.clone()]
    );
    assert_eq!(
        state
            .received_packets_from(user())
            .cloned()
            .collect::<Vec<_>>(),
        vec![first.clone(), second.clone()]
    );
    assert_eq!(
        state
            .received_packets_on_channel(1)
            .cloned()
            .collect::<Vec<_>>(),
        vec![third.clone()]
    );
    assert_eq!(state.last_packet_from(user()), Some(&second));
    assert_eq!(state.last_packet_on_channel(1), Some(&third));
    assert_eq!(state.sent_count(), 1);
    assert_eq!(
        state.last_sent_packet(),
        Some(SteamworksP2pPacketSent {
            remote: user(),
            send_type: SteamworksP2pSendType::Reliable,
            channel: 0,
            bytes: 3,
        })
    );
    assert_eq!(state.empty_read_count(), 1);
    assert_eq!(state.last_empty_read_channel(), Some(7));
    assert_eq!(state.last_packet(), Some(&third));
    assert_eq!(
        state.last_packet_availability(),
        Some(&SteamworksP2pPacketAvailability {
            channel: 0,
            bytes: Some(2),
        })
    );
    assert_eq!(
        state.packet_availability(0),
        Some(&SteamworksP2pPacketAvailability {
            channel: 0,
            bytes: Some(2),
        })
    );
    assert_eq!(state.session_request_count(), 1);
    assert_eq!(state.last_session_request(), Some(user()));
    assert_eq!(state.session_requests(), &[user()]);
    assert!(state.has_session_request(user()));
    assert_eq!(state.session_connect_failure_count(), 1);
    assert_eq!(
        state.last_session_connect_failure(),
        Some(SteamworksP2pSessionConnectFailure {
            remote: user(),
            error: steamworks::P2PSessionError::NoRightsToApp,
        })
    );
    assert_eq!(
        state.session_connect_failure(user()),
        Some(SteamworksP2pSessionConnectFailure {
            remote: user(),
            error: steamworks::P2PSessionError::NoRightsToApp,
        })
    );

    state.record_operation(&SteamworksNetworkingOperation::SessionClosed { user: user() });

    assert_eq!(state.last_closed_session(), Some(user()));
    assert!(state.last_session_state().is_none());
    assert_eq!(state.session_state(user()), None);
}

#[test]
fn state_lookup_caches_are_bounded() {
    let mut state = SteamworksNetworkingState::default();

    for raw in 1..=(super::state::STEAMWORKS_NETWORKING_STATE_CACHE_LIMIT as u64 + 1) {
        let remote = steamworks::SteamId::from_raw(raw);
        let channel = raw as u32;
        state.record_operation(&SteamworksNetworkingOperation::SessionStateRead {
            state: SteamworksP2pSessionStateResult {
                user: remote,
                state: None,
            },
        });
        state.record_operation(&SteamworksNetworkingOperation::PacketAvailabilityRead {
            availability: SteamworksP2pPacketAvailability {
                channel,
                bytes: Some(raw as usize),
            },
        });
        state.record_operation(&SteamworksNetworkingOperation::PacketRead {
            channel,
            packet: Some(SteamworksP2pPacket {
                remote,
                channel,
                data: vec![raw as u8],
            }),
        });
        state.record_operation(&SteamworksNetworkingOperation::SessionRequestReceived { remote });
        state.record_operation(&SteamworksNetworkingOperation::SessionConnectFailed {
            remote,
            error: steamworks::P2PSessionError::Timeout,
        });
    }

    let first = steamworks::SteamId::from_raw(1);
    let second = steamworks::SteamId::from_raw(2);

    assert_eq!(
        state.session_states().len(),
        super::state::STEAMWORKS_NETWORKING_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.packet_availabilities().len(),
        super::state::STEAMWORKS_NETWORKING_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.received_packets().len(),
        super::state::STEAMWORKS_NETWORKING_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.session_requests().len(),
        super::state::STEAMWORKS_NETWORKING_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.session_connect_failures().len(),
        super::state::STEAMWORKS_NETWORKING_STATE_CACHE_LIMIT
    );
    assert_eq!(state.session_state(first), None);
    assert_eq!(state.packet_availability(1), None);
    assert_eq!(state.last_packet_from(first), None);
    assert!(!state.has_session_request(first));
    assert_eq!(state.session_connect_failure(first), None);
    assert!(state.session_state(second).is_some());
    assert_eq!(
        state.packet_availability(2),
        Some(&SteamworksP2pPacketAvailability {
            channel: 2,
            bytes: Some(2),
        })
    );
    assert!(state.last_packet_from(second).is_some());
    assert!(state.has_session_request(second));
    assert_eq!(
        state.session_connect_failure(second),
        Some(SteamworksP2pSessionConnectFailure {
            remote: second,
            error: steamworks::P2PSessionError::Timeout,
        })
    );
}
