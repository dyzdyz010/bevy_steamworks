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
        SteamworksNetworkingOperation as PreludeNetworkingOperation,
        SteamworksNetworkingPlugin as PreludeNetworkingPlugin,
        SteamworksNetworkingResult as PreludeNetworkingResult,
        SteamworksRemoteStorageCommand as PreludeRemoteStorageCommand,
        SteamworksRemoteStorageError as PreludeRemoteStorageError,
        SteamworksRemoteStorageOperation as PreludeRemoteStorageOperation,
        SteamworksRemoteStoragePlugin as PreludeRemoteStoragePlugin,
        SteamworksRemoteStorageResult as PreludeRemoteStorageResult,
        SteamworksServerListFilters as PreludeServerListFilters,
        SteamworksServerListKind as PreludeServerListKind,
        SteamworksServerListRequestId as PreludeServerListRequestId,
        SteamworksUserCommand as PreludeUserCommand, SteamworksUserError as PreludeUserError,
        SteamworksUserOperation as PreludeUserOperation, SteamworksUserPlugin as PreludeUserPlugin,
        SteamworksUserResult as PreludeUserResult,
    },
    SteamworksAppsCommand, SteamworksAppsError, SteamworksAppsOperation, SteamworksAppsPlugin,
    SteamworksAppsResult, SteamworksFriendsCommand, SteamworksFriendsError,
    SteamworksFriendsOperation, SteamworksFriendsPlugin, SteamworksFriendsResult,
    SteamworksInputCommand, SteamworksInputError, SteamworksInputOperation, SteamworksInputPlugin,
    SteamworksInputResult, SteamworksMatchmakingServersCommand, SteamworksMatchmakingServersError,
    SteamworksMatchmakingServersOperation, SteamworksMatchmakingServersPlugin,
    SteamworksMatchmakingServersResult, SteamworksNetworkingCommand, SteamworksNetworkingError,
    SteamworksNetworkingOperation, SteamworksNetworkingPlugin, SteamworksNetworkingResult,
    SteamworksRemoteStorageCommand, SteamworksRemoteStorageError, SteamworksRemoteStorageOperation,
    SteamworksRemoteStoragePlugin, SteamworksRemoteStorageResult, SteamworksServerListFilters,
    SteamworksServerListKind, SteamworksServerListRequestId, SteamworksUserCommand,
    SteamworksUserError, SteamworksUserOperation, SteamworksUserPlugin, SteamworksUserResult,
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
