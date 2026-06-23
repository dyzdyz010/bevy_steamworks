use std::{net::Ipv4Addr, time::Duration};

use bevy_app::{App, Plugin};
use bevy_ecs::message::Messages;

use super::{
    requests::{SteamworksMatchmakingServerListRequests, SteamworksMatchmakingServersAsyncResults},
    *,
};

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
fn plugin_name_matches_matchmaking_servers_type_path_for_bevy_tracking() {
    let plugin = SteamworksMatchmakingServersPlugin::new();

    assert_eq!(
        plugin.name(),
        std::any::type_name::<SteamworksMatchmakingServersPlugin>()
    );
    assert_eq!(
        plugin.name(),
        "bevy_steamworks::matchmaking_servers::SteamworksMatchmakingServersPlugin"
    );
}

#[test]
fn server_list_commands_preserve_inputs() {
    let request = SteamworksServerListRequestId::from_raw(4);
    let target = SteamworksServerQueryTarget {
        address: Ipv4Addr::LOCALHOST,
        query_port: 27015,
    };
    let filters = SteamworksServerListFilters::new().with("map", "arena");

    assert_eq!(request.raw(), 4);
    assert_eq!(
        SteamworksMatchmakingServersCommand::ping_server(target.address, target.query_port),
        SteamworksMatchmakingServersCommand::PingServer { target }
    );
    assert_eq!(
        SteamworksMatchmakingServersCommand::query_player_details(
            target.address,
            target.query_port
        ),
        SteamworksMatchmakingServersCommand::QueryPlayerDetails { target }
    );
    assert_eq!(
        SteamworksMatchmakingServersCommand::query_server_rules(target.address, target.query_port),
        SteamworksMatchmakingServersCommand::QueryServerRules { target }
    );
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
fn state_records_callback_operations_without_unbounded_history() {
    let mut state = SteamworksMatchmakingServersState::default();
    let request = SteamworksServerListRequestId::from_raw(1);
    let query = SteamworksServerQueryId::from_raw(2);
    let target = SteamworksServerQueryTarget {
        address: Ipv4Addr::LOCALHOST,
        query_port: 27015,
    };
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
    let ping = SteamworksServerPing {
        query,
        target,
        server: server.clone(),
    };
    let player_details = SteamworksServerPlayerDetails {
        query,
        target,
        players: vec![SteamworksServerPlayerInfo {
            name: "Ada".to_owned(),
            score: 10,
            time_played: Duration::from_secs(60),
        }],
    };
    let rules = SteamworksServerRules {
        query,
        target,
        rules: vec![SteamworksServerRule {
            key: "map".to_owned(),
            value: "arena".to_owned(),
        }],
    };

    state.record_operation(
        &SteamworksMatchmakingServersOperation::ServerQuerySubmitted {
            query: SteamworksServerQueryInfo {
                query,
                kind: SteamworksServerQueryKind::Ping,
                target,
            },
        },
    );
    state.record_operation(
        &SteamworksMatchmakingServersOperation::ServerPingResponded { ping: ping.clone() },
    );
    state.record_operation(&SteamworksMatchmakingServersOperation::ServerPingFailed { query });
    state.record_operation(
        &SteamworksMatchmakingServersOperation::ServerPlayerDetailsReceived {
            details: player_details.clone(),
        },
    );
    state.record_operation(
        &SteamworksMatchmakingServersOperation::ServerPlayerDetailsFailed { query },
    );
    state.record_operation(
        &SteamworksMatchmakingServersOperation::ServerRulesReceived {
            rules: rules.clone(),
        },
    );
    state.record_operation(&SteamworksMatchmakingServersOperation::ServerRulesFailed { query });
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

    assert_eq!(
        state.server_query(query),
        Some(SteamworksServerQueryInfo {
            query,
            kind: SteamworksServerQueryKind::Ping,
            target,
        })
    );
    assert_eq!(state.server_query_target(query), Some(target));
    assert_eq!(
        state.server_query_kind(query),
        Some(SteamworksServerQueryKind::Ping)
    );
    assert_eq!(state.server_ping(query), Some(&ping));
    assert_eq!(state.server_ping_target(query), Some(target));
    assert_eq!(state.server_ping_server(query), Some(&server));
    assert_eq!(
        state.server_ping_latency(query),
        Some(Duration::from_millis(42))
    );
    assert_eq!(state.server_ping_server_name(query), Some("Local"));
    assert_eq!(state.server_ping_map(query), Some("arena"));
    assert_eq!(state.server_ping_player_count(query), Some(2));
    assert_eq!(state.server_ping_max_players(query), Some(8));
    assert!(state.server_ping_failed(query));
    assert_eq!(state.server_player_details(query), Some(&player_details));
    assert_eq!(state.server_player_details_target(query), Some(target));
    assert_eq!(
        state.server_players(query),
        Some(player_details.players.as_slice())
    );
    assert_eq!(state.server_player_count(query), Some(1));
    assert_eq!(state.last_server_player_count(), Some(1));
    assert_eq!(
        state.server_player(query, "Ada"),
        Some(&player_details.players[0])
    );
    assert_eq!(state.server_has_player(query, "Ada"), Some(true));
    assert_eq!(state.server_has_player(query, "Grace"), Some(false));
    assert_eq!(state.server_player_score(query, "Ada"), Some(10));
    assert_eq!(
        state.server_player_time_played(query, "Ada"),
        Some(Duration::from_secs(60))
    );
    assert!(state.server_player_details_failed(query));
    assert_eq!(state.server_rules(query), Some(&rules));
    assert_eq!(state.server_rules_target(query), Some(target));
    assert_eq!(
        state.server_rule_entries(query),
        Some(rules.rules.as_slice())
    );
    assert_eq!(state.server_rule_count(query), Some(1));
    assert_eq!(state.last_server_rule_count(), Some(1));
    assert_eq!(state.server_rule(query, "map"), Some(Some("arena")));
    assert_eq!(state.server_rule(query, "mode"), Some(None));
    assert_eq!(state.server_has_rule(query, "map"), Some(true));
    assert_eq!(state.server_has_rule(query, "mode"), Some(false));
    assert!(state.server_rules_failed(query));
    assert_eq!(
        state.server_list_request(request),
        Some(&SteamworksServerListRequestInfo {
            request,
            app_id: steamworks::AppId(480),
            kind: SteamworksServerListKind::Internet,
            filters: filters.clone(),
        })
    );
    assert_eq!(state.server_list_count(request), Some(4));
    assert_eq!(state.server_list_refreshing(request), Some(true));
    assert_eq!(state.server(request, 0), Some(&server));
    assert_eq!(state.server(request, 2), Some(&server));
    assert_eq!(state.server(request, 99), None);

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
    assert_eq!(
        state.last_server_query(),
        Some(SteamworksServerQueryInfo {
            query,
            kind: SteamworksServerQueryKind::Ping,
            target,
        })
    );
    assert_eq!(state.last_server_ping(), Some(&ping));
    assert_eq!(state.last_failed_server_ping(), Some(query));
    assert_eq!(state.last_server_player_details(), Some(&player_details));
    assert_eq!(state.last_failed_server_player_details(), Some(query));
    assert_eq!(state.last_server_rules(), Some(&rules));
    assert_eq!(state.last_failed_server_rules(), Some(query));
    assert_eq!(state.server_query_count(), 1);
    assert_eq!(state.server_ping_response_count(), 1);
    assert_eq!(state.server_ping_failure_count(), 1);
    assert_eq!(state.server_player_details_count(), 1);
    assert_eq!(state.server_player_details_failure_count(), 1);
    assert_eq!(state.server_rules_count(), 1);
    assert_eq!(state.server_rules_failure_count(), 1);
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
    assert_eq!(state.server_list_request(request), None);
    assert_eq!(state.server_list_count(request), None);
    assert_eq!(state.server_list_refreshing(request), None);
    assert_eq!(state.server(request, 0), None);
    assert!(state.server_query(query).is_some());
    assert_eq!(state.server_query_target(query), Some(target));
    assert_eq!(state.server_player_count(query), Some(1));
    assert_eq!(state.server_rule_count(query), Some(1));
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
