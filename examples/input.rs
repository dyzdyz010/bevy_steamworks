use std::time::Duration;

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

fn request_input(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksInputCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    if steam.is_none() {
        return;
    }

    commands.write(SteamworksInputCommand::init(false));
    commands.write(SteamworksInputCommand::RunFrame);
    commands.write(SteamworksInputCommand::ListControllers);

    if let Ok(path) = std::env::var("BEVY_STEAMWORKS_INPUT_MANIFEST") {
        commands.write(SteamworksInputCommand::set_action_manifest_file_path(path));
    }

    if let Ok(action_set) = std::env::var("BEVY_STEAMWORKS_INPUT_ACTION_SET") {
        commands.write(SteamworksInputCommand::get_action_set_handle(action_set));
    }

    if let Ok(action) = std::env::var("BEVY_STEAMWORKS_INPUT_DIGITAL_ACTION") {
        commands.write(SteamworksInputCommand::get_digital_action_handle(action));
    }

    if let Ok(action) = std::env::var("BEVY_STEAMWORKS_INPUT_ANALOG_ACTION") {
        commands.write(SteamworksInputCommand::get_analog_action_handle(action));
    }
}

fn log_input_results(mut results: MessageReader<SteamworksInputResult>) {
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
        .insert_resource(FramesRemaining(4))
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksInputPlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, request_input)
        .add_systems(Update, (log_input_results, exit_after_a_short_run))
        .run();
}
