use std::time::Duration;

use bevy_app::App;
use bevy_ecs::message::Messages;

use super::*;

#[test]
fn timeline_plugin_registers_resources_and_messages() {
    let mut app = App::new();

    app.add_plugins(SteamworksTimelinePlugin::new());

    assert!(app.world().contains_resource::<SteamworksTimelineState>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksTimelineCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksTimelineResult>>());
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksTimelinePlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksTimelineCommand>>()
        .write(SteamworksTimelineCommand::set_game_mode(
            SteamworksTimelineGameMode::Playing,
        ));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksTimelineResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksTimelineResult::Err {
            command: SteamworksTimelineCommand::set_game_mode(SteamworksTimelineGameMode::Playing),
            error: SteamworksTimelineError::ClientUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksTimelineState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksTimelineError::ClientUnavailable)
    );
}

#[test]
fn validation_rejects_interior_nul() {
    let command =
        SteamworksTimelineCommand::set_state_description("boss\0phase", Duration::from_secs(1));

    assert_eq!(
        validate_command(&command),
        Err(SteamworksTimelineError::InvalidString {
            field: "description",
        })
    );

    let command = SteamworksTimelineCommand::add_event(SteamworksTimelineEventInfo {
        icon: "skull".to_owned(),
        title: "wipe\0bad".to_owned(),
        description: "party defeated".to_owned(),
        priority: 10,
        start_offset_seconds: 0.0,
        duration: Duration::ZERO,
        clip_priority: SteamworksTimelineEventClipPriority::Featured,
    });

    assert_eq!(
        validate_command(&command),
        Err(SteamworksTimelineError::InvalidString { field: "title" })
    );
}

#[test]
fn validation_rejects_non_finite_values() {
    let command = SteamworksTimelineCommand::add_event(SteamworksTimelineEventInfo {
        icon: "star".to_owned(),
        title: "win".to_owned(),
        description: "match won".to_owned(),
        priority: 1,
        start_offset_seconds: f32::NAN,
        duration: Duration::from_secs(1),
        clip_priority: SteamworksTimelineEventClipPriority::Standard,
    });

    assert_eq!(
        validate_command(&command),
        Err(SteamworksTimelineError::InvalidFloat {
            field: "start_offset_seconds",
        })
    );
}

#[test]
fn state_tracks_last_event_and_count_without_unbounded_history() {
    let mut state = SteamworksTimelineState::default();
    let state_description = SteamworksTimelineStateDescription {
        description: "boss phase".to_owned(),
        duration: Duration::from_secs(5),
    };
    let first = SteamworksTimelineEventInfo {
        icon: "first".to_owned(),
        title: "first title".to_owned(),
        description: "first description".to_owned(),
        priority: 1,
        start_offset_seconds: 0.0,
        duration: Duration::ZERO,
        clip_priority: SteamworksTimelineEventClipPriority::Standard,
    };
    let second = SteamworksTimelineEventInfo {
        icon: "second".to_owned(),
        title: "second title".to_owned(),
        description: "second description".to_owned(),
        priority: 2,
        start_offset_seconds: -1.0,
        duration: Duration::from_secs(2),
        clip_priority: SteamworksTimelineEventClipPriority::Featured,
    };

    state.record_operation(&SteamworksTimelineOperation::StateDescriptionSet {
        description: state_description.description.clone(),
        duration: state_description.duration,
    });
    assert_eq!(state.state_description(), Some(&state_description));

    state.record_operation(&SteamworksTimelineOperation::StateDescriptionCleared {
        duration: Duration::ZERO,
    });
    assert_eq!(state.state_description(), None);

    state.record_operation(&SteamworksTimelineOperation::TimelineEventAdded { event: first });
    state.record_operation(&SteamworksTimelineOperation::TimelineEventAdded {
        event: second.clone(),
    });

    assert_eq!(state.event_count(), 2);
    assert_eq!(state.last_event(), Some(&second));

    state.set_event_count_for_test(u64::MAX);
    state.record_operation(&SteamworksTimelineOperation::TimelineEventAdded {
        event: second.clone(),
    });

    assert_eq!(state.event_count(), u64::MAX);
}
