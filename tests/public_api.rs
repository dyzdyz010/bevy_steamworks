use bevy_app::{Plugin, PluginGroup};
use bevy_steamworks::{
    prelude::{
        SteamAPIInitError as PreludeInitError, SteamworksAppsCommand as PreludeAppsCommand,
        SteamworksAppsError as PreludeAppsError, SteamworksAppsOperation as PreludeAppsOperation,
        SteamworksAppsPlugin as PreludeAppsPlugin, SteamworksAppsResult as PreludeAppsResult,
        SteamworksAppsState as PreludeAppsState, SteamworksAvatarSize as PreludeAvatarSize,
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
        SteamworksFriendsState as PreludeFriendsState,
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
        SteamworksLobbyListResult as PreludeLobbyListResult,
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
        SteamworksMatchmakingState as PreludeMatchmakingState,
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
        SteamworksNetworkingSocketsState as PreludeNetworkingSocketsState,
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
        SteamworksRemotePlaySessionInfo as PreludeRemotePlaySessionInfo,
        SteamworksRemotePlaySessionSnapshot as PreludeRemotePlaySessionSnapshot,
        SteamworksRemotePlayState as PreludeRemotePlayState,
        SteamworksRemoteStorageCommand as PreludeRemoteStorageCommand,
        SteamworksRemoteStorageError as PreludeRemoteStorageError,
        SteamworksRemoteStorageFileContents as PreludeRemoteStorageFileContents,
        SteamworksRemoteStorageFileReadRequest as PreludeRemoteStorageFileReadRequest,
        SteamworksRemoteStorageFileShareHandle as PreludeRemoteStorageFileShareHandle,
        SteamworksRemoteStorageFileShareRequest as PreludeRemoteStorageFileShareRequest,
        SteamworksRemoteStorageFileWrite as PreludeRemoteStorageFileWrite,
        SteamworksRemoteStorageFileWriteRequest as PreludeRemoteStorageFileWriteRequest,
        SteamworksRemoteStorageFileWritten as PreludeRemoteStorageFileWritten,
        SteamworksRemoteStorageOperation as PreludeRemoteStorageOperation,
        SteamworksRemoteStoragePlugin as PreludeRemoteStoragePlugin,
        SteamworksRemoteStorageResult as PreludeRemoteStorageResult,
        SteamworksRemoteStorageSharedFile as PreludeRemoteStorageSharedFile,
        SteamworksRemoteStorageState as PreludeRemoteStorageState,
        SteamworksScreenshotReady as PreludeScreenshotReady,
        SteamworksScreenshotReadyError as PreludeScreenshotReadyError,
        SteamworksScreenshotsCommand as PreludeScreenshotsCommand,
        SteamworksScreenshotsError as PreludeScreenshotsError,
        SteamworksScreenshotsOperation as PreludeScreenshotsOperation,
        SteamworksScreenshotsPlugin as PreludeScreenshotsPlugin,
        SteamworksScreenshotsResult as PreludeScreenshotsResult,
        SteamworksScreenshotsState as PreludeScreenshotsState,
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
        SteamworksServerRules as PreludeServerRules, SteamworksServerState as PreludeServerState,
        SteamworksServerUnavailable as PreludeServerUnavailable,
        SteamworksStatsCommand as PreludeStatsCommand, SteamworksStatsError as PreludeStatsError,
        SteamworksStatsOperation as PreludeStatsOperation,
        SteamworksStatsPlugin as PreludeStatsPlugin, SteamworksStatsResult as PreludeStatsResult,
        SteamworksSubmittedScreenshot as PreludeSubmittedScreenshot,
        SteamworksSystem as PreludeSystem, SteamworksTimelineCommand as PreludeTimelineCommand,
        SteamworksTimelineError as PreludeTimelineError,
        SteamworksTimelineEventClipPriority as PreludeTimelineEventClipPriority,
        SteamworksTimelineGameMode as PreludeTimelineGameMode,
        SteamworksTimelineOperation as PreludeTimelineOperation,
        SteamworksTimelinePlugin as PreludeTimelinePlugin,
        SteamworksTimelineResult as PreludeTimelineResult,
        SteamworksTimelineState as PreludeTimelineState, SteamworksUgcCommand as PreludeUgcCommand,
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
        SteamworksUgcQueryIdsResult as PreludeUgcQueryIdsResult,
        SteamworksUgcQueryOptions as PreludeUgcQueryOptions,
        SteamworksUgcQueryRequest as PreludeUgcQueryRequest,
        SteamworksUgcQueryResult as PreludeUgcQueryResult,
        SteamworksUgcQueryTotal as PreludeUgcQueryTotal,
        SteamworksUgcQueryTotalResult as PreludeUgcQueryTotalResult,
        SteamworksUgcResult as PreludeUgcResult, SteamworksUgcState as PreludeUgcState,
        SteamworksUgcStatistic as PreludeUgcStatistic,
        SteamworksUgcWorkshopDepotId as PreludeUgcWorkshopDepotId,
        SteamworksUnavailable as PreludeUnavailable, SteamworksUserCommand as PreludeUserCommand,
        SteamworksUserError as PreludeUserError, SteamworksUserOperation as PreludeUserOperation,
        SteamworksUserPlugin as PreludeUserPlugin, SteamworksUserResult as PreludeUserResult,
        SteamworksUserState as PreludeUserState, SteamworksUtilsCommand as PreludeUtilsCommand,
        SteamworksUtilsError as PreludeUtilsError,
        SteamworksUtilsOperation as PreludeUtilsOperation,
        SteamworksUtilsPlugin as PreludeUtilsPlugin, SteamworksUtilsResult as PreludeUtilsResult,
        SteamworksUtilsState as PreludeUtilsState,
    },
    SteamAPIInitError, SteamworksAppsCommand, SteamworksAppsError, SteamworksAppsOperation,
    SteamworksAppsPlugin, SteamworksAppsResult, SteamworksAppsState, SteamworksAvatarSize,
    SteamworksCallbackRegistry, SteamworksClient, SteamworksClientPlugins, SteamworksCommandError,
    SteamworksConnectionRequestPolicy, SteamworksEvent, SteamworksFailurePolicy,
    SteamworksFloatingGamepadTextInputDismissed, SteamworksFloatingGamepadTextInputMode,
    SteamworksFloatingGamepadTextInputRequest, SteamworksFloatingGamepadTextInputShown,
    SteamworksFriendsCommand, SteamworksFriendsError, SteamworksFriendsOperation,
    SteamworksFriendsPlugin, SteamworksFriendsResult, SteamworksFriendsState,
    SteamworksGamepadTextInputDismissed, SteamworksGamepadTextInputLineMode,
    SteamworksGamepadTextInputMode, SteamworksGamepadTextInputRequest,
    SteamworksGamepadTextInputShown, SteamworksGamepadTextInputSubmitted, SteamworksInitMode,
    SteamworksInputCommand, SteamworksInputError, SteamworksInputOperation, SteamworksInputPlugin,
    SteamworksInputResult, SteamworksIssuedAuthSessionTicketForIdentity, SteamworksLobbyListFilter,
    SteamworksLobbyListResult, SteamworksMatchmakingCommand, SteamworksMatchmakingError,
    SteamworksMatchmakingOperation, SteamworksMatchmakingPlugin, SteamworksMatchmakingResult,
    SteamworksMatchmakingServersCommand, SteamworksMatchmakingServersError,
    SteamworksMatchmakingServersOperation, SteamworksMatchmakingServersPlugin,
    SteamworksMatchmakingServersResult, SteamworksMatchmakingState, SteamworksNetworkingCommand,
    SteamworksNetworkingError, SteamworksNetworkingMessagesCommand,
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
    SteamworksNetworkingSocketsResult, SteamworksNetworkingSocketsState, SteamworksNetworkingState,
    SteamworksNetworkingUtilsCommand, SteamworksNetworkingUtilsError,
    SteamworksNetworkingUtilsOperation, SteamworksNetworkingUtilsPlugin,
    SteamworksNetworkingUtilsResult, SteamworksNotificationPosition,
    SteamworksOverlayToStoreAction, SteamworksPlugin, SteamworksPlugins,
    SteamworksRemotePlayCommand, SteamworksRemotePlayError, SteamworksRemotePlayOperation,
    SteamworksRemotePlayPlugin, SteamworksRemotePlayResult, SteamworksRemotePlaySessionInfo,
    SteamworksRemotePlaySessionSnapshot, SteamworksRemotePlayState, SteamworksRemoteStorageCommand,
    SteamworksRemoteStorageError, SteamworksRemoteStorageFileContents,
    SteamworksRemoteStorageFileReadRequest, SteamworksRemoteStorageFileShareHandle,
    SteamworksRemoteStorageFileShareRequest, SteamworksRemoteStorageFileWrite,
    SteamworksRemoteStorageFileWriteRequest, SteamworksRemoteStorageFileWritten,
    SteamworksRemoteStorageOperation, SteamworksRemoteStoragePlugin, SteamworksRemoteStorageResult,
    SteamworksRemoteStorageSharedFile, SteamworksRemoteStorageState, SteamworksScreenshotReady,
    SteamworksScreenshotReadyError, SteamworksScreenshotsCommand, SteamworksScreenshotsError,
    SteamworksScreenshotsOperation, SteamworksScreenshotsPlugin, SteamworksScreenshotsResult,
    SteamworksScreenshotsState, SteamworksServerCommand, SteamworksServerConfig,
    SteamworksServerError, SteamworksServerInitMode,
    SteamworksServerIssuedAuthSessionTicketForIdentity, SteamworksServerListFilters,
    SteamworksServerListKind, SteamworksServerListRequestId, SteamworksServerOperation,
    SteamworksServerPing, SteamworksServerPlayerDetails, SteamworksServerPlayerInfo,
    SteamworksServerPlugin, SteamworksServerQueryId, SteamworksServerQueryInfo,
    SteamworksServerQueryKind, SteamworksServerQueryTarget, SteamworksServerResult,
    SteamworksServerRule, SteamworksServerRules, SteamworksServerState,
    SteamworksServerUnavailable, SteamworksStatsCommand, SteamworksStatsError,
    SteamworksStatsOperation, SteamworksStatsPlugin, SteamworksStatsResult,
    SteamworksSubmittedScreenshot, SteamworksSystem, SteamworksTimelineCommand,
    SteamworksTimelineError, SteamworksTimelineEventClipPriority, SteamworksTimelineGameMode,
    SteamworksTimelineOperation, SteamworksTimelinePlugin, SteamworksTimelineResult,
    SteamworksTimelineState, SteamworksUgcCommand, SteamworksUgcContentDescriptor,
    SteamworksUgcDownloadItemResult, SteamworksUgcError, SteamworksUgcGameServerWorkshopInit,
    SteamworksUgcItemDetails, SteamworksUgcItemDownloadInfo, SteamworksUgcItemDownloadInfoResult,
    SteamworksUgcItemInstallInfo, SteamworksUgcItemInstallInfoResult, SteamworksUgcItemStateInfo,
    SteamworksUgcOperation, SteamworksUgcPlugin, SteamworksUgcQuery, SteamworksUgcQueryIds,
    SteamworksUgcQueryIdsResult, SteamworksUgcQueryOptions, SteamworksUgcQueryRequest,
    SteamworksUgcQueryResult, SteamworksUgcQueryResults, SteamworksUgcQueryTotal,
    SteamworksUgcQueryTotalResult, SteamworksUgcResult, SteamworksUgcState, SteamworksUgcStatistic,
    SteamworksUgcWorkshopDepotId, SteamworksUnavailable, SteamworksUserCommand,
    SteamworksUserError, SteamworksUserOperation, SteamworksUserPlugin, SteamworksUserResult,
    SteamworksUserState, SteamworksUtilsCommand, SteamworksUtilsError, SteamworksUtilsOperation,
    SteamworksUtilsPlugin, SteamworksUtilsResult, SteamworksUtilsState,
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
    let _restart: fn(steamworks::AppId) -> bool = SteamworksPlugin::restart_app_if_necessary;
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
    let _prelude_restart: fn(steamworks::AppId) -> bool = PreludePlugin::restart_app_if_necessary;
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
    let _restart: fn(steamworks::AppId) -> bool = SteamworksPlugins::restart_app_if_necessary;
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
    let _prelude_restart: fn(steamworks::AppId) -> bool = PreludePlugins::restart_app_if_necessary;
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
    let root_state = SteamworksAppsState::default();
    let app_id = steamworks::AppId(480);
    assert_eq!(root_state.current_app_id(), None);
    assert_eq!(root_state.current_app_owned(), None);
    assert_eq!(root_state.known_app_install_check_count(), 0);
    assert_eq!(root_state.known_dlc_install_check_count(), 0);
    assert_eq!(root_state.known_subscribed_app_check_count(), 0);
    assert_eq!(root_state.app_installed(app_id), None);
    assert_eq!(root_state.available_game_language_count(), None);
    assert_eq!(root_state.supports_game_language("english"), None);
    assert_eq!(root_state.current_game_language_is("english"), None);
    assert_eq!(root_state.is_on_beta_branch(), None);
    assert_eq!(root_state.has_launch_command_line(), None);
    assert!(!root_state.launch_query_param_was_read("connect"));
    assert_eq!(root_state.launch_query_param_has_value("connect"), None);

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
    let prelude_state = PreludeAppsState::default();
    assert_eq!(prelude_state.current_app_id(), None);
    assert_eq!(prelude_state.current_app_owned(), None);
    assert_eq!(prelude_state.known_app_install_check_count(), 0);
    assert_eq!(prelude_state.known_dlc_install_check_count(), 0);
    assert_eq!(prelude_state.known_subscribed_app_check_count(), 0);
    assert_eq!(prelude_state.app_installed(app_id), None);
    assert_eq!(prelude_state.available_game_language_count(), None);
    assert_eq!(prelude_state.supports_game_language("english"), None);
    assert_eq!(prelude_state.current_game_language_is("english"), None);
    assert_eq!(prelude_state.is_on_beta_branch(), None);
    assert_eq!(prelude_state.has_launch_command_line(), None);
    assert!(!prelude_state.launch_query_param_was_read("connect"));
    assert_eq!(prelude_state.launch_query_param_has_value("connect"), None);
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
    let friend = steamworks::SteamId::from_raw(11);
    let lobby = steamworks::LobbyId::from_raw(12);
    let game = steamworks::GameId::from_raw(480);
    let root_state = SteamworksFriendsState::default();
    assert_eq!(root_state.friend_count(), 0);
    assert_eq!(root_state.known_friend_count(), 0);
    assert_eq!(root_state.coplay_friend_count(), 0);
    assert!(!root_state.has_known_friend(friend));
    assert_eq!(root_state.friend_name(friend), None);
    assert_eq!(root_state.friend_nickname(friend), None);
    assert_eq!(root_state.friend_state(friend), None);
    assert_eq!(root_state.friend_game(friend), None);
    assert_eq!(root_state.online_friends().count(), 0);
    assert_eq!(root_state.friends_in_game().count(), 0);
    assert_eq!(root_state.friends_playing_game(game).count(), 0);
    assert_eq!(root_state.friends_in_lobby(lobby).count(), 0);
    assert_eq!(root_state.friend_is_in_game(friend), None);
    assert_eq!(root_state.friend_is_in_lobby(friend, lobby), None);
    assert_eq!(root_state.coplay_app_id(friend), None);
    assert_eq!(root_state.coplay_time(friend), None);
    assert_eq!(
        root_state.friend_avatar_dimensions(friend, SteamworksAvatarSize::Small),
        None
    );
    assert_eq!(
        root_state.friend_avatar_rgba(friend, SteamworksAvatarSize::Small),
        None
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
    let prelude_state = PreludeFriendsState::default();
    assert_eq!(prelude_state.friend_count(), 0);
    assert_eq!(prelude_state.known_friend_count(), 0);
    assert_eq!(prelude_state.coplay_friend_count(), 0);
    assert!(!prelude_state.has_known_friend(friend));
    assert_eq!(prelude_state.friend_name(friend), None);
    assert_eq!(prelude_state.friend_nickname(friend), None);
    assert_eq!(prelude_state.friend_state(friend), None);
    assert_eq!(prelude_state.friend_game(friend), None);
    assert_eq!(prelude_state.online_friends().count(), 0);
    assert_eq!(prelude_state.friends_in_game().count(), 0);
    assert_eq!(prelude_state.friends_playing_game(game).count(), 0);
    assert_eq!(prelude_state.friends_in_lobby(lobby).count(), 0);
    assert_eq!(prelude_state.friend_is_in_game(friend), None);
    assert_eq!(prelude_state.friend_is_in_lobby(friend, lobby), None);
    assert_eq!(prelude_state.coplay_app_id(friend), None);
    assert_eq!(prelude_state.coplay_time(friend), None);
    assert_eq!(
        prelude_state.friend_avatar_dimensions(friend, PreludeAvatarSize::Small),
        None
    );
    assert_eq!(
        prelude_state.friend_avatar_rgba(friend, PreludeAvatarSize::Small),
        None
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
    let user = steamworks::SteamId::from_raw(7);
    let group = steamworks::SteamId::from_raw(8);
    let root_state = SteamworksServerState::default();
    assert_eq!(root_state.active_auth_ticket_count(), 0);
    assert!(!root_state.is_user_authenticated(user));
    assert!(root_state.auth_session_tickets().is_empty());
    assert!(root_state.auth_session_tickets_for_identity().is_empty());
    assert!(root_state.auth_ticket_responses().is_empty());
    assert!(root_state.auth_ticket_validations().is_empty());
    assert_eq!(root_state.auth_ticket_validation(user), None);
    assert_eq!(root_state.auth_ticket_validation_succeeded(user), None);
    assert!(root_state.steam_server_connection_events().is_empty());
    assert!(root_state.client_approvals().is_empty());
    assert_eq!(root_state.client_approval(user), None);
    assert!(!root_state.has_client_approval(user));
    assert_eq!(root_state.client_approval_owner(user), None);
    assert!(root_state.client_denials().is_empty());
    assert_eq!(root_state.client_denial(user), None);
    assert!(!root_state.has_client_denial(user));
    assert_eq!(root_state.client_denial_reason(user), None);
    assert!(root_state.client_kicks().is_empty());
    assert_eq!(root_state.client_kick(user), None);
    assert!(!root_state.has_client_kick(user));
    assert_eq!(root_state.client_kick_reason(user), None);
    assert!(root_state.client_group_statuses().is_empty());
    assert_eq!(root_state.client_group_status(user, group), None);
    assert_eq!(root_state.client_group_member(user, group), None);
    assert_eq!(root_state.client_group_officer(user, group), None);
    assert_eq!(root_state.key_value("map"), None);

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
    let prelude_state = PreludeServerState::default();
    assert_eq!(prelude_state.active_auth_ticket_count(), 0);
    assert!(!prelude_state.is_user_authenticated(user));
    assert!(prelude_state.auth_session_tickets().is_empty());
    assert!(prelude_state.auth_session_tickets_for_identity().is_empty());
    assert!(prelude_state.auth_ticket_responses().is_empty());
    assert!(prelude_state.auth_ticket_validations().is_empty());
    assert_eq!(prelude_state.auth_ticket_validation(user), None);
    assert_eq!(prelude_state.auth_ticket_validation_succeeded(user), None);
    assert!(prelude_state.steam_server_connection_events().is_empty());
    assert!(prelude_state.client_approvals().is_empty());
    assert_eq!(prelude_state.client_approval(user), None);
    assert!(!prelude_state.has_client_approval(user));
    assert_eq!(prelude_state.client_approval_owner(user), None);
    assert!(prelude_state.client_denials().is_empty());
    assert_eq!(prelude_state.client_denial(user), None);
    assert!(!prelude_state.has_client_denial(user));
    assert_eq!(prelude_state.client_denial_reason(user), None);
    assert!(prelude_state.client_kicks().is_empty());
    assert_eq!(prelude_state.client_kick(user), None);
    assert!(!prelude_state.has_client_kick(user));
    assert_eq!(prelude_state.client_kick_reason(user), None);
    assert!(prelude_state.client_group_statuses().is_empty());
    assert_eq!(prelude_state.client_group_status(user, group), None);
    assert_eq!(prelude_state.client_group_member(user, group), None);
    assert_eq!(prelude_state.client_group_officer(user, group), None);
    assert_eq!(prelude_state.key_value("map"), None);

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
    let root_state = bevy_steamworks::SteamworksInputState::default();
    let root_controller = bevy_steamworks::SteamworksInputHandle::from_raw(1);
    let root_action_set = bevy_steamworks::SteamworksInputActionSetHandle::from_raw(1);
    let root_digital_action = bevy_steamworks::SteamworksInputDigitalActionHandle::from_raw(1);
    let root_analog_action = bevy_steamworks::SteamworksInputAnalogActionHandle::from_raw(1);
    assert_eq!(root_state.controller_count(), 0);
    assert!(!root_state.has_controller(root_controller));
    assert_eq!(root_state.controller_input_type(root_controller), None);
    assert_eq!(root_state.action_set_count(), 0);
    assert!(!root_state.has_action_set_handle("gameplay"));
    assert_eq!(root_state.digital_action_count(), 0);
    assert!(!root_state.has_digital_action_handle("jump"));
    assert_eq!(root_state.analog_action_count(), 0);
    assert!(!root_state.has_analog_action_handle("move"));
    assert!(root_state.action_set_activations().is_empty());
    assert_eq!(root_state.action_set_activation_count(), 0);
    assert!(root_state.action_set_activation(root_controller).is_none());
    assert!(!root_state.has_action_set_activation(root_controller));
    assert_eq!(root_state.active_action_set(root_controller), None);
    assert!(root_state.digital_action_data_snapshots().is_empty());
    assert_eq!(root_state.digital_action_snapshot_count(), 0);
    assert!(root_state
        .digital_action_data(root_controller, root_digital_action)
        .is_none());
    assert_eq!(
        root_state.digital_action_pressed(root_controller, root_digital_action),
        None
    );
    assert_eq!(
        root_state.digital_action_active(root_controller, root_digital_action),
        None
    );
    assert!(root_state.analog_action_data_snapshots().is_empty());
    assert_eq!(root_state.analog_action_snapshot_count(), 0);
    assert!(root_state
        .analog_action_data(root_controller, root_analog_action)
        .is_none());
    assert_eq!(
        root_state.analog_action_mode(root_controller, root_analog_action),
        None
    );
    assert_eq!(
        root_state.analog_action_vector(root_controller, root_analog_action),
        None
    );
    assert_eq!(
        root_state.analog_action_active(root_controller, root_analog_action),
        None
    );
    assert!(root_state.digital_action_origin_snapshots().is_empty());
    assert_eq!(root_state.digital_action_origin_snapshot_count(), 0);
    assert!(root_state
        .digital_action_origins(root_controller, root_action_set, root_digital_action)
        .is_none());
    assert_eq!(
        root_state.digital_action_origin_count(
            root_controller,
            root_action_set,
            root_digital_action
        ),
        None
    );
    assert!(root_state.analog_action_origin_snapshots().is_empty());
    assert_eq!(root_state.analog_action_origin_snapshot_count(), 0);
    assert!(root_state
        .analog_action_origins(root_controller, root_action_set, root_analog_action)
        .is_none());
    assert_eq!(
        root_state.analog_action_origin_count(root_controller, root_action_set, root_analog_action),
        None
    );
    assert_eq!(root_state.action_origin_info_count(), 0);
    assert!(!root_state.has_action_origin_info(root_origin));
    assert_eq!(root_state.action_origin_glyph_path(root_origin), None);
    assert_eq!(root_state.action_origin_name(root_origin), None);
    assert!(root_state.motion_snapshots().is_empty());
    assert_eq!(root_state.motion_snapshot_count(), 0);
    assert!(root_state.motion(root_controller).is_none());
    assert!(!root_state.has_motion(root_controller));
    assert_eq!(root_state.motion_rotation_quaternion(root_controller), None);
    assert_eq!(
        root_state.motion_position_acceleration(root_controller),
        None
    );
    assert_eq!(root_state.motion_rotation_velocity(root_controller), None);
    assert!(!root_state.binding_panel_was_shown());

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
    let prelude_state = bevy_steamworks::prelude::SteamworksInputState::default();
    let prelude_controller = bevy_steamworks::prelude::SteamworksInputHandle::from_raw(1);
    let prelude_action_set = bevy_steamworks::prelude::SteamworksInputActionSetHandle::from_raw(1);
    let prelude_digital_action =
        bevy_steamworks::prelude::SteamworksInputDigitalActionHandle::from_raw(1);
    let prelude_analog_action =
        bevy_steamworks::prelude::SteamworksInputAnalogActionHandle::from_raw(1);
    assert_eq!(prelude_state.controller_count(), 0);
    assert!(!prelude_state.has_controller(prelude_controller));
    assert_eq!(
        prelude_state.controller_input_type(prelude_controller),
        None
    );
    assert_eq!(prelude_state.action_set_count(), 0);
    assert!(!prelude_state.has_action_set_handle("gameplay"));
    assert_eq!(prelude_state.digital_action_count(), 0);
    assert!(!prelude_state.has_digital_action_handle("jump"));
    assert_eq!(prelude_state.analog_action_count(), 0);
    assert!(!prelude_state.has_analog_action_handle("move"));
    assert!(prelude_state.digital_action_data_snapshots().is_empty());
    assert_eq!(prelude_state.digital_action_snapshot_count(), 0);
    assert!(prelude_state.analog_action_data_snapshots().is_empty());
    assert_eq!(prelude_state.analog_action_snapshot_count(), 0);
    assert!(prelude_state.digital_action_origin_snapshots().is_empty());
    assert_eq!(prelude_state.digital_action_origin_snapshot_count(), 0);
    assert!(prelude_state.analog_action_origin_snapshots().is_empty());
    assert_eq!(prelude_state.analog_action_origin_snapshot_count(), 0);
    assert_eq!(
        prelude_state.digital_action_origin_count(
            prelude_controller,
            prelude_action_set,
            prelude_digital_action,
        ),
        None
    );
    assert_eq!(
        prelude_state.analog_action_origin_count(
            prelude_controller,
            prelude_action_set,
            prelude_analog_action,
        ),
        None
    );
    assert_eq!(prelude_state.action_origin_info_count(), 0);
    assert!(!prelude_state.has_action_origin_info(prelude_origin));
    assert_eq!(prelude_state.action_origin_glyph_path(prelude_origin), None);
    assert_eq!(prelude_state.action_origin_name(prelude_origin), None);
    assert!(prelude_state.motion_snapshots().is_empty());
    assert_eq!(prelude_state.motion_snapshot_count(), 0);
    assert!(!prelude_state.has_motion(prelude_controller));
    assert!(!prelude_state.binding_panel_was_shown());
    assert_eq!(
        prelude_state.digital_action_pressed(prelude_controller, prelude_digital_action),
        None
    );
    assert_eq!(
        prelude_state.analog_action_vector(prelude_controller, prelude_analog_action),
        None
    );
    assert_eq!(
        prelude_state.motion_rotation_velocity(prelude_controller),
        None
    );
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
    let root_state = SteamworksMatchmakingState::default();
    assert!(root_state.lobby_list_requests().is_empty());
    assert_eq!(root_state.last_lobby_list_count(), 0);
    assert_eq!(root_state.lobby_list_request(1), None);
    assert!(root_state.lobby_list_results().is_empty());
    assert_eq!(root_state.lobby_list_result(1), None);
    assert_eq!(root_state.joined_lobby_count(), 0);
    assert_eq!(root_state.last_joined_lobby_id(), None);
    assert!(!root_state.is_lobby_joined(lobby));
    assert!(root_state.lobby_data_values().is_empty());
    assert_eq!(root_state.lobby_data_count_value(lobby), None);
    assert_eq!(root_state.lobby_data_value(lobby, "mode"), None);
    assert_eq!(root_state.lobby_data(lobby, "mode"), None);
    assert_eq!(root_state.has_lobby_data(lobby, "mode"), None);
    assert_eq!(root_state.all_lobby_data_value(lobby, "mode"), None);
    assert!(root_state.lobby_member_data_values().is_empty());
    assert_eq!(
        root_state.lobby_member_data_value(lobby, user, "rank"),
        None
    );
    assert_eq!(root_state.lobby_member_data(lobby, user, "rank"), None);
    assert_eq!(root_state.has_lobby_member_data(lobby, user, "rank"), None);
    assert_eq!(root_state.lobby_member_limit_value(lobby), None);
    assert_eq!(root_state.lobby_owner_id(lobby), None);
    assert_eq!(root_state.lobby_member_count_value(lobby), None);
    assert_eq!(root_state.lobby_member_ids(lobby), None);
    assert_eq!(root_state.has_lobby_member(lobby, user), None);
    assert_eq!(root_state.lobby_joinable(lobby), None);
    assert_eq!(root_state.lobby_chat_entry_data(lobby, 1), None);
    assert_eq!(root_state.lobby_chat_entry_len(lobby, 1), None);
    assert_eq!(root_state.last_lobby_chat_entry_data(), None);
    assert_eq!(root_state.lobby_game_server(lobby), None);
    assert_eq!(root_state.has_lobby_game_server(lobby), None);
    assert_eq!(root_state.lobby_game_server_address(lobby), None);
    assert_eq!(root_state.lobby_game_server_steam_id(lobby), None);
    let _root_lobby_list_result = SteamworksLobbyListResult {
        request_id: 1,
        filter: SteamworksLobbyListFilter::new(),
        lobbies: vec![lobby],
    };

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
    let prelude_state = PreludeMatchmakingState::default();
    assert!(prelude_state.lobby_list_requests().is_empty());
    assert_eq!(prelude_state.last_lobby_list_count(), 0);
    assert_eq!(prelude_state.lobby_list_request(1), None);
    assert!(prelude_state.lobby_list_results().is_empty());
    assert_eq!(prelude_state.lobby_list_result(1), None);
    assert_eq!(prelude_state.joined_lobby_count(), 0);
    assert_eq!(prelude_state.last_joined_lobby_id(), None);
    assert!(!prelude_state.is_lobby_joined(lobby));
    assert!(prelude_state.lobby_data_values().is_empty());
    assert_eq!(prelude_state.lobby_data_count_value(lobby), None);
    assert_eq!(prelude_state.lobby_data_value(lobby, "mode"), None);
    assert_eq!(prelude_state.lobby_data(lobby, "mode"), None);
    assert_eq!(prelude_state.has_lobby_data(lobby, "mode"), None);
    assert_eq!(prelude_state.all_lobby_data_value(lobby, "mode"), None);
    assert!(prelude_state.lobby_member_data_values().is_empty());
    assert_eq!(
        prelude_state.lobby_member_data_value(lobby, user, "rank"),
        None
    );
    assert_eq!(prelude_state.lobby_member_data(lobby, user, "rank"), None);
    assert_eq!(
        prelude_state.has_lobby_member_data(lobby, user, "rank"),
        None
    );
    assert_eq!(prelude_state.lobby_member_limit_value(lobby), None);
    assert_eq!(prelude_state.lobby_owner_id(lobby), None);
    assert_eq!(prelude_state.lobby_member_count_value(lobby), None);
    assert_eq!(prelude_state.lobby_member_ids(lobby), None);
    assert_eq!(prelude_state.has_lobby_member(lobby, user), None);
    assert_eq!(prelude_state.lobby_joinable(lobby), None);
    assert_eq!(prelude_state.lobby_chat_entry_data(lobby, 1), None);
    assert_eq!(prelude_state.lobby_chat_entry_len(lobby, 1), None);
    assert_eq!(prelude_state.last_lobby_chat_entry_data(), None);
    assert_eq!(prelude_state.lobby_game_server(lobby), None);
    assert_eq!(prelude_state.has_lobby_game_server(lobby), None);
    assert_eq!(prelude_state.lobby_game_server_address(lobby), None);
    assert_eq!(prelude_state.lobby_game_server_steam_id(lobby), None);
    let _prelude_lobby_list_result = PreludeLobbyListResult {
        request_id: 1,
        filter: PreludeLobbyListFilter::new(),
        lobbies: vec![lobby],
    };
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
    assert_eq!(state.session_state_count(), 0);
    assert_eq!(state.session_state(user), None);
    assert!(!state.has_session_state(user));
    assert_eq!(state.p2p_session_state(user), None);
    assert_eq!(state.session_active(user), None);
    assert_eq!(state.session_connecting(user), None);
    assert_eq!(state.session_using_relay(user), None);
    assert_eq!(state.session_error(user), None);
    assert_eq!(state.session_bytes_queued_for_send(user), None);
    assert_eq!(state.session_packets_queued_for_send(user), None);
    assert_eq!(state.session_remote_ip(user), None);
    assert_eq!(state.session_remote_port(user), None);
    assert!(state.packet_availabilities().is_empty());
    assert_eq!(state.packet_availability_count(), 0);
    assert_eq!(state.packet_availability(0), None);
    assert_eq!(state.packet_available_bytes(0), None);
    assert_eq!(state.packet_available(0), None);
    assert_eq!(state.last_sent_packet_remote(), None);
    assert_eq!(state.last_sent_packet_send_type(), None);
    assert_eq!(state.last_sent_packet_channel(), None);
    assert_eq!(state.last_sent_packet_bytes(), None);
    assert!(state.received_packets().is_empty());
    assert_eq!(state.cached_received_packet_count(), 0);
    assert_eq!(state.last_packet_remote(), None);
    assert_eq!(state.last_packet_channel(), None);
    assert_eq!(state.last_packet_bytes(), None);
    assert_eq!(state.last_packet_data(), None);
    assert_eq!(state.received_packet_count_from(user), 0);
    assert_eq!(state.received_packet_count_on_channel(0), 0);
    assert_eq!(state.last_packet_from(user), None);
    assert_eq!(state.last_packet_bytes_from(user), None);
    assert_eq!(state.last_packet_on_channel(0), None);
    assert_eq!(state.last_packet_bytes_on_channel(0), None);
    assert!(state.session_requests().is_empty());
    assert_eq!(state.cached_session_request_count(), 0);
    assert!(!state.has_session_request(user));
    assert!(state.session_connect_failures().is_empty());
    assert_eq!(state.cached_session_connect_failure_count(), 0);
    assert_eq!(state.session_connect_failure(user), None);
    assert!(!state.has_session_connect_failure(user));
    assert_eq!(state.session_connect_failure_error(user), None);

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
    assert_eq!(state.session_state_count(), 0);
    assert_eq!(state.session_state(user), None);
    assert!(!state.has_session_state(user));
    assert_eq!(state.p2p_session_state(user), None);
    assert_eq!(state.session_active(user), None);
    assert_eq!(state.session_connecting(user), None);
    assert_eq!(state.session_using_relay(user), None);
    assert_eq!(state.session_error(user), None);
    assert_eq!(state.session_bytes_queued_for_send(user), None);
    assert_eq!(state.session_packets_queued_for_send(user), None);
    assert_eq!(state.session_remote_ip(user), None);
    assert_eq!(state.session_remote_port(user), None);
    assert!(state.packet_availabilities().is_empty());
    assert_eq!(state.packet_availability_count(), 0);
    assert_eq!(state.packet_availability(0), None);
    assert_eq!(state.packet_available_bytes(0), None);
    assert_eq!(state.packet_available(0), None);
    assert_eq!(state.last_sent_packet_remote(), None);
    assert_eq!(state.last_sent_packet_send_type(), None);
    assert_eq!(state.last_sent_packet_channel(), None);
    assert_eq!(state.last_sent_packet_bytes(), None);
    assert!(state.received_packets().is_empty());
    assert_eq!(state.cached_received_packet_count(), 0);
    assert_eq!(state.last_packet_remote(), None);
    assert_eq!(state.last_packet_channel(), None);
    assert_eq!(state.last_packet_bytes(), None);
    assert_eq!(state.last_packet_data(), None);
    assert_eq!(state.received_packet_count_from(user), 0);
    assert_eq!(state.received_packet_count_on_channel(0), 0);
    assert_eq!(state.last_packet_from(user), None);
    assert_eq!(state.last_packet_bytes_from(user), None);
    assert_eq!(state.last_packet_on_channel(0), None);
    assert_eq!(state.last_packet_bytes_on_channel(0), None);
    assert!(state.session_requests().is_empty());
    assert_eq!(state.cached_session_request_count(), 0);
    assert!(!state.has_session_request(user));
    assert!(state.session_connect_failures().is_empty());
    assert_eq!(state.cached_session_connect_failure_count(), 0);
    assert_eq!(state.session_connect_failure(user), None);
    assert!(!state.has_session_connect_failure(user));
    assert_eq!(state.session_connect_failure_error(user), None);

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
    let identity = peer.to_identity();
    assert_eq!(state.received_message_count(), 0);
    assert!(state.recent_received_messages().is_empty());
    assert_eq!(state.recent_received_message_count(), 0);
    assert_eq!(state.last_received_message(), None);
    assert_eq!(state.last_received_message_peer(), None);
    assert_eq!(state.last_received_message_channel(), None);
    assert_eq!(state.last_received_message_bytes(), None);
    assert_eq!(state.last_received_message_data(), None);
    assert_eq!(state.received_message_count_on_channel(0), 0);
    assert_eq!(state.recent_received_message_count_on_channel(0), 0);
    assert_eq!(state.last_received_message_on_channel(0), None);
    assert_eq!(state.last_recent_received_message_on_channel(0), None);
    assert_eq!(state.received_message_count_from_peer(&identity), 0);
    assert_eq!(state.recent_received_message_count_from_peer(&identity), 0);
    assert_eq!(state.last_received_message_from_peer(&identity), None);
    assert_eq!(
        state.last_recent_received_message_from_peer(&identity),
        None
    );
    assert_eq!(
        state.last_recent_received_message_bytes_from_peer(&identity),
        None
    );
    assert_eq!(state.last_connection_state(), None);
    assert_eq!(state.last_connection_remote(), None);
    assert_eq!(state.last_connection_user_data(), None);
    assert_eq!(state.last_connection_end_reason(), None);
    assert_eq!(state.last_connection_ping(), None);
    assert_eq!(state.last_connection_quality(), None);
    assert_eq!(state.recent_received_messages_on_channel(0).count(), 0);
    assert_eq!(
        state.recent_received_messages_from_peer(&identity).count(),
        0
    );
    assert!(state.session_requests().is_empty());
    assert_eq!(state.cached_session_request_count(), 0);
    assert_eq!(state.session_request(&identity), None);
    assert!(!state.has_session_request(&identity));
    assert_eq!(state.session_request_accepted(&identity), None);
    assert!(state.session_failures().is_empty());
    assert_eq!(state.cached_session_failure_count(), 0);
    assert_eq!(state.session_failure(&identity), None);
    assert!(!state.has_session_failure(&identity));
    assert_eq!(state.session_failure_state(&identity), None);
    assert_eq!(state.session_failure_end_reason(&identity), None);

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
    let identity = peer.to_identity();
    assert_eq!(state.received_message_count(), 0);
    assert!(state.recent_received_messages().is_empty());
    assert_eq!(state.recent_received_message_count(), 0);
    assert_eq!(state.last_received_message(), None);
    assert_eq!(state.last_received_message_peer(), None);
    assert_eq!(state.last_received_message_channel(), None);
    assert_eq!(state.last_received_message_bytes(), None);
    assert_eq!(state.last_received_message_data(), None);
    assert_eq!(state.received_message_count_on_channel(0), 0);
    assert_eq!(state.recent_received_message_count_on_channel(0), 0);
    assert_eq!(state.last_received_message_on_channel(0), None);
    assert_eq!(state.last_recent_received_message_on_channel(0), None);
    assert_eq!(state.received_message_count_from_peer(&identity), 0);
    assert_eq!(state.recent_received_message_count_from_peer(&identity), 0);
    assert_eq!(state.last_received_message_from_peer(&identity), None);
    assert_eq!(
        state.last_recent_received_message_from_peer(&identity),
        None
    );
    assert_eq!(
        state.last_recent_received_message_bytes_from_peer(&identity),
        None
    );
    assert_eq!(state.last_connection_state(), None);
    assert_eq!(state.last_connection_remote(), None);
    assert_eq!(state.last_connection_user_data(), None);
    assert_eq!(state.last_connection_end_reason(), None);
    assert_eq!(state.last_connection_ping(), None);
    assert_eq!(state.last_connection_quality(), None);
    assert_eq!(state.recent_received_messages_on_channel(0).count(), 0);
    assert_eq!(
        state.recent_received_messages_from_peer(&identity).count(),
        0
    );
    assert!(state.session_requests().is_empty());
    assert_eq!(state.cached_session_request_count(), 0);
    assert_eq!(state.session_request(&identity), None);
    assert!(!state.has_session_request(&identity));
    assert_eq!(state.session_request_accepted(&identity), None);
    assert!(state.session_failures().is_empty());
    assert_eq!(state.cached_session_failure_count(), 0);
    assert_eq!(state.session_failure(&identity), None);
    assert!(!state.has_session_failure(&identity));
    assert_eq!(state.session_failure_state(&identity), None);
    assert_eq!(state.session_failure_end_reason(&identity), None);

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
    let poll_group = bevy_steamworks::SteamworksNetworkingSocketsPollGroupId::from_raw(1);
    let poll_group_messages = SteamworksNetworkingSocketsPollGroupMessages {
        poll_group,
        messages: Vec::new(),
    };
    let state = SteamworksNetworkingSocketsState::default();
    assert!(state.listen_socket_events().is_empty());
    assert_eq!(state.listen_socket_event_batch_count(), 0);
    assert_eq!(
        state.listen_socket_event_batch(bevy_steamworks::SteamworksListenSocketId::from_raw(1)),
        None
    );
    assert_eq!(
        state.listen_socket_event_count(bevy_steamworks::SteamworksListenSocketId::from_raw(1)),
        None
    );
    assert!(state.connection_events().is_empty());
    assert_eq!(state.connection_event_batch_count(), 0);
    assert_eq!(state.connection_event_batch(connection), None);
    assert_eq!(state.connection_event_count(connection), None);
    assert_eq!(state.connection_event_removed(connection), None);
    assert!(state.connection_infos().is_empty());
    assert_eq!(state.connection_info_count(), 0);
    assert_eq!(state.connection_info(connection), None);
    assert_eq!(state.connection_state(connection), None);
    assert_eq!(state.connection_remote(connection), None);
    assert_eq!(state.connection_end_reason(connection), None);
    assert_eq!(state.connection_user_data(connection), None);
    assert_eq!(state.connection_name(connection), None);
    assert!(state.realtime_statuses().is_empty());
    assert_eq!(state.realtime_status_count(), 0);
    assert_eq!(state.realtime_status(connection), None);
    assert_eq!(state.connection_ping(connection), None);
    assert_eq!(state.connection_quality(connection), None);
    assert_eq!(state.connection_send_rate_bytes_per_sec(connection), None);
    assert_eq!(state.connection_pending_unreliable(connection), None);
    assert_eq!(state.connection_pending_reliable(connection), None);
    assert_eq!(state.connection_lane_count(connection), None);
    assert_eq!(state.last_sent_message_connection(), None);
    assert_eq!(state.last_sent_message_number(), None);
    assert_eq!(state.last_sent_message_bytes(), None);
    assert_eq!(state.last_sent_message_count(), 0);
    assert!(state.recent_sent_messages().is_empty());
    assert_eq!(state.recent_sent_message_count(), 0);
    assert_eq!(state.sent_message_count_for_connection(connection), 0);
    assert_eq!(state.last_sent_message_for_connection(connection), None);
    assert_eq!(
        state.last_sent_message_number_for_connection(connection),
        None
    );
    assert_eq!(
        state.last_sent_message_succeeded_for_connection(connection),
        None
    );
    assert_eq!(
        state.last_sent_message_bytes_for_connection(connection),
        None
    );
    assert_eq!(state.last_received_message_count(), 0);
    assert!(state.recent_received_messages().is_empty());
    assert_eq!(state.recent_received_message_count(), 0);
    assert_eq!(state.received_message_count_for_connection(connection), 0);
    assert_eq!(state.last_received_message_for_connection(connection), None);
    assert_eq!(
        state.last_received_message_bytes_for_connection(connection),
        None
    );
    assert_eq!(
        state.last_received_message_channel_for_connection(connection),
        None
    );
    assert_eq!(
        state.last_received_message_data_for_connection(connection),
        None
    );
    assert_eq!(state.last_poll_group_message_count(), 0);
    assert!(state.recent_poll_group_messages().is_empty());
    assert_eq!(state.recent_poll_group_message_count(), 0);
    assert_eq!(state.poll_group_message_count(poll_group), 0);
    assert_eq!(state.last_poll_group_message(poll_group), None);
    assert_eq!(state.last_poll_group_message_bytes(poll_group), None);
    assert_eq!(state.last_poll_group_message_channel(poll_group), None);
    assert_eq!(state.last_poll_group_message_data(poll_group), None);

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
    let poll_group = bevy_steamworks::prelude::SteamworksNetworkingSocketsPollGroupId::from_raw(1);
    let poll_group_messages = PreludeNetworkingSocketsPollGroupMessages {
        poll_group,
        messages: Vec::new(),
    };
    let state = PreludeNetworkingSocketsState::default();
    assert!(state.listen_socket_events().is_empty());
    assert_eq!(state.listen_socket_event_batch_count(), 0);
    assert_eq!(
        state.listen_socket_event_batch(
            bevy_steamworks::prelude::SteamworksListenSocketId::from_raw(1),
        ),
        None
    );
    assert_eq!(
        state.listen_socket_event_count(
            bevy_steamworks::prelude::SteamworksListenSocketId::from_raw(1),
        ),
        None
    );
    assert!(state.connection_events().is_empty());
    assert_eq!(state.connection_event_batch_count(), 0);
    assert_eq!(state.connection_event_batch(connection), None);
    assert_eq!(state.connection_event_count(connection), None);
    assert_eq!(state.connection_event_removed(connection), None);
    assert!(state.connection_infos().is_empty());
    assert_eq!(state.connection_info_count(), 0);
    assert_eq!(state.connection_info(connection), None);
    assert_eq!(state.connection_state(connection), None);
    assert_eq!(state.connection_remote(connection), None);
    assert_eq!(state.connection_end_reason(connection), None);
    assert_eq!(state.connection_user_data(connection), None);
    assert_eq!(state.connection_name(connection), None);
    assert!(state.realtime_statuses().is_empty());
    assert_eq!(state.realtime_status_count(), 0);
    assert_eq!(state.realtime_status(connection), None);
    assert_eq!(state.connection_ping(connection), None);
    assert_eq!(state.connection_quality(connection), None);
    assert_eq!(state.connection_send_rate_bytes_per_sec(connection), None);
    assert_eq!(state.connection_pending_unreliable(connection), None);
    assert_eq!(state.connection_pending_reliable(connection), None);
    assert_eq!(state.connection_lane_count(connection), None);
    assert_eq!(state.last_sent_message_connection(), None);
    assert_eq!(state.last_sent_message_number(), None);
    assert_eq!(state.last_sent_message_bytes(), None);
    assert_eq!(state.last_sent_message_count(), 0);
    assert!(state.recent_sent_messages().is_empty());
    assert_eq!(state.recent_sent_message_count(), 0);
    assert_eq!(state.sent_message_count_for_connection(connection), 0);
    assert_eq!(state.last_sent_message_for_connection(connection), None);
    assert_eq!(
        state.last_sent_message_number_for_connection(connection),
        None
    );
    assert_eq!(
        state.last_sent_message_succeeded_for_connection(connection),
        None
    );
    assert_eq!(
        state.last_sent_message_bytes_for_connection(connection),
        None
    );
    assert_eq!(state.last_received_message_count(), 0);
    assert!(state.recent_received_messages().is_empty());
    assert_eq!(state.recent_received_message_count(), 0);
    assert_eq!(state.received_message_count_for_connection(connection), 0);
    assert_eq!(state.last_received_message_for_connection(connection), None);
    assert_eq!(
        state.last_received_message_bytes_for_connection(connection),
        None
    );
    assert_eq!(
        state.last_received_message_channel_for_connection(connection),
        None
    );
    assert_eq!(
        state.last_received_message_data_for_connection(connection),
        None
    );
    assert_eq!(state.last_poll_group_message_count(), 0);
    assert!(state.recent_poll_group_messages().is_empty());
    assert_eq!(state.recent_poll_group_message_count(), 0);
    assert_eq!(state.poll_group_message_count(poll_group), 0);
    assert_eq!(state.last_poll_group_message(poll_group), None);
    assert_eq!(state.last_poll_group_message_bytes(poll_group), None);
    assert_eq!(state.last_poll_group_message_channel(poll_group), None);
    assert_eq!(state.last_poll_group_message_data(poll_group), None);

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
    let root_state = bevy_steamworks::SteamworksNetworkingUtilsState::default();
    assert_eq!(root_state.relay_network_availability(), None);
    assert_eq!(root_state.relay_network_availability_error(), None);
    assert_eq!(root_state.relay_network_available(), None);
    assert_eq!(root_state.relay_network_pending(), None);
    assert_eq!(root_state.relay_network_unavailable(), None);
    assert_eq!(root_state.relay_network_config_available(), None);
    assert_eq!(root_state.relay_network_config_availability(), None);
    assert_eq!(root_state.relay_network_config_availability_error(), None);
    assert_eq!(root_state.relay_network_config_pending(), None);
    assert_eq!(root_state.relay_network_config_unavailable(), None);
    assert_eq!(root_state.any_relay_available(), None);
    assert_eq!(root_state.any_relay_availability(), None);
    assert_eq!(root_state.any_relay_availability_error(), None);
    assert_eq!(root_state.any_relay_pending(), None);
    assert_eq!(root_state.any_relay_unavailable(), None);
    accepts_prelude_state_status(
        bevy_steamworks::prelude::SteamworksNetworkingUtilsState::default(),
        None,
    );
    let prelude_state = bevy_steamworks::prelude::SteamworksNetworkingUtilsState::default();
    assert_eq!(prelude_state.relay_network_availability(), None);
    assert_eq!(prelude_state.relay_network_availability_error(), None);
    assert_eq!(prelude_state.relay_network_available(), None);
    assert_eq!(prelude_state.relay_network_pending(), None);
    assert_eq!(prelude_state.relay_network_unavailable(), None);
    assert_eq!(prelude_state.relay_network_config_available(), None);
    assert_eq!(prelude_state.relay_network_config_availability(), None);
    assert_eq!(
        prelude_state.relay_network_config_availability_error(),
        None
    );
    assert_eq!(prelude_state.relay_network_config_pending(), None);
    assert_eq!(prelude_state.relay_network_config_unavailable(), None);
    assert_eq!(prelude_state.any_relay_available(), None);
    assert_eq!(prelude_state.any_relay_availability(), None);
    assert_eq!(prelude_state.any_relay_availability_error(), None);
    assert_eq!(prelude_state.any_relay_pending(), None);
    assert_eq!(prelude_state.any_relay_unavailable(), None);

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
    let root_state = SteamworksRemoteStorageState::default();
    assert_eq!(root_state.cloud_enabled_for_app(), None);
    assert_eq!(root_state.cloud_enabled_for_account(), None);
    assert_eq!(root_state.cloud_available(), None);
    assert_eq!(
        root_state.file_names().collect::<Vec<_>>(),
        Vec::<&str>::new()
    );
    assert_eq!(root_state.file_size("save.dat"), None);
    assert!(root_state.file_read_requests().is_empty());
    assert_eq!(root_state.file_read_request(0), None);
    assert!(root_state.file_contents().is_empty());
    assert_eq!(root_state.file_contents_by_request(0), None);
    assert_eq!(root_state.file_contents_by_name("save.dat"), None);
    assert_eq!(root_state.file_data("save.dat"), None);
    assert_eq!(root_state.file_data_by_request(0), None);
    assert_eq!(root_state.file_read_bytes("save.dat"), None);
    assert_eq!(root_state.file_read_bytes_by_request(0), None);
    assert!(root_state.file_write_requests().is_empty());
    assert_eq!(root_state.file_write_request(0), None);
    assert!(root_state.file_writes().is_empty());
    assert_eq!(root_state.file_write(0), None);
    assert_eq!(root_state.file_write_bytes(0), None);
    assert_eq!(root_state.file_write_by_name("save.dat"), None);
    assert_eq!(root_state.file_write_bytes_by_name("save.dat"), None);
    assert!(root_state.file_share_requests().is_empty());
    assert_eq!(root_state.file_share_request(0), None);
    assert!(root_state.shared_files().is_empty());
    assert_eq!(root_state.shared_file(0), None);
    assert_eq!(root_state.shared_file_by_name("save.dat"), None);
    assert_eq!(root_state.shared_file_handle("save.dat"), None);
    assert!(!root_state.has_shared_file("save.dat"));
    assert_eq!(root_state.shared_file_handle_raw("save.dat"), None);
    let _read_request = SteamworksRemoteStorageFileReadRequest {
        request_id: 0,
        name: "save.dat".to_owned(),
    };
    let _write_request = SteamworksRemoteStorageFileWriteRequest {
        request_id: 1,
        name: "save.dat".to_owned(),
        bytes: 7,
    };
    let _share_request = SteamworksRemoteStorageFileShareRequest {
        request_id: 2,
        name: "save.dat".to_owned(),
    };
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
    let _shared_file = SteamworksRemoteStorageSharedFile {
        request_id: 4,
        name: "save.dat".to_owned(),
        handle: SteamworksRemoteStorageFileShareHandle::from_raw(11),
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
    let prelude_state = PreludeRemoteStorageState::default();
    assert_eq!(prelude_state.cloud_enabled_for_app(), None);
    assert_eq!(prelude_state.cloud_enabled_for_account(), None);
    assert_eq!(prelude_state.cloud_available(), None);
    assert_eq!(
        prelude_state.file_names().collect::<Vec<_>>(),
        Vec::<&str>::new()
    );
    assert_eq!(prelude_state.file_size("save.dat"), None);
    assert_eq!(prelude_state.file_data_by_request(0), None);
    assert_eq!(prelude_state.file_read_bytes("save.dat"), None);
    assert_eq!(prelude_state.file_read_bytes_by_request(0), None);
    assert_eq!(prelude_state.file_write_bytes(0), None);
    assert_eq!(prelude_state.file_write_bytes_by_name("save.dat"), None);
    assert!(!prelude_state.has_shared_file("save.dat"));
    assert_eq!(prelude_state.shared_file_handle_raw("save.dat"), None);
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
    let prelude_state = PreludeRemoteStorageState::default();
    assert!(prelude_state.file_read_requests().is_empty());
    assert_eq!(prelude_state.file_read_request(0), None);
    assert!(prelude_state.file_contents().is_empty());
    assert_eq!(prelude_state.file_contents_by_request(0), None);
    assert_eq!(prelude_state.file_contents_by_name("save.dat"), None);
    assert_eq!(prelude_state.file_data("save.dat"), None);
    assert!(prelude_state.file_write_requests().is_empty());
    assert_eq!(prelude_state.file_write_request(0), None);
    assert!(prelude_state.file_writes().is_empty());
    assert_eq!(prelude_state.file_write(0), None);
    assert_eq!(prelude_state.file_write_by_name("save.dat"), None);
    assert!(prelude_state.file_share_requests().is_empty());
    assert_eq!(prelude_state.file_share_request(0), None);
    assert!(prelude_state.shared_files().is_empty());
    assert_eq!(prelude_state.shared_file(0), None);
    assert_eq!(prelude_state.shared_file_by_name("save.dat"), None);
    assert_eq!(prelude_state.shared_file_handle("save.dat"), None);
    let _prelude_read_request = PreludeRemoteStorageFileReadRequest {
        request_id: 0,
        name: "save.dat".to_owned(),
    };
    let _prelude_write_request = PreludeRemoteStorageFileWriteRequest {
        request_id: 1,
        name: "save.dat".to_owned(),
        bytes: 7,
    };
    let _prelude_share_request = PreludeRemoteStorageFileShareRequest {
        request_id: 2,
        name: "save.dat".to_owned(),
    };
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
    let _prelude_shared_file = PreludeRemoteStorageSharedFile {
        request_id: 4,
        name: "save.dat".to_owned(),
        handle: PreludeRemoteStorageFileShareHandle::from_raw(11),
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
    let session = steamworks::RemotePlaySessionId::from_raw(1);
    let user = steamworks::SteamId::from_raw(2);
    let _snapshot = SteamworksRemotePlaySessionSnapshot {
        user,
        client_name: Some("Deck".to_owned()),
        client_form_factor: Some(steamworks::SteamDeviceFormFactor::Computer),
        client_resolution: Some((1280, 800)),
    };
    let _info = SteamworksRemotePlaySessionInfo {
        session,
        user,
        client_name: Some("Deck".to_owned()),
        client_form_factor: Some(steamworks::SteamDeviceFormFactor::Computer),
        client_resolution: Some((1280, 800)),
    };
    let state = SteamworksRemotePlayState::default();
    assert!(state.sessions().is_empty());
    assert_eq!(state.session_count(), 0);
    assert_eq!(state.sessions_for_user(user).count(), 0);
    assert_eq!(state.latest_session_for_user(user), None);
    assert!(state.known_sessions().is_empty());
    assert_eq!(state.known_session_count(), 0);
    assert_eq!(state.known_session(session), None);
    assert!(!state.has_known_session(session));
    assert_eq!(state.known_sessions_for_user(user).count(), 0);
    assert_eq!(state.latest_known_session_for_user(user), None);
    assert_eq!(state.session_user(session), None);
    assert_eq!(state.session_client_name(session), None);
    assert_eq!(state.session_client_form_factor(session), None);
    assert_eq!(state.session_client_resolution(session), None);
    assert_eq!(state.observed_connected_session_count(), 0);

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
    let _prelude_snapshot = PreludeRemotePlaySessionSnapshot {
        user,
        client_name: Some("Deck".to_owned()),
        client_form_factor: Some(steamworks::SteamDeviceFormFactor::Computer),
        client_resolution: Some((1280, 800)),
    };
    let _prelude_info = PreludeRemotePlaySessionInfo {
        session,
        user,
        client_name: Some("Deck".to_owned()),
        client_form_factor: Some(steamworks::SteamDeviceFormFactor::Computer),
        client_resolution: Some((1280, 800)),
    };
    let state = PreludeRemotePlayState::default();
    assert!(state.sessions().is_empty());
    assert_eq!(state.session_count(), 0);
    assert_eq!(state.sessions_for_user(user).count(), 0);
    assert_eq!(state.latest_session_for_user(user), None);
    assert!(state.known_sessions().is_empty());
    assert_eq!(state.known_session_count(), 0);
    assert_eq!(state.known_session(session), None);
    assert!(!state.has_known_session(session));
    assert_eq!(state.known_sessions_for_user(user).count(), 0);
    assert_eq!(state.latest_known_session_for_user(user), None);
    assert_eq!(state.session_user(session), None);
    assert_eq!(state.session_client_name(session), None);
    assert_eq!(state.session_client_form_factor(session), None);
    assert_eq!(state.session_client_resolution(session), None);
    assert_eq!(state.observed_connected_session_count(), 0);

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
    let handle = 11;
    let _ready = SteamworksScreenshotReady {
        local_handle: Ok(handle),
    };
    let _ready_error = SteamworksScreenshotReadyError::IoFailure;
    let _submission = SteamworksSubmittedScreenshot {
        handle,
        filename: std::path::PathBuf::from("shot.png"),
        thumbnail_filename: Some(std::path::PathBuf::from("thumb.png")),
        width: 1920,
        height: 1080,
    };
    let state = SteamworksScreenshotsState::default();
    assert!(state.submitted_screenshots().is_empty());
    assert_eq!(state.submitted_screenshot(handle), None);
    assert_eq!(state.submitted_screenshot_by_filename("shot.png"), None);
    assert_eq!(state.submitted_screenshot_dimensions(handle), None);
    assert_eq!(state.submitted_screenshot_thumbnail(handle), None);
    assert_eq!(state.screenshot_ready_success_count(), 0);
    assert_eq!(state.screenshot_ready_error_count(), 0);
    assert_eq!(state.last_screenshot_ready_error(), None);

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
    let _prelude_ready = PreludeScreenshotReady {
        local_handle: Ok(handle),
    };
    let _prelude_ready_error = PreludeScreenshotReadyError::IoFailure;
    let _prelude_submission = PreludeSubmittedScreenshot {
        handle,
        filename: std::path::PathBuf::from("shot.png"),
        thumbnail_filename: Some(std::path::PathBuf::from("thumb.png")),
        width: 1920,
        height: 1080,
    };
    let state = PreludeScreenshotsState::default();
    assert!(state.submitted_screenshots().is_empty());
    assert_eq!(state.submitted_screenshot(handle), None);
    assert_eq!(state.submitted_screenshot_by_filename("shot.png"), None);
    assert_eq!(state.submitted_screenshot_dimensions(handle), None);
    assert_eq!(state.submitted_screenshot_thumbnail(handle), None);
    assert_eq!(state.screenshot_ready_success_count(), 0);
    assert_eq!(state.screenshot_ready_error_count(), 0);
    assert_eq!(state.last_screenshot_ready_error(), None);

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
    let root_state = SteamworksTimelineState::default();
    assert!(!root_state.has_state_description());
    assert_eq!(root_state.state_description_text(), None);
    assert_eq!(root_state.state_description_duration(), None);
    assert!(root_state.events().is_empty());
    assert!(!root_state.has_events());
    assert_eq!(root_state.cached_event_count(), 0);
    assert_eq!(root_state.events_with_icon("boss").count(), 0);
    assert_eq!(root_state.last_event(), None);
    assert_eq!(root_state.last_event_with_icon("boss"), None);
    assert_eq!(
        root_state
            .events_with_clip_priority(SteamworksTimelineEventClipPriority::Featured)
            .count(),
        0
    );
    assert_eq!(
        root_state.last_event_with_clip_priority(SteamworksTimelineEventClipPriority::Featured),
        None
    );
    assert_eq!(root_state.last_event_icon(), None);
    assert_eq!(root_state.last_event_title(), None);
    assert_eq!(root_state.last_event_description(), None);
    assert_eq!(root_state.last_event_priority(), None);
    assert_eq!(root_state.last_event_start_offset_seconds(), None);
    assert_eq!(root_state.last_event_duration(), None);
    assert_eq!(root_state.last_event_clip_priority(), None);
    assert_eq!(root_state.event_count(), 0);

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
    let prelude_state = PreludeTimelineState::default();
    assert!(!prelude_state.has_state_description());
    assert_eq!(prelude_state.state_description_text(), None);
    assert_eq!(prelude_state.state_description_duration(), None);
    assert!(prelude_state.events().is_empty());
    assert!(!prelude_state.has_events());
    assert_eq!(prelude_state.cached_event_count(), 0);
    assert_eq!(prelude_state.events_with_icon("boss").count(), 0);
    assert_eq!(prelude_state.last_event(), None);
    assert_eq!(prelude_state.last_event_with_icon("boss"), None);
    assert_eq!(
        prelude_state
            .events_with_clip_priority(PreludeTimelineEventClipPriority::Featured)
            .count(),
        0
    );
    assert_eq!(
        prelude_state.last_event_with_clip_priority(PreludeTimelineEventClipPriority::Featured),
        None
    );
    assert_eq!(prelude_state.last_event_icon(), None);
    assert_eq!(prelude_state.last_event_title(), None);
    assert_eq!(prelude_state.last_event_description(), None);
    assert_eq!(prelude_state.last_event_priority(), None);
    assert_eq!(prelude_state.last_event_start_offset_seconds(), None);
    assert_eq!(prelude_state.last_event_duration(), None);
    assert_eq!(prelude_state.last_event_clip_priority(), None);
    assert_eq!(prelude_state.event_count(), 0);
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
    let user = steamworks::SteamId::from_raw(1);
    let app_id = steamworks::AppId(480);
    let root_state = SteamworksUserState::default();
    assert_eq!(root_state.steam_id(), None);
    assert_eq!(root_state.level(), None);
    assert_eq!(root_state.logged_on(), None);
    assert!(root_state.auth_session_tickets().is_empty());
    assert!(root_state.auth_session_tickets_for_identity().is_empty());
    assert!(root_state.web_api_ticket_requests().is_empty());
    assert_eq!(root_state.active_auth_ticket_count(), 0);
    assert!(!root_state.is_user_authenticated(user));
    assert!(root_state.user_licenses_for_apps().is_empty());
    assert_eq!(root_state.user_license_for_app(user, app_id), None);
    assert_eq!(root_state.user_license(user, app_id), None);
    assert_eq!(root_state.user_has_license_for_app(user, app_id), None);
    assert!(root_state.auth_ticket_responses().is_empty());
    assert!(root_state.web_api_ticket_responses().is_empty());
    assert!(root_state.auth_ticket_validations().is_empty());
    assert_eq!(root_state.auth_ticket_validation(user), None);
    assert_eq!(root_state.auth_ticket_validation_succeeded(user), None);
    assert!(root_state.steam_server_connection_events().is_empty());
    assert!(root_state.micro_txn_authorization_responses().is_empty());
    assert_eq!(root_state.micro_txn_authorization_response(app_id, 1), None);
    assert_eq!(root_state.micro_txn_authorized(app_id, 1), None);

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
    let prelude_state = PreludeUserState::default();
    assert_eq!(prelude_state.steam_id(), None);
    assert_eq!(prelude_state.level(), None);
    assert_eq!(prelude_state.logged_on(), None);
    assert!(prelude_state.auth_session_tickets().is_empty());
    assert!(prelude_state.auth_session_tickets_for_identity().is_empty());
    assert!(prelude_state.web_api_ticket_requests().is_empty());
    assert_eq!(prelude_state.active_auth_ticket_count(), 0);
    assert!(!prelude_state.is_user_authenticated(user));
    assert!(prelude_state.user_licenses_for_apps().is_empty());
    assert_eq!(prelude_state.user_license_for_app(user, app_id), None);
    assert_eq!(prelude_state.user_license(user, app_id), None);
    assert_eq!(prelude_state.user_has_license_for_app(user, app_id), None);
    assert!(prelude_state.auth_ticket_responses().is_empty());
    assert!(prelude_state.web_api_ticket_responses().is_empty());
    assert!(prelude_state.auth_ticket_validations().is_empty());
    assert_eq!(prelude_state.auth_ticket_validation(user), None);
    assert_eq!(prelude_state.auth_ticket_validation_succeeded(user), None);
    assert!(prelude_state.steam_server_connection_events().is_empty());
    assert!(prelude_state.micro_txn_authorization_responses().is_empty());
    assert_eq!(
        prelude_state.micro_txn_authorization_response(app_id, 1),
        None
    );
    assert_eq!(prelude_state.micro_txn_authorized(app_id, 1), None);

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
    assert_eq!(root_state.stat_i32_values().count(), 0);
    assert_eq!(root_state.stat_i32_count(), 0);
    assert_eq!(root_state.stat_i32("kills"), None);
    assert!(!root_state.has_stat_i32("kills"));
    assert_eq!(root_state.stat_f32_values().count(), 0);
    assert_eq!(root_state.stat_f32_count(), 0);
    assert_eq!(root_state.stat_f32("accuracy"), None);
    assert!(!root_state.has_stat_f32("accuracy"));
    assert!(root_state.achievements().is_empty());
    assert_eq!(root_state.known_achievement_count(), 0);
    assert_eq!(root_state.achievement_names().count(), 0);
    assert!(!root_state.has_achievement("ACH_WIN"));
    assert_eq!(root_state.unlocked_achievements().count(), 0);
    assert_eq!(root_state.locked_achievements().count(), 0);
    assert_eq!(root_state.achievement_display_name("ACH_WIN"), None);
    assert_eq!(root_state.achievement_description("ACH_WIN"), None);
    assert_eq!(root_state.achievement_hidden("ACH_WIN"), None);
    assert!(root_state.global_achievement_percentages().is_empty());
    assert_eq!(root_state.global_achievement_percentage_count(), 0);
    let root_leaderboard: bevy_steamworks::SteamworksLeaderboardId =
        bevy_steamworks::SteamworksLeaderboardId::from_raw(1);
    assert_eq!(root_state.leaderboard_id("daily"), None);
    assert!(!root_state.has_leaderboard_id("daily"));
    assert_eq!(root_state.leaderboard_info(root_leaderboard), None);
    assert!(!root_state.has_leaderboard_info(root_leaderboard));
    assert_eq!(root_state.leaderboard_info_by_name("daily"), None);
    assert_eq!(root_state.leaderboard_name(root_leaderboard), None);
    assert_eq!(root_state.leaderboard_display_type(root_leaderboard), None);
    assert_eq!(root_state.leaderboard_sort_method(root_leaderboard), None);
    assert_eq!(root_state.leaderboard_entry_count(root_leaderboard), None);
    assert!(root_state.leaderboards().is_empty());
    assert!(root_state.global_stat_i64_values().is_empty());
    assert_eq!(root_state.global_stat_i64("kills"), None);
    assert!(root_state.global_stat_f64_values().is_empty());
    assert_eq!(root_state.global_stat_f64("accuracy"), None);
    assert!(root_state.global_stat_history_i64_values().is_empty());
    assert_eq!(root_state.global_stat_history_i64("daily_kills"), None);
    assert_eq!(
        root_state.global_stat_history_i64_series("daily_kills"),
        None
    );
    assert!(root_state.global_stat_history_f64_values().is_empty());
    assert_eq!(root_state.global_stat_history_f64("daily_accuracy"), None);
    assert_eq!(
        root_state.global_stat_history_f64_series("daily_accuracy"),
        None
    );
    assert!(root_state.leaderboard_score_upload_requests().is_empty());
    assert_eq!(
        root_state.leaderboard_score_upload_request(root_leaderboard),
        None
    );
    assert!(root_state.leaderboard_score_upload_results().is_empty());
    assert_eq!(
        root_state.leaderboard_score_upload_result(root_leaderboard),
        None
    );
    assert_eq!(root_state.leaderboard_score_upload(root_leaderboard), None);
    assert_eq!(
        root_state.leaderboard_uploaded_score(root_leaderboard),
        None
    );
    assert_eq!(
        root_state.leaderboard_score_was_changed(root_leaderboard),
        None
    );
    assert_eq!(
        root_state.leaderboard_uploaded_rank_new(root_leaderboard),
        None
    );
    assert_eq!(
        root_state.leaderboard_uploaded_rank_previous(root_leaderboard),
        None
    );
    assert!(root_state
        .leaderboard_entries_download_requests()
        .is_empty());
    assert_eq!(
        root_state.leaderboard_entries_download_request(root_leaderboard),
        None
    );
    assert!(root_state.leaderboard_entries_download_results().is_empty());
    assert_eq!(
        root_state.leaderboard_entries_download_result(root_leaderboard),
        None
    );
    assert_eq!(root_state.leaderboard_entries(root_leaderboard), None);
    assert_eq!(
        root_state.leaderboard_downloaded_entry_count(root_leaderboard),
        None
    );
    assert_eq!(root_state.last_leaderboard_downloaded_entry_count(), 0);
    let root_user = steamworks::SteamId::from_raw(1);
    assert_eq!(
        root_state.leaderboard_entry_by_user(root_leaderboard, root_user),
        None
    );
    assert_eq!(
        root_state.leaderboard_has_entry_for_user(root_leaderboard, root_user),
        None
    );
    assert_eq!(
        root_state.leaderboard_entry_by_rank(root_leaderboard, 1),
        None
    );
    assert_eq!(root_state.leaderboard_has_rank(root_leaderboard, 1), None);
    assert_eq!(
        root_state.leaderboard_score_by_user(root_leaderboard, root_user),
        None
    );
    assert_eq!(
        root_state.leaderboard_rank_by_user(root_leaderboard, root_user),
        None
    );
    assert_eq!(
        root_state.leaderboard_entry_details(root_leaderboard, root_user),
        None
    );
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
    assert_eq!(prelude_state.stat_i32_values().count(), 0);
    assert_eq!(prelude_state.stat_i32_count(), 0);
    assert_eq!(prelude_state.stat_i32("kills"), None);
    assert!(!prelude_state.has_stat_i32("kills"));
    assert_eq!(prelude_state.stat_f32_values().count(), 0);
    assert_eq!(prelude_state.stat_f32_count(), 0);
    assert_eq!(prelude_state.stat_f32("accuracy"), None);
    assert!(!prelude_state.has_stat_f32("accuracy"));
    assert!(prelude_state.achievements().is_empty());
    assert_eq!(prelude_state.known_achievement_count(), 0);
    assert_eq!(prelude_state.achievement_names().count(), 0);
    assert!(!prelude_state.has_achievement("ACH_WIN"));
    assert_eq!(prelude_state.unlocked_achievements().count(), 0);
    assert_eq!(prelude_state.locked_achievements().count(), 0);
    assert_eq!(prelude_state.achievement_display_name("ACH_WIN"), None);
    assert_eq!(prelude_state.achievement_description("ACH_WIN"), None);
    assert_eq!(prelude_state.achievement_hidden("ACH_WIN"), None);
    assert!(prelude_state.global_achievement_percentages().is_empty());
    assert_eq!(prelude_state.global_achievement_percentage_count(), 0);
    let prelude_leaderboard: bevy_steamworks::prelude::SteamworksLeaderboardId =
        bevy_steamworks::prelude::SteamworksLeaderboardId::from_raw(1);
    assert_eq!(prelude_state.leaderboard_id("daily"), None);
    assert!(!prelude_state.has_leaderboard_id("daily"));
    assert_eq!(prelude_state.leaderboard_info(prelude_leaderboard), None);
    assert!(!prelude_state.has_leaderboard_info(prelude_leaderboard));
    assert_eq!(prelude_state.leaderboard_info_by_name("daily"), None);
    assert_eq!(prelude_state.leaderboard_name(prelude_leaderboard), None);
    assert_eq!(
        prelude_state.leaderboard_display_type(prelude_leaderboard),
        None
    );
    assert_eq!(
        prelude_state.leaderboard_sort_method(prelude_leaderboard),
        None
    );
    assert_eq!(
        prelude_state.leaderboard_entry_count(prelude_leaderboard),
        None
    );
    assert!(prelude_state.leaderboards().is_empty());
    assert!(prelude_state.global_stat_i64_values().is_empty());
    assert_eq!(prelude_state.global_stat_i64("kills"), None);
    assert!(prelude_state.global_stat_f64_values().is_empty());
    assert_eq!(prelude_state.global_stat_f64("accuracy"), None);
    assert!(prelude_state.global_stat_history_i64_values().is_empty());
    assert_eq!(prelude_state.global_stat_history_i64("daily_kills"), None);
    assert_eq!(
        prelude_state.global_stat_history_i64_series("daily_kills"),
        None
    );
    assert!(prelude_state.global_stat_history_f64_values().is_empty());
    assert_eq!(
        prelude_state.global_stat_history_f64("daily_accuracy"),
        None
    );
    assert_eq!(
        prelude_state.global_stat_history_f64_series("daily_accuracy"),
        None
    );
    assert!(prelude_state.leaderboard_score_upload_requests().is_empty());
    assert_eq!(
        prelude_state.leaderboard_score_upload_request(prelude_leaderboard),
        None
    );
    assert!(prelude_state.leaderboard_score_upload_results().is_empty());
    assert_eq!(
        prelude_state.leaderboard_score_upload_result(prelude_leaderboard),
        None
    );
    assert_eq!(
        prelude_state.leaderboard_score_upload(prelude_leaderboard),
        None
    );
    assert_eq!(
        prelude_state.leaderboard_uploaded_score(prelude_leaderboard),
        None
    );
    assert_eq!(
        prelude_state.leaderboard_score_was_changed(prelude_leaderboard),
        None
    );
    assert_eq!(
        prelude_state.leaderboard_uploaded_rank_new(prelude_leaderboard),
        None
    );
    assert_eq!(
        prelude_state.leaderboard_uploaded_rank_previous(prelude_leaderboard),
        None
    );
    assert!(prelude_state
        .leaderboard_entries_download_requests()
        .is_empty());
    assert_eq!(
        prelude_state.leaderboard_entries_download_request(prelude_leaderboard),
        None
    );
    assert!(prelude_state
        .leaderboard_entries_download_results()
        .is_empty());
    assert_eq!(
        prelude_state.leaderboard_entries_download_result(prelude_leaderboard),
        None
    );
    assert_eq!(prelude_state.leaderboard_entries(prelude_leaderboard), None);
    assert_eq!(
        prelude_state.leaderboard_downloaded_entry_count(prelude_leaderboard),
        None
    );
    assert_eq!(prelude_state.last_leaderboard_downloaded_entry_count(), 0);
    let prelude_user = steamworks::SteamId::from_raw(1);
    assert_eq!(
        prelude_state.leaderboard_entry_by_user(prelude_leaderboard, prelude_user),
        None
    );
    assert_eq!(
        prelude_state.leaderboard_has_entry_for_user(prelude_leaderboard, prelude_user),
        None
    );
    assert_eq!(
        prelude_state.leaderboard_entry_by_rank(prelude_leaderboard, 1),
        None
    );
    assert_eq!(
        prelude_state.leaderboard_has_rank(prelude_leaderboard, 1),
        None
    );
    assert_eq!(
        prelude_state.leaderboard_score_by_user(prelude_leaderboard, prelude_user),
        None
    );
    assert_eq!(
        prelude_state.leaderboard_rank_by_user(prelude_leaderboard, prelude_user),
        None
    );
    assert_eq!(
        prelude_state.leaderboard_entry_details(prelude_leaderboard, prelude_user),
        None
    );
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
            statistics: vec![SteamworksUgcStatistic {
                statistic: steamworks::UGCStatisticType::Subscriptions,
                value: 7,
            }],
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
            statistics: vec![PreludeUgcStatistic {
                statistic: steamworks::UGCStatisticType::Subscriptions,
                value: 7,
            }],
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
    assert_eq!(root_state.subscribed_item_count(), 0);
    assert!(!root_state.is_item_subscribed(item));
    assert!(root_state.item_details().is_empty());
    assert_eq!(root_state.item_detail(item), None);
    assert_eq!(root_state.item_creator_app_id(item), None);
    assert_eq!(root_state.item_consumer_app_id(item), None);
    assert_eq!(root_state.item_title(item), None);
    assert_eq!(root_state.item_description(item), None);
    assert_eq!(root_state.item_tags(item), None);
    assert_eq!(root_state.item_preview_url(item), None);
    assert_eq!(root_state.item_content_descriptors(item), None);
    assert_eq!(root_state.item_statistics(item), None);
    assert_eq!(
        root_state.item_statistic(item, steamworks::UGCStatisticType::Subscriptions),
        None
    );
    assert_eq!(root_state.item_metadata(item), None);
    assert_eq!(root_state.item_children(item), None);
    assert_eq!(root_state.item_key_value_tags(item), None);
    assert_eq!(root_state.item_key_value_tag(item, "mode"), None);
    assert_eq!(root_state.item_state(item), None);
    assert_eq!(root_state.item_state_flags(item), None);
    assert_eq!(
        root_state.item_state_contains(item, steamworks::ItemState::SUBSCRIBED),
        None
    );
    assert_eq!(root_state.item_state_subscribed(item), None);
    assert_eq!(root_state.item_state_installed(item), None);
    assert_eq!(root_state.item_state_needs_update(item), None);
    assert_eq!(root_state.item_state_downloading(item), None);
    assert_eq!(root_state.item_state_download_pending(item), None);
    assert_eq!(root_state.item_download_info(item), None);
    assert_eq!(root_state.item_download_info_available(item), None);
    assert_eq!(root_state.item_downloaded_bytes(item), None);
    assert_eq!(root_state.item_download_total_bytes(item), None);
    assert_eq!(root_state.item_download_progress(item), None);
    assert_eq!(root_state.item_download_complete(item), None);
    assert_eq!(root_state.item_install_info(item), None);
    assert_eq!(root_state.item_install_info_available(item), None);
    assert_eq!(root_state.item_install_folder(item), None);
    assert_eq!(root_state.item_size_on_disk(item), None);
    assert_eq!(root_state.item_install_timestamp(item), None);
    assert!(root_state.download_item_results().is_empty());
    assert_eq!(root_state.download_item_result(item), None);
    assert_eq!(root_state.download_item_failed(item), None);
    assert!(root_state.query_requests().is_empty());
    assert_eq!(root_state.query_request(0), None);
    assert!(root_state.query_results().is_empty());
    assert_eq!(root_state.query_result(0), None);
    assert_eq!(root_state.query_result_items(0), None);
    assert_eq!(root_state.query_result_item_count(0), None);
    assert_eq!(root_state.query_result_total_count(0), None);
    assert_eq!(root_state.query_result_was_cached(0), None);
    assert_eq!(root_state.last_query_item_count(), None);
    assert_eq!(root_state.last_query_total_count(), None);
    assert_eq!(root_state.last_query_was_cached(), None);
    assert!(root_state.query_total_results().is_empty());
    assert_eq!(root_state.query_total_result(0), None);
    assert_eq!(root_state.query_total_count(0), None);
    assert!(root_state.query_ids_results().is_empty());
    assert_eq!(root_state.query_ids_result(0), None);
    assert_eq!(root_state.query_ids_items(0), None);
    assert_eq!(root_state.query_ids_item_count(0), None);
    assert_eq!(root_state.last_query_ids_count(), None);
    let _query_request = SteamworksUgcQueryRequest {
        request_id: 0,
        query: query.clone(),
    };
    let _query_result = SteamworksUgcQueryResult {
        request_id: 0,
        query: query.clone(),
        results: SteamworksUgcQueryResults {
            was_cached: false,
            total_results: 1,
            returned_results: 1,
            items: vec![root_item_detail(item)],
        },
    };
    let _query_total_result = SteamworksUgcQueryTotalResult {
        request_id: 1,
        query: query.clone(),
        total: SteamworksUgcQueryTotal { total_results: 1 },
    };
    let _query_ids_result = SteamworksUgcQueryIdsResult {
        request_id: 2,
        query: query.clone(),
        ids: SteamworksUgcQueryIds { items: vec![item] },
    };
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
    assert_eq!(prelude_state.subscribed_item_count(), 0);
    assert!(!prelude_state.is_item_subscribed(item));
    assert!(prelude_state.item_details().is_empty());
    assert_eq!(prelude_state.item_detail(item), None);
    assert_eq!(prelude_state.item_creator_app_id(item), None);
    assert_eq!(prelude_state.item_consumer_app_id(item), None);
    assert_eq!(prelude_state.item_title(item), None);
    assert_eq!(prelude_state.item_description(item), None);
    assert_eq!(prelude_state.item_tags(item), None);
    assert_eq!(prelude_state.item_preview_url(item), None);
    assert_eq!(prelude_state.item_content_descriptors(item), None);
    assert_eq!(prelude_state.item_statistics(item), None);
    assert_eq!(
        prelude_state.item_statistic(item, steamworks::UGCStatisticType::Subscriptions),
        None
    );
    assert_eq!(prelude_state.item_metadata(item), None);
    assert_eq!(prelude_state.item_children(item), None);
    assert_eq!(prelude_state.item_key_value_tags(item), None);
    assert_eq!(prelude_state.item_key_value_tag(item, "mode"), None);
    assert_eq!(prelude_state.item_state(item), None);
    assert_eq!(prelude_state.item_state_flags(item), None);
    assert_eq!(
        prelude_state.item_state_contains(item, steamworks::ItemState::SUBSCRIBED),
        None
    );
    assert_eq!(prelude_state.item_state_subscribed(item), None);
    assert_eq!(prelude_state.item_state_installed(item), None);
    assert_eq!(prelude_state.item_state_needs_update(item), None);
    assert_eq!(prelude_state.item_state_downloading(item), None);
    assert_eq!(prelude_state.item_state_download_pending(item), None);
    assert_eq!(prelude_state.item_download_info(item), None);
    assert_eq!(prelude_state.item_download_info_available(item), None);
    assert_eq!(prelude_state.item_downloaded_bytes(item), None);
    assert_eq!(prelude_state.item_download_total_bytes(item), None);
    assert_eq!(prelude_state.item_download_progress(item), None);
    assert_eq!(prelude_state.item_download_complete(item), None);
    assert_eq!(prelude_state.item_install_info(item), None);
    assert_eq!(prelude_state.item_install_info_available(item), None);
    assert_eq!(prelude_state.item_install_folder(item), None);
    assert_eq!(prelude_state.item_size_on_disk(item), None);
    assert_eq!(prelude_state.item_install_timestamp(item), None);
    assert!(prelude_state.download_item_results().is_empty());
    assert_eq!(prelude_state.download_item_result(item), None);
    assert_eq!(prelude_state.download_item_failed(item), None);
    assert!(prelude_state.query_requests().is_empty());
    assert_eq!(prelude_state.query_request(0), None);
    assert!(prelude_state.query_results().is_empty());
    assert_eq!(prelude_state.query_result(0), None);
    assert_eq!(prelude_state.query_result_items(0), None);
    assert_eq!(prelude_state.query_result_item_count(0), None);
    assert_eq!(prelude_state.query_result_total_count(0), None);
    assert_eq!(prelude_state.query_result_was_cached(0), None);
    assert_eq!(prelude_state.last_query_item_count(), None);
    assert_eq!(prelude_state.last_query_total_count(), None);
    assert_eq!(prelude_state.last_query_was_cached(), None);
    assert!(prelude_state.query_total_results().is_empty());
    assert_eq!(prelude_state.query_total_result(0), None);
    assert_eq!(prelude_state.query_total_count(0), None);
    assert!(prelude_state.query_ids_results().is_empty());
    assert_eq!(prelude_state.query_ids_result(0), None);
    assert_eq!(prelude_state.query_ids_items(0), None);
    assert_eq!(prelude_state.query_ids_item_count(0), None);
    assert_eq!(prelude_state.last_query_ids_count(), None);
    let _prelude_query_request = PreludeUgcQueryRequest {
        request_id: 0,
        query: prelude_query.clone(),
    };
    let _prelude_query_result = PreludeUgcQueryResult {
        request_id: 0,
        query: prelude_query.clone(),
        results: bevy_steamworks::prelude::SteamworksUgcQueryResults {
            was_cached: false,
            total_results: 1,
            returned_results: 1,
            items: vec![prelude_item_detail(item)],
        },
    };
    let _prelude_query_total_result = PreludeUgcQueryTotalResult {
        request_id: 1,
        query: prelude_query.clone(),
        total: PreludeUgcQueryTotal { total_results: 1 },
    };
    let _prelude_query_ids_result = PreludeUgcQueryIdsResult {
        request_id: 2,
        query: prelude_query.clone(),
        ids: PreludeUgcQueryIds { items: vec![item] },
    };
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
    let root_state = SteamworksUtilsState::default();
    assert_eq!(root_state.last_gamepad_text_input_request(), None);
    assert_eq!(root_state.gamepad_text_input_shown(), None);
    assert_eq!(root_state.last_floating_gamepad_text_input_request(), None);
    assert_eq!(root_state.floating_gamepad_text_input_shown(), None);
    assert_eq!(root_state.last_submitted_gamepad_text(), None);
    assert_eq!(root_state.last_submitted_gamepad_text_len(), None);
    assert_eq!(root_state.gamepad_text_input_was_submitted(), None);
    assert_eq!(root_state.last_dismissed_gamepad_text(), None);
    assert_eq!(root_state.last_dismissed_gamepad_text_len(), None);

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
    let prelude_state = PreludeUtilsState::default();
    assert_eq!(prelude_state.last_gamepad_text_input_request(), None);
    assert_eq!(prelude_state.gamepad_text_input_shown(), None);
    assert_eq!(
        prelude_state.last_floating_gamepad_text_input_request(),
        None
    );
    assert_eq!(prelude_state.floating_gamepad_text_input_shown(), None);
    assert_eq!(prelude_state.last_submitted_gamepad_text(), None);
    assert_eq!(prelude_state.last_submitted_gamepad_text_len(), None);
    assert_eq!(prelude_state.gamepad_text_input_was_submitted(), None);
    assert_eq!(prelude_state.last_dismissed_gamepad_text(), None);
    assert_eq!(prelude_state.last_dismissed_gamepad_text_len(), None);
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
    let state = bevy_steamworks::SteamworksMatchmakingServersState::default();
    assert_eq!(state.server_query_target(query), None);
    assert_eq!(state.server_query_kind(query), None);
    assert_eq!(state.server_ping_target(query), None);
    assert_eq!(state.server_ping_server(query), None);
    assert_eq!(state.server_ping_latency(query), None);
    assert_eq!(state.server_ping_server_name(query), None);
    assert_eq!(state.server_ping_map(query), None);
    assert_eq!(state.server_ping_player_count(query), None);
    assert_eq!(state.server_ping_max_players(query), None);
    assert_eq!(state.server_player_details_target(query), None);
    assert_eq!(state.server_players(query), None);
    assert_eq!(state.server_player_count(query), None);
    assert_eq!(state.last_server_player_count(), None);
    assert_eq!(state.server_player(query, "Ada"), None);
    assert_eq!(state.server_has_player(query, "Ada"), None);
    assert_eq!(state.server_player_score(query, "Ada"), None);
    assert_eq!(state.server_player_time_played(query, "Ada"), None);
    assert_eq!(state.server_rules_target(query), None);
    assert_eq!(state.server_rule_entries(query), None);
    assert_eq!(state.server_rule_count(query), None);
    assert_eq!(state.last_server_rule_count(), None);
    assert_eq!(state.server_rule(query, "map"), None);
    assert_eq!(state.server_has_rule(query, "map"), None);
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
    let state = bevy_steamworks::prelude::SteamworksMatchmakingServersState::default();
    assert_eq!(state.server_query_target(query), None);
    assert_eq!(state.server_query_kind(query), None);
    assert_eq!(state.server_ping_target(query), None);
    assert_eq!(state.server_ping_server(query), None);
    assert_eq!(state.server_ping_latency(query), None);
    assert_eq!(state.server_ping_server_name(query), None);
    assert_eq!(state.server_ping_map(query), None);
    assert_eq!(state.server_ping_player_count(query), None);
    assert_eq!(state.server_ping_max_players(query), None);
    assert_eq!(state.server_player_details_target(query), None);
    assert_eq!(state.server_players(query), None);
    assert_eq!(state.server_player_count(query), None);
    assert_eq!(state.last_server_player_count(), None);
    assert_eq!(state.server_player(query, "Ada"), None);
    assert_eq!(state.server_has_player(query, "Ada"), None);
    assert_eq!(state.server_player_score(query, "Ada"), None);
    assert_eq!(state.server_player_time_played(query, "Ada"), None);
    assert_eq!(state.server_rules_target(query), None);
    assert_eq!(state.server_rule_entries(query), None);
    assert_eq!(state.server_rule_count(query), None);
    assert_eq!(state.last_server_rule_count(), None);
    assert_eq!(state.server_rule(query, "map"), None);
    assert_eq!(state.server_has_rule(query, "map"), None);
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
