use bevy_steamworks::{
    prelude::{
        SteamworksAppsCommand as PreludeAppsCommand, SteamworksAppsError as PreludeAppsError,
        SteamworksAppsOperation as PreludeAppsOperation, SteamworksAppsPlugin as PreludeAppsPlugin,
        SteamworksAppsResult as PreludeAppsResult,
        SteamworksFriendsCommand as PreludeFriendsCommand,
        SteamworksFriendsError as PreludeFriendsError,
        SteamworksFriendsOperation as PreludeFriendsOperation,
        SteamworksFriendsPlugin as PreludeFriendsPlugin,
        SteamworksFriendsResult as PreludeFriendsResult,
        SteamworksInputCommand as PreludeInputCommand, SteamworksInputError as PreludeInputError,
        SteamworksInputOperation as PreludeInputOperation,
        SteamworksInputPlugin as PreludeInputPlugin, SteamworksInputResult as PreludeInputResult,
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
        SteamworksNotificationPosition as PreludeNotificationPosition,
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
        SteamworksServerListFilters as PreludeServerListFilters,
        SteamworksServerListKind as PreludeServerListKind,
        SteamworksServerListRequestId as PreludeServerListRequestId,
        SteamworksTimelineCommand as PreludeTimelineCommand,
        SteamworksTimelineError as PreludeTimelineError,
        SteamworksTimelineGameMode as PreludeTimelineGameMode,
        SteamworksTimelineOperation as PreludeTimelineOperation,
        SteamworksTimelinePlugin as PreludeTimelinePlugin,
        SteamworksTimelineResult as PreludeTimelineResult,
        SteamworksUserCommand as PreludeUserCommand, SteamworksUserError as PreludeUserError,
        SteamworksUserOperation as PreludeUserOperation, SteamworksUserPlugin as PreludeUserPlugin,
        SteamworksUserResult as PreludeUserResult, SteamworksUtilsCommand as PreludeUtilsCommand,
        SteamworksUtilsError as PreludeUtilsError,
        SteamworksUtilsOperation as PreludeUtilsOperation,
        SteamworksUtilsPlugin as PreludeUtilsPlugin, SteamworksUtilsResult as PreludeUtilsResult,
    },
    SteamworksAppsCommand, SteamworksAppsError, SteamworksAppsOperation, SteamworksAppsPlugin,
    SteamworksAppsResult, SteamworksFriendsCommand, SteamworksFriendsError,
    SteamworksFriendsOperation, SteamworksFriendsPlugin, SteamworksFriendsResult,
    SteamworksInputCommand, SteamworksInputError, SteamworksInputOperation, SteamworksInputPlugin,
    SteamworksInputResult, SteamworksMatchmakingServersCommand, SteamworksMatchmakingServersError,
    SteamworksMatchmakingServersOperation, SteamworksMatchmakingServersPlugin,
    SteamworksMatchmakingServersResult, SteamworksNetworkingCommand, SteamworksNetworkingError,
    SteamworksNetworkingMessagesCommand, SteamworksNetworkingMessagesError,
    SteamworksNetworkingMessagesOperation, SteamworksNetworkingMessagesPlugin,
    SteamworksNetworkingMessagesResult, SteamworksNetworkingOperation, SteamworksNetworkingPlugin,
    SteamworksNetworkingResult, SteamworksNotificationPosition, SteamworksRemotePlayCommand,
    SteamworksRemotePlayError, SteamworksRemotePlayOperation, SteamworksRemotePlayPlugin,
    SteamworksRemotePlayResult, SteamworksRemoteStorageCommand, SteamworksRemoteStorageError,
    SteamworksRemoteStorageOperation, SteamworksRemoteStoragePlugin, SteamworksRemoteStorageResult,
    SteamworksScreenshotsCommand, SteamworksScreenshotsError, SteamworksScreenshotsOperation,
    SteamworksScreenshotsPlugin, SteamworksScreenshotsResult, SteamworksServerListFilters,
    SteamworksServerListKind, SteamworksServerListRequestId, SteamworksTimelineCommand,
    SteamworksTimelineError, SteamworksTimelineGameMode, SteamworksTimelineOperation,
    SteamworksTimelinePlugin, SteamworksTimelineResult, SteamworksUserCommand, SteamworksUserError,
    SteamworksUserOperation, SteamworksUserPlugin, SteamworksUserResult, SteamworksUtilsCommand,
    SteamworksUtilsError, SteamworksUtilsOperation, SteamworksUtilsPlugin, SteamworksUtilsResult,
};

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
