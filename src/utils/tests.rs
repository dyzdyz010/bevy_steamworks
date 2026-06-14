use bevy_app::App;
use bevy_ecs::message::Messages;

use super::*;

#[test]
fn utils_plugin_registers_resources_and_messages() {
    let mut app = App::new();

    app.add_plugins(SteamworksUtilsPlugin::new());

    assert!(app.world().contains_resource::<SteamworksUtilsState>());
    assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksUtilsCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksUtilsResult>>());
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksUtilsPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksUtilsCommand>>()
        .write(SteamworksUtilsCommand::GetCurrentInfo);

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksUtilsResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksUtilsResult::Err {
            command: SteamworksUtilsCommand::GetCurrentInfo,
            error: SteamworksUtilsError::ClientUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksUtilsState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksUtilsError::ClientUnavailable)
    );
}

#[test]
fn text_input_callbacks_are_bridged_without_client() {
    let mut app = App::new();

    app.add_plugins(SteamworksUtilsPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::GamepadTextInputDismissed(
            steamworks::GamepadTextInputDismissed {
                submitted_text_len: Some(12),
            },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::GamepadTextInputDismissed(
            steamworks::GamepadTextInputDismissed {
                submitted_text_len: None,
            },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::FloatingGamepadTextInputDismissed(
            steamworks::FloatingGamepadTextInputDismissed,
        ));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksUtilsResult>>();
    let drained = results.drain().collect::<Vec<_>>();
    let submitted = SteamworksGamepadTextInputDismissed {
        submitted_text_len: Some(12),
    };
    let cancelled = SteamworksGamepadTextInputDismissed {
        submitted_text_len: None,
    };
    let floating = SteamworksFloatingGamepadTextInputDismissed;

    assert_eq!(
        drained,
        vec![
            SteamworksUtilsResult::Ok(SteamworksUtilsOperation::GamepadTextInputDismissed {
                dismissed: submitted,
            }),
            SteamworksUtilsResult::Ok(SteamworksUtilsOperation::GamepadTextInputDismissed {
                dismissed: cancelled.clone(),
            }),
            SteamworksUtilsResult::Ok(
                SteamworksUtilsOperation::FloatingGamepadTextInputDismissed {
                    dismissed: floating,
                },
            ),
        ]
    );

    let state = app.world().resource::<SteamworksUtilsState>();
    assert_eq!(state.last_gamepad_text_input_dismissed(), Some(&cancelled));
    assert_eq!(
        state.last_floating_gamepad_text_input_dismissed(),
        Some(&floating)
    );
    assert_eq!(state.last_error(), None);
}

#[test]
fn state_records_utility_operations() {
    let mut state = SteamworksUtilsState::default();
    let info = SteamworksUtilsInfo {
        app_id: steamworks::AppId(480),
        ip_country: "US".to_owned(),
        overlay_enabled: true,
        ui_language: "english".to_owned(),
        server_real_time: 123,
        steam_in_big_picture_mode: false,
        steam_running_on_steam_deck: false,
    };

    state.record_operation(&SteamworksUtilsOperation::CurrentInfoRead { info: info.clone() });
    assert_eq!(state.current_info(), Some(&info));
    assert_eq!(state.app_id(), Some(steamworks::AppId(480)));
    assert_eq!(state.ip_country(), Some("US"));
    assert_eq!(state.overlay_enabled(), Some(true));
    assert_eq!(state.ui_language(), Some("english"));
    assert_eq!(state.server_real_time(), Some(123));
    assert_eq!(state.steam_in_big_picture_mode(), Some(false));
    assert_eq!(state.steam_running_on_steam_deck(), Some(false));

    state.record_operation(&SteamworksUtilsOperation::AppIdRead {
        app_id: steamworks::AppId(481),
    });
    state.record_operation(&SteamworksUtilsOperation::IpCountryRead {
        country: "CN".to_owned(),
    });
    state.record_operation(&SteamworksUtilsOperation::OverlayEnabledRead { enabled: false });
    state.record_operation(&SteamworksUtilsOperation::UiLanguageRead {
        language: "schinese".to_owned(),
    });
    state.record_operation(&SteamworksUtilsOperation::ServerRealTimeRead {
        unix_epoch_seconds: 456,
    });
    state.record_operation(&SteamworksUtilsOperation::SteamInBigPictureModeRead { enabled: true });
    state
        .record_operation(&SteamworksUtilsOperation::SteamRunningOnSteamDeckRead { enabled: true });
    state.record_operation(&SteamworksUtilsOperation::OverlayNotificationPositionSet {
        position: SteamworksNotificationPosition::BottomRight,
    });

    assert_eq!(state.current_info(), Some(&info));
    assert_eq!(state.app_id(), Some(steamworks::AppId(481)));
    assert_eq!(state.ip_country(), Some("CN"));
    assert_eq!(state.overlay_enabled(), Some(false));
    assert_eq!(state.ui_language(), Some("schinese"));
    assert_eq!(state.server_real_time(), Some(456));
    assert_eq!(state.steam_in_big_picture_mode(), Some(true));
    assert_eq!(state.steam_running_on_steam_deck(), Some(true));
    assert_eq!(
        state.overlay_notification_position(),
        Some(SteamworksNotificationPosition::BottomRight)
    );
}
