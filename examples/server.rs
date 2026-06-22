use std::{net::Ipv4Addr, time::Duration};

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

fn log_server_state(
    server: Option<Res<SteamworksServer>>,
    unavailable: Option<Res<SteamworksServerUnavailable>>,
    mut commands: MessageWriter<SteamworksServerCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steam Game Server unavailable: {}", &*unavailable);
        return;
    }

    if let Some(server) = server {
        println!("Steam Game Server ID: {:?}", server.steam_id());
        commands.write(SteamworksServerCommand::set_product("480"));
        commands.write(SteamworksServerCommand::set_game_description("Spacewar"));
        commands.write(SteamworksServerCommand::set_dedicated_server(true));
        commands.write(SteamworksServerCommand::set_server_name(
            "bevy_steamworks example",
        ));
        commands.write(SteamworksServerCommand::set_max_players(16));
        commands.write(SteamworksServerCommand::set_bot_player_count(0));
        commands.write(SteamworksServerCommand::log_on_anonymous());
        commands.write(SteamworksServerCommand::set_advertise_server_active(true));
        commands.write(SteamworksServerCommand::enable_heartbeats(true));
    }
}

fn log_server_results(mut results: MessageReader<SteamworksServerResult>) {
    for result in results.read() {
        println!("{result:?}");
    }
}

fn log_server_callbacks(mut events: MessageReader<SteamworksEvent>) {
    for event in events.read() {
        match event {
            SteamworksEvent::GSClientApprove(event) => {
                println!("GSClientApprove: {event:?}");
            }
            SteamworksEvent::GSClientDeny(event) => {
                println!("GSClientDeny: {event:?}");
            }
            SteamworksEvent::GSClientKick(event) => {
                println!("GSClientKick: {event:?}");
            }
            SteamworksEvent::GSClientGroupStatus(event) => {
                println!("GSClientGroupStatus: {event:?}");
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

fn main() {
    App::new()
        .insert_resource(FramesRemaining(120))
        .add_plugins(
            SteamworksServerPlugin::new(SteamworksServerConfig::new(
                Ipv4Addr::UNSPECIFIED,
                27015,
                27016,
                steamworks::ServerMode::Authentication,
                env!("CARGO_PKG_VERSION"),
            ))
            .log_and_continue(),
        )
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, log_server_state)
        .add_systems(
            Update,
            (
                log_server_results,
                log_server_callbacks,
                exit_after_a_short_run,
            ),
        )
        .run();
}
