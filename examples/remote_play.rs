use std::time::Duration;

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

fn request_remote_play(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksRemotePlayCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    if steam.is_none() {
        return;
    }

    commands.write(SteamworksRemotePlayCommand::list_sessions());

    if let (Some(session), Some(friend)) = (
        env_u32("BEVY_STEAMWORKS_REMOTE_PLAY_SESSION"),
        env_u64("BEVY_STEAMWORKS_REMOTE_PLAY_FRIEND"),
    ) {
        commands.write(SteamworksRemotePlayCommand::invite(
            RemotePlaySessionId::from_raw(session),
            SteamId::from_raw(friend),
        ));
    }
}

fn log_remote_play_results(mut results: MessageReader<SteamworksRemotePlayResult>) {
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

fn env_u64(name: &str) -> Option<u64> {
    std::env::var(name).ok()?.parse().ok()
}

fn main() {
    App::new()
        .insert_resource(FramesRemaining(120))
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksRemotePlayPlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, request_remote_play)
        .add_systems(Update, (log_remote_play_results, exit_after_a_short_run))
        .run();
}
