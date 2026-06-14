use bevy_app::{App, PluginGroup};
use bevy_ecs::message::Messages;

use super::*;
use crate::{
    SteamworksAppsCommand, SteamworksAppsResult, SteamworksAppsState, SteamworksCallbackRegistry,
    SteamworksEvent, SteamworksFriendsCommand, SteamworksFriendsResult, SteamworksFriendsState,
    SteamworksInputCommand, SteamworksInputResult, SteamworksInputState,
    SteamworksMatchmakingCommand, SteamworksMatchmakingResult, SteamworksMatchmakingServersCommand,
    SteamworksMatchmakingServersResult, SteamworksMatchmakingServersState,
    SteamworksMatchmakingState, SteamworksNetworkingCommand, SteamworksNetworkingMessagesCommand,
    SteamworksNetworkingMessagesResult, SteamworksNetworkingMessagesState,
    SteamworksNetworkingResult, SteamworksNetworkingSocketsCommand,
    SteamworksNetworkingSocketsResult, SteamworksNetworkingSocketsState, SteamworksNetworkingState,
    SteamworksNetworkingUtilsCommand, SteamworksNetworkingUtilsResult,
    SteamworksNetworkingUtilsState, SteamworksPlugin, SteamworksRemotePlayCommand,
    SteamworksRemotePlayResult, SteamworksRemotePlayState, SteamworksRemoteStorageCommand,
    SteamworksRemoteStorageResult, SteamworksRemoteStorageState, SteamworksScreenshotsCommand,
    SteamworksScreenshotsResult, SteamworksScreenshotsState, SteamworksServer,
    SteamworksServerUnavailable, SteamworksStatsCommand, SteamworksStatsPlugin,
    SteamworksStatsResult, SteamworksStatsSettings, SteamworksStatsState,
    SteamworksTimelineCommand, SteamworksTimelineResult, SteamworksTimelineState,
    SteamworksUgcCommand, SteamworksUgcResult, SteamworksUgcState, SteamworksUnavailable,
    SteamworksUserCommand, SteamworksUserResult, SteamworksUserState, SteamworksUtilsCommand,
    SteamworksUtilsResult, SteamworksUtilsState,
};

#[test]
fn client_plugins_register_default_feature_resources_and_messages() {
    let mut app = App::new();

    app.add_plugins(SteamworksClientPlugins::new());

    assert!(app.world().contains_resource::<SteamworksAppsState>());
    assert!(app.world().contains_resource::<SteamworksFriendsState>());
    assert!(app.world().contains_resource::<SteamworksInputState>());
    assert!(app
        .world()
        .contains_resource::<SteamworksMatchmakingState>());
    assert!(app
        .world()
        .contains_resource::<SteamworksMatchmakingServersState>());
    assert!(app.world().contains_resource::<SteamworksNetworkingState>());
    assert!(app
        .world()
        .contains_resource::<SteamworksNetworkingMessagesState>());
    assert!(app
        .world()
        .contains_resource::<SteamworksNetworkingSocketsState>());
    assert!(app
        .world()
        .contains_resource::<SteamworksNetworkingUtilsState>());
    assert!(app.world().contains_resource::<SteamworksRemotePlayState>());
    assert!(app
        .world()
        .contains_resource::<SteamworksRemoteStorageState>());
    assert!(app
        .world()
        .contains_resource::<SteamworksScreenshotsState>());
    assert!(app.world().contains_resource::<SteamworksStatsState>());
    assert!(app.world().contains_resource::<SteamworksTimelineState>());
    assert!(app.world().contains_resource::<SteamworksUgcState>());
    assert!(app.world().contains_resource::<SteamworksUserState>());
    assert!(app.world().contains_resource::<SteamworksUtilsState>());

    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksAppsCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksFriendsCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksInputCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksMatchmakingCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksMatchmakingServersCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingMessagesCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingSocketsCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingUtilsCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksRemotePlayCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksRemoteStorageCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksScreenshotsCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksStatsCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksTimelineCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksUgcCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksUserCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksUtilsCommand>>());

    assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksAppsResult>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksFriendsResult>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksInputResult>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksMatchmakingResult>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksMatchmakingServersResult>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingResult>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingMessagesResult>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingSocketsResult>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingUtilsResult>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksRemotePlayResult>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksRemoteStorageResult>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksScreenshotsResult>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksStatsResult>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksTimelineResult>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksUgcResult>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksUserResult>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksUtilsResult>>());

    assert!(!app.world().contains_resource::<crate::SteamworksClient>());
    assert!(!app.world().contains_resource::<SteamworksUnavailable>());
    assert!(!app.world().contains_resource::<SteamworksServer>());
    assert!(!app
        .world()
        .contains_resource::<SteamworksServerUnavailable>());

    app.update();
}

#[test]
fn client_plugins_group_can_disable_individual_feature_plugins() {
    let mut app = App::new();

    app.add_plugins(
        SteamworksClientPlugins::new()
            .build()
            .disable::<SteamworksStatsPlugin>(),
    );

    assert!(app.world().contains_resource::<SteamworksAppsState>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksAppsCommand>>());
    assert!(!app.world().contains_resource::<SteamworksStatsState>());
    assert!(!app.world().contains_resource::<SteamworksStatsSettings>());
    assert!(!app
        .world()
        .contains_resource::<Messages<SteamworksStatsCommand>>());

    app.update();
}

#[test]
fn plugins_group_can_continue_without_client_and_register_default_features() {
    let mut app = App::new();

    app.add_plugins(SteamworksPlugins::manual().log_and_continue());

    assert!(app.world().contains_resource::<SteamworksUnavailable>());
    assert!(app
        .world()
        .contains_resource::<SteamworksCallbackRegistry>());
    assert!(app.world().contains_resource::<SteamworksAppsState>());
    assert!(app.world().contains_resource::<SteamworksStatsState>());
    assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksAppsCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksAppsResult>>());
    assert!(!app.world().contains_resource::<crate::SteamworksClient>());
    assert!(!app.world().contains_resource::<SteamworksServer>());
    assert!(!app
        .world()
        .contains_resource::<SteamworksServerUnavailable>());

    app.update();
}

#[test]
fn plugins_group_can_replace_individual_feature_plugins() {
    let mut app = App::new();

    app.add_plugins(
        SteamworksPlugins::manual()
            .log_and_continue()
            .set(SteamworksStatsPlugin::new().auto_store(false)),
    );

    assert!(app.world().contains_resource::<SteamworksUnavailable>());
    assert!(app.world().contains_resource::<SteamworksStatsState>());
    assert!(!app.world().resource::<SteamworksStatsSettings>().auto_store);

    app.update();
}

#[test]
fn plugins_group_can_disable_individual_feature_plugins() {
    let mut app = App::new();

    app.add_plugins(
        SteamworksPlugins::manual()
            .log_and_continue()
            .build()
            .disable::<SteamworksNetworkingPlugin>(),
    );

    assert!(app.world().contains_resource::<SteamworksUnavailable>());
    assert!(app.world().contains_resource::<SteamworksAppsState>());
    assert!(!app.world().contains_resource::<SteamworksNetworkingState>());
    assert!(!app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingCommand>>());

    app.update();
}

#[test]
#[should_panic(expected = "manual Steamworks initialization was selected")]
fn plugins_group_panics_by_default_in_manual_mode() {
    let mut app = App::new();

    app.add_plugins(SteamworksPlugins::manual());
}

#[test]
fn plugins_group_wraps_configured_core_plugin() {
    let mut app = App::new();

    app.add_plugins(SteamworksPlugins::from_plugin(
        SteamworksPlugin::manual().log_and_continue(),
    ));

    assert!(app.world().contains_resource::<SteamworksUnavailable>());
    assert!(app.world().contains_resource::<SteamworksAppsState>());
}
