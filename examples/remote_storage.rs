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

    commands.write(SteamworksRemoteStorageCommand::get_cloud_info());
    commands.write(SteamworksRemoteStorageCommand::list_files());

    if let Ok(name) = std::env::var("BEVY_STEAMWORKS_REMOTE_STORAGE_FILE") {
        commands.write(SteamworksRemoteStorageCommand::get_file_info(name.clone()));
        commands.write(SteamworksRemoteStorageCommand::get_file_exists(
            name.clone(),
        ));
        commands.write(SteamworksRemoteStorageCommand::is_file_persisted(
            name.clone(),
        ));
        commands.write(SteamworksRemoteStorageCommand::get_file_timestamp(
            name.clone(),
        ));
        commands.write(SteamworksRemoteStorageCommand::get_sync_platforms(
            name.clone(),
        ));

        let requested_write =
            if let Ok(contents) = std::env::var("BEVY_STEAMWORKS_REMOTE_STORAGE_WRITE") {
                commands.write(SteamworksRemoteStorageCommand::write_file(
                    name.clone(),
                    contents.into_bytes(),
                ));
                true
            } else {
                false
            };

        let requested_read =
            std::env::var("BEVY_STEAMWORKS_REMOTE_STORAGE_READ").as_deref() == Ok("1");
        if requested_read && !requested_write {
            commands.write(SteamworksRemoteStorageCommand::read_file(name.clone()));
        }

        if std::env::var("BEVY_STEAMWORKS_REMOTE_STORAGE_SHARE").as_deref() == Ok("1") {
            commands.write(SteamworksRemoteStorageCommand::share_file(name));
        }
    }
}

fn handle_remote_storage_results(
    mut results: MessageReader<SteamworksRemoteStorageResult>,
    mut commands: MessageWriter<SteamworksRemoteStorageCommand>,
) {
    for result in results.read() {
        println!("{result:?}");
        if std::env::var("BEVY_STEAMWORKS_REMOTE_STORAGE_READ").as_deref() != Ok("1") {
            continue;
        }
        if let SteamworksRemoteStorageResult::Ok(SteamworksRemoteStorageOperation::FileWritten {
            written,
        }) = result
        {
            commands.write(SteamworksRemoteStorageCommand::read_file(
                written.name.clone(),
            ));
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
        .insert_resource(FramesRemaining(120))
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksRemoteStoragePlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, request_remote_storage)
        .add_systems(
            Update,
            (handle_remote_storage_results, exit_after_a_short_run),
        )
        .run();
}
