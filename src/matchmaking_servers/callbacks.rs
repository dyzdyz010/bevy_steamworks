use super::{
    requests::SteamworksMatchmakingServersAsyncResults, SteamworksGameServerItem,
    SteamworksMatchmakingServersCommand, SteamworksMatchmakingServersError,
    SteamworksMatchmakingServersOperation, SteamworksMatchmakingServersResult,
    SteamworksServerListRequestId, SteamworksServerPing, SteamworksServerPlayerDetails,
    SteamworksServerPlayerInfo, SteamworksServerQueryId, SteamworksServerQueryTarget,
    SteamworksServerRule, SteamworksServerRules,
};

pub(super) fn ping_callbacks(
    query: SteamworksServerQueryId,
    target: SteamworksServerQueryTarget,
    async_results: SteamworksMatchmakingServersAsyncResults,
) -> steamworks::PingCallbacks {
    let responded_results = async_results.clone();
    let failed_results = async_results.clone();

    steamworks::PingCallbacks::new(
        Box::new(move |server| {
            responded_results.push(SteamworksMatchmakingServersResult::Ok(
                SteamworksMatchmakingServersOperation::ServerPingResponded {
                    ping: SteamworksServerPing {
                        query,
                        target,
                        server: SteamworksGameServerItem::from_steam(server),
                    },
                },
            ));
        }),
        Box::new(move || {
            failed_results.push(SteamworksMatchmakingServersResult::Ok(
                SteamworksMatchmakingServersOperation::ServerPingFailed { query },
            ));
        }),
    )
}

pub(super) fn player_details_callbacks(
    query: SteamworksServerQueryId,
    target: SteamworksServerQueryTarget,
    async_results: SteamworksMatchmakingServersAsyncResults,
) -> steamworks::PlayerDetailsCallbacks {
    let players = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let add_players = players.clone();
    let complete_players = players.clone();

    steamworks::PlayerDetailsCallbacks::new(
        Box::new(move |name, score, time_played| {
            add_players
                .lock()
                .expect("Steamworks player-details callback mutex was poisoned")
                .push(SteamworksServerPlayerInfo::from_steam(
                    name,
                    score,
                    time_played,
                ));
        }),
        Box::new({
            let async_results = async_results.clone();
            move || {
                async_results.push(SteamworksMatchmakingServersResult::Ok(
                    SteamworksMatchmakingServersOperation::ServerPlayerDetailsFailed { query },
                ));
            }
        }),
        Box::new(move || {
            let players = complete_players
                .lock()
                .expect("Steamworks player-details callback mutex was poisoned")
                .clone();
            async_results.push(SteamworksMatchmakingServersResult::Ok(
                SteamworksMatchmakingServersOperation::ServerPlayerDetailsReceived {
                    details: SteamworksServerPlayerDetails {
                        query,
                        target,
                        players,
                    },
                },
            ));
        }),
    )
}

pub(super) fn server_rules_callbacks(
    query: SteamworksServerQueryId,
    target: SteamworksServerQueryTarget,
    async_results: SteamworksMatchmakingServersAsyncResults,
) -> steamworks::ServerRulesCallbacks {
    let rules = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let add_rules = rules.clone();
    let complete_rules = rules.clone();

    steamworks::ServerRulesCallbacks::new(
        Box::new(move |rule, value| {
            add_rules
                .lock()
                .expect("Steamworks server-rules callback mutex was poisoned")
                .push(SteamworksServerRule::from_steam(rule, value));
        }),
        Box::new({
            let async_results = async_results.clone();
            move || {
                async_results.push(SteamworksMatchmakingServersResult::Ok(
                    SteamworksMatchmakingServersOperation::ServerRulesFailed { query },
                ));
            }
        }),
        Box::new(move || {
            let rules = complete_rules
                .lock()
                .expect("Steamworks server-rules callback mutex was poisoned")
                .clone();
            async_results.push(SteamworksMatchmakingServersResult::Ok(
                SteamworksMatchmakingServersOperation::ServerRulesReceived {
                    rules: SteamworksServerRules {
                        query,
                        target,
                        rules,
                    },
                },
            ));
        }),
    )
}

pub(super) fn server_list_callbacks(
    request: SteamworksServerListRequestId,
    async_results: SteamworksMatchmakingServersAsyncResults,
) -> steamworks::ServerListCallbacks {
    let responded_results = async_results.clone();
    let failed_results = async_results.clone();

    steamworks::ServerListCallbacks::new(
        Box::new(move |list, server_index| {
            let result = list
                .lock()
                .expect("Steamworks server-list request mutex was poisoned")
                .get_server_details(server_index);
            responded_results.push(match result {
                Ok(server) => SteamworksMatchmakingServersResult::Ok(
                    SteamworksMatchmakingServersOperation::ServerResponded {
                        request,
                        server_index,
                        server: SteamworksGameServerItem::from_steam(server),
                    },
                ),
                Err(()) => SteamworksMatchmakingServersResult::Err {
                    command: SteamworksMatchmakingServersCommand::GetServerDetails {
                        request,
                        server: server_index,
                    },
                    error: SteamworksMatchmakingServersError::ServerDetailsUnavailable {
                        request,
                        server: server_index,
                    },
                },
            });
        }),
        Box::new(move |_list, server_index| {
            failed_results.push(SteamworksMatchmakingServersResult::Ok(
                SteamworksMatchmakingServersOperation::ServerFailedToRespond {
                    request,
                    server_index,
                },
            ));
        }),
        Box::new(move |_list, response| {
            async_results.push(SteamworksMatchmakingServersResult::Ok(
                SteamworksMatchmakingServersOperation::ServerListRefreshCompleted {
                    request,
                    response: response.into(),
                },
            ));
        }),
    )
}
