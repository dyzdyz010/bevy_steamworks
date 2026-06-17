use std::{net::SocketAddrV4, time::Duration};

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

#[derive(Default, Resource)]
struct MatchmakingServersExampleState {
    request: Option<SteamworksServerListRequestId>,
    queried_after_complete: bool,
}

fn request_server_list(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksMatchmakingServersCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    if steam.is_none() {
        return;
    }

    let app_id = std::env::var("BEVY_STEAMWORKS_SERVER_APP_ID")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(480);
    let list_kind = std::env::var("BEVY_STEAMWORKS_SERVER_LIST")
        .unwrap_or_else(|_| "lan".to_owned())
        .to_ascii_lowercase();
    let filters = server_filters_from_env();

    if let Ok(target) = std::env::var("BEVY_STEAMWORKS_DIRECT_SERVER") {
        if let Ok(target) = target.parse::<SocketAddrV4>() {
            commands.write(SteamworksMatchmakingServersCommand::ping_server(
                *target.ip(),
                target.port(),
            ));
            if std::env::var("BEVY_STEAMWORKS_DIRECT_SERVER_PLAYERS").is_ok() {
                commands.write(SteamworksMatchmakingServersCommand::query_player_details(
                    *target.ip(),
                    target.port(),
                ));
            }
            if std::env::var("BEVY_STEAMWORKS_DIRECT_SERVER_RULES").is_ok() {
                commands.write(SteamworksMatchmakingServersCommand::query_server_rules(
                    *target.ip(),
                    target.port(),
                ));
            }
        }
    }

    let command = match list_kind.as_str() {
        "internet" => {
            SteamworksMatchmakingServersCommand::request_internet_server_list(app_id, filters)
        }
        "favorites" => {
            SteamworksMatchmakingServersCommand::request_favorites_server_list(app_id, filters)
        }
        "history" => {
            SteamworksMatchmakingServersCommand::request_history_server_list(app_id, filters)
        }
        "friends" => {
            SteamworksMatchmakingServersCommand::request_friends_server_list(app_id, filters)
        }
        _ => SteamworksMatchmakingServersCommand::request_lan_server_list(app_id),
    };
    commands.write(command);
}

fn server_filters_from_env() -> SteamworksServerListFilters {
    let mut filters = SteamworksServerListFilters::new();
    if let (Ok(key), Ok(value)) = (
        std::env::var("BEVY_STEAMWORKS_SERVER_FILTER_KEY"),
        std::env::var("BEVY_STEAMWORKS_SERVER_FILTER_VALUE"),
    ) {
        filters.insert(key, value);
    }
    filters
}

fn log_matchmaking_server_results(
    mut state: ResMut<MatchmakingServersExampleState>,
    mut results: MessageReader<SteamworksMatchmakingServersResult>,
    mut commands: MessageWriter<SteamworksMatchmakingServersCommand>,
) {
    for result in results.read() {
        println!("{result:?}");

        let SteamworksMatchmakingServersResult::Ok(operation) = result else {
            continue;
        };

        match operation {
            SteamworksMatchmakingServersOperation::ServerListRequested { request, .. } => {
                state.request = Some(*request);
            }
            SteamworksMatchmakingServersOperation::ServerListRefreshCompleted {
                request, ..
            } if Some(*request) == state.request && !state.queried_after_complete => {
                commands.write(SteamworksMatchmakingServersCommand::get_server_list_count(
                    *request,
                ));
                commands.write(
                    SteamworksMatchmakingServersCommand::is_server_list_refreshing(*request),
                );
                commands.write(SteamworksMatchmakingServersCommand::release_server_list(
                    *request,
                ));
                state.queried_after_complete = true;
            }
            _ => {}
        }
    }
}

fn exit_after_a_short_run(mut frames: ResMut<FramesRemaining>, mut exit: MessageWriter<AppExit>) {
    if frames.0 == 0 {
        exit.write(AppExit::Success);
    } else {
        frames.0 -= 1;
    }
}

fn main() {
    App::new()
        .insert_resource(FramesRemaining(240))
        .init_resource::<MatchmakingServersExampleState>()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksMatchmakingServersPlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, request_server_list)
        .add_systems(
            Update,
            (log_matchmaking_server_results, exit_after_a_short_run),
        )
        .run();
}
