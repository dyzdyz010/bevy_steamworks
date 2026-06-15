use std::path::PathBuf;

use bevy_app::{App, Plugin};
use bevy_ecs::message::Messages;

use crate::SteamworksEvent;

use super::*;

#[test]
fn screenshots_plugin_registers_resources_and_messages() {
    let mut app = App::new();

    app.add_plugins(SteamworksScreenshotsPlugin::new());

    assert!(app
        .world()
        .contains_resource::<SteamworksScreenshotsState>());
    assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksScreenshotsCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksScreenshotsResult>>());
}

#[test]
fn plugin_name_matches_screenshots_type_path_for_bevy_tracking() {
    let plugin = SteamworksScreenshotsPlugin::new();

    assert_eq!(
        plugin.name(),
        std::any::type_name::<SteamworksScreenshotsPlugin>()
    );
    assert_eq!(
        plugin.name(),
        "bevy_steamworks::screenshots::SteamworksScreenshotsPlugin"
    );
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksScreenshotsPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksScreenshotsCommand>>()
        .write(SteamworksScreenshotsCommand::is_screenshots_hooked());

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksScreenshotsResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksScreenshotsResult::Err {
            command: SteamworksScreenshotsCommand::IsScreenshotsHooked,
            error: SteamworksScreenshotsError::ClientUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksScreenshotsState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksScreenshotsError::ClientUnavailable)
    );
}

#[test]
fn constructors_preserve_inputs() {
    assert_eq!(
        SteamworksScreenshotsCommand::hook_screenshots(true),
        SteamworksScreenshotsCommand::HookScreenshots { hook: true }
    );
    assert_eq!(
        SteamworksScreenshotsCommand::is_screenshots_hooked(),
        SteamworksScreenshotsCommand::IsScreenshotsHooked
    );
    assert_eq!(
        SteamworksScreenshotsCommand::trigger_screenshot(),
        SteamworksScreenshotsCommand::TriggerScreenshot
    );
    assert_eq!(
        SteamworksScreenshotsCommand::add_screenshot_to_library(
            "shot.png",
            Some("thumb.png"),
            1920,
            1080,
        ),
        SteamworksScreenshotsCommand::AddScreenshotToLibrary {
            filename: PathBuf::from("shot.png"),
            thumbnail_filename: Some(PathBuf::from("thumb.png")),
            width: 1920,
            height: 1080,
        }
    );
}

#[test]
fn screenshot_library_errors_are_cloneable_and_comparable() {
    assert_eq!(
        SteamworksScreenshotLibraryError::from(
            steamworks::screenshots::ScreenshotLibraryAddError::SavingFailed
        ),
        SteamworksScreenshotLibraryError::SavingFailed
    );
    assert_eq!(
        SteamworksScreenshotLibraryError::from(
            steamworks::screenshots::ScreenshotLibraryAddError::InvalidPath
        ),
        SteamworksScreenshotLibraryError::InvalidPath
    );
}

#[test]
fn screenshot_ready_errors_are_cloneable_and_comparable() {
    assert_eq!(
        SteamworksScreenshotReadyError::from(steamworks::screenshots::ScreenshotReadyError::Fail),
        SteamworksScreenshotReadyError::Fail
    );
    assert_eq!(
        SteamworksScreenshotReadyError::from(
            steamworks::screenshots::ScreenshotReadyError::IoFailure
        ),
        SteamworksScreenshotReadyError::IoFailure
    );
}

#[test]
fn state_records_screenshot_operations() {
    let mut state = SteamworksScreenshotsState::default();
    let first_submission = SteamworksSubmittedScreenshot {
        handle: 11,
        filename: PathBuf::from("first.png"),
        thumbnail_filename: Some(PathBuf::from("first_thumb.png")),
        width: 1920,
        height: 1080,
    };
    let updated_submission = SteamworksSubmittedScreenshot {
        handle: 11,
        filename: PathBuf::from("updated.png"),
        thumbnail_filename: None,
        width: 1280,
        height: 720,
    };
    let second_submission = SteamworksSubmittedScreenshot {
        handle: 22,
        filename: PathBuf::from("second.png"),
        thumbnail_filename: None,
        width: 800,
        height: 600,
    };

    state.record_operation(&SteamworksScreenshotsOperation::ScreenshotsHookSet { hook: true });
    state
        .record_operation(&SteamworksScreenshotsOperation::ScreenshotsHookedRead { hooked: false });
    state.record_operation(&SteamworksScreenshotsOperation::ScreenshotTriggered);
    state.record_operation(&SteamworksScreenshotsOperation::ScreenshotTriggered);
    state.record_operation(
        &SteamworksScreenshotsOperation::ScreenshotLibraryAddSubmitted {
            handle: first_submission.handle,
            filename: first_submission.filename.clone(),
            thumbnail_filename: first_submission.thumbnail_filename.clone(),
            width: first_submission.width,
            height: first_submission.height,
        },
    );
    state.record_operation(
        &SteamworksScreenshotsOperation::ScreenshotLibraryAddSubmitted {
            handle: second_submission.handle,
            filename: second_submission.filename.clone(),
            thumbnail_filename: second_submission.thumbnail_filename.clone(),
            width: second_submission.width,
            height: second_submission.height,
        },
    );
    state.record_operation(
        &SteamworksScreenshotsOperation::ScreenshotLibraryAddSubmitted {
            handle: updated_submission.handle,
            filename: updated_submission.filename.clone(),
            thumbnail_filename: updated_submission.thumbnail_filename.clone(),
            width: updated_submission.width,
            height: updated_submission.height,
        },
    );
    state.record_operation(&SteamworksScreenshotsOperation::ScreenshotRequested { count: 3 });
    state.record_operation(&SteamworksScreenshotsOperation::ScreenshotReady {
        ready: SteamworksScreenshotReady {
            local_handle: Ok(updated_submission.handle),
        },
    });

    assert_eq!(state.screenshots_hooked(), Some(false));
    assert_eq!(state.screenshot_trigger_count(), 2);
    assert_eq!(
        state.added_screenshots(),
        &[updated_submission.handle, second_submission.handle]
    );
    assert_eq!(
        state.submitted_screenshots(),
        &[second_submission.clone(), updated_submission.clone()]
    );
    assert_eq!(
        state.submitted_screenshot(updated_submission.handle),
        Some(&updated_submission)
    );
    assert_eq!(state.last_submitted_screenshot(), Some(&updated_submission));
    assert_eq!(state.screenshot_requested_count(), 3);
    assert_eq!(
        state.last_screenshot_ready(),
        Some(&SteamworksScreenshotReady {
            local_handle: Ok(updated_submission.handle),
        })
    );
}

#[test]
fn screenshot_callbacks_are_bridged_without_client() {
    let mut app = App::new();
    let handle = 7;

    app.add_plugins(SteamworksScreenshotsPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::ScreenshotRequested(
            steamworks::screenshots::ScreenshotRequested,
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::ScreenshotRequested(
            steamworks::screenshots::ScreenshotRequested,
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::ScreenshotReady(
            steamworks::screenshots::ScreenshotReady {
                local_handle: Ok(handle),
            },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::ScreenshotReady(
            steamworks::screenshots::ScreenshotReady {
                local_handle: Err(steamworks::screenshots::ScreenshotReadyError::IoFailure),
            },
        ));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksScreenshotsResult>>();
    let drained = results.drain().collect::<Vec<_>>();
    assert_eq!(
        drained,
        vec![
            SteamworksScreenshotsResult::Ok(SteamworksScreenshotsOperation::ScreenshotRequested {
                count: 1
            },),
            SteamworksScreenshotsResult::Ok(SteamworksScreenshotsOperation::ScreenshotRequested {
                count: 2
            },),
            SteamworksScreenshotsResult::Ok(SteamworksScreenshotsOperation::ScreenshotReady {
                ready: SteamworksScreenshotReady {
                    local_handle: Ok(handle),
                },
            },),
            SteamworksScreenshotsResult::Ok(SteamworksScreenshotsOperation::ScreenshotReady {
                ready: SteamworksScreenshotReady {
                    local_handle: Err(SteamworksScreenshotReadyError::IoFailure),
                },
            },),
        ]
    );

    let state = app.world().resource::<SteamworksScreenshotsState>();
    assert_eq!(state.screenshot_requested_count(), 2);
    assert_eq!(
        state.last_screenshot_ready(),
        Some(&SteamworksScreenshotReady {
            local_handle: Err(SteamworksScreenshotReadyError::IoFailure),
        })
    );
    assert_eq!(state.last_error(), None);
}
