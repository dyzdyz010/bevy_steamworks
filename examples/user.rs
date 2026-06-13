use std::time::Duration;

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

fn request_user_info(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksUserCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    let Some(steam) = steam else {
        return;
    };

    commands.write(SteamworksUserCommand::GetCurrentUserInfo);
    commands.write(SteamworksUserCommand::IsLoggedOn);

    let steam_id = steam.user().steam_id();
    if std::env::var("BEVY_STEAMWORKS_AUTH_TICKET").as_deref() == Ok("1") {
        commands.write(SteamworksUserCommand::get_authentication_session_ticket(
            steam_id,
        ));
    }

    if let Ok(identity) = std::env::var("BEVY_STEAMWORKS_WEBAPI_IDENTITY") {
        commands
            .write(SteamworksUserCommand::get_authentication_session_ticket_for_web_api(identity));
    }
}

fn log_user_results(mut results: MessageReader<SteamworksUserResult>) {
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
        .add_plugins(SteamworksUserPlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, request_user_info)
        .add_systems(Update, (log_user_results, exit_after_a_short_run))
        .run();
}
