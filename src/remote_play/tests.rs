use bevy_app::{App, Plugin};
use bevy_ecs::message::Messages;

use crate::SteamworksEvent;

use super::*;

#[test]
fn remote_play_plugin_registers_resources_and_messages() {
    let mut app = App::new();

    app.add_plugins(SteamworksRemotePlayPlugin::new());

    assert!(app.world().contains_resource::<SteamworksRemotePlayState>());
    assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksRemotePlayCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksRemotePlayResult>>());
}

#[test]
fn plugin_name_matches_remote_play_type_path_for_bevy_tracking() {
    let plugin = SteamworksRemotePlayPlugin::new();

    assert_eq!(
        plugin.name(),
        std::any::type_name::<SteamworksRemotePlayPlugin>()
    );
    assert_eq!(
        plugin.name(),
        "bevy_steamworks::remote_play::SteamworksRemotePlayPlugin"
    );
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksRemotePlayPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksRemotePlayCommand>>()
        .write(SteamworksRemotePlayCommand::list_sessions());

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksRemotePlayResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksRemotePlayResult::Err {
            command: SteamworksRemotePlayCommand::ListSessions,
            error: SteamworksRemotePlayError::ClientUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksRemotePlayState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksRemotePlayError::ClientUnavailable)
    );
}

#[test]
fn constructors_preserve_session_context() {
    let session = steamworks::RemotePlaySessionId::from_raw(7);
    let friend = steamworks::SteamId::from_raw(42);

    assert_eq!(
        SteamworksRemotePlayCommand::list_sessions(),
        SteamworksRemotePlayCommand::ListSessions
    );
    assert_eq!(
        SteamworksRemotePlayCommand::get_session(session),
        SteamworksRemotePlayCommand::GetSession { session }
    );
    assert_eq!(
        SteamworksRemotePlayCommand::invite(session, friend),
        SteamworksRemotePlayCommand::Invite { session, friend }
    );
}

#[test]
fn state_records_remote_play_operations() {
    let mut state = SteamworksRemotePlayState::default();
    let first_session = steamworks::RemotePlaySessionId::from_raw(1);
    let second_session = steamworks::RemotePlaySessionId::from_raw(2);
    let friend = steamworks::SteamId::from_raw(42);
    let first_info = SteamworksRemotePlaySessionInfo {
        session: first_session,
        user: steamworks::SteamId::from_raw(100),
        client_name: Some("Deck".to_owned()),
        client_form_factor: None,
        client_resolution: Some((1280, 800)),
    };
    let updated_info = SteamworksRemotePlaySessionInfo {
        client_name: Some("Living Room".to_owned()),
        client_resolution: Some((1920, 1080)),
        ..first_info.clone()
    };
    let listed_session = SteamworksRemotePlaySessionSnapshot {
        user: steamworks::SteamId::from_raw(200),
        client_name: Some("Laptop".to_owned()),
        client_form_factor: None,
        client_resolution: None,
    };

    state.record_operation(&SteamworksRemotePlayOperation::SessionsListed {
        sessions: vec![listed_session.clone()],
    });
    state.record_operation(&SteamworksRemotePlayOperation::SessionRead {
        session: first_info,
    });
    state.record_operation(&SteamworksRemotePlayOperation::SessionRead {
        session: updated_info.clone(),
    });
    state.record_operation(&SteamworksRemotePlayOperation::SessionConnected {
        session: first_session,
    });
    state.record_operation(&SteamworksRemotePlayOperation::SessionConnected {
        session: first_session,
    });
    state.record_operation(&SteamworksRemotePlayOperation::SessionConnected {
        session: second_session,
    });
    state.record_operation(&SteamworksRemotePlayOperation::InviteSubmitted {
        session: first_session,
        friend,
    });
    state.record_operation(&SteamworksRemotePlayOperation::InviteSubmitted {
        session: second_session,
        friend,
    });

    assert_eq!(state.sessions(), std::slice::from_ref(&listed_session));
    assert_eq!(state.session_count(), 1);
    assert_eq!(
        state
            .sessions_for_user(listed_session.user)
            .cloned()
            .collect::<Vec<_>>(),
        vec![listed_session.clone()]
    );
    assert_eq!(
        state.latest_session_for_user(listed_session.user),
        Some(&listed_session)
    );
    assert_eq!(
        state.latest_session_for_user(steamworks::SteamId::from_raw(999)),
        None
    );
    assert_eq!(state.known_sessions(), std::slice::from_ref(&updated_info));
    assert_eq!(state.known_session_count(), 1);
    assert_eq!(state.known_session(first_session), Some(&updated_info));
    assert!(state.has_known_session(first_session));
    assert!(state.known_session(second_session).is_none());
    assert!(!state.has_known_session(second_session));
    assert_eq!(
        state
            .known_sessions_for_user(updated_info.user)
            .cloned()
            .collect::<Vec<_>>(),
        vec![updated_info.clone()]
    );
    assert_eq!(
        state.latest_known_session_for_user(updated_info.user),
        Some(&updated_info)
    );
    assert_eq!(
        state.latest_known_session_for_user(steamworks::SteamId::from_raw(999)),
        None
    );
    assert_eq!(state.session_user(first_session), Some(updated_info.user));
    assert_eq!(state.session_user(second_session), None);
    assert_eq!(
        state.session_client_name(first_session),
        Some(Some("Living Room"))
    );
    assert_eq!(state.session_client_name(second_session), None);
    assert_eq!(state.session_client_form_factor(first_session), Some(None));
    assert_eq!(
        state.session_client_resolution(first_session),
        Some(Some((1920, 1080)))
    );
    assert_eq!(
        state.observed_connected_sessions(),
        &[first_session, second_session]
    );
    assert_eq!(state.observed_connected_session_count(), 2);
    assert!(state.is_session_observed_connected(first_session));
    assert_eq!(
        state.last_submitted_invite(),
        Some(SteamworksRemotePlayInvite {
            session: second_session,
            friend,
        })
    );
    assert_eq!(state.submitted_invite_count(), 2);

    state.record_operation(&SteamworksRemotePlayOperation::SessionDisconnected {
        session: first_session,
    });

    assert_eq!(state.observed_connected_sessions(), &[second_session]);
    assert_eq!(state.observed_connected_session_count(), 1);
    assert!(!state.is_session_observed_connected(first_session));
    assert!(!state.has_known_session(first_session));
    assert!(state.known_session(first_session).is_none());
    assert_eq!(state.known_session_count(), 0);
    assert_eq!(state.session_user(first_session), None);
    assert_eq!(state.session_client_name(first_session), None);
    assert_eq!(state.session_client_form_factor(first_session), None);
    assert_eq!(state.session_client_resolution(first_session), None);
    assert_eq!(state.latest_known_session_for_user(updated_info.user), None);
}

#[test]
fn remote_play_session_caches_are_bounded() {
    let mut state = SteamworksRemotePlayState::default();

    for raw in 1..=(super::state::STEAMWORKS_REMOTE_PLAY_STATE_CACHE_LIMIT as u32 + 1) {
        let session = steamworks::RemotePlaySessionId::from_raw(raw);
        state.record_operation(&SteamworksRemotePlayOperation::SessionRead {
            session: SteamworksRemotePlaySessionInfo {
                session,
                user: steamworks::SteamId::from_raw(raw as u64),
                client_name: Some(format!("Client {raw}")),
                client_form_factor: None,
                client_resolution: Some((1280, 720)),
            },
        });
        state.record_operation(&SteamworksRemotePlayOperation::SessionConnected { session });
    }

    assert_eq!(
        state.known_sessions().len(),
        super::state::STEAMWORKS_REMOTE_PLAY_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.observed_connected_sessions().len(),
        super::state::STEAMWORKS_REMOTE_PLAY_STATE_CACHE_LIMIT
    );

    let evicted = steamworks::RemotePlaySessionId::from_raw(1);
    let retained = steamworks::RemotePlaySessionId::from_raw(2);
    assert_eq!(state.known_session(evicted), None);
    assert!(!state.is_session_observed_connected(evicted));
    assert!(state.known_session(retained).is_some());
    assert!(state.is_session_observed_connected(retained));
}

#[test]
fn remote_play_callbacks_are_bridged_without_client() {
    let mut app = App::new();
    let first = steamworks::RemotePlaySessionId::from_raw(1);
    let second = steamworks::RemotePlaySessionId::from_raw(2);

    app.add_plugins(SteamworksRemotePlayPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::RemotePlayConnected(
            steamworks::RemotePlayConnected { session: first },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::RemotePlayConnected(
            steamworks::RemotePlayConnected { session: second },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::RemotePlayDisconnected(
            steamworks::RemotePlayDisconnected { session: first },
        ));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksRemotePlayResult>>();
    let drained = results.drain().collect::<Vec<_>>();
    assert_eq!(
        drained,
        vec![
            SteamworksRemotePlayResult::Ok(SteamworksRemotePlayOperation::SessionConnected {
                session: first
            },),
            SteamworksRemotePlayResult::Ok(SteamworksRemotePlayOperation::SessionConnected {
                session: second
            },),
            SteamworksRemotePlayResult::Ok(SteamworksRemotePlayOperation::SessionDisconnected {
                session: first
            },),
        ]
    );

    let state = app.world().resource::<SteamworksRemotePlayState>();
    assert_eq!(state.observed_connected_sessions(), &[second]);
    assert_eq!(state.last_error(), None);
}
