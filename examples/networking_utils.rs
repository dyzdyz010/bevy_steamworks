use std::time::Duration;

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

#[derive(Resource)]
struct RelayStatusPollCountdown(u32);

fn configure_networking_utils(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksNetworkingUtilsCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    if steam.is_none() {
        return;
    }

    if std::env::var("BEVY_STEAMWORKS_RELAY_INIT").as_deref() != Ok("0") {
        commands.write(SteamworksNetworkingUtilsCommand::init_relay_network_access());
    }

    commands.write(SteamworksNetworkingUtilsCommand::get_relay_network_status());
    commands.write(SteamworksNetworkingUtilsCommand::get_detailed_relay_network_status());
}

fn poll_relay_status(
    mut countdown: ResMut<RelayStatusPollCountdown>,
    mut commands: MessageWriter<SteamworksNetworkingUtilsCommand>,
) {
    if countdown.0 == 0 {
        commands.write(SteamworksNetworkingUtilsCommand::get_detailed_relay_network_status());
        countdown.0 = env_u32("BEVY_STEAMWORKS_RELAY_POLL_FRAMES").unwrap_or(30);
    } else {
        countdown.0 -= 1;
    }
}

fn log_networking_utils_results(mut results: MessageReader<SteamworksNetworkingUtilsResult>) {
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

fn env_u32(name: &str) -> Option<u32> {
    std::env::var(name).ok()?.parse().ok()
}

fn main() {
    App::new()
        .insert_resource(FramesRemaining(120))
        .insert_resource(RelayStatusPollCountdown(0))
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksNetworkingUtilsPlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, configure_networking_utils)
        .add_systems(
            Update,
            (
                poll_relay_status,
                log_networking_utils_results,
                exit_after_a_short_run,
            ),
        )
        .run();
}
