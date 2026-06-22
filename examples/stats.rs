use std::time::Duration;

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

#[derive(Default, Resource)]
struct StatsExampleState {
    leaderboard: Option<SteamworksLeaderboardId>,
    requested_leaderboard_reads: bool,
    requested_global_stat_reads: bool,
    uploaded_score: bool,
    listed_global_achievement_percentages: bool,
}

fn request_stats(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksStatsCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    if steam.is_none() {
        return;
    }

    commands.write(SteamworksStatsCommand::request_current_user_stats());
    commands.write(SteamworksStatsCommand::get_achievement_count());
    commands.write(SteamworksStatsCommand::request_global_achievement_percentages());

    if std::env::var("BEVY_STEAMWORKS_GLOBAL_STAT_I64").is_ok()
        || std::env::var("BEVY_STEAMWORKS_GLOBAL_STAT_F64").is_ok()
    {
        commands.write(SteamworksStatsCommand::request_global_stats(7));
    }

    if std::env::var("BEVY_STEAMWORKS_ACHIEVEMENT_CATALOG").as_deref() == Ok("1") {
        commands.write(SteamworksStatsCommand::list_achievements(true, true));
    }

    if let Ok(stat_name) = std::env::var("BEVY_STEAMWORKS_STAT_I32") {
        commands.write(SteamworksStatsCommand::get_stat_i32(stat_name));
    }

    if let Ok(achievement_name) = std::env::var("BEVY_STEAMWORKS_ACHIEVEMENT") {
        commands.write(SteamworksStatsCommand::get_achievement(
            achievement_name.clone(),
        ));
        if std::env::var("BEVY_STEAMWORKS_ACHIEVEMENT_ICON").as_deref() == Ok("1") {
            commands.write(SteamworksStatsCommand::get_achievement_icon(
                achievement_name,
            ));
        }
    }

    if let Ok(leaderboard_name) = std::env::var("BEVY_STEAMWORKS_LEADERBOARD") {
        if std::env::var("BEVY_STEAMWORKS_LEADERBOARD_CREATE").as_deref() == Ok("1") {
            commands.write(SteamworksStatsCommand::find_or_create_leaderboard(
                leaderboard_name,
                SteamworksLeaderboardSortMethod::Descending,
                SteamworksLeaderboardDisplayType::Numeric,
            ));
        } else {
            commands.write(SteamworksStatsCommand::find_leaderboard(leaderboard_name));
        }
    }
}

fn log_stats_results(
    mut state: ResMut<StatsExampleState>,
    mut results: MessageReader<SteamworksStatsResult>,
    mut commands: MessageWriter<SteamworksStatsCommand>,
) {
    for result in results.read() {
        println!("{result:?}");

        let SteamworksStatsResult::Ok(operation) = result else {
            continue;
        };

        match operation {
            SteamworksStatsOperation::LeaderboardFindCompleted {
                leaderboard: Some(leaderboard),
                ..
            }
            | SteamworksStatsOperation::LeaderboardFindOrCreateCompleted {
                leaderboard: Some(leaderboard),
                ..
            } => {
                state.leaderboard = Some(*leaderboard);
            }
            SteamworksStatsOperation::GlobalAchievementPercentagesReceived { .. }
                if !state.listed_global_achievement_percentages
                    && std::env::var("BEVY_STEAMWORKS_GLOBAL_ACHIEVEMENT_PERCENTAGES")
                        .as_deref()
                        == Ok("1") =>
            {
                commands.write(SteamworksStatsCommand::list_achievement_global_percentages());
                state.listed_global_achievement_percentages = true;
            }
            SteamworksStatsOperation::GlobalStatsReceived { .. }
                if !state.requested_global_stat_reads =>
            {
                if let Ok(name) = std::env::var("BEVY_STEAMWORKS_GLOBAL_STAT_I64") {
                    commands.write(SteamworksStatsCommand::get_global_stat_i64(name.clone()));
                    commands.write(SteamworksStatsCommand::get_global_stat_history_i64(name, 7));
                }
                if let Ok(name) = std::env::var("BEVY_STEAMWORKS_GLOBAL_STAT_F64") {
                    commands.write(SteamworksStatsCommand::get_global_stat_f64(name.clone()));
                    commands.write(SteamworksStatsCommand::get_global_stat_history_f64(name, 7));
                }
                state.requested_global_stat_reads = true;
            }
            _ => {}
        }
    }

    let Some(leaderboard) = state.leaderboard else {
        return;
    };

    if !state.requested_leaderboard_reads {
        commands.write(SteamworksStatsCommand::get_leaderboard_info(leaderboard));
        commands.write(SteamworksStatsCommand::download_leaderboard_entries(
            leaderboard,
            SteamworksLeaderboardDataRequest::Global { start: 1, end: 10 },
            0,
        ));
        state.requested_leaderboard_reads = true;
    }

    if !state.uploaded_score {
        if let Some(score) = std::env::var("BEVY_STEAMWORKS_LEADERBOARD_SCORE")
            .ok()
            .and_then(|value| value.parse::<i32>().ok())
        {
            commands.write(SteamworksStatsCommand::upload_leaderboard_score(
                leaderboard,
                SteamworksLeaderboardUploadScoreMethod::KeepBest,
                score,
                Vec::<i32>::new(),
            ));
            state.uploaded_score = true;
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

fn example_app_id() -> u32 {
    std::env::var("BEVY_STEAMWORKS_APP_ID")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(480)
}

fn main() {
    App::new()
        .insert_resource(FramesRemaining(120))
        .init_resource::<StatsExampleState>()
        .add_plugins(SteamworksPlugin::app_id(example_app_id()).log_and_continue())
        .add_plugins(
            SteamworksStatsPlugin::new()
                .request_current_user_stats_on_startup(false)
                .auto_store(false),
        )
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, request_stats)
        .add_systems(Update, (log_stats_results, exit_after_a_short_run))
        .run();
}
