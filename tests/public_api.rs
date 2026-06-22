use bevy_app::{Plugin, PluginGroup};
use bevy_steamworks::{
    prelude::{
        SteamAPIInitError as PreludeInitError, SteamworksAppsCommand as PreludeAppsCommand,
        SteamworksAppsError as PreludeAppsError, SteamworksAppsOperation as PreludeAppsOperation,
        SteamworksAppsPlugin as PreludeAppsPlugin, SteamworksAppsResult as PreludeAppsResult,
        SteamworksCallbackRegistry as PreludeCallbackRegistry, SteamworksClient as PreludeClient,
        SteamworksClientPlugins as PreludeClientPlugins,
        SteamworksCommandError as PreludeCommandError,
        SteamworksConnectionRequestPolicy as PreludeConnectionRequestPolicy,
        SteamworksEvent as PreludeEvent, SteamworksFailurePolicy as PreludeFailurePolicy,
        SteamworksFloatingGamepadTextInputDismissed as PreludeFloatingGamepadTextInputDismissed,
        SteamworksFloatingGamepadTextInputMode as PreludeFloatingGamepadTextInputMode,
        SteamworksFloatingGamepadTextInputRequest as PreludeFloatingGamepadTextInputRequest,
        SteamworksFloatingGamepadTextInputShown as PreludeFloatingGamepadTextInputShown,
        SteamworksFriendsCommand as PreludeFriendsCommand,
        SteamworksFriendsError as PreludeFriendsError,
        SteamworksFriendsOperation as PreludeFriendsOperation,
        SteamworksFriendsPlugin as PreludeFriendsPlugin,
        SteamworksFriendsResult as PreludeFriendsResult,
        SteamworksGamepadTextInputDismissed as PreludeGamepadTextInputDismissed,
        SteamworksGamepadTextInputLineMode as PreludeGamepadTextInputLineMode,
        SteamworksGamepadTextInputMode as PreludeGamepadTextInputMode,
        SteamworksGamepadTextInputRequest as PreludeGamepadTextInputRequest,
        SteamworksGamepadTextInputShown as PreludeGamepadTextInputShown,
        SteamworksGamepadTextInputSubmitted as PreludeGamepadTextInputSubmitted,
        SteamworksInitMode as PreludeInitMode, SteamworksInputCommand as PreludeInputCommand,
        SteamworksInputError as PreludeInputError,
        SteamworksInputOperation as PreludeInputOperation,
        SteamworksInputPlugin as PreludeInputPlugin, SteamworksInputResult as PreludeInputResult,
        SteamworksIssuedAuthSessionTicketForIdentity as PreludeIssuedAuthSessionTicketForIdentity,
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
        SteamworksNetworkingMessagesState as PreludeNetworkingMessagesState,
        SteamworksNetworkingOperation as PreludeNetworkingOperation,
        SteamworksNetworkingPeer as PreludeNetworkingPeer,
        SteamworksNetworkingPlugin as PreludeNetworkingPlugin,
        SteamworksNetworkingResult as PreludeNetworkingResult,
        SteamworksNetworkingSocketsCommand as PreludeNetworkingSocketsCommand,
        SteamworksNetworkingSocketsConfigEntry as PreludeNetworkingSocketsConfigEntry,
        SteamworksNetworkingSocketsConnectionId as PreludeNetworkingSocketsConnectionId,
        SteamworksNetworkingSocketsConnectionMessages as PreludeNetworkingSocketsConnectionMessages,
        SteamworksNetworkingSocketsConnectionName as PreludeNetworkingSocketsConnectionName,
        SteamworksNetworkingSocketsConnectionUserData as PreludeNetworkingSocketsConnectionUserData,
        SteamworksNetworkingSocketsError as PreludeNetworkingSocketsError,
        SteamworksNetworkingSocketsListenEndpoint as PreludeNetworkingSocketsListenEndpoint,
        SteamworksNetworkingSocketsMessageSendResult as PreludeNetworkingSocketsMessageSendResult,
        SteamworksNetworkingSocketsOperation as PreludeNetworkingSocketsOperation,
        SteamworksNetworkingSocketsOutboundMessage as PreludeNetworkingSocketsOutboundMessage,
        SteamworksNetworkingSocketsPlugin as PreludeNetworkingSocketsPlugin,
        SteamworksNetworkingSocketsPollGroupMessages as PreludeNetworkingSocketsPollGroupMessages,
        SteamworksNetworkingSocketsResult as PreludeNetworkingSocketsResult,
        SteamworksNetworkingState as PreludeNetworkingState,
        SteamworksNetworkingUtilsCommand as PreludeNetworkingUtilsCommand,
        SteamworksNetworkingUtilsError as PreludeNetworkingUtilsError,
        SteamworksNetworkingUtilsOperation as PreludeNetworkingUtilsOperation,
        SteamworksNetworkingUtilsPlugin as PreludeNetworkingUtilsPlugin,
        SteamworksNetworkingUtilsResult as PreludeNetworkingUtilsResult,
        SteamworksNotificationPosition as PreludeNotificationPosition,
        SteamworksOverlayToStoreAction as PreludeOverlayToStoreAction,
        SteamworksPlugin as PreludePlugin, SteamworksPlugins as PreludePlugins,
        SteamworksRemotePlayCommand as PreludeRemotePlayCommand,
        SteamworksRemotePlayError as PreludeRemotePlayError,
        SteamworksRemotePlayOperation as PreludeRemotePlayOperation,
        SteamworksRemotePlayPlugin as PreludeRemotePlayPlugin,
        SteamworksRemotePlayResult as PreludeRemotePlayResult,
        SteamworksRemoteStorageCommand as PreludeRemoteStorageCommand,
        SteamworksRemoteStorageError as PreludeRemoteStorageError,
        SteamworksRemoteStorageFileContents as PreludeRemoteStorageFileContents,
        SteamworksRemoteStorageFileWrite as PreludeRemoteStorageFileWrite,
        SteamworksRemoteStorageFileWritten as PreludeRemoteStorageFileWritten,
        SteamworksRemoteStorageOperation as PreludeRemoteStorageOperation,
        SteamworksRemoteStoragePlugin as PreludeRemoteStoragePlugin,
        SteamworksRemoteStorageResult as PreludeRemoteStorageResult,
        SteamworksScreenshotsCommand as PreludeScreenshotsCommand,
        SteamworksScreenshotsError as PreludeScreenshotsError,
        SteamworksScreenshotsOperation as PreludeScreenshotsOperation,
        SteamworksScreenshotsPlugin as PreludeScreenshotsPlugin,
        SteamworksScreenshotsResult as PreludeScreenshotsResult,
        SteamworksServerCommand as PreludeServerCommand,
        SteamworksServerConfig as PreludeServerConfig, SteamworksServerError as PreludeServerError,
        SteamworksServerInitMode as PreludeServerInitMode,
        SteamworksServerIssuedAuthSessionTicketForIdentity as PreludeServerIssuedAuthSessionTicketForIdentity,
        SteamworksServerListFilters as PreludeServerListFilters,
        SteamworksServerListKind as PreludeServerListKind,
        SteamworksServerListRequestId as PreludeServerListRequestId,
        SteamworksServerOperation as PreludeServerOperation,
        SteamworksServerPing as PreludeServerPing,
        SteamworksServerPlayerDetails as PreludeServerPlayerDetails,
        SteamworksServerPlayerInfo as PreludeServerPlayerInfo,
        SteamworksServerPlugin as PreludeServerPlugin,
        SteamworksServerQueryId as PreludeServerQueryId,
        SteamworksServerQueryInfo as PreludeServerQueryInfo,
        SteamworksServerQueryKind as PreludeServerQueryKind,
        SteamworksServerQueryTarget as PreludeServerQueryTarget,
        SteamworksServerResult as PreludeServerResult, SteamworksServerRule as PreludeServerRule,
        SteamworksServerRules as PreludeServerRules,
        SteamworksServerUnavailable as PreludeServerUnavailable,
        SteamworksStatsCommand as PreludeStatsCommand, SteamworksStatsError as PreludeStatsError,
        SteamworksStatsOperation as PreludeStatsOperation,
        SteamworksStatsPlugin as PreludeStatsPlugin, SteamworksStatsResult as PreludeStatsResult,
        SteamworksSystem as PreludeSystem, SteamworksTimelineCommand as PreludeTimelineCommand,
        SteamworksTimelineError as PreludeTimelineError,
        SteamworksTimelineGameMode as PreludeTimelineGameMode,
        SteamworksTimelineOperation as PreludeTimelineOperation,
        SteamworksTimelinePlugin as PreludeTimelinePlugin,
        SteamworksTimelineResult as PreludeTimelineResult,
        SteamworksUgcCommand as PreludeUgcCommand,
        SteamworksUgcContentDescriptor as PreludeUgcContentDescriptor,
        SteamworksUgcDownloadItemResult as PreludeUgcDownloadItemResult,
        SteamworksUgcError as PreludeUgcError,
        SteamworksUgcGameServerWorkshopInit as PreludeUgcGameServerWorkshopInit,
        SteamworksUgcItemDetails as PreludeUgcItemDetails,
        SteamworksUgcItemDownloadInfo as PreludeUgcItemDownloadInfo,
        SteamworksUgcItemDownloadInfoResult as PreludeUgcItemDownloadInfoResult,
        SteamworksUgcItemInstallInfo as PreludeUgcItemInstallInfo,
        SteamworksUgcItemInstallInfoResult as PreludeUgcItemInstallInfoResult,
        SteamworksUgcItemStateInfo as PreludeUgcItemStateInfo,
        SteamworksUgcOperation as PreludeUgcOperation, SteamworksUgcPlugin as PreludeUgcPlugin,
        SteamworksUgcQuery as PreludeUgcQuery, SteamworksUgcQueryIds as PreludeUgcQueryIds,
        SteamworksUgcQueryOptions as PreludeUgcQueryOptions,
        SteamworksUgcQueryTotal as PreludeUgcQueryTotal, SteamworksUgcResult as PreludeUgcResult,
        SteamworksUgcState as PreludeUgcState,
        SteamworksUgcWorkshopDepotId as PreludeUgcWorkshopDepotId,
        SteamworksUnavailable as PreludeUnavailable, SteamworksUserCommand as PreludeUserCommand,
        SteamworksUserError as PreludeUserError, SteamworksUserOperation as PreludeUserOperation,
        SteamworksUserPlugin as PreludeUserPlugin, SteamworksUserResult as PreludeUserResult,
        SteamworksUtilsCommand as PreludeUtilsCommand, SteamworksUtilsError as PreludeUtilsError,
        SteamworksUtilsOperation as PreludeUtilsOperation,
        SteamworksUtilsPlugin as PreludeUtilsPlugin, SteamworksUtilsResult as PreludeUtilsResult,
    },
    SteamAPIInitError, SteamworksAppsCommand, SteamworksAppsError, SteamworksAppsOperation,
    SteamworksAppsPlugin, SteamworksAppsResult, SteamworksCallbackRegistry, SteamworksClient,
    SteamworksClientPlugins, SteamworksCommandError, SteamworksConnectionRequestPolicy,
    SteamworksEvent, SteamworksFailurePolicy, SteamworksFloatingGamepadTextInputDismissed,
    SteamworksFloatingGamepadTextInputMode, SteamworksFloatingGamepadTextInputRequest,
    SteamworksFloatingGamepadTextInputShown, SteamworksFriendsCommand, SteamworksFriendsError,
    SteamworksFriendsOperation, SteamworksFriendsPlugin, SteamworksFriendsResult,
    SteamworksGamepadTextInputDismissed, SteamworksGamepadTextInputLineMode,
    SteamworksGamepadTextInputMode, SteamworksGamepadTextInputRequest,
    SteamworksGamepadTextInputShown, SteamworksGamepadTextInputSubmitted, SteamworksInitMode,
    SteamworksInputCommand, SteamworksInputError, SteamworksInputOperation, SteamworksInputPlugin,
    SteamworksInputResult, SteamworksIssuedAuthSessionTicketForIdentity, SteamworksLobbyListFilter,
    SteamworksMatchmakingCommand, SteamworksMatchmakingError, SteamworksMatchmakingOperation,
    SteamworksMatchmakingPlugin, SteamworksMatchmakingResult, SteamworksMatchmakingServersCommand,
    SteamworksMatchmakingServersError, SteamworksMatchmakingServersOperation,
    SteamworksMatchmakingServersPlugin, SteamworksMatchmakingServersResult,
    SteamworksNetworkingCommand, SteamworksNetworkingError, SteamworksNetworkingMessagesCommand,
    SteamworksNetworkingMessagesError, SteamworksNetworkingMessagesOperation,
    SteamworksNetworkingMessagesPlugin, SteamworksNetworkingMessagesResult,
    SteamworksNetworkingMessagesState, SteamworksNetworkingOperation, SteamworksNetworkingPeer,
    SteamworksNetworkingPlugin, SteamworksNetworkingResult, SteamworksNetworkingSocketsCommand,
    SteamworksNetworkingSocketsConfigEntry, SteamworksNetworkingSocketsConnectionId,
    SteamworksNetworkingSocketsConnectionMessages, SteamworksNetworkingSocketsConnectionName,
    SteamworksNetworkingSocketsConnectionUserData, SteamworksNetworkingSocketsError,
    SteamworksNetworkingSocketsListenEndpoint, SteamworksNetworkingSocketsMessageSendResult,
    SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsOutboundMessage,
    SteamworksNetworkingSocketsPlugin, SteamworksNetworkingSocketsPollGroupMessages,
    SteamworksNetworkingSocketsResult, SteamworksNetworkingState, SteamworksNetworkingUtilsCommand,
    SteamworksNetworkingUtilsError, SteamworksNetworkingUtilsOperation,
    SteamworksNetworkingUtilsPlugin, SteamworksNetworkingUtilsResult,
    SteamworksNotificationPosition, SteamworksOverlayToStoreAction, SteamworksPlugin,
    SteamworksPlugins, SteamworksRemotePlayCommand, SteamworksRemotePlayError,
    SteamworksRemotePlayOperation, SteamworksRemotePlayPlugin, SteamworksRemotePlayResult,
    SteamworksRemoteStorageCommand, SteamworksRemoteStorageError,
    SteamworksRemoteStorageFileContents, SteamworksRemoteStorageFileWrite,
    SteamworksRemoteStorageFileWritten, SteamworksRemoteStorageOperation,
    SteamworksRemoteStoragePlugin, SteamworksRemoteStorageResult, SteamworksScreenshotsCommand,
    SteamworksScreenshotsError, SteamworksScreenshotsOperation, SteamworksScreenshotsPlugin,
    SteamworksScreenshotsResult, SteamworksServerCommand, SteamworksServerConfig,
    SteamworksServerError, SteamworksServerInitMode,
    SteamworksServerIssuedAuthSessionTicketForIdentity, SteamworksServerListFilters,
    SteamworksServerListKind, SteamworksServerListRequestId, SteamworksServerOperation,
    SteamworksServerPing, SteamworksServerPlayerDetails, SteamworksServerPlayerInfo,
    SteamworksServerPlugin, SteamworksServerQueryId, SteamworksServerQueryInfo,
    SteamworksServerQueryKind, SteamworksServerQueryTarget, SteamworksServerResult,
    SteamworksServerRule, SteamworksServerRules, SteamworksServerUnavailable,
    SteamworksStatsCommand, SteamworksStatsError, SteamworksStatsOperation, SteamworksStatsPlugin,
    SteamworksStatsResult, SteamworksSystem, SteamworksTimelineCommand, SteamworksTimelineError,
    SteamworksTimelineGameMode, SteamworksTimelineOperation, SteamworksTimelinePlugin,
    SteamworksTimelineResult, SteamworksUgcCommand, SteamworksUgcContentDescriptor,
    SteamworksUgcDownloadItemResult, SteamworksUgcError, SteamworksUgcGameServerWorkshopInit,
    SteamworksUgcItemDetails, SteamworksUgcItemDownloadInfo, SteamworksUgcItemDownloadInfoResult,
    SteamworksUgcItemInstallInfo, SteamworksUgcItemInstallInfoResult, SteamworksUgcItemStateInfo,
    SteamworksUgcOperation, SteamworksUgcPlugin, SteamworksUgcQuery, SteamworksUgcQueryIds,
    SteamworksUgcQueryOptions, SteamworksUgcQueryTotal, SteamworksUgcResult, SteamworksUgcState,
    SteamworksUgcWorkshopDepotId, SteamworksUnavailable, SteamworksUserCommand,
    SteamworksUserError, SteamworksUserOperation, SteamworksUserPlugin, SteamworksUserResult,
    SteamworksUtilsCommand, SteamworksUtilsError, SteamworksUtilsOperation, SteamworksUtilsPlugin,
    SteamworksUtilsResult,
};
use std::error::Error;

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

    let command = SteamworksAppsCommand::get_current_app_info();
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
    accepts_root_exports(
        SteamworksAppsPlugin::new(),
        SteamworksAppsCommand::get_launch_command_line(),
        SteamworksAppsOperation::LaunchCommandLineRead {
            command_line: "+connect 127.0.0.1".to_owned(),
        },
        SteamworksAppsResult::Ok(SteamworksAppsOperation::AppBuildIdRead { build_id: 1 }),
        SteamworksAppsError::ClientUnavailable,
    );

    let command = PreludeAppsCommand::is_subscribed();
    let operation = PreludeAppsOperation::SubscriptionRead { subscribed: true };
    let error = PreludeAppsError::ClientUnavailable;
    let result = PreludeAppsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(PreludeAppsPlugin::new(), command, operation, result, error);
    accepts_prelude_exports(
        PreludeAppsPlugin::new(),
        PreludeAppsCommand::get_available_game_languages(),
        PreludeAppsOperation::AvailableGameLanguagesRead {
            languages: Vec::new(),
        },
        PreludeAppsResult::Ok(PreludeAppsOperation::CurrentGameLanguageRead {
            language: "english".to_owned(),
        }),
        PreludeAppsError::ClientUnavailable,
    );
}

#[test]
fn result_helper_api_is_exported_from_root_and_prelude() {
    fn accepts_root_command_error(
        _error: SteamworksCommandError<SteamworksAppsCommand, SteamworksAppsError>,
    ) {
    }

    fn accepts_prelude_command_error(
        _error: PreludeCommandError<PreludeAppsCommand, PreludeAppsError>,
    ) {
    }

    let operation = SteamworksAppsOperation::SubscriptionRead { subscribed: true };
    let ok = SteamworksAppsResult::Ok(operation.clone());

    assert!(ok.is_ok());
    assert!(!ok.is_err());
    assert_eq!(ok.operation(), Some(&operation));
    assert_eq!(ok.command(), None);
    assert_eq!(ok.error(), None);
    assert_eq!(ok.as_result(), Ok(&operation));
    assert_eq!(ok.into_result(), Ok(operation));

    let command = SteamworksAppsCommand::is_subscribed();
    let error = SteamworksAppsError::ClientUnavailable;
    let err = SteamworksAppsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    assert!(!err.is_ok());
    assert!(err.is_err());
    assert_eq!(err.operation(), None);
    assert_eq!(err.command(), Some(&command));
    assert_eq!(err.error(), Some(&error));
    assert_eq!(err.as_result(), Err((&command, &error)));
    let failure = err.into_result().expect_err("apps result should fail");
    assert_eq!(failure.command(), &command);
    assert_eq!(failure.error(), &error);
    assert_eq!(
        failure.to_string(),
        "Steamworks command IsSubscribed failed: SteamworksClient resource is not available"
    );
    assert_eq!(
        failure.source().map(ToString::to_string),
        Some("SteamworksClient resource is not available".into())
    );
    assert_eq!(failure.into_parts(), (command.clone(), error.clone()));
    accepts_root_command_error(SteamworksCommandError::new(command, error));

    let operation = PreludeAppsOperation::SubscriptionRead { subscribed: false };
    let ok = PreludeAppsResult::Ok(operation.clone());
    assert_eq!(ok.operation(), Some(&operation));
    assert_eq!(ok.as_result(), Ok(&operation));
    assert_eq!(ok.into_result(), Ok(operation));
    accepts_prelude_command_error(PreludeCommandError::new(
        PreludeAppsCommand::is_subscribed(),
        PreludeAppsError::ClientUnavailable,
    ));

    let command = SteamworksServerCommand::get_steam_id();
    let error = SteamworksServerError::ServerUnavailable;
    let err = SteamworksServerResult::Err {
        command: command.clone(),
        error: error.clone(),
    };
    assert_eq!(err.command(), Some(&command));
    assert_eq!(err.error(), Some(&error));
    let failure = err.into_result().expect_err("server result should fail");
    assert_eq!(failure.into_parts(), (command, error));
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

    let command = SteamworksFriendsCommand::get_persona_name();
    let operation = SteamworksFriendsOperation::PersonaNameRead {
        name: String::new(),
    };
    let error = SteamworksFriendsError::ClientUnavailable;
    let result = SteamworksFriendsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };
    assert!(matches!(
        steamworks::OverlayToStoreFlag::from(SteamworksOverlayToStoreAction::AddToCart),
        steamworks::OverlayToStoreFlag::AddToCart
    ));

    accepts_root_exports(
        SteamworksFriendsPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeFriendsCommand::get_persona_name();
    let operation = PreludeFriendsOperation::PersonaNameRead {
        name: String::new(),
    };
    let error = PreludeFriendsError::ClientUnavailable;
    let result = PreludeFriendsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };
    assert!(matches!(
        steamworks::OverlayToStoreFlag::from(PreludeOverlayToStoreAction::AddToCartAndShow),
        steamworks::OverlayToStoreFlag::AddToCartAndShow
    ));

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
        _unavailable: SteamworksServerUnavailable,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeServerPlugin,
        _command: PreludeServerCommand,
        _operation: PreludeServerOperation,
        _result: PreludeServerResult,
        _error: PreludeServerError,
        _unavailable: PreludeServerUnavailable,
    ) {
    }

    let command = SteamworksServerCommand::enable_heartbeats(true);
    let operation = SteamworksServerOperation::HeartbeatsEnabled { active: true };
    let error = SteamworksServerError::ServerUnavailable;
    let result = SteamworksServerResult::Err {
        command: command.clone(),
        error: error.clone(),
    };
    let config = SteamworksServerConfig::new(
        std::net::Ipv4Addr::LOCALHOST,
        27015,
        27016,
        steamworks::ServerMode::Authentication,
        "1.0.0",
    );
    let plugin = SteamworksServerPlugin::new(config.clone())
        .failure_policy(SteamworksFailurePolicy::LogAndContinue)
        .run_callbacks(false);
    assert_eq!(
        plugin.init_mode(),
        &SteamworksServerInitMode::Config(config)
    );
    assert_eq!(
        plugin.failure_policy_setting(),
        SteamworksFailurePolicy::LogAndContinue
    );
    assert!(!plugin.runs_callbacks());
    let source = SteamAPIInitError::NoSteamClient("offline".into());
    let unavailable = SteamworksServerUnavailable::InitFailed {
        config: SteamworksServerConfig::new(
            std::net::Ipv4Addr::LOCALHOST,
            27015,
            27016,
            steamworks::ServerMode::Authentication,
            "1.0.0",
        ),
        source: source.clone(),
    };
    assert!(unavailable.is_init_failed());
    assert!(!unavailable.is_manual_server_missing());
    assert!(!unavailable.is_invalid_string());
    assert_eq!(unavailable.init_error(), Some(&source));
    assert!(unavailable.init_config().is_some());
    assert_eq!(unavailable.invalid_string_field(), None);
    let identity = steamworks::networking_types::NetworkingIdentity::new_ip(
        std::net::SocketAddr::from(([127, 0, 0, 1], 27015)),
    );
    let identity_command = SteamworksServerCommand::get_authentication_session_ticket_for_identity(
        SteamworksNetworkingPeer::from(identity),
    );
    assert!(matches!(
        identity_command,
        SteamworksServerCommand::GetAuthenticationSessionTicketForIdentity { .. }
    ));
    let _identity_ticket: Option<SteamworksServerIssuedAuthSessionTicketForIdentity> = None;
    let _identity_error = SteamworksServerError::InvalidNetworkingIdentity;

    accepts_root_exports(plugin, command, operation, result, error, unavailable);

    let command = PreludeServerCommand::enable_heartbeats(true);
    let operation = PreludeServerOperation::HeartbeatsEnabled { active: true };
    let error = PreludeServerError::ServerUnavailable;
    let result = PreludeServerResult::Err {
        command: command.clone(),
        error: error.clone(),
    };
    let config = PreludeServerConfig::new(
        std::net::Ipv4Addr::LOCALHOST,
        27015,
        27016,
        steamworks::ServerMode::Authentication,
        "1.0.0",
    );
    let plugin = PreludeServerPlugin::new(config.clone())
        .failure_policy(PreludeFailurePolicy::LogAndContinue)
        .run_callbacks(false);
    assert_eq!(plugin.init_mode(), &PreludeServerInitMode::Config(config));
    assert_eq!(
        plugin.failure_policy_setting(),
        PreludeFailurePolicy::LogAndContinue
    );
    assert!(!plugin.runs_callbacks());
    let unavailable = PreludeServerUnavailable::InvalidString { field: "version" };
    assert!(!unavailable.is_init_failed());
    assert!(!unavailable.is_manual_server_missing());
    assert!(unavailable.is_invalid_string());
    assert_eq!(unavailable.init_error(), None);
    assert_eq!(unavailable.init_config(), None);
    assert_eq!(unavailable.invalid_string_field(), Some("version"));
    let identity = steamworks::networking_types::NetworkingIdentity::new_ip(
        std::net::SocketAddr::from(([127, 0, 0, 1], 27015)),
    );
    let identity_command = PreludeServerCommand::get_authentication_session_ticket_for_identity(
        PreludeNetworkingPeer::from(identity),
    );
    assert!(matches!(
        identity_command,
        PreludeServerCommand::GetAuthenticationSessionTicketForIdentity { .. }
    ));
    let _identity_ticket: Option<PreludeServerIssuedAuthSessionTicketForIdentity> = None;
    let _identity_error = PreludeServerError::InvalidNetworkingIdentity;

    accepts_prelude_exports(plugin, command, operation, result, error, unavailable);
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

    assert!(bevy_steamworks::SteamworksInputHandle::from_raw(1).is_valid());
    assert!(!bevy_steamworks::SteamworksInputHandle::from_raw(0).is_valid());
    assert!(bevy_steamworks::SteamworksInputActionSetHandle::from_raw(1).is_valid());
    assert!(!bevy_steamworks::SteamworksInputActionSetHandle::from_raw(0).is_valid());
    assert!(bevy_steamworks::SteamworksInputDigitalActionHandle::from_raw(1).is_valid());
    assert!(!bevy_steamworks::SteamworksInputDigitalActionHandle::from_raw(0).is_valid());
    assert!(bevy_steamworks::SteamworksInputAnalogActionHandle::from_raw(1).is_valid());
    assert!(!bevy_steamworks::SteamworksInputAnalogActionHandle::from_raw(0).is_valid());
    let root_origin = bevy_steamworks::SteamworksInputActionOrigin::from_code(1);
    assert_eq!(root_origin.code(), 1);
    let root_origin_info = bevy_steamworks::SteamworksInputActionOriginInfo {
        origin: root_origin,
        glyph_path: "glyph.svg".to_owned(),
        name: "A Button".to_owned(),
    };
    assert_eq!(root_origin_info.origin, root_origin);

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
    accepts_root_exports(
        SteamworksInputPlugin::new(),
        SteamworksInputCommand::run_frame(),
        SteamworksInputOperation::FrameRun,
        SteamworksInputResult::Ok(SteamworksInputOperation::Shutdown),
        SteamworksInputError::ClientUnavailable,
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
    accepts_prelude_exports(
        PreludeInputPlugin::new(),
        PreludeInputCommand::list_controllers(),
        PreludeInputOperation::ControllersListed {
            controllers: Vec::new(),
        },
        PreludeInputResult::Ok(PreludeInputOperation::FrameRun),
        PreludeInputError::ClientUnavailable,
    );
    let prelude_origin = bevy_steamworks::prelude::SteamworksInputActionOrigin::from_code(2);
    let prelude_origin_info = bevy_steamworks::prelude::SteamworksInputActionOriginInfo {
        origin: prelude_origin,
        glyph_path: "glyph-b.svg".to_owned(),
        name: "B Button".to_owned(),
    };
    assert_eq!(prelude_origin_info.origin, prelude_origin);
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

    let lobby = steamworks::LobbyId::from_raw(11);
    let user = steamworks::SteamId::from_raw(22);
    let address = "127.0.0.1:27015".parse().expect("valid socket address");
    let _ = SteamworksMatchmakingCommand::get_lobby_data_count(lobby);
    let _ = SteamworksMatchmakingCommand::get_lobby_data_by_index(lobby, 0);
    let _ = SteamworksMatchmakingCommand::get_all_lobby_data(lobby);
    let _ = SteamworksMatchmakingCommand::set_lobby_member_data(lobby, "loadout", "rail");
    let _ = SteamworksMatchmakingCommand::get_lobby_member_data(lobby, user, "rank");
    let _ = SteamworksMatchmakingCommand::get_lobby_member_limit(lobby);
    let _ = SteamworksMatchmakingCommand::get_lobby_owner(lobby);
    let _ = SteamworksMatchmakingCommand::get_lobby_member_count(lobby);
    let _ = SteamworksMatchmakingCommand::list_lobby_members(lobby);
    let _ = SteamworksMatchmakingCommand::set_lobby_joinable(lobby, true);
    let _ = SteamworksMatchmakingCommand::get_lobby_chat_entry(lobby, 1, 128);
    let _ = SteamworksMatchmakingCommand::set_lobby_game_server(lobby, address, Some(user));
    let _ = SteamworksMatchmakingCommand::get_lobby_game_server(lobby);

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

    let address = "127.0.0.1:27015".parse().expect("valid socket address");
    let _ = PreludeMatchmakingCommand::get_lobby_data_count(lobby);
    let _ = PreludeMatchmakingCommand::get_lobby_data_by_index(lobby, 0);
    let _ = PreludeMatchmakingCommand::get_all_lobby_data(lobby);
    let _ = PreludeMatchmakingCommand::set_lobby_member_data(lobby, "loadout", "rail");
    let _ = PreludeMatchmakingCommand::get_lobby_member_data(lobby, user, "rank");
    let _ = PreludeMatchmakingCommand::get_lobby_member_limit(lobby);
    let _ = PreludeMatchmakingCommand::get_lobby_owner(lobby);
    let _ = PreludeMatchmakingCommand::get_lobby_member_count(lobby);
    let _ = PreludeMatchmakingCommand::list_lobby_members(lobby);
    let _ = PreludeMatchmakingCommand::set_lobby_joinable(lobby, true);
    let _ = PreludeMatchmakingCommand::get_lobby_chat_entry(lobby, 1, 128);
    let _ = PreludeMatchmakingCommand::set_lobby_game_server(lobby, address, Some(user));
    let _ = PreludeMatchmakingCommand::get_lobby_game_server(lobby);
}

#[test]
fn networking_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksNetworkingPlugin,
        _command: SteamworksNetworkingCommand,
        _operation: SteamworksNetworkingOperation,
        _result: SteamworksNetworkingResult,
        _error: SteamworksNetworkingError,
        _state: SteamworksNetworkingState,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeNetworkingPlugin,
        _command: PreludeNetworkingCommand,
        _operation: PreludeNetworkingOperation,
        _result: PreludeNetworkingResult,
        _error: PreludeNetworkingError,
        _state: PreludeNetworkingState,
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
    let user = steamworks::SteamId::from_raw(7);
    let state = SteamworksNetworkingState::default();
    assert!(state.session_states().is_empty());
    assert_eq!(state.session_state(user), None);
    assert!(state.packet_availabilities().is_empty());
    assert_eq!(state.packet_availability(0), None);
    assert!(state.received_packets().is_empty());
    assert_eq!(state.last_packet_from(user), None);
    assert_eq!(state.last_packet_on_channel(0), None);
    assert!(state.session_requests().is_empty());
    assert!(!state.has_session_request(user));
    assert!(state.session_connect_failures().is_empty());
    assert_eq!(state.session_connect_failure(user), None);

    accepts_root_exports(
        SteamworksNetworkingPlugin::new(),
        command,
        operation,
        result,
        error,
        state,
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
    let user = steamworks::SteamId::from_raw(7);
    let state = PreludeNetworkingState::default();
    assert!(state.session_states().is_empty());
    assert_eq!(state.session_state(user), None);
    assert!(state.packet_availabilities().is_empty());
    assert_eq!(state.packet_availability(0), None);
    assert!(state.received_packets().is_empty());
    assert_eq!(state.last_packet_from(user), None);
    assert_eq!(state.last_packet_on_channel(0), None);
    assert!(state.session_requests().is_empty());
    assert!(!state.has_session_request(user));
    assert!(state.session_connect_failures().is_empty());
    assert_eq!(state.session_connect_failure(user), None);

    accepts_prelude_exports(
        PreludeNetworkingPlugin::new(),
        command,
        operation,
        result,
        error,
        state,
    );
}

#[test]
fn networking_messages_api_is_exported_from_root_and_prelude() {
    fn accepts_root_exports(
        _plugin: SteamworksNetworkingMessagesPlugin,
        _peer: SteamworksNetworkingPeer,
        _command: SteamworksNetworkingMessagesCommand,
        _operation: SteamworksNetworkingMessagesOperation,
        _result: SteamworksNetworkingMessagesResult,
        _error: SteamworksNetworkingMessagesError,
        _state: SteamworksNetworkingMessagesState,
    ) {
    }

    fn accepts_prelude_exports(
        _plugin: PreludeNetworkingMessagesPlugin,
        _peer: PreludeNetworkingPeer,
        _command: PreludeNetworkingMessagesCommand,
        _operation: PreludeNetworkingMessagesOperation,
        _result: PreludeNetworkingMessagesResult,
        _error: PreludeNetworkingMessagesError,
        _state: PreludeNetworkingMessagesState,
    ) {
    }

    let steam_id = steamworks::SteamId::from_raw(7);
    let peer = SteamworksNetworkingPeer::from(steam_id);
    assert_eq!(
        peer.to_identity(),
        steamworks::networking_types::NetworkingIdentity::new_steam_id(steam_id)
    );
    let command = SteamworksNetworkingMessagesCommand::send_message(
        steam_id,
        steamworks::networking_types::SendFlags::RELIABLE,
        0,
        b"ping",
    );
    let operation =
        SteamworksNetworkingMessagesOperation::AutoAcceptSessionRequestsSet { enabled: true };
    let error = SteamworksNetworkingMessagesError::ClientUnavailable;
    let result = SteamworksNetworkingMessagesResult::Err {
        command: command.clone(),
        error: error.clone(),
    };
    let plugin = SteamworksNetworkingMessagesPlugin::new().auto_accept_session_requests(false);
    assert!(!plugin.auto_accepts_session_requests());
    let state = SteamworksNetworkingMessagesState::default();
    assert_eq!(state.last_received_message(), None);
    assert_eq!(
        state.last_received_message_from_peer(&peer.to_identity()),
        None
    );
    assert!(state.session_requests().is_empty());
    assert_eq!(state.session_request(&peer.to_identity()), None);
    assert!(state.session_failures().is_empty());
    assert_eq!(state.session_failure(&peer.to_identity()), None);

    accepts_root_exports(plugin, peer, command, operation, result, error, state);

    let steam_id = steamworks::SteamId::from_raw(7);
    let peer = PreludeNetworkingPeer::from(steam_id);
    assert_eq!(
        peer.to_identity(),
        steamworks::networking_types::NetworkingIdentity::new_steam_id(steam_id)
    );
    let command = PreludeNetworkingMessagesCommand::send_message(
        steam_id,
        steamworks::networking_types::SendFlags::RELIABLE,
        0,
        b"ping",
    );
    let operation =
        PreludeNetworkingMessagesOperation::AutoAcceptSessionRequestsSet { enabled: true };
    let error = PreludeNetworkingMessagesError::ClientUnavailable;
    let result = PreludeNetworkingMessagesResult::Err {
        command: command.clone(),
        error: error.clone(),
    };
    let plugin = PreludeNetworkingMessagesPlugin::new().auto_accept_session_requests(false);
    assert!(!plugin.auto_accepts_session_requests());
    let state = PreludeNetworkingMessagesState::default();
    assert_eq!(state.last_received_message(), None);
    assert_eq!(
        state.last_received_message_from_peer(&peer.to_identity()),
        None
    );
    assert!(state.session_requests().is_empty());
    assert_eq!(state.session_request(&peer.to_identity()), None);
    assert!(state.session_failures().is_empty());
    assert_eq!(state.session_failure(&peer.to_identity()), None);

    accepts_prelude_exports(plugin, peer, command, operation, result, error, state);
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

    fn accepts_root_message_exports(
        _config: SteamworksNetworkingSocketsConfigEntry,
        _name: SteamworksNetworkingSocketsConnectionName,
        _user_data: SteamworksNetworkingSocketsConnectionUserData,
        _outbound: SteamworksNetworkingSocketsOutboundMessage,
        _send_result: SteamworksNetworkingSocketsMessageSendResult,
        _connection_messages: SteamworksNetworkingSocketsConnectionMessages,
        _poll_group_messages: SteamworksNetworkingSocketsPollGroupMessages,
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

    fn accepts_prelude_message_exports(
        _config: PreludeNetworkingSocketsConfigEntry,
        _name: PreludeNetworkingSocketsConnectionName,
        _user_data: PreludeNetworkingSocketsConnectionUserData,
        _outbound: PreludeNetworkingSocketsOutboundMessage,
        _send_result: PreludeNetworkingSocketsMessageSendResult,
        _connection_messages: PreludeNetworkingSocketsConnectionMessages,
        _poll_group_messages: PreludeNetworkingSocketsPollGroupMessages,
    ) {
    }

    let connection = SteamworksNetworkingSocketsConnectionId::from_raw(2);
    let command =
        SteamworksNetworkingSocketsCommand::create_hosted_dedicated_server_listen_socket(27015);
    let server_poll_group_command = SteamworksNetworkingSocketsCommand::create_server_poll_group();
    let operation = SteamworksNetworkingSocketsOperation::ListenSocketCreated {
        listen_socket: bevy_steamworks::SteamworksListenSocketId::from_raw(1),
        endpoint: SteamworksNetworkingSocketsListenEndpoint::HostedDedicatedServer {
            local_virtual_port: 27015,
        },
    };
    let error = SteamworksNetworkingSocketsError::ServerUnavailable;
    let result = SteamworksNetworkingSocketsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };
    let config = SteamworksNetworkingSocketsConfigEntry::int32(
        steamworks::networking_types::NetworkingConfigValue::IPAllowWithoutAuth,
        1,
    );
    let connection_name = SteamworksNetworkingSocketsConnectionName {
        connection,
        name: "player-1".to_owned(),
    };
    let connection_user_data = SteamworksNetworkingSocketsConnectionUserData {
        connection,
        user_data: 7,
    };
    let outbound = SteamworksNetworkingSocketsOutboundMessage::new(
        connection,
        steamworks::networking_types::SendFlags::RELIABLE,
        [1, 2, 3],
    )
    .with_channel(1)
    .with_user_data(7);
    let send_result = SteamworksNetworkingSocketsMessageSendResult {
        connection,
        send_flags: steamworks::networking_types::SendFlags::RELIABLE,
        channel: 1,
        bytes: 3,
        user_data: 7,
        result: Ok(42),
    };
    let connection_messages = SteamworksNetworkingSocketsConnectionMessages {
        connection,
        messages: Vec::new(),
    };
    let poll_group_messages = SteamworksNetworkingSocketsPollGroupMessages {
        poll_group: bevy_steamworks::SteamworksNetworkingSocketsPollGroupId::from_raw(1),
        messages: Vec::new(),
    };

    accepts_root_exports(
        SteamworksNetworkingSocketsPlugin::new(),
        command,
        operation,
        result,
        error,
    );
    accepts_root_message_exports(
        config,
        connection_name,
        connection_user_data,
        outbound,
        send_result,
        connection_messages,
        poll_group_messages,
    );
    accepts_root_exports(
        SteamworksNetworkingSocketsPlugin::new(),
        SteamworksNetworkingSocketsCommand::get_connection_user_data(connection),
        SteamworksNetworkingSocketsOperation::ConnectionUserDataRead {
            connection,
            user_data: 7,
        },
        SteamworksNetworkingSocketsResult::Ok(
            SteamworksNetworkingSocketsOperation::ConnectionUserDataRead {
                connection,
                user_data: 7,
            },
        ),
        SteamworksNetworkingSocketsError::ConnectionNotFound { id: connection },
    );
    accepts_root_exports(
        SteamworksNetworkingSocketsPlugin::new(),
        SteamworksNetworkingSocketsCommand::poll_all_listen_socket_events(
            16,
            SteamworksConnectionRequestPolicy::Accept,
        ),
        SteamworksNetworkingSocketsOperation::AllListenSocketEventsPolled {
            listen_sockets: Vec::new(),
        },
        SteamworksNetworkingSocketsResult::Ok(
            SteamworksNetworkingSocketsOperation::AllConnectionEventsPolled {
                connections: Vec::new(),
            },
        ),
        SteamworksNetworkingSocketsError::InvalidEventLimit,
    );
    accepts_root_exports(
        SteamworksNetworkingSocketsPlugin::new(),
        SteamworksNetworkingSocketsCommand::poll_all_connection_events(16),
        SteamworksNetworkingSocketsOperation::AllConnectionEventsPolled {
            connections: Vec::new(),
        },
        SteamworksNetworkingSocketsResult::Ok(
            SteamworksNetworkingSocketsOperation::AllListenSocketEventsPolled {
                listen_sockets: Vec::new(),
            },
        ),
        SteamworksNetworkingSocketsError::InvalidEventLimit,
    );
    accepts_root_exports(
        SteamworksNetworkingSocketsPlugin::new(),
        SteamworksNetworkingSocketsCommand::receive_all_messages(16),
        SteamworksNetworkingSocketsOperation::AllMessagesReceived {
            connections: Vec::new(),
        },
        SteamworksNetworkingSocketsResult::Ok(
            SteamworksNetworkingSocketsOperation::AllPollGroupMessagesReceived {
                poll_groups: Vec::new(),
            },
        ),
        SteamworksNetworkingSocketsError::InvalidBatchSize,
    );
    accepts_root_exports(
        SteamworksNetworkingSocketsPlugin::new(),
        SteamworksNetworkingSocketsCommand::receive_all_poll_group_messages(16),
        SteamworksNetworkingSocketsOperation::AllPollGroupMessagesReceived {
            poll_groups: Vec::new(),
        },
        SteamworksNetworkingSocketsResult::Ok(
            SteamworksNetworkingSocketsOperation::AllMessagesReceived {
                connections: Vec::new(),
            },
        ),
        SteamworksNetworkingSocketsError::InvalidBatchSize,
    );
    accepts_root_exports(
        SteamworksNetworkingSocketsPlugin::new(),
        SteamworksNetworkingSocketsCommand::flush_all_messages(),
        SteamworksNetworkingSocketsOperation::AllMessagesFlushed {
            connections: Vec::new(),
        },
        SteamworksNetworkingSocketsResult::Ok(
            SteamworksNetworkingSocketsOperation::AllMessagesFlushed {
                connections: Vec::new(),
            },
        ),
        SteamworksNetworkingSocketsError::InvalidHandle {
            operation: "net_connection.flush_messages",
        },
    );
    accepts_root_exports(
        SteamworksNetworkingSocketsPlugin::new(),
        SteamworksNetworkingSocketsCommand::close_all_connections(),
        SteamworksNetworkingSocketsOperation::AllConnectionsClosed {
            connections: Vec::new(),
        },
        SteamworksNetworkingSocketsResult::Ok(
            SteamworksNetworkingSocketsOperation::AllListenSocketsClosed {
                listen_sockets: Vec::new(),
            },
        ),
        SteamworksNetworkingSocketsError::InvalidString { field: "debug" },
    );
    accepts_root_exports(
        SteamworksNetworkingSocketsPlugin::new(),
        SteamworksNetworkingSocketsCommand::close_all_listen_sockets(),
        SteamworksNetworkingSocketsOperation::AllListenSocketsClosed {
            listen_sockets: Vec::new(),
        },
        SteamworksNetworkingSocketsResult::Ok(
            SteamworksNetworkingSocketsOperation::AllPollGroupsClosed {
                poll_groups: Vec::new(),
            },
        ),
        SteamworksNetworkingSocketsError::ListenSocketNotFound {
            id: bevy_steamworks::SteamworksListenSocketId::from_raw(1),
        },
    );
    accepts_root_exports(
        SteamworksNetworkingSocketsPlugin::new(),
        SteamworksNetworkingSocketsCommand::close_all_poll_groups(),
        SteamworksNetworkingSocketsOperation::AllPollGroupsClosed {
            poll_groups: Vec::new(),
        },
        SteamworksNetworkingSocketsResult::Ok(
            SteamworksNetworkingSocketsOperation::AllConnectionsClosed {
                connections: Vec::new(),
            },
        ),
        SteamworksNetworkingSocketsError::PollGroupNotFound {
            id: bevy_steamworks::SteamworksNetworkingSocketsPollGroupId::from_raw(1),
        },
    );
    accepts_root_exports(
        SteamworksNetworkingSocketsPlugin::new(),
        server_poll_group_command,
        SteamworksNetworkingSocketsOperation::PollGroupCreated {
            poll_group: bevy_steamworks::SteamworksNetworkingSocketsPollGroupId::from_raw(1),
        },
        SteamworksNetworkingSocketsResult::Err {
            command: SteamworksNetworkingSocketsCommand::create_server_poll_group(),
            error: SteamworksNetworkingSocketsError::ServerUnavailable,
        },
        SteamworksNetworkingSocketsError::HandleOwnerMismatch {
            connection,
            poll_group: bevy_steamworks::SteamworksNetworkingSocketsPollGroupId::from_raw(1),
        },
    );
    assert_eq!(
        bevy_steamworks::STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIG_ENTRIES,
        bevy_steamworks::prelude::STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIG_ENTRIES,
    );
    assert_eq!(
        bevy_steamworks::STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES,
        bevy_steamworks::prelude::STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES,
    );
    assert_eq!(
        bevy_steamworks::STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND,
        bevy_steamworks::prelude::STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND,
    );
    assert_eq!(
        bevy_steamworks::STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND,
        bevy_steamworks::prelude::STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND,
    );
    assert_eq!(
        bevy_steamworks::STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES,
        bevy_steamworks::prelude::STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES,
    );
    assert_eq!(
        bevy_steamworks::STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES,
        bevy_steamworks::prelude::STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES,
    );

    let connection = PreludeNetworkingSocketsConnectionId::from_raw(2);
    let command =
        PreludeNetworkingSocketsCommand::create_hosted_dedicated_server_listen_socket(27015);
    let server_poll_group_command = PreludeNetworkingSocketsCommand::create_server_poll_group();
    let operation = PreludeNetworkingSocketsOperation::ListenSocketCreated {
        listen_socket: bevy_steamworks::prelude::SteamworksListenSocketId::from_raw(1),
        endpoint: PreludeNetworkingSocketsListenEndpoint::HostedDedicatedServer {
            local_virtual_port: 27015,
        },
    };
    let error = PreludeNetworkingSocketsError::ServerUnavailable;
    let result = PreludeNetworkingSocketsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };
    let config = PreludeNetworkingSocketsConfigEntry::int32(
        steamworks::networking_types::NetworkingConfigValue::IPAllowWithoutAuth,
        1,
    );
    let connection_name = PreludeNetworkingSocketsConnectionName {
        connection,
        name: "player-1".to_owned(),
    };
    let connection_user_data = PreludeNetworkingSocketsConnectionUserData {
        connection,
        user_data: 7,
    };
    let outbound = PreludeNetworkingSocketsOutboundMessage::new(
        connection,
        steamworks::networking_types::SendFlags::RELIABLE,
        [1, 2, 3],
    )
    .with_channel(1)
    .with_user_data(7);
    let send_result = PreludeNetworkingSocketsMessageSendResult {
        connection,
        send_flags: steamworks::networking_types::SendFlags::RELIABLE,
        channel: 1,
        bytes: 3,
        user_data: 7,
        result: Ok(42),
    };
    let connection_messages = PreludeNetworkingSocketsConnectionMessages {
        connection,
        messages: Vec::new(),
    };
    let poll_group_messages = PreludeNetworkingSocketsPollGroupMessages {
        poll_group: bevy_steamworks::prelude::SteamworksNetworkingSocketsPollGroupId::from_raw(1),
        messages: Vec::new(),
    };

    accepts_prelude_exports(
        PreludeNetworkingSocketsPlugin::new(),
        command,
        operation,
        result,
        error,
    );
    accepts_prelude_message_exports(
        config,
        connection_name,
        connection_user_data,
        outbound,
        send_result,
        connection_messages,
        poll_group_messages,
    );
    accepts_prelude_exports(
        PreludeNetworkingSocketsPlugin::new(),
        PreludeNetworkingSocketsCommand::get_connection_user_data(connection),
        PreludeNetworkingSocketsOperation::ConnectionUserDataRead {
            connection,
            user_data: 7,
        },
        PreludeNetworkingSocketsResult::Ok(
            PreludeNetworkingSocketsOperation::ConnectionUserDataRead {
                connection,
                user_data: 7,
            },
        ),
        PreludeNetworkingSocketsError::ConnectionNotFound { id: connection },
    );
    accepts_prelude_exports(
        PreludeNetworkingSocketsPlugin::new(),
        PreludeNetworkingSocketsCommand::poll_all_listen_socket_events(
            16,
            PreludeConnectionRequestPolicy::Accept,
        ),
        PreludeNetworkingSocketsOperation::AllListenSocketEventsPolled {
            listen_sockets: Vec::new(),
        },
        PreludeNetworkingSocketsResult::Ok(
            PreludeNetworkingSocketsOperation::AllConnectionEventsPolled {
                connections: Vec::new(),
            },
        ),
        PreludeNetworkingSocketsError::InvalidEventLimit,
    );
    accepts_prelude_exports(
        PreludeNetworkingSocketsPlugin::new(),
        PreludeNetworkingSocketsCommand::poll_all_connection_events(16),
        PreludeNetworkingSocketsOperation::AllConnectionEventsPolled {
            connections: Vec::new(),
        },
        PreludeNetworkingSocketsResult::Ok(
            PreludeNetworkingSocketsOperation::AllListenSocketEventsPolled {
                listen_sockets: Vec::new(),
            },
        ),
        PreludeNetworkingSocketsError::InvalidEventLimit,
    );
    accepts_prelude_exports(
        PreludeNetworkingSocketsPlugin::new(),
        PreludeNetworkingSocketsCommand::receive_all_messages(16),
        PreludeNetworkingSocketsOperation::AllMessagesReceived {
            connections: Vec::new(),
        },
        PreludeNetworkingSocketsResult::Ok(
            PreludeNetworkingSocketsOperation::AllPollGroupMessagesReceived {
                poll_groups: Vec::new(),
            },
        ),
        PreludeNetworkingSocketsError::InvalidBatchSize,
    );
    accepts_prelude_exports(
        PreludeNetworkingSocketsPlugin::new(),
        PreludeNetworkingSocketsCommand::receive_all_poll_group_messages(16),
        PreludeNetworkingSocketsOperation::AllPollGroupMessagesReceived {
            poll_groups: Vec::new(),
        },
        PreludeNetworkingSocketsResult::Ok(
            PreludeNetworkingSocketsOperation::AllMessagesReceived {
                connections: Vec::new(),
            },
        ),
        PreludeNetworkingSocketsError::InvalidBatchSize,
    );
    accepts_prelude_exports(
        PreludeNetworkingSocketsPlugin::new(),
        PreludeNetworkingSocketsCommand::flush_all_messages(),
        PreludeNetworkingSocketsOperation::AllMessagesFlushed {
            connections: Vec::new(),
        },
        PreludeNetworkingSocketsResult::Ok(PreludeNetworkingSocketsOperation::AllMessagesFlushed {
            connections: Vec::new(),
        }),
        PreludeNetworkingSocketsError::InvalidHandle {
            operation: "net_connection.flush_messages",
        },
    );
    accepts_prelude_exports(
        PreludeNetworkingSocketsPlugin::new(),
        PreludeNetworkingSocketsCommand::close_all_connections(),
        PreludeNetworkingSocketsOperation::AllConnectionsClosed {
            connections: Vec::new(),
        },
        PreludeNetworkingSocketsResult::Ok(
            PreludeNetworkingSocketsOperation::AllListenSocketsClosed {
                listen_sockets: Vec::new(),
            },
        ),
        PreludeNetworkingSocketsError::InvalidString { field: "debug" },
    );
    accepts_prelude_exports(
        PreludeNetworkingSocketsPlugin::new(),
        PreludeNetworkingSocketsCommand::close_all_listen_sockets(),
        PreludeNetworkingSocketsOperation::AllListenSocketsClosed {
            listen_sockets: Vec::new(),
        },
        PreludeNetworkingSocketsResult::Ok(
            PreludeNetworkingSocketsOperation::AllPollGroupsClosed {
                poll_groups: Vec::new(),
            },
        ),
        PreludeNetworkingSocketsError::ListenSocketNotFound {
            id: bevy_steamworks::prelude::SteamworksListenSocketId::from_raw(1),
        },
    );
    accepts_prelude_exports(
        PreludeNetworkingSocketsPlugin::new(),
        PreludeNetworkingSocketsCommand::close_all_poll_groups(),
        PreludeNetworkingSocketsOperation::AllPollGroupsClosed {
            poll_groups: Vec::new(),
        },
        PreludeNetworkingSocketsResult::Ok(
            PreludeNetworkingSocketsOperation::AllConnectionsClosed {
                connections: Vec::new(),
            },
        ),
        PreludeNetworkingSocketsError::PollGroupNotFound {
            id: bevy_steamworks::prelude::SteamworksNetworkingSocketsPollGroupId::from_raw(1),
        },
    );
    accepts_prelude_exports(
        PreludeNetworkingSocketsPlugin::new(),
        server_poll_group_command,
        PreludeNetworkingSocketsOperation::PollGroupCreated {
            poll_group: bevy_steamworks::prelude::SteamworksNetworkingSocketsPollGroupId::from_raw(
                1,
            ),
        },
        PreludeNetworkingSocketsResult::Err {
            command: PreludeNetworkingSocketsCommand::create_server_poll_group(),
            error: PreludeNetworkingSocketsError::ServerUnavailable,
        },
        PreludeNetworkingSocketsError::HandleOwnerMismatch {
            connection,
            poll_group: bevy_steamworks::prelude::SteamworksNetworkingSocketsPollGroupId::from_raw(
                1,
            ),
        },
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

    let command = SteamworksNetworkingUtilsCommand::get_relay_debug_message();
    let operation = SteamworksNetworkingUtilsOperation::RelayDebugMessageRead {
        message: String::new(),
    };
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

    let command = PreludeNetworkingUtilsCommand::get_any_relay_status();
    let operation = PreludeNetworkingUtilsOperation::AnyRelayStatusRead {
        availability: Err(steamworks::networking_types::NetworkingAvailabilityError::Unknown),
    };
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

    let command = SteamworksRemoteStorageCommand::get_cloud_info();
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
    accepts_root_exports(
        SteamworksRemoteStoragePlugin::new(),
        SteamworksRemoteStorageCommand::list_files(),
        SteamworksRemoteStorageOperation::FilesListed { files: Vec::new() },
        SteamworksRemoteStorageResult::Ok(
            SteamworksRemoteStorageOperation::CloudEnabledForAccountRead { enabled: true },
        ),
        SteamworksRemoteStorageError::ClientUnavailable,
    );
    accepts_root_exports(
        SteamworksRemoteStoragePlugin::new(),
        SteamworksRemoteStorageCommand::get_file_exists("save.dat"),
        SteamworksRemoteStorageOperation::FileExistsRead {
            name: "save.dat".to_owned(),
            exists: true,
        },
        SteamworksRemoteStorageResult::Ok(SteamworksRemoteStorageOperation::FilePersistedRead {
            name: "save.dat".to_owned(),
            persisted: true,
        }),
        SteamworksRemoteStorageError::ClientUnavailable,
    );
    accepts_root_exports(
        SteamworksRemoteStoragePlugin::new(),
        SteamworksRemoteStorageCommand::get_file_timestamp("save.dat"),
        SteamworksRemoteStorageOperation::FileTimestampRead {
            name: "save.dat".to_owned(),
            timestamp: 7,
        },
        SteamworksRemoteStorageResult::Ok(SteamworksRemoteStorageOperation::FileTimestampRead {
            name: "save.dat".to_owned(),
            timestamp: 7,
        }),
        SteamworksRemoteStorageError::ClientUnavailable,
    );

    let write = SteamworksRemoteStorageFileWrite::new("save.dat", b"payload".to_vec());
    accepts_root_exports(
        SteamworksRemoteStoragePlugin::new(),
        SteamworksRemoteStorageCommand::write_file("save.dat", b"payload".to_vec()),
        SteamworksRemoteStorageOperation::FileWriteRequested {
            request_id: 0,
            name: "save.dat".to_owned(),
            bytes: 7,
        },
        SteamworksRemoteStorageResult::Ok(SteamworksRemoteStorageOperation::FileRead {
            contents: SteamworksRemoteStorageFileContents {
                request_id: 1,
                name: "save.dat".to_owned(),
                data: b"payload".to_vec(),
            },
        }),
        SteamworksRemoteStorageError::FileIo {
            operation: "remote_storage.file.write",
            request_id: 0,
            name: "save.dat".to_owned(),
            message: "failed".to_owned(),
        },
    );
    let _unavailable = SteamworksRemoteStorageError::FileUnavailableForRequest {
        request_id: 3,
        name: "save.dat".to_owned(),
    };
    assert_eq!(write.bytes(), 7);
    let _written = SteamworksRemoteStorageFileWritten {
        request_id: 2,
        name: "save.dat".to_owned(),
        bytes: 7,
    };

    let command = PreludeRemoteStorageCommand::get_cloud_info();
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
    accepts_prelude_exports(
        PreludeRemoteStoragePlugin::new(),
        PreludeRemoteStorageCommand::is_cloud_enabled_for_app(),
        PreludeRemoteStorageOperation::CloudEnabledForAppRead { enabled: true },
        PreludeRemoteStorageResult::Ok(PreludeRemoteStorageOperation::CloudEnabledForAccountRead {
            enabled: true,
        }),
        PreludeRemoteStorageError::ClientUnavailable,
    );
    accepts_prelude_exports(
        PreludeRemoteStoragePlugin::new(),
        PreludeRemoteStorageCommand::is_file_persisted("save.dat"),
        PreludeRemoteStorageOperation::FilePersistedRead {
            name: "save.dat".to_owned(),
            persisted: true,
        },
        PreludeRemoteStorageResult::Ok(PreludeRemoteStorageOperation::FileExistsRead {
            name: "save.dat".to_owned(),
            exists: true,
        }),
        PreludeRemoteStorageError::ClientUnavailable,
    );

    let write = PreludeRemoteStorageFileWrite::new("save.dat", b"payload".to_vec());
    accepts_prelude_exports(
        PreludeRemoteStoragePlugin::new(),
        PreludeRemoteStorageCommand::read_file("save.dat"),
        PreludeRemoteStorageOperation::FileReadRequested {
            request_id: 0,
            name: "save.dat".to_owned(),
        },
        PreludeRemoteStorageResult::Ok(PreludeRemoteStorageOperation::FileWritten {
            written: PreludeRemoteStorageFileWritten {
                request_id: 1,
                name: "save.dat".to_owned(),
                bytes: 7,
            },
        }),
        PreludeRemoteStorageError::FileIo {
            operation: "remote_storage.file.read",
            request_id: 0,
            name: "save.dat".to_owned(),
            message: "failed".to_owned(),
        },
    );
    assert_eq!(write.bytes(), 7);
    let _contents = PreludeRemoteStorageFileContents {
        request_id: 2,
        name: "save.dat".to_owned(),
        data: b"payload".to_vec(),
    };
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

    let command = SteamworksRemotePlayCommand::list_sessions();
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

    let command = PreludeRemotePlayCommand::list_sessions();
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
    accepts_root_exports(
        SteamworksScreenshotsPlugin::new(),
        SteamworksScreenshotsCommand::trigger_screenshot(),
        SteamworksScreenshotsOperation::ScreenshotTriggered,
        SteamworksScreenshotsResult::Ok(SteamworksScreenshotsOperation::ScreenshotRequested {
            count: 1,
        }),
        SteamworksScreenshotsError::InvalidDimensions {
            width: 0,
            height: 1,
        },
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
    accepts_prelude_exports(
        PreludeScreenshotsPlugin::new(),
        PreludeScreenshotsCommand::is_screenshots_hooked(),
        PreludeScreenshotsOperation::ScreenshotsHookedRead { hooked: true },
        PreludeScreenshotsResult::Ok(PreludeScreenshotsOperation::ScreenshotTriggered),
        PreludeScreenshotsError::ClientUnavailable,
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

    let command = SteamworksUserCommand::get_level();
    let operation = SteamworksUserOperation::LevelRead { level: 1 };
    let error = SteamworksUserError::ClientUnavailable;
    let result = SteamworksUserResult::Err {
        command: command.clone(),
        error: error.clone(),
    };
    let identity = steamworks::networking_types::NetworkingIdentity::new_ip(
        std::net::SocketAddr::from(([127, 0, 0, 1], 27015)),
    );
    let identity_command = SteamworksUserCommand::get_authentication_session_ticket_for_identity(
        SteamworksNetworkingPeer::from(identity),
    );
    assert!(matches!(
        identity_command,
        SteamworksUserCommand::GetAuthenticationSessionTicketForIdentity { .. }
    ));
    let _identity_ticket: Option<SteamworksIssuedAuthSessionTicketForIdentity> = None;
    let _identity_error = SteamworksUserError::InvalidNetworkingIdentity;

    accepts_root_exports(
        SteamworksUserPlugin::new(),
        command,
        operation,
        result,
        error,
    );

    let command = PreludeUserCommand::get_level();
    let operation = PreludeUserOperation::LevelRead { level: 1 };
    let error = PreludeUserError::ClientUnavailable;
    let result = PreludeUserResult::Err {
        command: command.clone(),
        error: error.clone(),
    };
    let identity = steamworks::networking_types::NetworkingIdentity::new_ip(
        std::net::SocketAddr::from(([127, 0, 0, 1], 27015)),
    );
    let identity_command = PreludeUserCommand::get_authentication_session_ticket_for_identity(
        PreludeNetworkingPeer::from(identity),
    );
    assert!(matches!(
        identity_command,
        PreludeUserCommand::GetAuthenticationSessionTicketForIdentity { .. }
    ));
    let _identity_ticket: Option<PreludeIssuedAuthSessionTicketForIdentity> = None;
    let _identity_error = PreludeUserError::InvalidNetworkingIdentity;

    accepts_prelude_exports(PreludeUserPlugin::new(), command, operation, result, error);
}

#[test]
fn stats_api_is_exported_from_root_and_prelude() {
    fn assert_eq_type<T: Eq>() {}

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

    let root_settings = bevy_steamworks::SteamworksStatsSettings {
        request_current_user_stats_on_startup: false,
        auto_store: false,
    };
    assert_eq_type::<bevy_steamworks::SteamworksStatsSettings>();
    let root_plugin = SteamworksStatsPlugin::with_settings(root_settings.clone());
    assert_eq!(root_plugin.settings(), &root_settings);
    assert!(!root_plugin.requests_current_user_stats_on_startup());
    assert!(!root_plugin.auto_store_enabled());
    let root_state = bevy_steamworks::SteamworksStatsState::default();
    let root_leaderboard: bevy_steamworks::SteamworksLeaderboardId =
        bevy_steamworks::SteamworksLeaderboardId::from_raw(1);
    assert_eq!(root_state.leaderboard_id("daily"), None);
    assert_eq!(root_state.leaderboard_info(root_leaderboard), None);
    assert_eq!(root_state.leaderboard_info_by_name("daily"), None);
    assert!(root_state.leaderboards().is_empty());
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

    let prelude_settings = bevy_steamworks::prelude::SteamworksStatsSettings {
        request_current_user_stats_on_startup: false,
        auto_store: false,
    };
    assert_eq_type::<bevy_steamworks::prelude::SteamworksStatsSettings>();
    let prelude_plugin = PreludeStatsPlugin::with_settings(prelude_settings.clone());
    assert_eq!(prelude_plugin.settings(), &prelude_settings);
    assert!(!prelude_plugin.requests_current_user_stats_on_startup());
    assert!(!prelude_plugin.auto_store_enabled());
    let prelude_state = bevy_steamworks::prelude::SteamworksStatsState::default();
    let prelude_leaderboard: bevy_steamworks::prelude::SteamworksLeaderboardId =
        bevy_steamworks::prelude::SteamworksLeaderboardId::from_raw(1);
    assert_eq!(prelude_state.leaderboard_id("daily"), None);
    assert_eq!(prelude_state.leaderboard_info(prelude_leaderboard), None);
    assert_eq!(prelude_state.leaderboard_info_by_name("daily"), None);
    assert!(prelude_state.leaderboards().is_empty());
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

    let command = SteamworksStatsCommand::get_achievement_count();
    let operation = SteamworksStatsOperation::AchievementCountRead { count: 0 };
    let error = SteamworksStatsError::ClientUnavailable;
    let result = SteamworksStatsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_root_exports(root_plugin, command, operation, result, error);

    let command = PreludeStatsCommand::get_achievement_count();
    let operation = PreludeStatsOperation::AchievementCountRead { count: 0 };
    let error = PreludeStatsError::ClientUnavailable;
    let result = PreludeStatsResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(prelude_plugin, command, operation, result, error);
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

    fn root_item_detail(item: steamworks::PublishedFileId) -> SteamworksUgcItemDetails {
        SteamworksUgcItemDetails {
            published_file_id: item,
            creator_app_id: Some(steamworks::AppId(480)),
            consumer_app_id: Some(steamworks::AppId(480)),
            title: "Title".to_owned(),
            description: "Description".to_owned(),
            owner: steamworks::SteamId::from_raw(1),
            time_created: 1,
            time_updated: 2,
            time_added_to_user_list: 3,
            visibility: steamworks::PublishedFileVisibility::Public,
            banned: false,
            accepted_for_use: true,
            tags: vec!["tag".to_owned()],
            tags_truncated: false,
            file_name: "file.dat".to_owned(),
            file_type: steamworks::FileType::Community,
            file_size: 1024,
            url: "https://example.invalid/item".to_owned(),
            num_upvotes: 10,
            num_downvotes: 1,
            score: 0.9,
            num_children: 0,
            preview_url: Some("https://example.invalid/preview.png".to_owned()),
            content_descriptors: vec![SteamworksUgcContentDescriptor::AnyMatureContent],
            statistics: Vec::new(),
            metadata: Some(b"metadata".to_vec()),
            children: Some(Vec::new()),
            key_value_tags: vec![("mode".to_owned(), "arena".to_owned())],
        }
    }

    fn prelude_item_detail(item: steamworks::PublishedFileId) -> PreludeUgcItemDetails {
        PreludeUgcItemDetails {
            published_file_id: item,
            creator_app_id: Some(steamworks::AppId(480)),
            consumer_app_id: Some(steamworks::AppId(480)),
            title: "Title".to_owned(),
            description: "Description".to_owned(),
            owner: steamworks::SteamId::from_raw(1),
            time_created: 1,
            time_updated: 2,
            time_added_to_user_list: 3,
            visibility: steamworks::PublishedFileVisibility::Public,
            banned: false,
            accepted_for_use: true,
            tags: vec!["tag".to_owned()],
            tags_truncated: false,
            file_name: "file.dat".to_owned(),
            file_type: steamworks::FileType::Community,
            file_size: 1024,
            url: "https://example.invalid/item".to_owned(),
            num_upvotes: 10,
            num_downvotes: 1,
            score: 0.9,
            num_children: 0,
            preview_url: Some("https://example.invalid/preview.png".to_owned()),
            content_descriptors: vec![PreludeUgcContentDescriptor::AnyMatureContent],
            statistics: Vec::new(),
            metadata: Some(b"metadata".to_vec()),
            children: Some(Vec::new()),
            key_value_tags: vec![("mode".to_owned(), "arena".to_owned())],
        }
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

    let item = steamworks::PublishedFileId(42);
    let query = SteamworksUgcQuery::item(42_u64)
        .with_metadata(true)
        .with_key_value_tags(true)
        .with_statistic(steamworks::UGCStatisticType::Subscriptions);
    assert_eq!(
        SteamworksUgcQuery::items([42_u64]),
        SteamworksUgcQuery::Items {
            items: vec![item],
            options: SteamworksUgcQueryOptions::new(),
        }
    );
    let workshop_depot = SteamworksUgcWorkshopDepotId::from(480_u32);
    let _game_server_workshop_init = SteamworksUgcGameServerWorkshopInit {
        workshop_depot,
        folder: "workshop_server".to_owned(),
    };
    let _query_options = SteamworksUgcQueryOptions::new().with_additional_previews(true);
    let _item_detail = root_item_detail(item);
    let root_state = SteamworksUgcState::default();
    assert!(root_state.item_details().is_empty());
    assert_eq!(root_state.item_detail(item), None);
    assert_eq!(root_state.item_state(item), None);
    assert_eq!(root_state.item_download_info(item), None);
    assert_eq!(root_state.item_install_info(item), None);
    assert!(root_state.download_item_results().is_empty());
    assert_eq!(root_state.download_item_result(item), None);
    assert_eq!(root_state.download_item_failed(item), None);
    let _download_result = SteamworksUgcDownloadItemResult {
        app_id: steamworks::AppId(480),
        item,
        error: None,
    };
    let _item_state = SteamworksUgcItemStateInfo {
        item,
        state: steamworks::ItemState::SUBSCRIBED,
    };
    let _item_download_info = SteamworksUgcItemDownloadInfoResult {
        item,
        info: Some(SteamworksUgcItemDownloadInfo {
            downloaded_bytes: 1,
            total_bytes: 2,
        }),
    };
    let _item_install_info = SteamworksUgcItemInstallInfoResult {
        item,
        info: Some(SteamworksUgcItemInstallInfo {
            folder: "workshop/item".to_owned(),
            size_on_disk: 3,
            timestamp: 4,
        }),
    };
    accepts_root_exports(
        SteamworksUgcPlugin::new(),
        SteamworksUgcCommand::init_workshop_for_game_server(workshop_depot, "workshop_server"),
        SteamworksUgcOperation::GameServerWorkshopInitialized {
            workshop_depot,
            folder: "workshop_server".to_owned(),
        },
        SteamworksUgcResult::Err {
            command: SteamworksUgcCommand::init_workshop_for_game_server(
                workshop_depot,
                "workshop_server",
            ),
            error: SteamworksUgcError::ServerUnavailable,
        },
        SteamworksUgcError::InvalidWorkshopDepot,
    );
    accepts_root_exports(
        SteamworksUgcPlugin::new(),
        SteamworksUgcCommand::query_total(query.clone()),
        SteamworksUgcOperation::QueryTotalRequested {
            request_id: 0,
            query: query.clone(),
        },
        SteamworksUgcResult::Ok(SteamworksUgcOperation::QueryTotalCompleted {
            request_id: 1,
            query: query.clone(),
            total: SteamworksUgcQueryTotal { total_results: 1 },
        }),
        SteamworksUgcError::ClientUnavailable,
    );
    accepts_root_exports(
        SteamworksUgcPlugin::new(),
        SteamworksUgcCommand::download_item(42_u64, true),
        SteamworksUgcOperation::DownloadItemSubmitted {
            item,
            high_priority: true,
        },
        SteamworksUgcResult::Ok(SteamworksUgcOperation::DownloadItemSubmitted {
            item,
            high_priority: true,
        }),
        SteamworksUgcError::ClientUnavailable,
    );
    accepts_root_exports(
        SteamworksUgcPlugin::new(),
        SteamworksUgcCommand::create_item(480_u32, steamworks::FileType::Community),
        SteamworksUgcOperation::ItemCreateRequested {
            request_id: 4,
            app_id: steamworks::AppId(480),
            file_type: steamworks::FileType::Community,
        },
        SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemCreateRequested {
            request_id: 4,
            app_id: steamworks::AppId(480),
            file_type: steamworks::FileType::Community,
        }),
        SteamworksUgcError::ClientUnavailable,
    );
    accepts_root_exports(
        SteamworksUgcPlugin::new(),
        SteamworksUgcCommand::query_ids(query.clone()),
        SteamworksUgcOperation::QueryIdsRequested {
            request_id: 2,
            query: query.clone(),
        },
        SteamworksUgcResult::Ok(SteamworksUgcOperation::QueryIdsCompleted {
            request_id: 3,
            query,
            ids: SteamworksUgcQueryIds { items: vec![item] },
        }),
        SteamworksUgcError::ClientUnavailable,
    );

    let command = PreludeUgcCommand::suspend_downloads(false);
    let operation = PreludeUgcOperation::DownloadsSuspended { suspend: false };
    let error = PreludeUgcError::ClientUnavailable;
    let result = PreludeUgcResult::Err {
        command: command.clone(),
        error: error.clone(),
    };

    accepts_prelude_exports(PreludeUgcPlugin::new(), command, operation, result, error);

    let prelude_query = PreludeUgcQuery::item(42_u64)
        .with_metadata(true)
        .with_key_value_tags(true)
        .with_statistic(steamworks::UGCStatisticType::Subscriptions);
    let _prelude_download = PreludeUgcCommand::download_item(42_u64, true);
    let _prelude_create = PreludeUgcCommand::create_item(480_u32, steamworks::FileType::Community);
    let _prelude_query_options = PreludeUgcQueryOptions::new().with_additional_previews(true);
    let _prelude_item_detail = prelude_item_detail(item);
    let prelude_state = PreludeUgcState::default();
    assert!(prelude_state.item_details().is_empty());
    assert_eq!(prelude_state.item_detail(item), None);
    assert_eq!(prelude_state.item_state(item), None);
    assert_eq!(prelude_state.item_download_info(item), None);
    assert_eq!(prelude_state.item_install_info(item), None);
    assert!(prelude_state.download_item_results().is_empty());
    assert_eq!(prelude_state.download_item_result(item), None);
    assert_eq!(prelude_state.download_item_failed(item), None);
    let _prelude_download_result = PreludeUgcDownloadItemResult {
        app_id: steamworks::AppId(480),
        item,
        error: None,
    };
    let _prelude_item_state = PreludeUgcItemStateInfo {
        item,
        state: steamworks::ItemState::SUBSCRIBED,
    };
    let _prelude_item_download_info = PreludeUgcItemDownloadInfoResult {
        item,
        info: Some(PreludeUgcItemDownloadInfo {
            downloaded_bytes: 1,
            total_bytes: 2,
        }),
    };
    let _prelude_item_install_info = PreludeUgcItemInstallInfoResult {
        item,
        info: Some(PreludeUgcItemInstallInfo {
            folder: "workshop/item".to_owned(),
            size_on_disk: 3,
            timestamp: 4,
        }),
    };
    let prelude_workshop_depot = PreludeUgcWorkshopDepotId::from(480_u32);
    let _prelude_game_server_workshop_init = PreludeUgcGameServerWorkshopInit {
        workshop_depot: prelude_workshop_depot,
        folder: "workshop_server".to_owned(),
    };
    accepts_prelude_exports(
        PreludeUgcPlugin::new(),
        PreludeUgcCommand::init_workshop_for_game_server(prelude_workshop_depot, "workshop_server"),
        PreludeUgcOperation::GameServerWorkshopInitialized {
            workshop_depot: prelude_workshop_depot,
            folder: "workshop_server".to_owned(),
        },
        PreludeUgcResult::Err {
            command: PreludeUgcCommand::init_workshop_for_game_server(
                prelude_workshop_depot,
                "workshop_server",
            ),
            error: PreludeUgcError::ServerUnavailable,
        },
        PreludeUgcError::InvalidWorkshopDepot,
    );
    accepts_prelude_exports(
        PreludeUgcPlugin::new(),
        PreludeUgcCommand::query_total(prelude_query.clone()),
        PreludeUgcOperation::QueryTotalRequested {
            request_id: 0,
            query: prelude_query.clone(),
        },
        PreludeUgcResult::Ok(PreludeUgcOperation::QueryTotalCompleted {
            request_id: 1,
            query: prelude_query.clone(),
            total: PreludeUgcQueryTotal { total_results: 1 },
        }),
        PreludeUgcError::ClientUnavailable,
    );
    accepts_prelude_exports(
        PreludeUgcPlugin::new(),
        PreludeUgcCommand::query_ids(prelude_query.clone()),
        PreludeUgcOperation::QueryIdsRequested {
            request_id: 2,
            query: prelude_query.clone(),
        },
        PreludeUgcResult::Ok(PreludeUgcOperation::QueryIdsCompleted {
            request_id: 3,
            query: prelude_query,
            ids: PreludeUgcQueryIds { items: vec![item] },
        }),
        PreludeUgcError::ClientUnavailable,
    );
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

    let gamepad_request = SteamworksGamepadTextInputRequest::new("Name", 32)
        .with_input_mode(SteamworksGamepadTextInputMode::Normal)
        .with_line_mode(SteamworksGamepadTextInputLineMode::SingleLine)
        .with_existing_text("Player");
    let floating_request = SteamworksFloatingGamepadTextInputRequest::new(
        SteamworksFloatingGamepadTextInputMode::SingleLine,
        1,
        2,
        300,
        40,
    );
    let _root_gamepad_dismissed = SteamworksGamepadTextInputDismissed {
        submitted_text_len: Some(4),
        submitted_text: Some("Name".to_owned()),
    };
    let _root_floating_dismissed = SteamworksFloatingGamepadTextInputDismissed;
    let _root_floating_modes = [
        SteamworksFloatingGamepadTextInputMode::SingleLine,
        SteamworksFloatingGamepadTextInputMode::MultipleLines,
        SteamworksFloatingGamepadTextInputMode::Email,
        SteamworksFloatingGamepadTextInputMode::Numeric,
    ];
    let _root_gamepad_modes = [
        SteamworksGamepadTextInputMode::Normal,
        SteamworksGamepadTextInputMode::Password,
    ];
    let _root_line_modes = [
        SteamworksGamepadTextInputLineMode::SingleLine,
        SteamworksGamepadTextInputLineMode::MultipleLines,
    ];
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
            SteamworksFloatingGamepadTextInputMode::Numeric
        ),
        steamworks::FloatingGamepadTextInputMode::Numeric
    ));

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
    accepts_root_exports(
        SteamworksUtilsPlugin::new(),
        SteamworksUtilsCommand::is_overlay_enabled(),
        SteamworksUtilsOperation::OverlayEnabledRead { enabled: true },
        SteamworksUtilsResult::Ok(SteamworksUtilsOperation::ServerRealTimeRead {
            unix_epoch_seconds: 1,
        }),
        SteamworksUtilsError::ClientUnavailable,
    );
    accepts_root_exports(
        SteamworksUtilsPlugin::new(),
        SteamworksUtilsCommand::install_warning_callback(),
        SteamworksUtilsOperation::WarningCallbackInstalled,
        SteamworksUtilsResult::Ok(SteamworksUtilsOperation::WarningCallbackInstalled),
        SteamworksUtilsError::ClientUnavailable,
    );

    accepts_root_exports(
        SteamworksUtilsPlugin::new(),
        SteamworksUtilsCommand::show_gamepad_text_input(gamepad_request.clone()),
        SteamworksUtilsOperation::GamepadTextInputShown {
            shown: SteamworksGamepadTextInputShown {
                request: gamepad_request,
                shown: true,
            },
        },
        SteamworksUtilsResult::Ok(SteamworksUtilsOperation::GamepadTextInputSubmitted {
            submitted: SteamworksGamepadTextInputSubmitted {
                text: "Name".to_owned(),
                submitted_text_len: 4,
            },
        }),
        SteamworksUtilsError::InvalidString {
            field: "description",
        },
    );
    accepts_root_exports(
        SteamworksUtilsPlugin::new(),
        SteamworksUtilsCommand::show_floating_gamepad_text_input(floating_request.clone()),
        SteamworksUtilsOperation::FloatingGamepadTextInputShown {
            shown: SteamworksFloatingGamepadTextInputShown {
                request: floating_request,
                shown: true,
            },
        },
        SteamworksUtilsResult::Ok(
            SteamworksUtilsOperation::FloatingGamepadTextInputDismissed {
                dismissed: SteamworksFloatingGamepadTextInputDismissed,
            },
        ),
        SteamworksUtilsError::InvalidFloatingTextInputBounds {
            width: 0,
            height: 1,
        },
    );

    let gamepad_request = PreludeGamepadTextInputRequest::new("Name", 32)
        .with_input_mode(PreludeGamepadTextInputMode::Normal)
        .with_line_mode(PreludeGamepadTextInputLineMode::SingleLine)
        .with_existing_text("Player");
    let floating_request = PreludeFloatingGamepadTextInputRequest::new(
        PreludeFloatingGamepadTextInputMode::SingleLine,
        1,
        2,
        300,
        40,
    );
    let _prelude_gamepad_dismissed = PreludeGamepadTextInputDismissed {
        submitted_text_len: Some(4),
        submitted_text: Some("Name".to_owned()),
    };
    let _prelude_floating_dismissed = PreludeFloatingGamepadTextInputDismissed;
    let _prelude_floating_modes = [
        PreludeFloatingGamepadTextInputMode::SingleLine,
        PreludeFloatingGamepadTextInputMode::MultipleLines,
        PreludeFloatingGamepadTextInputMode::Email,
        PreludeFloatingGamepadTextInputMode::Numeric,
    ];
    let _prelude_gamepad_modes = [
        PreludeGamepadTextInputMode::Normal,
        PreludeGamepadTextInputMode::Password,
    ];
    let _prelude_line_modes = [
        PreludeGamepadTextInputLineMode::SingleLine,
        PreludeGamepadTextInputLineMode::MultipleLines,
    ];
    assert!(matches!(
        steamworks::NotificationPosition::from(PreludeNotificationPosition::TopRight),
        steamworks::NotificationPosition::TopRight
    ));
    assert!(matches!(
        steamworks::GamepadTextInputMode::from(PreludeGamepadTextInputMode::Normal),
        steamworks::GamepadTextInputMode::Normal
    ));
    assert!(matches!(
        steamworks::GamepadTextInputLineMode::from(PreludeGamepadTextInputLineMode::SingleLine),
        steamworks::GamepadTextInputLineMode::SingleLine
    ));
    assert!(matches!(
        steamworks::FloatingGamepadTextInputMode::from(
            PreludeFloatingGamepadTextInputMode::MultipleLines
        ),
        steamworks::FloatingGamepadTextInputMode::MultipleLines
    ));

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
    accepts_prelude_exports(
        PreludeUtilsPlugin::new(),
        PreludeUtilsCommand::get_current_info(),
        PreludeUtilsOperation::CurrentInfoRead {
            info: bevy_steamworks::prelude::SteamworksUtilsInfo {
                app_id: steamworks::AppId(480),
                ip_country: "US".to_owned(),
                overlay_enabled: true,
                ui_language: "english".to_owned(),
                server_real_time: 1,
                steam_in_big_picture_mode: false,
                steam_running_on_steam_deck: false,
            },
        },
        PreludeUtilsResult::Ok(PreludeUtilsOperation::OverlayEnabledRead { enabled: true }),
        PreludeUtilsError::ClientUnavailable,
    );
    accepts_prelude_exports(
        PreludeUtilsPlugin::new(),
        PreludeUtilsCommand::install_warning_callback(),
        PreludeUtilsOperation::WarningCallbackInstalled,
        PreludeUtilsResult::Ok(PreludeUtilsOperation::WarningCallbackInstalled),
        PreludeUtilsError::ClientUnavailable,
    );
    accepts_prelude_exports(
        PreludeUtilsPlugin::new(),
        PreludeUtilsCommand::show_gamepad_text_input(gamepad_request.clone()),
        PreludeUtilsOperation::GamepadTextInputShown {
            shown: PreludeGamepadTextInputShown {
                request: gamepad_request,
                shown: true,
            },
        },
        PreludeUtilsResult::Ok(PreludeUtilsOperation::GamepadTextInputSubmitted {
            submitted: PreludeGamepadTextInputSubmitted {
                text: "Name".to_owned(),
                submitted_text_len: 4,
            },
        }),
        PreludeUtilsError::InvalidString {
            field: "description",
        },
    );
    accepts_prelude_exports(
        PreludeUtilsPlugin::new(),
        PreludeUtilsCommand::show_floating_gamepad_text_input(floating_request.clone()),
        PreludeUtilsOperation::FloatingGamepadTextInputShown {
            shown: PreludeFloatingGamepadTextInputShown {
                request: floating_request,
                shown: true,
            },
        },
        PreludeUtilsResult::Ok(PreludeUtilsOperation::FloatingGamepadTextInputDismissed {
            dismissed: PreludeFloatingGamepadTextInputDismissed,
        }),
        PreludeUtilsError::InvalidFloatingTextInputBounds {
            width: 0,
            height: 1,
        },
    );
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
    let query = SteamworksServerQueryId::from_raw(2);
    let target = SteamworksServerQueryTarget {
        address: std::net::Ipv4Addr::LOCALHOST,
        query_port: 27015,
    };
    let query_info = SteamworksServerQueryInfo {
        query,
        kind: SteamworksServerQueryKind::Ping,
        target,
    };
    let _root_query_types: (
        Option<SteamworksServerPing>,
        Option<SteamworksServerPlayerDetails>,
        Option<SteamworksServerPlayerInfo>,
        SteamworksServerQueryInfo,
        Option<SteamworksServerRule>,
        Option<SteamworksServerRules>,
    ) = (None, None, None, query_info, None, None);
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
    let query = PreludeServerQueryId::from_raw(2);
    let target = PreludeServerQueryTarget {
        address: std::net::Ipv4Addr::LOCALHOST,
        query_port: 27015,
    };
    let query_info = PreludeServerQueryInfo {
        query,
        kind: PreludeServerQueryKind::Ping,
        target,
    };
    let _prelude_query_types: (
        Option<PreludeServerPing>,
        Option<PreludeServerPlayerDetails>,
        Option<PreludeServerPlayerInfo>,
        PreludeServerQueryInfo,
        Option<PreludeServerRule>,
        Option<PreludeServerRules>,
    ) = (None, None, None, query_info, None, None);
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
