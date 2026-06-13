use std::time::Duration;

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

fn configure_networking(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksNetworkingCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    if steam.is_none() {
        return;
    }

    if let Some(peer) = env_steam_id("BEVY_STEAMWORKS_P2P_PEER") {
        if std::env::var("BEVY_STEAMWORKS_P2P_CLOSE").as_deref() == Ok("1") {
            commands.write(SteamworksNetworkingCommand::close_p2p_session(peer));
        }

        commands.write(SteamworksNetworkingCommand::get_p2p_session_state(peer));

        if let Ok(message) = std::env::var("BEVY_STEAMWORKS_P2P_MESSAGE") {
            commands.write(SteamworksNetworkingCommand::send_p2p_packet(
                peer,
                SteamworksP2pSendType::Reliable,
                env_u32("BEVY_STEAMWORKS_P2P_CHANNEL").unwrap_or(0),
                message.into_bytes(),
            ));
        }
    }
}

fn poll_networking(mut commands: MessageWriter<SteamworksNetworkingCommand>) {
    let channel = env_u32("BEVY_STEAMWORKS_P2P_CHANNEL").unwrap_or(0);
    commands.write(SteamworksNetworkingCommand::get_available_packet_size(
        channel,
    ));
    commands.write(SteamworksNetworkingCommand::read_p2p_packet(
        channel,
        env_usize("BEVY_STEAMWORKS_P2P_READ_BYTES").unwrap_or(4096),
    ));
}

fn log_networking_results(
    mut results: MessageReader<SteamworksNetworkingResult>,
    mut commands: MessageWriter<SteamworksNetworkingCommand>,
) {
    for result in results.read() {
        println!("{result:?}");

        if std::env::var("BEVY_STEAMWORKS_P2P_ACCEPT").as_deref() == Ok("1") {
            if let SteamworksNetworkingResult::Ok(
                SteamworksNetworkingOperation::SessionRequestReceived { remote },
            ) = result
            {
                commands.write(SteamworksNetworkingCommand::accept_p2p_session(*remote));
            }
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

fn env_steam_id(name: &str) -> Option<SteamId> {
    std::env::var(name)
        .ok()?
        .parse::<u64>()
        .ok()
        .map(SteamId::from_raw)
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
        .add_plugins(SteamworksNetworkingPlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, configure_networking)
        .add_systems(
            Update,
            (
                poll_networking,
                log_networking_results,
                exit_after_a_short_run,
            ),
        )
        .run();
}
