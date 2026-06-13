use std::time::Duration;

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

fn request_matchmaking(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksMatchmakingCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    if steam.is_none() {
        return;
    }

    let filter = SteamworksLobbyListFilter::new()
        .with_distance(DistanceFilter::Default)
        .with_max_results(10);
    commands.write(SteamworksMatchmakingCommand::request_lobby_list(filter));

    if std::env::var("BEVY_STEAMWORKS_CREATE_PRIVATE_LOBBY").as_deref() == Ok("1") {
        commands.write(SteamworksMatchmakingCommand::create_lobby(
            LobbyType::Private,
            4,
        ));
    }
}

fn log_matchmaking_results(mut results: MessageReader<SteamworksMatchmakingResult>) {
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
        .insert_resource(FramesRemaining(180))
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksMatchmakingPlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, request_matchmaking)
        .add_systems(Update, (log_matchmaking_results, exit_after_a_short_run))
        .run();
}
