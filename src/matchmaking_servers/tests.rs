use std::{net::Ipv4Addr, time::Duration};

use bevy_app::App;
use bevy_ecs::message::Messages;

use super::*;

#[test]
fn matchmaking_servers_plugin_registers_resources_and_messages() {
    let mut app = App::new();
    app.add_plugins(SteamworksMatchmakingServersPlugin::new());

    assert!(app
        .world()
        .contains_resource::<SteamworksMatchmakingServersState>());
    assert!(app
        .world()
        .contains_resource::<SteamworksMatchmakingServersAsyncResults>());
    assert!(app
        .world()
        .contains_resource::<SteamworksMatchmakingServerListRequests>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksMatchmakingServersCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksMatchmakingServersResult>>());
}

#[test]
fn server_list_commands_preserve_inputs() {
    let request = SteamworksServerListRequestId::from_raw(4);
    let filters = SteamworksServerListFilters::new().with("map", "arena");

    assert_eq!(request.raw(), 4);
    assert_eq!(
        SteamworksMatchmakingServersCommand::request_lan_server_list(480),
        SteamworksMatchmakingServersCommand::RequestServerList {
            app_id: steamworks::AppId(480),
            kind: SteamworksServerListKind::Lan,
            filters: SteamworksServerListFilters::new(),
        }
    );
    assert_eq!(
        SteamworksMatchmakingServersCommand::request_internet_server_list(480, filters.clone(),),
        SteamworksMatchmakingServersCommand::RequestServerList {
            app_id: steamworks::AppId(480),
            kind: SteamworksServerListKind::Internet,
            filters: filters.clone(),
        }
    );
    assert_eq!(
        SteamworksMatchmakingServersCommand::request_favorites_server_list(480, filters.clone(),),
        SteamworksMatchmakingServersCommand::RequestServerList {
            app_id: steamworks::AppId(480),
            kind: SteamworksServerListKind::Favorites,
            filters: filters.clone(),
        }
    );
    assert_eq!(
        SteamworksMatchmakingServersCommand::request_history_server_list(480, filters.clone()),
        SteamworksMatchmakingServersCommand::RequestServerList {
            app_id: steamworks::AppId(480),
            kind: SteamworksServerListKind::History,
            filters: filters.clone(),
        }
    );
    assert_eq!(
        SteamworksMatchmakingServersCommand::request_friends_server_list(480, filters),
        SteamworksMatchmakingServersCommand::RequestServerList {
            app_id: steamworks::AppId(480),
            kind: SteamworksServerListKind::Friends,
            filters: SteamworksServerListFilters::new().with("map", "arena"),
        }
    );
    assert_eq!(
        SteamworksMatchmakingServersCommand::refresh_server_list(request),
        SteamworksMatchmakingServersCommand::RefreshServerList { request }
    );
    assert_eq!(
        SteamworksMatchmakingServersCommand::refresh_server(request, 2),
        SteamworksMatchmakingServersCommand::RefreshServer { request, server: 2 }
    );
    assert_eq!(
        SteamworksMatchmakingServersCommand::get_server_list_count(request),
        SteamworksMatchmakingServersCommand::GetServerListCount { request }
    );
    assert_eq!(
        SteamworksMatchmakingServersCommand::get_server_details(request, 2),
        SteamworksMatchmakingServersCommand::GetServerDetails { request, server: 2 }
    );
    assert_eq!(
        SteamworksMatchmakingServersCommand::is_server_list_refreshing(request),
        SteamworksMatchmakingServersCommand::IsServerListRefreshing { request }
    );
    assert_eq!(
        SteamworksMatchmakingServersCommand::release_server_list(request),
        SteamworksMatchmakingServersCommand::ReleaseServerList { request }
    );
}

#[test]
fn filter_validation_rejects_invalid_inputs() {
    assert_eq!(
        validate_command(&SteamworksMatchmakingServersCommand::RequestServerList {
            app_id: steamworks::AppId(480),
            kind: SteamworksServerListKind::Lan,
            filters: SteamworksServerListFilters::new().with("map", "arena"),
        }),
        Err(SteamworksMatchmakingServersError::LanFiltersUnsupported)
    );
    assert_eq!(
        validate_command(
            &SteamworksMatchmakingServersCommand::request_internet_server_list(
                480,
                SteamworksServerListFilters::new().with("bad\0key", "arena"),
            )
        ),
        Err(SteamworksMatchmakingServersError::InvalidString {
            field: "filter key",
        })
    );
    assert_eq!(
        validate_command(
            &SteamworksMatchmakingServersCommand::request_internet_server_list(
                480,
                SteamworksServerListFilters::new().with(
                    "map",
                    "x".repeat(STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES + 1),
                ),
            )
        ),
        Err(SteamworksMatchmakingServersError::FilterTooLong {
            field: "filter value",
            requested: STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES + 1,
            max_supported: STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES,
        })
    );
    assert_eq!(
        validate_command(&SteamworksMatchmakingServersCommand::get_server_details(
            SteamworksServerListRequestId::from_raw(1),
            -1,
        )),
        Err(SteamworksMatchmakingServersError::InvalidServerIndex { server: -1 })
    );
}

#[test]
fn state_records_callback_operations_without_unbounded_history() {
    let mut state = SteamworksMatchmakingServersState::default();
    let request = SteamworksServerListRequestId::from_raw(1);
    let filters = SteamworksServerListFilters::new().with("map", "arena");
    let server = SteamworksGameServerItem {
        app_id: 480,
        players: 2,
        do_not_refresh: false,
        successful_response: true,
        have_password: false,
        secure: true,
        bot_players: 0,
        ping: Duration::from_millis(42),
        max_players: 8,
        server_version: 1,
        steam_id: 123,
        last_time_played: Duration::from_secs(0),
        address: Ipv4Addr::LOCALHOST,
        query_port: 27015,
        connection_port: 27016,
        game_description: "Example".to_owned(),
        server_name: "Local".to_owned(),
        game_dir: "example".to_owned(),
        map: "arena".to_owned(),
        tags: "tag".to_owned(),
    };

    state.record_operation(
        &SteamworksMatchmakingServersOperation::ServerListRequested {
            request,
            app_id: steamworks::AppId(480),
            kind: SteamworksServerListKind::Internet,
            filters: filters.clone(),
        },
    );
    state.record_operation(
        &SteamworksMatchmakingServersOperation::ServerListRefreshSubmitted { request },
    );
    state.record_operation(
        &SteamworksMatchmakingServersOperation::ServerRefreshSubmitted {
            request,
            server_index: 3,
        },
    );
    state.record_operation(
        &SteamworksMatchmakingServersOperation::ServerListCountRead { request, count: 4 },
    );
    state.record_operation(
        &SteamworksMatchmakingServersOperation::ServerListRefreshingRead {
            request,
            refreshing: true,
        },
    );
    state.record_operation(&SteamworksMatchmakingServersOperation::ServerResponded {
        request,
        server_index: 0,
        server: server.clone(),
    });
    state.record_operation(
        &SteamworksMatchmakingServersOperation::ServerFailedToRespond {
            request,
            server_index: 1,
        },
    );
    state.record_operation(
        &SteamworksMatchmakingServersOperation::ServerListRefreshCompleted {
            request,
            response: SteamworksServerListResponse::ServerResponded,
        },
    );
    state.record_operation(&SteamworksMatchmakingServersOperation::ServerDetailsRead {
        request,
        server_index: 2,
        server: server.clone(),
    });
    state.record_operation(&SteamworksMatchmakingServersOperation::ServerListReleased { request });

    assert_eq!(
        state.last_server_list_request(),
        Some(&SteamworksServerListRequestInfo {
            request,
            app_id: steamworks::AppId(480),
            kind: SteamworksServerListKind::Internet,
            filters,
        })
    );
    assert_eq!(state.server_list_request_count(), 1);
    assert_eq!(state.last_server_list_refresh_request(), Some(request));
    assert_eq!(state.server_list_refresh_request_count(), 1);
    assert_eq!(
        state.last_server_refresh_request(),
        Some(SteamworksServerListServerIndex {
            request,
            server_index: 3,
        })
    );
    assert_eq!(state.server_refresh_request_count(), 1);
    assert_eq!(
        state.last_server_list_count(),
        Some(SteamworksServerListCount { request, count: 4 })
    );
    assert_eq!(state.server_list_count_read_count(), 1);
    assert_eq!(
        state.last_server_list_refreshing(),
        Some(SteamworksServerListRefreshing {
            request,
            refreshing: true,
        })
    );
    assert_eq!(state.server_list_refreshing_read_count(), 1);
    assert_eq!(state.last_server(), Some(&server));
    assert_eq!(
        state.last_server_response(),
        Some(SteamworksServerListServerIndex {
            request,
            server_index: 0,
        })
    );
    assert_eq!(
        state.last_server_failure(),
        Some(SteamworksServerListServerIndex {
            request,
            server_index: 1,
        })
    );
    assert_eq!(state.last_refresh_completion_request(), Some(request));
    assert_eq!(
        state.last_refresh_response(),
        Some(SteamworksServerListResponse::ServerResponded)
    );
    assert_eq!(
        state.last_server_details_read(),
        Some(SteamworksServerListServerIndex {
            request,
            server_index: 2,
        })
    );
    assert_eq!(state.last_released_server_list_request(), Some(request));
    assert_eq!(state.server_list_release_count(), 1);
    assert_eq!(state.server_response_count(), 1);
    assert_eq!(state.server_failure_count(), 1);
    assert_eq!(state.refresh_complete_count(), 1);
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();
    app.add_plugins(SteamworksMatchmakingServersPlugin::new());
    app.world_mut()
        .write_message(SteamworksMatchmakingServersCommand::request_lan_server_list(480));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksMatchmakingServersResult>>();
    let drained = results.drain().collect::<Vec<_>>();
    assert_eq!(drained.len(), 1);
    assert!(matches!(
        &drained[0],
        SteamworksMatchmakingServersResult::Err {
            error: SteamworksMatchmakingServersError::ClientUnavailable,
            ..
        }
    ));
}
