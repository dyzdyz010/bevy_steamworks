use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

fn request_app_info(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksAppsCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    if steam.is_none() {
        return;
    }

    commands.write(SteamworksAppsCommand::get_current_app_info());
    commands.write(SteamworksAppsCommand::get_launch_command_line());

    if let Ok(key) = std::env::var("BEVY_STEAMWORKS_LAUNCH_PARAM") {
        commands.write(SteamworksAppsCommand::get_launch_query_param(key));
    }
}

fn log_app_results(
    mut results: MessageReader<SteamworksAppsResult>,
    mut exit: MessageWriter<AppExit>,
) {
    for result in results.read() {
        println!("{result:?}");
    }

    exit.write(AppExit::Success);
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksAppsPlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_once())
        .add_systems(Startup, request_app_info)
        .add_systems(Update, log_app_results)
        .run();
}
