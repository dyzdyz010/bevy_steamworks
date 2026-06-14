use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

fn report_steam_status(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
) {
    if let Some(steam) = steam {
        println!("Steam user: {}", steam.friends().name());
    } else if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
    }
}

fn log_steam_callbacks(mut callbacks: MessageReader<SteamworksEvent>) {
    for event in callbacks.read() {
        println!("{event:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugins::app_id(480).log_and_continue())
        .add_plugins(ScheduleRunnerPlugin::run_once())
        .add_systems(Startup, report_steam_status)
        .add_systems(Update, log_steam_callbacks)
        .run();
}
