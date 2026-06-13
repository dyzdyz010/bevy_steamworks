use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

fn request_utils(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksUtilsCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    if steam.is_none() {
        return;
    }

    commands.write(SteamworksUtilsCommand::GetCurrentInfo);
    commands.write(SteamworksUtilsCommand::IsOverlayEnabled);
    commands.write(SteamworksUtilsCommand::IsSteamRunningOnSteamDeck);

    if std::env::var("BEVY_STEAMWORKS_OVERLAY_BOTTOM_RIGHT").as_deref() == Ok("1") {
        commands.write(SteamworksUtilsCommand::set_overlay_notification_position(
            SteamworksNotificationPosition::BottomRight,
        ));
    }
}

fn log_utils_results(
    mut results: MessageReader<SteamworksUtilsResult>,
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
        .add_plugins(SteamworksUtilsPlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_once())
        .add_systems(Startup, request_utils)
        .add_systems(Update, log_utils_results)
        .run();
}
