use std::{net::SocketAddr, time::Duration};

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

#[derive(Default, Resource)]
struct SocketExampleHandles {
    listen_sockets: Vec<SteamworksListenSocketId>,
    connections: Vec<SteamworksNetworkingSocketsConnectionId>,
    poll_groups: Vec<SteamworksNetworkingSocketsPollGroupId>,
    sent_message: bool,
}

fn configure_networking_sockets(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksNetworkingSocketsCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    if steam.is_none() {
        return;
    }

    commands.write(SteamworksNetworkingSocketsCommand::InitAuthentication);
    commands.write(SteamworksNetworkingSocketsCommand::GetAuthenticationStatus);

    if std::env::var("BEVY_STEAMWORKS_SOCKETS_POLL_GROUP").as_deref() == Ok("1") {
        commands.write(SteamworksNetworkingSocketsCommand::create_poll_group());
    }

    if let Some(address) = env_socket_addr("BEVY_STEAMWORKS_SOCKETS_LISTEN_IP") {
        commands.write(SteamworksNetworkingSocketsCommand::create_listen_socket_ip(
            address,
        ));
    }

    if let Some(port) = env_i32("BEVY_STEAMWORKS_SOCKETS_LISTEN_P2P") {
        commands.write(SteamworksNetworkingSocketsCommand::create_listen_socket_p2p(port));
    }

    if let Some(address) = env_socket_addr("BEVY_STEAMWORKS_SOCKETS_CONNECT_IP") {
        commands.write(SteamworksNetworkingSocketsCommand::connect_by_ip_address(
            address,
        ));
    }

    if let Some(steam_id) = env_steam_id("BEVY_STEAMWORKS_SOCKETS_CONNECT_STEAM_ID") {
        commands.write(SteamworksNetworkingSocketsCommand::connect_p2p_steam_id(
            steam_id,
            env_i32("BEVY_STEAMWORKS_SOCKETS_REMOTE_PORT").unwrap_or(0),
        ));
    }
}

fn poll_networking_sockets(
    handles: Res<SocketExampleHandles>,
    mut commands: MessageWriter<SteamworksNetworkingSocketsCommand>,
) {
    let policy = if std::env::var("BEVY_STEAMWORKS_SOCKETS_ACCEPT").as_deref() == Ok("1") {
        SteamworksConnectionRequestPolicy::Accept
    } else {
        SteamworksConnectionRequestPolicy::default()
    };

    for listen_socket in &handles.listen_sockets {
        commands.write(
            SteamworksNetworkingSocketsCommand::poll_listen_socket_events(
                *listen_socket,
                16,
                policy.clone(),
            ),
        );
    }

    for connection in &handles.connections {
        commands.write(SteamworksNetworkingSocketsCommand::poll_connection_events(
            *connection,
            16,
        ));
        if handles.poll_groups.is_empty() {
            commands.write(SteamworksNetworkingSocketsCommand::receive_messages(
                *connection,
                16,
            ));
        }
        commands.write(SteamworksNetworkingSocketsCommand::get_connection_info(
            *connection,
        ));
    }

    for poll_group in &handles.poll_groups {
        commands.write(
            SteamworksNetworkingSocketsCommand::receive_poll_group_messages(*poll_group, 16),
        );
    }
}

fn send_optional_message(
    mut handles: ResMut<SocketExampleHandles>,
    mut commands: MessageWriter<SteamworksNetworkingSocketsCommand>,
) {
    if handles.sent_message {
        return;
    }

    let Some(connection) = handles.connections.first().copied() else {
        return;
    };

    let Ok(message) = std::env::var("BEVY_STEAMWORKS_SOCKETS_MESSAGE") else {
        return;
    };

    commands.write(SteamworksNetworkingSocketsCommand::send_message(
        connection,
        steamworks::networking_types::SendFlags::RELIABLE,
        message.into_bytes(),
    ));
    handles.sent_message = true;
}

fn log_networking_sockets_results(
    mut handles: ResMut<SocketExampleHandles>,
    mut results: MessageReader<SteamworksNetworkingSocketsResult>,
    mut commands: MessageWriter<SteamworksNetworkingSocketsCommand>,
) {
    for result in results.read() {
        println!("{result:?}");

        if let SteamworksNetworkingSocketsResult::Ok(operation) = result {
            match operation {
                SteamworksNetworkingSocketsOperation::PollGroupCreated { poll_group }
                    if !handles.poll_groups.contains(poll_group) =>
                {
                    handles.poll_groups.push(*poll_group);
                    for connection in &handles.connections {
                        commands.write(
                            SteamworksNetworkingSocketsCommand::set_connection_poll_group(
                                *connection,
                                *poll_group,
                            ),
                        );
                    }
                }
                SteamworksNetworkingSocketsOperation::ListenSocketCreated {
                    listen_socket, ..
                } if !handles.listen_sockets.contains(listen_socket) => {
                    handles.listen_sockets.push(*listen_socket);
                }
                SteamworksNetworkingSocketsOperation::ConnectionCreated { connection, .. }
                    if !handles.connections.contains(connection) =>
                {
                    handles.connections.push(*connection);
                    if let Some(poll_group) = handles.poll_groups.first() {
                        commands.write(
                            SteamworksNetworkingSocketsCommand::set_connection_poll_group(
                                *connection,
                                *poll_group,
                            ),
                        );
                    }
                }
                SteamworksNetworkingSocketsOperation::ListenSocketEventsPolled {
                    events, ..
                } => {
                    for event in events {
                        match event {
                            SteamworksListenSocketEventInfo::Connected { connection, .. }
                                if !handles.connections.contains(connection) =>
                            {
                                handles.connections.push(*connection);
                                if let Some(poll_group) = handles.poll_groups.first() {
                                    commands.write(
                                        SteamworksNetworkingSocketsCommand::set_connection_poll_group(
                                            *connection,
                                            *poll_group,
                                        ),
                                    );
                                }
                            }
                            SteamworksListenSocketEventInfo::Disconnected {
                                connection: Some(connection),
                                ..
                            } => {
                                handles.connections.retain(|known| known != connection);
                            }
                            _ => {}
                        }
                    }
                }
                SteamworksNetworkingSocketsOperation::ConnectionEventsPolled {
                    connection,
                    connection_removed: true,
                    ..
                }
                | SteamworksNetworkingSocketsOperation::ConnectionClosed { connection, .. } => {
                    handles.connections.retain(|known| known != connection);
                }
                SteamworksNetworkingSocketsOperation::ListenSocketClosed {
                    listen_socket,
                    closed_connections,
                } => {
                    handles
                        .listen_sockets
                        .retain(|known| known != listen_socket);
                    handles
                        .connections
                        .retain(|known| !closed_connections.contains(known));
                }
                SteamworksNetworkingSocketsOperation::PollGroupClosed { poll_group } => {
                    handles.poll_groups.retain(|known| known != poll_group);
                }
                _ => {}
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

fn env_socket_addr(name: &str) -> Option<SocketAddr> {
    std::env::var(name).ok()?.parse().ok()
}

fn env_steam_id(name: &str) -> Option<SteamId> {
    std::env::var(name)
        .ok()?
        .parse::<u64>()
        .ok()
        .map(SteamId::from_raw)
}

fn env_i32(name: &str) -> Option<i32> {
    std::env::var(name).ok()?.parse().ok()
}

fn main() {
    App::new()
        .insert_resource(FramesRemaining(180))
        .init_resource::<SocketExampleHandles>()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksNetworkingSocketsPlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, configure_networking_sockets)
        .add_systems(
            Update,
            (
                poll_networking_sockets,
                send_optional_message,
                log_networking_sockets_results,
                exit_after_a_short_run,
            ),
        )
        .run();
}
