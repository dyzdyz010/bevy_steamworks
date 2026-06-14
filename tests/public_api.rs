use bevy_steamworks::{
    prelude::{
        SteamworksFriendsCommand as PreludeFriendsCommand,
        SteamworksFriendsError as PreludeFriendsError,
        SteamworksFriendsOperation as PreludeFriendsOperation,
        SteamworksFriendsPlugin as PreludeFriendsPlugin,
        SteamworksFriendsResult as PreludeFriendsResult,
        SteamworksMatchmakingServersCommand as PreludeMatchmakingServersCommand,
        SteamworksMatchmakingServersError as PreludeMatchmakingServersError,
        SteamworksMatchmakingServersOperation as PreludeMatchmakingServersOperation,
        SteamworksMatchmakingServersPlugin as PreludeMatchmakingServersPlugin,
        SteamworksMatchmakingServersResult as PreludeMatchmakingServersResult,
        SteamworksServerListFilters as PreludeServerListFilters,
        SteamworksServerListKind as PreludeServerListKind,
        SteamworksServerListRequestId as PreludeServerListRequestId,
    },
    SteamworksFriendsCommand, SteamworksFriendsError, SteamworksFriendsOperation,
    SteamworksFriendsPlugin, SteamworksFriendsResult, SteamworksMatchmakingServersCommand,
    SteamworksMatchmakingServersError, SteamworksMatchmakingServersOperation,
    SteamworksMatchmakingServersPlugin, SteamworksMatchmakingServersResult,
    SteamworksServerListFilters, SteamworksServerListKind, SteamworksServerListRequestId,
};

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
