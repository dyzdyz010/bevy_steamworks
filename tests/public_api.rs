use bevy_app::{Plugin, PluginGroup};
use bevy_steamworks::{
    prelude::{
        SteamAPIInitError as PreludeInitError, SteamworksAppsCommand as PreludeAppsCommand,
        SteamworksAppsError as PreludeAppsError, SteamworksAppsOperation as PreludeAppsOperation,
        SteamworksAppsPlugin as PreludeAppsPlugin, SteamworksAppsResult as PreludeAppsResult,
        SteamworksCallbackRegistry as PreludeCallbackRegistry, SteamworksClient as PreludeClient,
        SteamworksClientPlugins as PreludeClientPlugins, SteamworksEvent as PreludeEvent,
        SteamworksFailurePolicy as PreludeFailurePolicy,
        SteamworksFriendsCommand as PreludeFriendsCommand,
        SteamworksFriendsError as PreludeFriendsError,
        SteamworksFriendsOperation as PreludeFriendsOperation,
        SteamworksFriendsPlugin as PreludeFriendsPlugin,
        SteamworksFriendsResult as PreludeFriendsResult, SteamworksInitMode as PreludeInitMode,
        SteamworksInputCommand as PreludeInputCommand, SteamworksInputError as PreludeInputError,
        SteamworksInputOperation as PreludeInputOperation,
        SteamworksInputPlugin as PreludeInputPlugin, SteamworksInputResult as PreludeInputResult,
        SteamworksLobbyListFilter as PreludeLobbyListFilter,
        SteamworksMatchmakingCommand as PreludeMatchmakingCommand,
        SteamworksMatchmakingError as PreludeMatchmakingError,
        SteamworksMatchmakingOperation as PreludeMatchmakingOperation,
        SteamworksMatchmakingPlugin as PreludeMatchmakingPlugin,
        SteamworksMatchmakingResult as PreludeMatchmakingResult,
        SteamworksMatchmakingServersCommand as PreludeMatchmakingServersCommand,
        SteamworksMatchmakingServersError as PreludeMatchmakingServersError,
        SteamworksMatchmakingServersOperation as PreludeMatchmakingServersOperation,
        SteamworksMatchmakingServersPlugin as PreludeMatchmakingServersPlugin,
        SteamworksMatchmakingServersResult as PreludeMatchmakingServersResult,
        SteamworksNetworkingCommand as PreludeNetworkingCommand,
        SteamworksNetworkingError as PreludeNetworkingError,
        SteamworksNetworkingMessagesCommand as PreludeNetworkingMessagesCommand,
        SteamworksNetworkingMessagesError as PreludeNetworkingMessagesError,
        SteamworksNetworkingMessagesOperation as PreludeNetworkingMessagesOperation,
        SteamworksNetworkingMessagesPlugin as PreludeNetworkingMessagesPlugin,
        SteamworksNetworkingMessagesResult as PreludeNetworkingMessagesResult,
        SteamworksNetworkingOperation as PreludeNetworkingOperation,
        SteamworksNetworkingPlugin as PreludeNetworkingPlugin,
        SteamworksNetworkingResult as PreludeNetworkingResult,
        SteamworksNetworkingSocketsCommand as PreludeNetworkingSocketsCommand,
        SteamworksNetworkingSocketsError as PreludeNetworkingSocketsError,
        SteamworksNetworkingSocketsOperation as PreludeNetworkingSocketsOperation,
        SteamworksNetworkingSocketsPlugin as PreludeNetworkingSocketsPlugin,
        SteamworksNetworkingSocketsPollGroupId as PreludeNetworkingSocketsPollGroupId,
        SteamworksNetworkingSocketsResult as PreludeNetworkingSocketsResult,
        SteamworksNetworkingUtilsCommand as PreludeNetworkingUtilsCommand,
        SteamworksNetworkingUtilsError as PreludeNetworkingUtilsError,
        SteamworksNetworkingUtilsOperation as PreludeNetworkingUtilsOperation,
        SteamworksNetworkingUtilsPlugin as PreludeNetworkingUtilsPlugin,
        SteamworksNetworkingUtilsResult as PreludeNetworkingUtilsResult,
        SteamworksNotificationPosition as PreludeNotificationPosition,
        SteamworksPlugin as PreludePlugin, SteamworksPlugins as PreludePlugins,
        SteamworksRemotePlayCommand as PreludeRemotePlayCommand,
        SteamworksRemotePlayError as PreludeRemotePlayError,
        SteamworksRemotePlayOperation as PreludeRemotePlayOperation,
        SteamworksRemotePlayPlugin as PreludeRemotePlayPlugin,
        SteamworksRemotePlayResult as PreludeRemotePlayResult,
        SteamworksRemoteStorageCommand as PreludeRemoteStorageCommand,
        SteamworksRemoteStorageError as PreludeRemoteStorageError,
        SteamworksRemoteStorageOperation as PreludeRemoteStorageOperation,
        SteamworksRemoteStoragePlugin as PreludeRemoteStoragePlugin,
        SteamworksRemoteStorageResult as PreludeRemoteStorageResult,
        SteamworksScreenshotsCommand as PreludeScreenshotsCommand,
        SteamworksScreenshotsError as PreludeScreenshotsError,
        SteamworksScreenshotsOperation as PreludeScreenshotsOperation,
        SteamworksScreenshotsPlugin as PreludeScreenshotsPlugin,
        SteamworksScreenshotsResult as PreludeScreenshotsResult,
        SteamworksServerCommand as PreludeServerCommand,
        SteamworksServerError as PreludeServerError,
        SteamworksServerListFilters as PreludeServerListFilters,
        SteamworksServerListKind as PreludeServerListKind,
        SteamworksServerListRequestId as PreludeServerListRequestId,
        SteamworksServerOperation as PreludeServerOperation,
        SteamworksServerPlugin as PreludeServerPlugin,
        SteamworksServerResult as PreludeServerResult,
        SteamworksStatsCommand as PreludeStatsCommand, SteamworksStatsError as PreludeStatsError,
        SteamworksStatsOperation as PreludeStatsOperation,
        SteamworksStatsPlugin as PreludeStatsPlugin, SteamworksStatsResult as PreludeStatsResult,
        SteamworksSystem as PreludeSystem, SteamworksTimelineCommand as PreludeTimelineCommand,
        SteamworksTimelineError as PreludeTimelineError,
        SteamworksTimelineGameMode as PreludeTimelineGameMode,
        SteamworksTimelineOperation as PreludeTimelineOperation,
        SteamworksTimelinePlugin as PreludeTimelinePlugin,
        SteamworksTimelineResult as PreludeTimelineResult,
        SteamworksUgcCommand as PreludeUgcCommand, SteamworksUgcError as PreludeUgcError,
        SteamworksUgcOperation as PreludeUgcOperation, SteamworksUgcPlugin as PreludeUgcPlugin,
        SteamworksUgcResult as PreludeUgcResult, SteamworksUnavailable as PreludeUnavailable,
        SteamworksUserCommand as PreludeUserCommand, SteamworksUserError as PreludeUserError,
        SteamworksUserOperation as PreludeUserOperation, SteamworksUserPlugin as PreludeUserPlugin,
        SteamworksUserResult as PreludeUserResult, SteamworksUtilsCommand as PreludeUtilsCommand,
        SteamworksUtilsError as PreludeUtilsError,
        SteamworksUtilsOperation as PreludeUtilsOperation,
        SteamworksUtilsPlugin as PreludeUtilsPlugin, SteamworksUtilsResult as PreludeUtilsResult,
    },
    SteamAPIInitError, SteamworksAppsCommand, SteamworksAppsError, SteamworksAppsOperation,
    SteamworksAppsPlugin, SteamworksAppsResult, SteamworksCallbackRegistry, SteamworksClient,
    SteamworksClientPlugins, SteamworksEvent, SteamworksFailurePolicy, SteamworksFriendsCommand,
    SteamworksFriendsError, SteamworksFriendsOperation, SteamworksFriendsPlugin,
    SteamworksFriendsResult, SteamworksInitMode, SteamworksInputCommand, SteamworksInputError,
    SteamworksInputOperation, SteamworksInputPlugin, SteamworksInputResult,
    SteamworksLobbyListFilter, SteamworksMatchmakingCommand, SteamworksMatchmakingError,
    SteamworksMatchmakingOperation, SteamworksMatchmakingPlugin, SteamworksMatchmakingResult,
    SteamworksMatchmakingServersCommand, SteamworksMatchmakingServersError,
    SteamworksMatchmakingServersOperation, SteamworksMatchmakingServersPlugin,
    SteamworksMatchmakingServersResult, SteamworksNetworkingCommand, SteamworksNetworkingError,
    SteamworksNetworkingMessagesCommand, SteamworksNetworkingMessagesError,
    SteamworksNetworkingMessagesOperation, SteamworksNetworkingMessagesPlugin,
    SteamworksNetworkingMessagesResult, SteamworksNetworkingOperation, SteamworksNetworkingPlugin,
    SteamworksNetworkingResult, SteamworksNetworkingSocketsCommand,
    SteamworksNetworkingSocketsError, SteamworksNetworkingSocketsOperation,
    SteamworksNetworkingSocketsPlugin, SteamworksNetworkingSocketsPollGroupId,
    SteamworksNetworkingSocketsResult, SteamworksNetworkingUtilsCommand,
    SteamworksNetworkingUtilsError, SteamworksNetworkingUtilsOperation,
    SteamworksNetworkingUtilsPlugin, SteamworksNetworkingUtilsResult,
    SteamworksNotificationPosition, SteamworksPlugin, SteamworksPlugins,
    SteamworksRemotePlayCommand, SteamworksRemotePlayError, SteamworksRemotePlayOperation,
    SteamworksRemotePlayPlugin, SteamworksRemotePlayResult, SteamworksRemoteStorageCommand,
    SteamworksRemoteStorageError, SteamworksRemoteStorageOperation, SteamworksRemoteStoragePlugin,
    SteamworksRemoteStorageResult, SteamworksScreenshotsCommand, SteamworksScreenshotsError,
    SteamworksScreenshotsOperation, SteamworksScreenshotsPlugin, SteamworksScreenshotsResult,
    SteamworksServerCommand, SteamworksServerError, SteamworksServerListFilters,
    SteamworksServerListKind, SteamworksServerListRequestId, SteamworksServerOperation,
    SteamworksServerPlugin, SteamworksServerResult, SteamworksStatsCommand, SteamworksStatsError,
    SteamworksStatsOperation, SteamworksStatsPlugin, SteamworksStatsResult, SteamworksSystem,
    SteamworksTimelineCommand, SteamworksTimelineError, SteamworksTimelineGameMode,
    SteamworksTimelineOperation, SteamworksTimelinePlugin, SteamworksTimelineResult,
    SteamworksUgcCommand, SteamworksUgcError, SteamworksUgcOperation, SteamworksUgcPlugin,
    SteamworksUgcResult, SteamworksUnavailable, SteamworksUserCommand, SteamworksUserError,
    SteamworksUserOperation, SteamworksUserPlugin, SteamworksUserResult, SteamworksUtilsCommand,
    SteamworksUtilsError, SteamworksUtilsOperation, SteamworksUtilsPlugin, SteamworksUtilsResult,
};

#[test]
fn core_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksPlugin,
        _registry: SteamworksCallbackRegistry,
        _system: SteamworksSystem,
        _client: Option<&SteamworksClient>,
        _event: Option<SteamworksEvent>,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludePlugin,
        _registry: PreludeCallbackRegistry,
        _system: PreludeSystem,
        _client: Option<&PreludeClient>,
        _event: Option<PreludeEvent>,
    ) {
    }

    let plugin = SteamworksPlugin::app_id(480)
        .failure_policy(SteamworksFailurePolicy::LogAndContinue)
        .run_callbacks(false);
    assert_eq!(plugin.name(), "bevy_steamworks::SteamworksPlugin");
    assert_eq!(plugin.init_mode(), SteamworksInitMode::AppId(480.into()));
    assert_eq!(
        plugin.failure_policy_setting(),
        SteamworksFailurePolicy::LogAndContinue
    );
    assert!(!plugin.runs_callbacks());
    accepts_root_exports(
        plugin,
        SteamworksCallbackRegistry::default(),
        SteamworksSystem::RunCallbacks,
        None,
        None,
    );

    let plugin = PreludePlugin::app_id(480)
        .failure_policy(PreludeFailurePolicy::LogAndContinue)
        .run_callbacks(false);
    assert_eq!(plugin.name(), "bevy_steamworks::SteamworksPlugin");
    assert_eq!(plugin.init_mode(), PreludeInitMode::AppId(480.into()));
    assert_eq!(
        plugin.failure_policy_setting(),
        PreludeFailurePolicy::LogAndContinue
    );
    assert!(!plugin.runs_callbacks());
    accepts_prelude_exports(
        plugin,
        PreludeCallbackRegistry::default(),
        PreludeSystem::RunCallbacks,
        None,
        None,
    );
}

#[test]
fn availability_status_api_is_exported_from_root_and_prelude() {
    let mode = SteamworksInitMode::AppId(480.into());
    let source = SteamAPIInitError::NoSteamClient("offline".into());
    let unavailable = SteamworksUnavailable::InitFailed {
        mode,
        source: source.clone(),
    };

    assert_eq!(mode.raw_app_id(), Some(480));
    assert_eq!(mode.app_id(), Some(480.into()));
    assert!(unavailable.is_init_failed());
    assert!(!unavailable.is_manual_client_missing());
    assert_eq!(unavailable.init_mode(), Some(mode));
    assert_eq!(unavailable.raw_app_id(), Some(480));
    assert_eq!(unavailable.app_id(), Some(480.into()));
    assert_eq!(unavailable.init_error(), Some(&source));

    let mode = PreludeInitMode::AppId(480.into());
    let source = PreludeInitError::NoSteamClient("offline".into());
    let unavailable = PreludeUnavailable::InitFailed {
        mode,
        source: source.clone(),
    };

    assert_eq!(mode.raw_app_id(), Some(480));
    assert_eq!(mode.app_id(), Some(480.into()));
    assert!(unavailable.is_init_failed());
    assert!(!unavailable.is_manual_client_missing());
    assert_eq!(unavailable.init_mode(), Some(mode));
    assert_eq!(unavailable.raw_app_id(), Some(480));
    assert_eq!(unavailable.app_id(), Some(480.into()));
    assert_eq!(unavailable.init_error(), Some(&source));

    assert!(SteamworksUnavailable::ManualClientMissing.is_manual_client_missing());
    assert!(PreludeUnavailable::ManualClientMissing.is_manual_client_missing());
}

#[test]
fn client_plugin_bundle_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(_plugin: SteamworksClientPlugins) {}
    fn accepts_prelude_exports(_plugin: PreludeClientPlugins) {}

    accepts_root_exports(SteamworksClientPlugins::new());
    accepts_prelude_exports(PreludeClientPlugins::new());

    let _ = SteamworksClientPlugins::new()
        .build()
        .disable::<SteamworksStatsPlugin>();
    let _ = PreludeClientPlugins::new()
        .build()
        .disable::<PreludeStatsPlugin>();
}

#[test]
fn full_plugin_group_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(_plugins: SteamworksPlugins) {}
    fn accepts_prelude_exports(_plugins: PreludePlugins) {}

    let plugins = SteamworksPlugins::app_id(480)
        .log_and_continue()
        .run_callbacks(false);
    assert_eq!(plugins.init_mode(), SteamworksInitMode::AppId(480.into()));
    assert_eq!(
        plugins.failure_policy_setting(),
        SteamworksFailurePolicy::LogAndContinue
    );
    assert!(!plugins.runs_callbacks());
    assert_eq!(plugins.core_plugin().init_mode(), plugins.init_mode());
    accepts_root_exports(plugins);

    let plugins = PreludePlugins::app_id(480)
        .log_and_continue()
        .run_callbacks(false);
    assert_eq!(plugins.init_mode(), PreludeInitMode::AppId(480.into()));
    assert_eq!(
        plugins.failure_policy_setting(),
        PreludeFailurePolicy::LogAndContinue
    );
    assert!(!plugins.runs_callbacks());
    assert_eq!(plugins.core_plugin().init_mode(), plugins.init_mode());
    accepts_prelude_exports(plugins);

    let _ = SteamworksPlugins::manual()
        .log_and_continue()
        .set(SteamworksStatsPlugin::new().auto_store(false));
    let _ = PreludePlugins::manual()
        .log_and_continue()
        .set(PreludeStatsPlugin::new().auto_store(false));
}

#[test]
fn apps_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksAppsPlugin,
        _command: SteamworksAppsCommand,
        _operation: SteamworksAppsOperation,
        _result: SteamworksAppsResult,
        _error: SteamworksAppsError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeAppsPlugin,
        _command: PreludeAppsCommand,
        _operation: PreludeAppsOperation,
        _result: PreludeAppsResult,
        _error: PreludeAppsError,
    ) {
    }

    let command = SteamworksAppsCommand::IsSubscribed;
    let operation = SteamworksAppsOperation::SubscriptionRead { subscribed: true };
    let error = SteamworksAppsError::ClientUnavailable;
    let result = SteamworksAppsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksAppsPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeAppsCommand::IsSubscribed;
    let operation = PreludeAppsOperation::SubscriptionRead { subscribed: true };
    let error = PreludeAppsError::ClientUnavailable;
    let result = PreludeAppsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(PreludeAppsPlugin::new(), command, operation, result, error);
}

#[test]
fn friends_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksFriendsPlugin,
        _command: SteamworksFriendsCommand,
        _operation: SteamworksFriendsOperation,
        _result: SteamworksFriendsResult,
        _error: SteamworksFriendsError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeFriendsPlugin,
        _command: PreludeFriendsCommand,
        _operation: PreludeFriendsOperation,
        _result: PreludeFriendsResult,
        _error: PreludeFriendsError,
    ) {
    }

    let command = SteamworksFriendsCommand::GetPersonaName;
    let operation = SteamworksFriendsOperation::PersonaNameRead {
        name: String::new(),
    };
    let error = SteamworksFriendsError::ClientUnavailable;
    let result = SteamworksFriendsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksFriendsPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeFriendsCommand::GetPersonaName;
    let operation = PreludeFriendsOperation::PersonaNameRead {
        name: String::new(),
    };
    let error = PreludeFriendsError::ClientUnavailable;
    let result = PreludeFriendsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(
        PreludeFriendsPlugin::new(),
        command,
        operation,
        result,
        error,
    );
}

#[test]
fn game_server_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksServerPlugin,
        _command: SteamworksServerCommand,
        _operation: SteamworksServerOperation,
        _result: SteamworksServerResult,
        _error: SteamworksServerError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeServerPlugin,
        _command: PreludeServerCommand,
        _operation: PreludeServerOperation,
        _result: PreludeServerResult,
        _error: PreludeServerError,
    ) {
    }

    let command = SteamworksServerCommand::GetSteamId;
    let operation = SteamworksServerOperation::AnonymousLogonSubmitted;
    let error = SteamworksServerError::ServerUnavailable;
    let result = SteamworksServerResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksServerPlugin::manual(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeServerCommand::GetSteamId;
    let operation = PreludeServerOperation::AnonymousLogonSubmitted;
    let error = PreludeServerError::ServerUnavailable;
    let result = PreludeServerResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(
        PreludeServerPlugin::manual(),
        command,
        operation,
        result,
        error,
    );
}

#[test]
fn input_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksInputPlugin,
        _command: SteamworksInputCommand,
        _operation: SteamworksInputOperation,
        _result: SteamworksInputResult,
        _error: SteamworksInputError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeInputPlugin,
        _command: PreludeInputCommand,
        _operation: PreludeInputOperation,
        _result: PreludeInputResult,
        _error: PreludeInputError,
    ) {
    }

    let command = SteamworksInputCommand::init(false);
    let operation = SteamworksInputOperation::Initialized {
        explicitly_call_run_frame: false,
    };
    let error = SteamworksInputError::ClientUnavailable;
    let result = SteamworksInputResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksInputPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeInputCommand::init(false);
    let operation = PreludeInputOperation::Initialized {
        explicitly_call_run_frame: false,
    };
    let error = PreludeInputError::ClientUnavailable;
    let result = PreludeInputResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(PreludeInputPlugin::new(), command, operation, result, error);
}

#[test]
fn matchmaking_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksMatchmakingPlugin,
        _command: SteamworksMatchmakingCommand,
        _operation: SteamworksMatchmakingOperation,
        _result: SteamworksMatchmakingResult,
        _error: SteamworksMatchmakingError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeMatchmakingPlugin,
        _command: PreludeMatchmakingCommand,
        _operation: PreludeMatchmakingOperation,
        _result: PreludeMatchmakingResult,
        _error: PreludeMatchmakingError,
    ) {
    }

    let filter = SteamworksLobbyListFilter::new();
    let command = SteamworksMatchmakingCommand::request_lobby_list(filter.clone());
    let operation = SteamworksMatchmakingOperation::LobbyListRequested {
        request_id: 1,
        filter,
    };
    let error = SteamworksMatchmakingError::ClientUnavailable;
    let result = SteamworksMatchmakingResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksMatchmakingPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let filter = PreludeLobbyListFilter::new();
    let command = PreludeMatchmakingCommand::request_lobby_list(filter.clone());
    let operation = PreludeMatchmakingOperation::LobbyListRequested {
        request_id: 1,
        filter,
    };
    let error = PreludeMatchmakingError::ClientUnavailable;
    let result = PreludeMatchmakingResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(
        PreludeMatchmakingPlugin::new(),
        command,
        operation,
        result,
        error,
    );
}

#[test]
fn networking_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksNetworkingPlugin,
        _command: SteamworksNetworkingCommand,
        _operation: SteamworksNetworkingOperation,
        _result: SteamworksNetworkingResult,
        _error: SteamworksNetworkingError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeNetworkingPlugin,
        _command: PreludeNetworkingCommand,
        _operation: PreludeNetworkingOperation,
        _result: PreludeNetworkingResult,
        _error: PreludeNetworkingError,
    ) {
    }

    fn accepts_usize(_value: usize) {}

    accepts_usize(bevy_steamworks::STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES);
    accepts_usize(bevy_steamworks::STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES);
    accepts_usize(bevy_steamworks::STEAMWORKS_P2P_MAX_READ_PACKET_BYTES);
    accepts_usize(bevy_steamworks::prelude::STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES);
    accepts_usize(bevy_steamworks::prelude::STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES);
    accepts_usize(bevy_steamworks::prelude::STEAMWORKS_P2P_MAX_READ_PACKET_BYTES);

    assert_eq!(
        bevy_steamworks::STEAMWORKS_P2P_MAX_READ_PACKET_BYTES,
        bevy_steamworks::STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES
    );
    assert_eq!(
        bevy_steamworks::prelude::STEAMWORKS_P2P_MAX_READ_PACKET_BYTES,
        bevy_steamworks::prelude::STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES
    );

    let command = SteamworksNetworkingCommand::get_available_packet_size(0);
    let operation = SteamworksNetworkingOperation::PacketRead {
        channel: 0,
        packet: None,
    };
    let error = SteamworksNetworkingError::ClientUnavailable;
    let result = SteamworksNetworkingResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksNetworkingPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeNetworkingCommand::get_available_packet_size(0);
    let operation = PreludeNetworkingOperation::PacketRead {
        channel: 0,
        packet: None,
    };
    let error = PreludeNetworkingError::ClientUnavailable;
    let result = PreludeNetworkingResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(
        PreludeNetworkingPlugin::new(),
        command,
        operation,
        result,
        error,
    );
}

#[test]
fn networking_messages_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksNetworkingMessagesPlugin,
        _command: SteamworksNetworkingMessagesCommand,
        _operation: SteamworksNetworkingMessagesOperation,
        _result: SteamworksNetworkingMessagesResult,
        _error: SteamworksNetworkingMessagesError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeNetworkingMessagesPlugin,
        _command: PreludeNetworkingMessagesCommand,
        _operation: PreludeNetworkingMessagesOperation,
        _result: PreludeNetworkingMessagesResult,
        _error: PreludeNetworkingMessagesError,
    ) {
    }

    let command = SteamworksNetworkingMessagesCommand::receive_messages(0, 1);
    let operation =
        SteamworksNetworkingMessagesOperation::AutoAcceptSessionRequestsSet { enabled: true };
    let error = SteamworksNetworkingMessagesError::ClientUnavailable;
    let result = SteamworksNetworkingMessagesResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksNetworkingMessagesPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeNetworkingMessagesCommand::receive_messages(0, 1);
    let operation =
        PreludeNetworkingMessagesOperation::AutoAcceptSessionRequestsSet { enabled: true };
    let error = PreludeNetworkingMessagesError::ClientUnavailable;
    let result = PreludeNetworkingMessagesResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(
        PreludeNetworkingMessagesPlugin::new(),
        command,
        operation,
        result,
        error,
    );
}

#[test]
fn networking_sockets_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksNetworkingSocketsPlugin,
        _command: SteamworksNetworkingSocketsCommand,
        _operation: SteamworksNetworkingSocketsOperation,
        _result: SteamworksNetworkingSocketsResult,
        _error: SteamworksNetworkingSocketsError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeNetworkingSocketsPlugin,
        _command: PreludeNetworkingSocketsCommand,
        _operation: PreludeNetworkingSocketsOperation,
        _result: PreludeNetworkingSocketsResult,
        _error: PreludeNetworkingSocketsError,
    ) {
    }

    let poll_group = SteamworksNetworkingSocketsPollGroupId::from_raw(1);
    let command = SteamworksNetworkingSocketsCommand::create_poll_group();
    let operation = SteamworksNetworkingSocketsOperation::PollGroupCreated { poll_group };
    let error = SteamworksNetworkingSocketsError::ClientUnavailable;
    let result = SteamworksNetworkingSocketsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksNetworkingSocketsPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let poll_group = PreludeNetworkingSocketsPollGroupId::from_raw(1);
    let command = PreludeNetworkingSocketsCommand::create_poll_group();
    let operation = PreludeNetworkingSocketsOperation::PollGroupCreated { poll_group };
    let error = PreludeNetworkingSocketsError::ClientUnavailable;
    let result = PreludeNetworkingSocketsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(
        PreludeNetworkingSocketsPlugin::new(),
        command,
        operation,
        result,
        error,
    );
}

#[test]
fn networking_utils_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksNetworkingUtilsPlugin,
        _command: SteamworksNetworkingUtilsCommand,
        _operation: SteamworksNetworkingUtilsOperation,
        _result: SteamworksNetworkingUtilsResult,
        _error: SteamworksNetworkingUtilsError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeNetworkingUtilsPlugin,
        _command: PreludeNetworkingUtilsCommand,
        _operation: PreludeNetworkingUtilsOperation,
        _result: PreludeNetworkingUtilsResult,
        _error: PreludeNetworkingUtilsError,
    ) {
    }

    fn accepts_root_state_status(
        _state: bevy_steamworks::SteamworksNetworkingUtilsState,
        _status: Option<bevy_steamworks::SteamworksRelayNetworkStatus>,
    ) {
    }

    fn accepts_prelude_state_status(
        _state: bevy_steamworks::prelude::SteamworksNetworkingUtilsState,
        _status: Option<bevy_steamworks::prelude::SteamworksRelayNetworkStatus>,
    ) {
    }

    accepts_root_state_status(
        bevy_steamworks::SteamworksNetworkingUtilsState::default(),
        None,
    );
    accepts_prelude_state_status(
        bevy_steamworks::prelude::SteamworksNetworkingUtilsState::default(),
        None,
    );

    let command = SteamworksNetworkingUtilsCommand::init_relay_network_access();
    let operation = SteamworksNetworkingUtilsOperation::RelayNetworkAccessInitialized;
    let error = SteamworksNetworkingUtilsError::ClientUnavailable;
    let result = SteamworksNetworkingUtilsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksNetworkingUtilsPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeNetworkingUtilsCommand::init_relay_network_access();
    let operation = PreludeNetworkingUtilsOperation::RelayNetworkAccessInitialized;
    let error = PreludeNetworkingUtilsError::ClientUnavailable;
    let result = PreludeNetworkingUtilsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(
        PreludeNetworkingUtilsPlugin::new(),
        command,
        operation,
        result,
        error,
    );
}

#[test]
fn remote_storage_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksRemoteStoragePlugin,
        _command: SteamworksRemoteStorageCommand,
        _operation: SteamworksRemoteStorageOperation,
        _result: SteamworksRemoteStorageResult,
        _error: SteamworksRemoteStorageError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeRemoteStoragePlugin,
        _command: PreludeRemoteStorageCommand,
        _operation: PreludeRemoteStorageOperation,
        _result: PreludeRemoteStorageResult,
        _error: PreludeRemoteStorageError,
    ) {
    }

    let command = SteamworksRemoteStorageCommand::GetCloudInfo;
    let operation = SteamworksRemoteStorageOperation::CloudEnabledForAppRead { enabled: true };
    let error = SteamworksRemoteStorageError::ClientUnavailable;
    let result = SteamworksRemoteStorageResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksRemoteStoragePlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeRemoteStorageCommand::GetCloudInfo;
    let operation = PreludeRemoteStorageOperation::CloudEnabledForAppRead { enabled: true };
    let error = PreludeRemoteStorageError::ClientUnavailable;
    let result = PreludeRemoteStorageResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(
        PreludeRemoteStoragePlugin::new(),
        command,
        operation,
        result,
        error,
    );
}

#[test]
fn remote_play_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksRemotePlayPlugin,
        _command: SteamworksRemotePlayCommand,
        _operation: SteamworksRemotePlayOperation,
        _result: SteamworksRemotePlayResult,
        _error: SteamworksRemotePlayError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeRemotePlayPlugin,
        _command: PreludeRemotePlayCommand,
        _operation: PreludeRemotePlayOperation,
        _result: PreludeRemotePlayResult,
        _error: PreludeRemotePlayError,
    ) {
    }

    let command = SteamworksRemotePlayCommand::ListSessions;
    let operation = SteamworksRemotePlayOperation::SessionsListed { sessions: vec![] };
    let error = SteamworksRemotePlayError::ClientUnavailable;
    let result = SteamworksRemotePlayResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksRemotePlayPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeRemotePlayCommand::ListSessions;
    let operation = PreludeRemotePlayOperation::SessionsListed { sessions: vec![] };
    let error = PreludeRemotePlayError::ClientUnavailable;
    let result = PreludeRemotePlayResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(
        PreludeRemotePlayPlugin::new(),
        command,
        operation,
        result,
        error,
    );
}

#[test]
fn screenshots_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksScreenshotsPlugin,
        _command: SteamworksScreenshotsCommand,
        _operation: SteamworksScreenshotsOperation,
        _result: SteamworksScreenshotsResult,
        _error: SteamworksScreenshotsError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeScreenshotsPlugin,
        _command: PreludeScreenshotsCommand,
        _operation: PreludeScreenshotsOperation,
        _result: PreludeScreenshotsResult,
        _error: PreludeScreenshotsError,
    ) {
    }

    let command = SteamworksScreenshotsCommand::hook_screenshots(true);
    let operation = SteamworksScreenshotsOperation::ScreenshotsHookSet { hook: true };
    let error = SteamworksScreenshotsError::ClientUnavailable;
    let result = SteamworksScreenshotsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksScreenshotsPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeScreenshotsCommand::hook_screenshots(true);
    let operation = PreludeScreenshotsOperation::ScreenshotsHookSet { hook: true };
    let error = PreludeScreenshotsError::ClientUnavailable;
    let result = PreludeScreenshotsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(
        PreludeScreenshotsPlugin::new(),
        command,
        operation,
        result,
        error,
    );
}

#[test]
fn timeline_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksTimelinePlugin,
        _command: SteamworksTimelineCommand,
        _operation: SteamworksTimelineOperation,
        _result: SteamworksTimelineResult,
        _error: SteamworksTimelineError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeTimelinePlugin,
        _command: PreludeTimelineCommand,
        _operation: PreludeTimelineOperation,
        _result: PreludeTimelineResult,
        _error: PreludeTimelineError,
    ) {
    }

    let command = SteamworksTimelineCommand::set_game_mode(SteamworksTimelineGameMode::Playing);
    let operation = SteamworksTimelineOperation::GameModeSet {
        mode: SteamworksTimelineGameMode::Playing,
    };
    let error = SteamworksTimelineError::ClientUnavailable;
    let result = SteamworksTimelineResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksTimelinePlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeTimelineCommand::set_game_mode(PreludeTimelineGameMode::Playing);
    let operation = PreludeTimelineOperation::GameModeSet {
        mode: PreludeTimelineGameMode::Playing,
    };
    let error = PreludeTimelineError::ClientUnavailable;
    let result = PreludeTimelineResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(
        PreludeTimelinePlugin::new(),
        command,
        operation,
        result,
        error,
    );
}

#[test]
fn user_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksUserPlugin,
        _command: SteamworksUserCommand,
        _operation: SteamworksUserOperation,
        _result: SteamworksUserResult,
        _error: SteamworksUserError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeUserPlugin,
        _command: PreludeUserCommand,
        _operation: PreludeUserOperation,
        _result: PreludeUserResult,
        _error: PreludeUserError,
    ) {
    }

    let command = SteamworksUserCommand::GetLevel;
    let operation = SteamworksUserOperation::LevelRead { level: 1 };
    let error = SteamworksUserError::ClientUnavailable;
    let result = SteamworksUserResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksUserPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeUserCommand::GetLevel;
    let operation = PreludeUserOperation::LevelRead { level: 1 };
    let error = PreludeUserError::ClientUnavailable;
    let result = PreludeUserResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(PreludeUserPlugin::new(), command, operation, result, error);
}

#[test]
fn stats_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksStatsPlugin,
        _command: SteamworksStatsCommand,
        _operation: SteamworksStatsOperation,
        _result: SteamworksStatsResult,
        _error: SteamworksStatsError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeStatsPlugin,
        _command: PreludeStatsCommand,
        _operation: PreludeStatsOperation,
        _result: PreludeStatsResult,
        _error: PreludeStatsError,
    ) {
    }

    let _root_settings: bevy_steamworks::SteamworksStatsSettings =
        bevy_steamworks::SteamworksStatsSettings::default();
    let _root_state: bevy_steamworks::SteamworksStatsState =
        bevy_steamworks::SteamworksStatsState::default();
    let _root_leaderboard: bevy_steamworks::SteamworksLeaderboardId =
        bevy_steamworks::SteamworksLeaderboardId::from_raw(1);
    let _root_achievement: Option<bevy_steamworks::SteamworksAchievementInfo> = None;
    let _root_achievement_icon: Option<bevy_steamworks::SteamworksAchievementIcon> = None;
    let _root_achievement_icon_status: bevy_steamworks::SteamworksAchievementIconStatus =
        bevy_steamworks::SteamworksAchievementIconStatus::PendingOrUnavailable;
    let _root_achievement_percentage: Option<
        bevy_steamworks::SteamworksAchievementGlobalPercentage,
    > = None;
    let _root_achievement_attribute: Option<
        bevy_steamworks::SteamworksAchievementDisplayAttribute,
    > = None;
    let _root_global_stat: Option<bevy_steamworks::SteamworksGlobalStatValue<i64>> = None;
    let _root_global_stat_history: Option<bevy_steamworks::SteamworksGlobalStatHistory<i64>> = None;
    let _root_leaderboard_info: Option<bevy_steamworks::SteamworksLeaderboardInfo> = None;
    let _root_leaderboard_entry: Option<bevy_steamworks::SteamworksLeaderboardEntry> = None;
    let _root_leaderboard_score_upload: Option<
        bevy_steamworks::SteamworksLeaderboardScoreUploaded,
    > = None;

    let _prelude_settings: bevy_steamworks::prelude::SteamworksStatsSettings =
        bevy_steamworks::prelude::SteamworksStatsSettings::default();
    let _prelude_state: bevy_steamworks::prelude::SteamworksStatsState =
        bevy_steamworks::prelude::SteamworksStatsState::default();
    let _prelude_leaderboard: bevy_steamworks::prelude::SteamworksLeaderboardId =
        bevy_steamworks::prelude::SteamworksLeaderboardId::from_raw(1);
    let _prelude_achievement: Option<bevy_steamworks::prelude::SteamworksAchievementInfo> = None;
    let _prelude_achievement_icon: Option<bevy_steamworks::prelude::SteamworksAchievementIcon> =
        None;
    let _prelude_achievement_icon_status:
        bevy_steamworks::prelude::SteamworksAchievementIconStatus =
        bevy_steamworks::prelude::SteamworksAchievementIconStatus::PendingOrUnavailable;
    let _prelude_achievement_percentage: Option<
        bevy_steamworks::prelude::SteamworksAchievementGlobalPercentage,
    > = None;
    let _prelude_achievement_attribute: Option<
        bevy_steamworks::prelude::SteamworksAchievementDisplayAttribute,
    > = None;
    let _prelude_global_stat: Option<bevy_steamworks::prelude::SteamworksGlobalStatValue<i64>> =
        None;
    let _prelude_global_stat_history: Option<
        bevy_steamworks::prelude::SteamworksGlobalStatHistory<i64>,
    > = None;
    let _prelude_leaderboard_info: Option<bevy_steamworks::prelude::SteamworksLeaderboardInfo> =
        None;
    let _prelude_leaderboard_entry: Option<bevy_steamworks::prelude::SteamworksLeaderboardEntry> =
        None;
    let _prelude_leaderboard_score_upload: Option<
        bevy_steamworks::prelude::SteamworksLeaderboardScoreUploaded,
    > = None;

    assert_eq!(
        bevy_steamworks::STEAMWORKS_LEADERBOARD_MAX_DETAILS,
        bevy_steamworks::prelude::STEAMWORKS_LEADERBOARD_MAX_DETAILS,
    );
    assert_eq!(
        bevy_steamworks::STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND,
        bevy_steamworks::prelude::STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND,
    );
    assert_eq!(
        bevy_steamworks::STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND,
        bevy_steamworks::prelude::STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND,
    );
    assert_eq!(
        bevy_steamworks::STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND,
        bevy_steamworks::prelude::STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND,
    );

    let command = SteamworksStatsCommand::RequestCurrentUserStats;
    let operation = SteamworksStatsOperation::GlobalAchievementPercentagesRequested;
    let error = SteamworksStatsError::ClientUnavailable;
    let result = SteamworksStatsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksStatsPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeStatsCommand::RequestCurrentUserStats;
    let operation = PreludeStatsOperation::GlobalAchievementPercentagesRequested;
    let error = PreludeStatsError::ClientUnavailable;
    let result = PreludeStatsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(PreludeStatsPlugin::new(), command, operation, result, error);
}

#[test]
fn ugc_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksUgcPlugin,
        _command: SteamworksUgcCommand,
        _operation: SteamworksUgcOperation,
        _result: SteamworksUgcResult,
        _error: SteamworksUgcError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeUgcPlugin,
        _command: PreludeUgcCommand,
        _operation: PreludeUgcOperation,
        _result: PreludeUgcResult,
        _error: PreludeUgcError,
    ) {
    }

    let command = SteamworksUgcCommand::suspend_downloads(false);
    let operation = SteamworksUgcOperation::DownloadsSuspended { suspend: false };
    let error = SteamworksUgcError::ClientUnavailable;
    let result = SteamworksUgcResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksUgcPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeUgcCommand::suspend_downloads(false);
    let operation = PreludeUgcOperation::DownloadsSuspended { suspend: false };
    let error = PreludeUgcError::ClientUnavailable;
    let result = PreludeUgcResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(PreludeUgcPlugin::new(), command, operation, result, error);
}

#[test]
fn utils_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksUtilsPlugin,
        _command: SteamworksUtilsCommand,
        _operation: SteamworksUtilsOperation,
        _result: SteamworksUtilsResult,
        _error: SteamworksUtilsError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeUtilsPlugin,
        _command: PreludeUtilsCommand,
        _operation: PreludeUtilsOperation,
        _result: PreludeUtilsResult,
        _error: PreludeUtilsError,
    ) {
    }

    let command = SteamworksUtilsCommand::set_overlay_notification_position(
        SteamworksNotificationPosition::TopLeft,
    );
    let operation = SteamworksUtilsOperation::OverlayNotificationPositionSet {
        position: SteamworksNotificationPosition::TopLeft,
    };
    let error = SteamworksUtilsError::ClientUnavailable;
    let result = SteamworksUtilsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksUtilsPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeUtilsCommand::set_overlay_notification_position(
        PreludeNotificationPosition::TopLeft,
    );
    let operation = PreludeUtilsOperation::OverlayNotificationPositionSet {
        position: PreludeNotificationPosition::TopLeft,
    };
    let error = PreludeUtilsError::ClientUnavailable;
    let result = PreludeUtilsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(PreludeUtilsPlugin::new(), command, operation, result, error);
}

#[test]
fn matchmaking_servers_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksMatchmakingServersPlugin,
        _command: SteamworksMatchmakingServersCommand,
        _operation: SteamworksMatchmakingServersOperation,
        _result: SteamworksMatchmakingServersResult,
        _error: SteamworksMatchmakingServersError,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeMatchmakingServersPlugin,
        _command: PreludeMatchmakingServersCommand,
        _operation: PreludeMatchmakingServersOperation,
        _result: PreludeMatchmakingServersResult,
        _error: PreludeMatchmakingServersError,
    ) {
    }

    let request = SteamworksServerListRequestId::from_raw(1);
    let filters = SteamworksServerListFilters::new().with("map", "arena");
    let command =
        SteamworksMatchmakingServersCommand::request_internet_server_list(480, filters.clone());
    let operation = SteamworksMatchmakingServersOperation::ServerListRequested {
        request,
        app_id: 480.into(),
        kind: SteamworksServerListKind::Internet,
        filters,
    };
    let error = SteamworksMatchmakingServersError::ClientUnavailable;
    let result = SteamworksMatchmakingServersResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(
        SteamworksMatchmakingServersPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let request = PreludeServerListRequestId::from_raw(1);
    let filters = PreludeServerListFilters::new().with("map", "arena");
    let command =
        PreludeMatchmakingServersCommand::request_internet_server_list(480, filters.clone());
    let operation = PreludeMatchmakingServersOperation::ServerListRequested {
        request,
        app_id: 480.into(),
        kind: PreludeServerListKind::Internet,
        filters,
    };
    let error = PreludeMatchmakingServersError::ClientUnavailable;
    let result = PreludeMatchmakingServersResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(
        PreludeMatchmakingServersPlugin::new(),
        command,
        operation,
        result,
        error,
    );
}
