use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

fn list_friends(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut exit: MessageWriter<AppExit>,
) {
    if let Some(steam) = steam {
        println!("Steam persona: {}", steam.friends().name());

        for friend in steam.friends().get_friends(FriendFlags::IMMEDIATE) {
            println!(
                "Friend: {:?} - {} ({:?})",
                friend.id(),
                friend.name(),
                friend.state()
            );
        }
    } else if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
    }

    exit.write(AppExit::Success);
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(ScheduleRunnerPlugin::run_once())
        .add_systems(Startup, list_friends)
        .run();
}
