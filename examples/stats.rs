use std::time::Duration;

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

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

    commands.write(SteamworksStatsCommand::RequestCurrentUserStats);
    commands.write(SteamworksStatsCommand::RequestGlobalAchievementPercentages);

    if let Ok(stat_name) = std::env::var("BEVY_STEAMWORKS_STAT_I32") {
        commands.write(SteamworksStatsCommand::get_stat_i32(stat_name));
    }

    if let Ok(achievement_name) = std::env::var("BEVY_STEAMWORKS_ACHIEVEMENT") {
        commands.write(SteamworksStatsCommand::get_achievement(achievement_name));
    }
}

fn log_stats_results(mut results: MessageReader<SteamworksStatsResult>) {
    for result in results.read() {
        println!("{result:?}");
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
        .insert_resource(FramesRemaining(120))
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
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
