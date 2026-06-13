use std::time::Duration;

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

fn request_remote_storage(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksRemoteStorageCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    if steam.is_none() {
        return;
    }

    commands.write(SteamworksRemoteStorageCommand::GetCloudInfo);
    commands.write(SteamworksRemoteStorageCommand::ListFiles);

    if let Ok(name) = std::env::var("BEVY_STEAMWORKS_REMOTE_STORAGE_FILE") {
        commands.write(SteamworksRemoteStorageCommand::get_file_info(name.clone()));
        commands.write(SteamworksRemoteStorageCommand::get_sync_platforms(
            name.clone(),
        ));

        if std::env::var("BEVY_STEAMWORKS_REMOTE_STORAGE_SHARE").as_deref() == Ok("1") {
            commands.write(SteamworksRemoteStorageCommand::share_file(name));
        }
    }
}

fn log_remote_storage_results(mut results: MessageReader<SteamworksRemoteStorageResult>) {
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
        .add_plugins(SteamworksRemoteStoragePlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, request_remote_storage)
        .add_systems(Update, (log_remote_storage_results, exit_after_a_short_run))
        .run();
}
