use bevy_app::{App, Plugin};
use bevy_ecs::message::Messages;

use crate::SteamworksEvent;

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
    assert!(app
        .world()
        .contains_resource::<SteamworksUtilsCallbackQueue>());
}

#[test]
fn plugin_name_matches_utils_type_path_for_bevy_tracking() {
    let plugin = SteamworksUtilsPlugin::new();

    assert_eq!(
        plugin.name(),
        std::any::type_name::<SteamworksUtilsPlugin>()
    );
    assert_eq!(
        plugin.name(),
        "bevy_steamworks::utils::SteamworksUtilsPlugin"
    );
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksUtilsPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksUtilsCommand>>()
        .write(SteamworksUtilsCommand::get_current_info());

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
fn client_only_commands_still_fail_without_client_even_when_read_commands_can_use_server() {
    let mut app = App::new();
    let command = SteamworksUtilsCommand::install_warning_callback();

    app.add_plugins(SteamworksUtilsPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksUtilsCommand>>()
        .write(command.clone());

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksUtilsResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksUtilsResult::Err {
            command,
            error: SteamworksUtilsError::ClientUnavailable,
        }]
    );
}

#[test]
fn overlay_position_still_fails_without_client() {
    let mut app = App::new();
    let command = SteamworksUtilsCommand::set_overlay_notification_position(
        SteamworksNotificationPosition::BottomRight,
    );

    app.add_plugins(SteamworksUtilsPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksUtilsCommand>>()
        .write(command.clone());

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksUtilsResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksUtilsResult::Err {
            command,
            error: SteamworksUtilsError::ClientUnavailable,
        }]
    );
}

#[test]
fn constructors_preserve_inputs() {
    let gamepad_request = SteamworksGamepadTextInputRequest::new("Name", 32);
    let floating_request = SteamworksFloatingGamepadTextInputRequest::new(
        SteamworksFloatingGamepadTextInputMode::SingleLine,
        1,
        2,
        300,
        40,
    );

    assert_eq!(
        SteamworksUtilsCommand::get_current_info(),
        SteamworksUtilsCommand::GetCurrentInfo
    );
    assert_eq!(
        SteamworksUtilsCommand::get_app_id(),
        SteamworksUtilsCommand::GetAppId
    );
    assert_eq!(
        SteamworksUtilsCommand::get_ip_country(),
        SteamworksUtilsCommand::GetIpCountry
    );
    assert_eq!(
        SteamworksUtilsCommand::is_overlay_enabled(),
        SteamworksUtilsCommand::IsOverlayEnabled
    );
    assert_eq!(
        SteamworksUtilsCommand::get_ui_language(),
        SteamworksUtilsCommand::GetUiLanguage
    );
    assert_eq!(
        SteamworksUtilsCommand::get_server_real_time(),
        SteamworksUtilsCommand::GetServerRealTime
    );
    assert_eq!(
        SteamworksUtilsCommand::is_steam_in_big_picture_mode(),
        SteamworksUtilsCommand::IsSteamInBigPictureMode
    );
    assert_eq!(
        SteamworksUtilsCommand::is_steam_running_on_steam_deck(),
        SteamworksUtilsCommand::IsSteamRunningOnSteamDeck
    );
    assert_eq!(
        SteamworksUtilsCommand::install_warning_callback(),
        SteamworksUtilsCommand::InstallWarningCallback
    );
    assert_eq!(
        SteamworksUtilsCommand::set_overlay_notification_position(
            SteamworksNotificationPosition::BottomRight,
        ),
        SteamworksUtilsCommand::SetOverlayNotificationPosition {
            position: SteamworksNotificationPosition::BottomRight,
        }
    );
    assert!(matches!(
        steamworks::NotificationPosition::from(SteamworksNotificationPosition::TopLeft),
        steamworks::NotificationPosition::TopLeft
    ));
    assert!(matches!(
        steamworks::GamepadTextInputMode::from(SteamworksGamepadTextInputMode::Password),
        steamworks::GamepadTextInputMode::Password
    ));
    assert!(matches!(
        steamworks::GamepadTextInputLineMode::from(
            SteamworksGamepadTextInputLineMode::MultipleLines
        ),
        steamworks::GamepadTextInputLineMode::MultipleLines
    ));
    assert!(matches!(
        steamworks::FloatingGamepadTextInputMode::from(
            SteamworksFloatingGamepadTextInputMode::Email
        ),
        steamworks::FloatingGamepadTextInputMode::Email
    ));
    assert_eq!(
        SteamworksUtilsCommand::show_gamepad_text_input(gamepad_request.clone()),
        SteamworksUtilsCommand::ShowGamepadTextInput {
            request: gamepad_request,
        }
    );
    assert_eq!(
        SteamworksUtilsCommand::show_floating_gamepad_text_input(floating_request.clone()),
        SteamworksUtilsCommand::ShowFloatingGamepadTextInput {
            request: floating_request,
        }
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
        submitted_text: None,
    };
    let cancelled = SteamworksGamepadTextInputDismissed {
        submitted_text_len: None,
        submitted_text: None,
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
fn queued_text_input_callback_captures_submitted_text_before_dismissal() {
    let mut app = App::new();

    app.add_plugins(SteamworksUtilsPlugin::new());

    let dismissed = SteamworksGamepadTextInputDismissed {
        submitted_text_len: Some(4),
        submitted_text: Some("Name".to_owned()),
    };
    app.world_mut()
        .resource_mut::<SteamworksUtilsCallbackQueue>()
        .push_gamepad_text_input_dismissed(dismissed.clone());

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksUtilsResult>>();
    let drained = results.drain().collect::<Vec<_>>();
    let submitted = SteamworksGamepadTextInputSubmitted {
        text: "Name".to_owned(),
        submitted_text_len: 4,
    };

    assert_eq!(
        drained,
        vec![
            SteamworksUtilsResult::Ok(SteamworksUtilsOperation::GamepadTextInputSubmitted {
                submitted: submitted.clone(),
            }),
            SteamworksUtilsResult::Ok(SteamworksUtilsOperation::GamepadTextInputDismissed {
                dismissed: dismissed.clone(),
            }),
        ]
    );

    let state = app.world().resource::<SteamworksUtilsState>();
    assert_eq!(state.last_gamepad_text_input_submitted(), Some(&submitted));
    assert_eq!(state.last_gamepad_text_input_dismissed(), Some(&dismissed));
}

#[test]
fn queued_text_input_callback_suppresses_duplicate_raw_event_bridge() {
    let mut app = App::new();

    app.add_plugins(SteamworksUtilsPlugin::new());

    let dismissed = SteamworksGamepadTextInputDismissed {
        submitted_text_len: Some(4),
        submitted_text: Some("Name".to_owned()),
    };
    app.world_mut()
        .resource_mut::<SteamworksUtilsCallbackQueue>()
        .push_gamepad_text_input_dismissed(dismissed.clone());
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::GamepadTextInputDismissed(
            steamworks::GamepadTextInputDismissed {
                submitted_text_len: Some(4),
            },
        ));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksUtilsResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(drained.len(), 2);
    assert!(matches!(
        &drained[0],
        SteamworksUtilsResult::Ok(SteamworksUtilsOperation::GamepadTextInputSubmitted {
            submitted
        }) if submitted.text == "Name"
    ));
    assert_eq!(
        drained[1],
        SteamworksUtilsResult::Ok(SteamworksUtilsOperation::GamepadTextInputDismissed {
            dismissed,
        })
    );
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
    state.record_operation(&SteamworksUtilsOperation::WarningCallbackInstalled);
    state.record_operation(&SteamworksUtilsOperation::OverlayNotificationPositionSet {
        position: SteamworksNotificationPosition::BottomRight,
    });
    let gamepad_shown = SteamworksGamepadTextInputShown {
        request: SteamworksGamepadTextInputRequest::new("Name", 32),
        shown: true,
    };
    let floating_shown = SteamworksFloatingGamepadTextInputShown {
        request: SteamworksFloatingGamepadTextInputRequest::new(
            SteamworksFloatingGamepadTextInputMode::SingleLine,
            1,
            2,
            300,
            40,
        ),
        shown: true,
    };
    let submitted = SteamworksGamepadTextInputSubmitted {
        text: "Name".to_owned(),
        submitted_text_len: 4,
    };
    let dismissed = SteamworksGamepadTextInputDismissed {
        submitted_text_len: Some(4),
        submitted_text: Some("Name".to_owned()),
    };
    let floating_dismissed = SteamworksFloatingGamepadTextInputDismissed;
    state.record_operation(&SteamworksUtilsOperation::GamepadTextInputShown {
        shown: gamepad_shown.clone(),
    });
    state.record_operation(&SteamworksUtilsOperation::FloatingGamepadTextInputShown {
        shown: floating_shown.clone(),
    });
    state.record_operation(&SteamworksUtilsOperation::GamepadTextInputSubmitted {
        submitted: submitted.clone(),
    });
    state.record_operation(&SteamworksUtilsOperation::GamepadTextInputDismissed {
        dismissed: dismissed.clone(),
    });
    state.record_operation(
        &SteamworksUtilsOperation::FloatingGamepadTextInputDismissed {
            dismissed: floating_dismissed,
        },
    );

    assert_eq!(state.current_info(), Some(&info));
    assert_eq!(state.app_id(), Some(steamworks::AppId(481)));
    assert_eq!(state.ip_country(), Some("CN"));
    assert_eq!(state.overlay_enabled(), Some(false));
    assert_eq!(state.ui_language(), Some("schinese"));
    assert_eq!(state.server_real_time(), Some(456));
    assert_eq!(state.steam_in_big_picture_mode(), Some(true));
    assert_eq!(state.steam_running_on_steam_deck(), Some(true));
    assert!(state.warning_callback_installed());
    assert_eq!(
        state.overlay_notification_position(),
        Some(SteamworksNotificationPosition::BottomRight)
    );
    assert_eq!(state.last_gamepad_text_input_shown(), Some(&gamepad_shown));
    assert_eq!(
        state.last_gamepad_text_input_request(),
        Some(&gamepad_shown.request)
    );
    assert_eq!(state.gamepad_text_input_shown(), Some(true));
    assert_eq!(
        state.last_floating_gamepad_text_input_shown(),
        Some(&floating_shown)
    );
    assert_eq!(
        state.last_floating_gamepad_text_input_request(),
        Some(&floating_shown.request)
    );
    assert_eq!(state.floating_gamepad_text_input_shown(), Some(true));
    assert_eq!(state.last_gamepad_text_input_submitted(), Some(&submitted));
    assert_eq!(state.last_submitted_gamepad_text(), Some("Name"));
    assert_eq!(state.last_submitted_gamepad_text_len(), Some(4));
    assert_eq!(state.last_gamepad_text_input_dismissed(), Some(&dismissed));
    assert_eq!(state.gamepad_text_input_was_submitted(), Some(true));
    assert_eq!(state.last_dismissed_gamepad_text(), Some(Some("Name")));
    assert_eq!(state.last_dismissed_gamepad_text_len(), Some(Some(4)));
    assert_eq!(
        state.last_floating_gamepad_text_input_dismissed(),
        Some(&floating_dismissed)
    );
}

#[test]
fn gamepad_text_input_request_debug_hides_existing_text() {
    let request =
        SteamworksGamepadTextInputRequest::new("Name", 32).with_existing_text("secret-value");

    let debug = format!("{request:?}");

    assert!(debug.contains("existing_text_len: Some(12)"));
    assert!(!debug.contains("secret-value"));
    assert!(!debug.contains("existing_text:"));
}
