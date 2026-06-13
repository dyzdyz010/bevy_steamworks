use std::time::Duration;

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

fn request_timeline(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksTimelineCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    if steam.is_none() {
        return;
    }

    commands.write(SteamworksTimelineCommand::set_game_mode(
        SteamworksTimelineGameMode::Menus,
    ));
    commands.write(SteamworksTimelineCommand::set_state_description(
        "Browsing menu",
        Duration::from_secs(2),
    ));

    if std::env::var("BEVY_STEAMWORKS_TIMELINE_EVENT").as_deref() == Ok("1") {
        commands.write(SteamworksTimelineCommand::add_event(
            SteamworksTimelineEventInfo {
                icon: "star".to_owned(),
                title: "Example Event".to_owned(),
                description: "Submitted by bevy_steamworks".to_owned(),
                priority: 1,
                start_offset_seconds: 0.0,
                duration: Duration::ZERO,
                clip_priority: SteamworksTimelineEventClipPriority::Standard,
            },
        ));
    }
}

fn log_timeline_results(
    mut results: MessageReader<SteamworksTimelineResult>,
    mut exit: MessageWriter<AppExit>,
) {
    for result in results.read() {
        println!("{result:?}");
    }

    exit.write(AppExit::Success);
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksTimelinePlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_once())
        .add_systems(Startup, request_timeline)
        .add_systems(Update, log_timeline_results)
        .run();
}
