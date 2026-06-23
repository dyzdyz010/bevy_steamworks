use std::time::Duration;

use bevy_app::{App, Plugin};
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
fn plugin_name_matches_timeline_type_path_for_bevy_tracking() {
    let plugin = SteamworksTimelinePlugin::new();

    assert_eq!(
        plugin.name(),
        std::any::type_name::<SteamworksTimelinePlugin>()
    );
    assert_eq!(
        plugin.name(),
        "bevy_steamworks::timeline::SteamworksTimelinePlugin"
    );
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
fn state_tracks_events_and_count_without_unbounded_history() {
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
    assert!(state.has_state_description());
    assert_eq!(state.state_description_text(), Some("boss phase"));
    assert_eq!(
        state.state_description_duration(),
        Some(Duration::from_secs(5))
    );

    state.record_operation(&SteamworksTimelineOperation::StateDescriptionCleared {
        duration: Duration::ZERO,
    });
    assert_eq!(state.state_description(), None);
    assert!(!state.has_state_description());
    assert_eq!(state.state_description_text(), None);
    assert_eq!(state.state_description_duration(), None);

    state.record_operation(&SteamworksTimelineOperation::TimelineEventAdded { event: first });
    state.record_operation(&SteamworksTimelineOperation::TimelineEventAdded {
        event: second.clone(),
    });

    assert_eq!(state.event_count(), 2);
    assert!(state.has_events());
    assert_eq!(state.cached_event_count(), 2);
    assert_eq!(state.last_event(), Some(&second));
    assert_eq!(
        state
            .events_with_icon("second")
            .cloned()
            .collect::<Vec<_>>(),
        vec![second.clone()]
    );
    assert_eq!(state.last_event_with_icon("first"), state.events().first());
    assert_eq!(state.last_event_with_icon("missing"), None);
    assert_eq!(
        state
            .events_with_clip_priority(SteamworksTimelineEventClipPriority::Featured)
            .cloned()
            .collect::<Vec<_>>(),
        vec![second.clone()]
    );
    assert_eq!(
        state.last_event_with_clip_priority(SteamworksTimelineEventClipPriority::Featured),
        Some(&second)
    );
    assert_eq!(
        state.last_event_with_clip_priority(SteamworksTimelineEventClipPriority::None),
        None
    );
    assert_eq!(state.last_event_icon(), Some("second"));
    assert_eq!(state.last_event_title(), Some("second title"));
    assert_eq!(state.last_event_description(), Some("second description"));
    assert_eq!(state.last_event_priority(), Some(2));
    assert_eq!(state.last_event_start_offset_seconds(), Some(-1.0));
    assert_eq!(state.last_event_duration(), Some(Duration::from_secs(2)));
    assert_eq!(
        state.last_event_clip_priority(),
        Some(SteamworksTimelineEventClipPriority::Featured)
    );
    assert_eq!(state.events().len(), 2);
    assert_eq!(state.events().last(), Some(&second));

    state.set_event_count_for_test(u64::MAX);
    state.record_operation(&SteamworksTimelineOperation::TimelineEventAdded {
        event: second.clone(),
    });

    assert_eq!(state.event_count(), u64::MAX);
}

#[test]
fn timeline_event_history_is_bounded() {
    let mut state = SteamworksTimelineState::default();
    let limit = 1_024;

    for index in 0..(limit + 4) {
        state.record_operation(&SteamworksTimelineOperation::TimelineEventAdded {
            event: SteamworksTimelineEventInfo {
                icon: format!("icon-{index}"),
                title: format!("title-{index}"),
                description: format!("description-{index}"),
                priority: index as u32,
                start_offset_seconds: 0.0,
                duration: Duration::ZERO,
                clip_priority: SteamworksTimelineEventClipPriority::None,
            },
        });
    }

    assert_eq!(state.event_count(), (limit + 4) as u64);
    assert_eq!(state.events().len(), limit);
    assert_eq!(
        state.events().first().map(|event| event.icon.as_str()),
        Some("icon-4")
    );
    let expected_last_icon = format!("icon-{}", limit + 3);
    assert_eq!(
        state.last_event().map(|event| event.icon.as_str()),
        Some(expected_last_icon.as_str())
    );
}
