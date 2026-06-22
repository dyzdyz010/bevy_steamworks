use bevy_app::{App, Plugin};
use bevy_ecs::message::Messages;

use crate::SteamworksEvent;

use super::*;

#[test]
fn apps_plugin_registers_resources_and_messages() {
    let mut app = App::new();

    app.add_plugins(SteamworksAppsPlugin::new());

    assert!(app.world().contains_resource::<SteamworksAppsState>());
    assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksAppsCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksAppsResult>>());
}

#[test]
fn plugin_name_matches_apps_type_path_for_bevy_tracking() {
    let plugin = SteamworksAppsPlugin::new();

    assert_eq!(plugin.name(), std::any::type_name::<SteamworksAppsPlugin>());
    assert_eq!(plugin.name(), "bevy_steamworks::apps::SteamworksAppsPlugin");
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksAppsPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksAppsCommand>>()
        .write(SteamworksAppsCommand::get_current_app_info());

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksAppsResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksAppsResult::Err {
            command: SteamworksAppsCommand::GetCurrentAppInfo,
            error: SteamworksAppsError::ClientUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksAppsState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksAppsError::ClientUnavailable)
    );
}

#[test]
fn constructors_preserve_inputs() {
    let app_id = steamworks::AppId(480);

    assert_eq!(
        SteamworksAppsCommand::get_current_app_info(),
        SteamworksAppsCommand::GetCurrentAppInfo
    );
    assert_eq!(
        SteamworksAppsCommand::is_subscribed(),
        SteamworksAppsCommand::IsSubscribed
    );
    assert_eq!(
        SteamworksAppsCommand::is_app_installed(app_id),
        SteamworksAppsCommand::IsAppInstalled { app_id }
    );
    assert_eq!(
        SteamworksAppsCommand::is_dlc_installed(app_id),
        SteamworksAppsCommand::IsDlcInstalled { app_id }
    );
    assert_eq!(
        SteamworksAppsCommand::is_subscribed_app(app_id),
        SteamworksAppsCommand::IsSubscribedApp { app_id }
    );
    assert_eq!(
        SteamworksAppsCommand::is_subscribed_from_free_weekend(),
        SteamworksAppsCommand::IsSubscribedFromFreeWeekend
    );
    assert_eq!(
        SteamworksAppsCommand::is_vac_banned(),
        SteamworksAppsCommand::IsVacBanned
    );
    assert_eq!(
        SteamworksAppsCommand::is_cybercafe(),
        SteamworksAppsCommand::IsCybercafe
    );
    assert_eq!(
        SteamworksAppsCommand::is_low_violence(),
        SteamworksAppsCommand::IsLowViolence
    );
    assert_eq!(
        SteamworksAppsCommand::get_app_build_id(),
        SteamworksAppsCommand::GetAppBuildId
    );
    assert_eq!(
        SteamworksAppsCommand::get_app_install_dir(app_id),
        SteamworksAppsCommand::GetAppInstallDir { app_id }
    );
    assert_eq!(
        SteamworksAppsCommand::get_app_owner(),
        SteamworksAppsCommand::GetAppOwner
    );
    assert_eq!(
        SteamworksAppsCommand::get_available_game_languages(),
        SteamworksAppsCommand::GetAvailableGameLanguages
    );
    assert_eq!(
        SteamworksAppsCommand::get_current_game_language(),
        SteamworksAppsCommand::GetCurrentGameLanguage
    );
    assert_eq!(
        SteamworksAppsCommand::get_current_beta_name(),
        SteamworksAppsCommand::GetCurrentBetaName
    );
    assert_eq!(
        SteamworksAppsCommand::get_launch_command_line(),
        SteamworksAppsCommand::GetLaunchCommandLine
    );
    assert_eq!(
        SteamworksAppsCommand::get_launch_query_param("connect"),
        SteamworksAppsCommand::GetLaunchQueryParam {
            key: "connect".to_owned(),
        }
    );
}

#[test]
fn state_records_app_operations() {
    let mut state = SteamworksAppsState::default();
    let app_id = steamworks::AppId(480);
    let dlc_id = steamworks::AppId(481);
    let owner = steamworks::SteamId::from_raw(7);
    let owner_from_command = steamworks::SteamId::from_raw(8);
    let info = SteamworksCurrentAppInfo {
        app_id,
        build_id: 12,
        owner,
        subscribed: true,
        subscribed_from_free_weekend: false,
        vac_banned: false,
        cybercafe: false,
        low_violence: true,
        available_game_languages: vec!["english".to_owned(), "schinese".to_owned()],
        current_game_language: "english".to_owned(),
        current_beta_name: Some("preview".to_owned()),
    };

    assert_eq!(state.current_beta_name_result(), None);
    state.record_operation(&SteamworksAppsOperation::CurrentAppInfoRead { info: info.clone() });
    assert_eq!(state.current_beta_name(), Some("preview"));
    assert_eq!(state.current_beta_name_result(), Some(Some("preview")));
    state.record_operation(&SteamworksAppsOperation::AppInstalledRead {
        app_id,
        installed: false,
    });
    state.record_operation(&SteamworksAppsOperation::AppInstalledRead {
        app_id,
        installed: true,
    });
    state.record_operation(&SteamworksAppsOperation::DlcInstalledRead {
        app_id: dlc_id,
        installed: true,
    });
    state.record_operation(&SteamworksAppsOperation::SubscribedAppRead {
        app_id,
        subscribed: true,
    });
    state.record_operation(&SteamworksAppsOperation::AppInstallDirRead {
        app_id,
        install_dir: "C:/Steam/steamapps/common/Spacewar".to_owned(),
    });
    state.record_operation(&SteamworksAppsOperation::LaunchCommandLineRead {
        command_line: "+connect 127.0.0.1".to_owned(),
    });
    state.record_operation(&SteamworksAppsOperation::LaunchQueryParamRead {
        key: "connect".to_owned(),
        value: "127.0.0.1".to_owned(),
    });
    state.record_operation(&SteamworksAppsOperation::LaunchQueryParamRead {
        key: "connect".to_owned(),
        value: "localhost".to_owned(),
    });
    state.record_operation(&SteamworksAppsOperation::CurrentBetaNameRead { beta_name: None });
    state.record_operation(&SteamworksAppsOperation::NewUrlLaunchParametersReceived { count: 3 });
    state.record_operation(&SteamworksAppsOperation::SubscriptionRead { subscribed: false });
    state.record_operation(&SteamworksAppsOperation::SubscribedFromFreeWeekendRead {
        subscribed_from_free_weekend: true,
    });
    state.record_operation(&SteamworksAppsOperation::VacBannedRead { vac_banned: true });
    state.record_operation(&SteamworksAppsOperation::CybercafeRead { cybercafe: true });
    state.record_operation(&SteamworksAppsOperation::LowViolenceRead {
        low_violence: false,
    });
    state.record_operation(&SteamworksAppsOperation::AppBuildIdRead { build_id: 34 });
    state.record_operation(&SteamworksAppsOperation::AppOwnerRead {
        owner: owner_from_command,
    });
    state.record_operation(&SteamworksAppsOperation::AvailableGameLanguagesRead {
        languages: vec!["german".to_owned()],
    });
    state.record_operation(&SteamworksAppsOperation::CurrentGameLanguageRead {
        language: "german".to_owned(),
    });

    assert_eq!(state.current_app_info(), Some(&info));
    assert_eq!(state.subscribed(), Some(false));
    assert_eq!(state.subscribed_from_free_weekend(), Some(true));
    assert_eq!(state.vac_banned(), Some(true));
    assert_eq!(state.cybercafe(), Some(true));
    assert_eq!(state.low_violence(), Some(false));
    assert_eq!(state.app_build_id(), Some(34));
    assert_eq!(state.app_owner(), Some(owner_from_command));
    assert_eq!(
        state.available_game_languages(),
        Some(["german".to_owned()].as_slice())
    );
    assert_eq!(state.current_game_language(), Some("german"));
    assert_eq!(state.current_beta_name(), None);
    assert_eq!(state.current_beta_name_result(), Some(None));
    assert_eq!(state.app_installed(app_id), Some(true));
    assert_eq!(state.dlc_installed(dlc_id), Some(true));
    assert_eq!(state.subscribed_app(app_id), Some(true));
    assert_eq!(
        state.app_install_dir(app_id),
        Some("C:/Steam/steamapps/common/Spacewar")
    );
    assert_eq!(state.launch_command_line(), Some("+connect 127.0.0.1"));
    assert_eq!(state.launch_query_param("connect"), Some("localhost"));
    assert_eq!(
        state.launch_query_params(),
        &[("connect".to_owned(), "localhost".to_owned())]
    );
    assert_eq!(state.new_url_launch_parameters_count(), 3);
}

#[test]
fn app_state_caches_are_bounded() {
    let mut state = SteamworksAppsState::default();

    for raw in 1..=(super::state::STEAMWORKS_APPS_STATE_CACHE_LIMIT as u32 + 1) {
        let app_id = steamworks::AppId(raw);
        state.record_operation(&SteamworksAppsOperation::AppInstalledRead {
            app_id,
            installed: raw % 2 == 0,
        });
        state.record_operation(&SteamworksAppsOperation::DlcInstalledRead {
            app_id,
            installed: raw % 2 != 0,
        });
        state.record_operation(&SteamworksAppsOperation::SubscribedAppRead {
            app_id,
            subscribed: raw % 3 == 0,
        });
        state.record_operation(&SteamworksAppsOperation::AppInstallDirRead {
            app_id,
            install_dir: format!("C:/Steam/app-{raw}"),
        });
        state.record_operation(&SteamworksAppsOperation::LaunchQueryParamRead {
            key: format!("key-{raw}"),
            value: format!("value-{raw}"),
        });
    }

    let evicted = steamworks::AppId(1);
    let retained = steamworks::AppId(2);
    assert_eq!(state.app_installed(evicted), None);
    assert_eq!(state.dlc_installed(evicted), None);
    assert_eq!(state.subscribed_app(evicted), None);
    assert_eq!(state.app_install_dir(evicted), None);
    assert_eq!(state.launch_query_param("key-1"), None);

    assert_eq!(state.app_installed(retained), Some(true));
    assert_eq!(state.dlc_installed(retained), Some(false));
    assert_eq!(state.subscribed_app(retained), Some(false));
    assert_eq!(state.app_install_dir(retained), Some("C:/Steam/app-2"));
    assert_eq!(state.launch_query_param("key-2"), Some("value-2"));
    assert_eq!(
        state.launch_query_params().len(),
        super::state::STEAMWORKS_APPS_STATE_CACHE_LIMIT
    );
}

#[test]
fn new_url_launch_parameters_callbacks_are_bridged_without_client() {
    let mut app = App::new();

    app.add_plugins(SteamworksAppsPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::NewUrlLaunchParameters(
            steamworks::NewUrlLaunchParameters,
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::NewUrlLaunchParameters(
            steamworks::NewUrlLaunchParameters,
        ));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksAppsResult>>();
    let drained = results.drain().collect::<Vec<_>>();
    assert_eq!(
        drained,
        vec![
            SteamworksAppsResult::Ok(SteamworksAppsOperation::NewUrlLaunchParametersReceived {
                count: 1
            },),
            SteamworksAppsResult::Ok(SteamworksAppsOperation::NewUrlLaunchParametersReceived {
                count: 2
            },),
        ]
    );

    let state = app.world().resource::<SteamworksAppsState>();
    assert_eq!(state.new_url_launch_parameters_count(), 2);
    assert_eq!(state.last_error(), None);
}
