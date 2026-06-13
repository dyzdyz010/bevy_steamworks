use std::{path::PathBuf, time::Duration};

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

fn request_screenshots(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksScreenshotsCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    if steam.is_none() {
        return;
    }

    commands.write(SteamworksScreenshotsCommand::IsScreenshotsHooked);

    if std::env::var("BEVY_STEAMWORKS_HOOK_SCREENSHOTS").as_deref() == Ok("1") {
        commands.write(SteamworksScreenshotsCommand::hook_screenshots(true));
    }

    if std::env::var("BEVY_STEAMWORKS_TRIGGER_SCREENSHOT").as_deref() == Ok("1") {
        commands.write(SteamworksScreenshotsCommand::TriggerScreenshot);
    }

    if let Ok(filename) = std::env::var("BEVY_STEAMWORKS_SCREENSHOT_FILE") {
        let width = env_i32("BEVY_STEAMWORKS_SCREENSHOT_WIDTH", 1280);
        let height = env_i32("BEVY_STEAMWORKS_SCREENSHOT_HEIGHT", 720);
        let thumbnail = std::env::var("BEVY_STEAMWORKS_SCREENSHOT_THUMBNAIL")
            .ok()
            .map(PathBuf::from);

        commands.write(SteamworksScreenshotsCommand::add_screenshot_to_library(
            PathBuf::from(filename),
            thumbnail,
            width,
            height,
        ));
    }
}

fn log_screenshot_results(mut results: MessageReader<SteamworksScreenshotsResult>) {
    for result in results.read() {
        println!("{result:?}");
    }
}

fn log_screenshot_callbacks(mut events: MessageReader<SteamworksEvent>) {
    for event in events.read() {
        match event {
            SteamworksEvent::ScreenshotRequested(event) => {
                println!("ScreenshotRequested: {event:?}");
            }
            SteamworksEvent::ScreenshotReady(event) => {
                println!("ScreenshotReady: {event:?}");
            }
            _ => {}
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

fn env_i32(name: &str, default: i32) -> i32 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

fn main() {
    App::new()
        .insert_resource(FramesRemaining(120))
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksScreenshotsPlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, request_screenshots)
        .add_systems(
            Update,
            (
                log_screenshot_results,
                log_screenshot_callbacks,
                exit_after_a_short_run,
            ),
        )
        .run();
}
