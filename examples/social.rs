use std::time::Duration;

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

fn request_social_data(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksFriendsCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    if steam.is_none() {
        return;
    }

    commands.write(SteamworksFriendsCommand::get_persona_name());
    commands.write(SteamworksFriendsCommand::list_friends(
        FriendFlags::IMMEDIATE,
    ));

    if let Ok(status) = std::env::var("BEVY_STEAMWORKS_RICH_PRESENCE_STATUS") {
        commands.write(SteamworksFriendsCommand::set_rich_presence(
            "status", status,
        ));
    }
}

fn log_social_results(mut results: MessageReader<SteamworksFriendsResult>) {
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
        .add_plugins(SteamworksFriendsPlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, request_social_data)
        .add_systems(Update, (log_social_results, exit_after_a_short_run))
        .run();
}
