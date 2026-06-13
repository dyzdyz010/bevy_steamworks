use std::time::Duration;

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

fn configure_networking_messages(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksNetworkingMessagesCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    if steam.is_none() {
        return;
    }

    let auto_accept = std::env::var("BEVY_STEAMWORKS_NETWORKING_AUTO_ACCEPT").as_deref() != Ok("0");
    commands
        .write(SteamworksNetworkingMessagesCommand::set_auto_accept_session_requests(auto_accept));

    if let Ok(peer) = std::env::var("BEVY_STEAMWORKS_NETWORKING_PEER") {
        if let Ok(peer) = peer.parse::<u64>() {
            let payload = std::env::var("BEVY_STEAMWORKS_NETWORKING_MESSAGE")
                .unwrap_or_else(|_| "hello from bevy_steamworks".to_string());
            commands.write(
                SteamworksNetworkingMessagesCommand::send_message_to_steam_id(
                    SteamId::from_raw(peer),
                    steamworks::networking_types::SendFlags::RELIABLE_NO_NAGLE,
                    env_u32("BEVY_STEAMWORKS_NETWORKING_CHANNEL").unwrap_or(0),
                    payload.into_bytes(),
                ),
            );
            commands.write(
                SteamworksNetworkingMessagesCommand::get_session_connection_info(
                    SteamworksNetworkingPeer::steam_id(SteamId::from_raw(peer)),
                ),
            );
        }
    }
}

fn poll_networking_messages(mut commands: MessageWriter<SteamworksNetworkingMessagesCommand>) {
    commands.write(SteamworksNetworkingMessagesCommand::receive_messages(
        env_u32("BEVY_STEAMWORKS_NETWORKING_CHANNEL").unwrap_or(0),
        env_usize("BEVY_STEAMWORKS_NETWORKING_BATCH").unwrap_or(16),
    ));
}

fn log_networking_messages_results(mut results: MessageReader<SteamworksNetworkingMessagesResult>) {
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

fn env_u32(name: &str) -> Option<u32> {
    std::env::var(name).ok()?.parse().ok()
}

fn env_usize(name: &str) -> Option<usize> {
    std::env::var(name).ok()?.parse().ok()
}

fn main() {
    App::new()
        .insert_resource(FramesRemaining(120))
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksNetworkingMessagesPlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, configure_networking_messages)
        .add_systems(
            Update,
            (
                poll_networking_messages,
                log_networking_messages_results,
                exit_after_a_short_run,
            ),
        )
        .run();
}
