use std::{
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
};

use bevy_app::{App, Plugin};
use bevy_ecs::message::Messages;

use super::handles::{
    SteamworksNetworkingSocketsHandleOwner, SteamworksNetworkingSocketsHandleStorage,
    SteamworksNetworkingSocketsHandles,
};
use super::*;

#[test]
fn plugin_name_matches_networking_sockets_type_path_for_bevy_tracking() {
    let plugin = SteamworksNetworkingSocketsPlugin::new();

    assert_eq!(
        plugin.name(),
        std::any::type_name::<SteamworksNetworkingSocketsPlugin>()
    );
    assert_eq!(
        plugin.name(),
        "bevy_steamworks::networking_sockets::SteamworksNetworkingSocketsPlugin"
    );
}

fn connection_id() -> SteamworksNetworkingSocketsConnectionId {
    SteamworksNetworkingSocketsConnectionId::from_raw(42)
}

fn listen_socket_id() -> SteamworksListenSocketId {
    SteamworksListenSocketId::from_raw(7)
}

fn poll_group_id() -> SteamworksNetworkingSocketsPollGroupId {
    SteamworksNetworkingSocketsPollGroupId::from_raw(9)
}

fn localhost() -> SocketAddr {
    SocketAddr::from((Ipv4Addr::LOCALHOST, 27015))
}

fn no_auth_config() -> SteamworksNetworkingSocketsConfigEntry {
    SteamworksNetworkingSocketsConfigEntry::int32(
        steamworks::networking_types::NetworkingConfigValue::IPAllowWithoutAuth,
        1,
    )
}

#[test]
fn networking_sockets_plugin_registers_resources_and_messages() {
    let mut app = App::new();

    app.add_plugins(SteamworksNetworkingSocketsPlugin::new());

    assert!(app
        .world()
        .contains_resource::<SteamworksNetworkingSocketsState>());
    assert!(app
        .world()
        .contains_resource::<SteamworksNetworkingSocketsHandles>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingSocketsCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksNetworkingSocketsResult>>());
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksNetworkingSocketsPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksNetworkingSocketsCommand>>()
        .write(SteamworksNetworkingSocketsCommand::get_authentication_status());

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksNetworkingSocketsResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksNetworkingSocketsResult::Err {
            command: SteamworksNetworkingSocketsCommand::GetAuthenticationStatus,
            error: SteamworksNetworkingSocketsError::ClientUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksNetworkingSocketsState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksNetworkingSocketsError::ClientUnavailable)
    );
}

#[test]
fn hosted_dedicated_server_listen_socket_requires_server() {
    let mut app = App::new();
    let command =
        SteamworksNetworkingSocketsCommand::create_hosted_dedicated_server_listen_socket(27015);

    app.add_plugins(SteamworksNetworkingSocketsPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksNetworkingSocketsCommand>>()
        .write(command.clone());

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksNetworkingSocketsResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksNetworkingSocketsResult::Err {
            command,
            error: SteamworksNetworkingSocketsError::ServerUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksNetworkingSocketsState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksNetworkingSocketsError::ServerUnavailable)
    );
}

#[test]
fn batch_send_still_requires_client_allocator() {
    let mut app = App::new();
    let command = SteamworksNetworkingSocketsCommand::send_messages(vec![
        SteamworksNetworkingSocketsOutboundMessage::new(
            connection_id(),
            steamworks::networking_types::SendFlags::RELIABLE,
            [1, 2, 3],
        ),
    ]);

    app.add_plugins(SteamworksNetworkingSocketsPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksNetworkingSocketsCommand>>()
        .write(command.clone());

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksNetworkingSocketsResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksNetworkingSocketsResult::Err {
            command,
            error: SteamworksNetworkingSocketsError::ClientUnavailable,
        }]
    );
}

#[test]
fn validation_rejects_invalid_inputs() {
    assert_eq!(
        validate_command(&SteamworksNetworkingSocketsCommand::poll_connection_events(
            connection_id(),
            0,
        )),
        Err(SteamworksNetworkingSocketsError::InvalidEventLimit)
    );
    assert_eq!(
        validate_command(
            &SteamworksNetworkingSocketsCommand::poll_all_listen_socket_events(
                0,
                SteamworksConnectionRequestPolicy::Accept,
            ),
        ),
        Err(SteamworksNetworkingSocketsError::InvalidEventLimit)
    );
    assert_eq!(
        validate_command(&SteamworksNetworkingSocketsCommand::poll_all_connection_events(0,)),
        Err(SteamworksNetworkingSocketsError::InvalidEventLimit)
    );
    assert_eq!(
        validate_command(&SteamworksNetworkingSocketsCommand::poll_connection_events(
            connection_id(),
            STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND + 1,
        )),
        Err(SteamworksNetworkingSocketsError::TooManyEvents {
            requested: STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND + 1,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND,
        })
    );
    assert_eq!(
        validate_command(
            &SteamworksNetworkingSocketsCommand::poll_all_listen_socket_events(
                STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND + 1,
                SteamworksConnectionRequestPolicy::Accept,
            ),
        ),
        Err(SteamworksNetworkingSocketsError::TooManyEvents {
            requested: STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND + 1,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND,
        })
    );
    assert_eq!(
        validate_command(
            &SteamworksNetworkingSocketsCommand::poll_all_connection_events(
                STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND + 1,
            )
        ),
        Err(SteamworksNetworkingSocketsError::TooManyEvents {
            requested: STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND + 1,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND,
        })
    );
    assert_eq!(
        validate_command(&SteamworksNetworkingSocketsCommand::receive_messages(
            connection_id(),
            0,
        )),
        Err(SteamworksNetworkingSocketsError::InvalidBatchSize)
    );
    assert_eq!(
        validate_command(&SteamworksNetworkingSocketsCommand::receive_all_messages(0,)),
        Err(SteamworksNetworkingSocketsError::InvalidBatchSize)
    );
    assert_eq!(
        validate_command(&SteamworksNetworkingSocketsCommand::receive_messages(
            connection_id(),
            STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND + 1,
        )),
        Err(SteamworksNetworkingSocketsError::BatchSizeTooLarge {
            requested: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND + 1,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND,
        })
    );
    assert_eq!(
        validate_command(&SteamworksNetworkingSocketsCommand::receive_all_messages(
            STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND + 1,
        )),
        Err(SteamworksNetworkingSocketsError::BatchSizeTooLarge {
            requested: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND + 1,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND,
        })
    );
    assert_eq!(
        validate_command(
            &SteamworksNetworkingSocketsCommand::receive_poll_group_messages(poll_group_id(), 0,),
        ),
        Err(SteamworksNetworkingSocketsError::InvalidBatchSize)
    );
    assert_eq!(
        validate_command(&SteamworksNetworkingSocketsCommand::receive_all_poll_group_messages(0,)),
        Err(SteamworksNetworkingSocketsError::InvalidBatchSize)
    );
    assert_eq!(
        validate_command(
            &SteamworksNetworkingSocketsCommand::receive_all_poll_group_messages(
                STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND + 1,
            )
        ),
        Err(SteamworksNetworkingSocketsError::BatchSizeTooLarge {
            requested: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND + 1,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND,
        })
    );
    assert_eq!(
        validate_command(&SteamworksNetworkingSocketsCommand::send_message(
            connection_id(),
            steamworks::networking_types::SendFlags::RELIABLE,
            vec![0; STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES + 1],
        )),
        Err(SteamworksNetworkingSocketsError::MessageTooLarge {
            bytes: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES + 1,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES,
        })
    );
    assert_eq!(
        validate_command(&SteamworksNetworkingSocketsCommand::send_messages(Vec::<
            SteamworksNetworkingSocketsOutboundMessage,
        >::new(
        ))),
        Err(SteamworksNetworkingSocketsError::EmptyMessageBatch)
    );
    assert_eq!(
        validate_command(&SteamworksNetworkingSocketsCommand::send_messages(vec![
            SteamworksNetworkingSocketsOutboundMessage::new(
                connection_id(),
                steamworks::networking_types::SendFlags::RELIABLE,
                Vec::<u8>::new(),
            );
            STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND + 1
        ])),
        Err(SteamworksNetworkingSocketsError::SendBatchTooLarge {
            requested: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND + 1,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND,
        })
    );
    assert_eq!(
        validate_command(&SteamworksNetworkingSocketsCommand::send_messages(vec![
            SteamworksNetworkingSocketsOutboundMessage::new(
                connection_id(),
                steamworks::networking_types::SendFlags::RELIABLE,
                vec![0; STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES + 1],
            )
        ])),
        Err(SteamworksNetworkingSocketsError::MessageTooLarge {
            bytes: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES + 1,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES,
        })
    );
    assert_eq!(
        validate_command(&SteamworksNetworkingSocketsCommand::send_messages(vec![
            SteamworksNetworkingSocketsOutboundMessage::new(
                connection_id(),
                steamworks::networking_types::SendFlags::RELIABLE,
                Vec::<u8>::new(),
            )
            .with_channel(-1)
        ])),
        Err(SteamworksNetworkingSocketsError::InvalidMessageChannel { channel: -1 })
    );
    assert_eq!(
        validate_command(
            &SteamworksNetworkingSocketsCommand::GetRealtimeConnectionStatus {
                connection: connection_id(),
                lanes: STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES + 1,
            },
        ),
        Err(SteamworksNetworkingSocketsError::InvalidLaneCount {
            lanes: STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES + 1,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES,
        })
    );
    assert_eq!(
        validate_command(
            &SteamworksNetworkingSocketsCommand::configure_connection_lanes(
                connection_id(),
                vec![0, 10],
                vec![100],
            )
        ),
        Err(SteamworksNetworkingSocketsError::InvalidLaneConfiguration {
            priorities: 2,
            weights: 1,
        })
    );
    assert_eq!(
        validate_command(
            &SteamworksNetworkingSocketsCommand::configure_connection_lanes(
                connection_id(),
                Vec::<i32>::new(),
                Vec::<u16>::new(),
            )
        ),
        Err(SteamworksNetworkingSocketsError::InvalidLaneConfiguration {
            priorities: 0,
            weights: 0,
        })
    );
    assert_eq!(
        validate_command(
            &SteamworksNetworkingSocketsCommand::configure_connection_lanes(
                connection_id(),
                vec![0; STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES + 1],
                vec![100; STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES + 1],
            )
        ),
        Err(SteamworksNetworkingSocketsError::TooManyConfiguredLanes {
            requested: STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES + 1,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES,
        })
    );
    assert_eq!(
        validate_command(
            &SteamworksNetworkingSocketsCommand::create_listen_socket_ip_with_options(
                localhost(),
                vec![no_auth_config(); STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIG_ENTRIES + 1],
            )
        ),
        Err(SteamworksNetworkingSocketsError::TooManyConfigEntries {
            requested: STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIG_ENTRIES + 1,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIG_ENTRIES,
        })
    );
    assert_eq!(
        validate_command(
            &SteamworksNetworkingSocketsCommand::create_hosted_dedicated_server_listen_socket_with_options(
                27015,
                vec![no_auth_config(); STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIG_ENTRIES + 1],
            )
        ),
        Err(SteamworksNetworkingSocketsError::TooManyConfigEntries {
            requested: STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIG_ENTRIES + 1,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIG_ENTRIES,
        })
    );
    assert_eq!(
        validate_command(
            &SteamworksNetworkingSocketsCommand::connect_by_ip_address_with_options(
                localhost(),
                vec![SteamworksNetworkingSocketsConfigEntry::int32(
                    steamworks::networking_types::NetworkingConfigValue::P2PSTUNServerList,
                    1,
                )],
            )
        ),
        Err(SteamworksNetworkingSocketsError::InvalidConfigEntryType {
            index: 0,
            expected: steamworks::networking_types::NetworkingConfigDataType::String,
            actual: steamworks::networking_types::NetworkingConfigDataType::Int32,
        })
    );
    assert_eq!(
        validate_command(
            &SteamworksNetworkingSocketsCommand::connect_by_ip_address_with_options(
                localhost(),
                vec![SteamworksNetworkingSocketsConfigEntry::string(
                    steamworks::networking_types::NetworkingConfigValue::P2PSTUNServerList,
                    "bad\0server",
                )],
            )
        ),
        Err(SteamworksNetworkingSocketsError::InvalidConfigString { index: 0 })
    );
    assert_eq!(
        validate_command(
            &SteamworksNetworkingSocketsCommand::connect_by_ip_address_with_options(
                localhost(),
                vec![SteamworksNetworkingSocketsConfigEntry::float(
                    steamworks::networking_types::NetworkingConfigValue::FakePacketLossSend,
                    f32::NAN,
                )],
            )
        ),
        Err(SteamworksNetworkingSocketsError::InvalidConfigFloat { index: 0 })
    );
    assert_eq!(
        validate_command(&SteamworksNetworkingSocketsCommand::create_listen_socket_p2p(-1,)),
        Err(SteamworksNetworkingSocketsError::InvalidVirtualPort { port: -1 })
    );
    assert_eq!(
        validate_command(&SteamworksNetworkingSocketsCommand::CloseConnection {
            connection: connection_id(),
            reason: steamworks::networking_types::NetConnectionEnd::App(
                steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
            ),
            debug: Some("bad\0debug".to_owned()),
            enable_linger: false,
        }),
        Err(SteamworksNetworkingSocketsError::InvalidString { field: "debug" })
    );
    assert_eq!(
        validate_command(
            &SteamworksNetworkingSocketsCommand::close_all_connections_with_reason(
                steamworks::networking_types::NetConnectionEnd::App(
                    steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
                ),
                Some("bad\0debug".to_owned()),
                false,
            ),
        ),
        Err(SteamworksNetworkingSocketsError::InvalidString { field: "debug" })
    );
    assert_eq!(
        validate_command(&SteamworksNetworkingSocketsCommand::set_connection_name(
            connection_id(),
            "bad\0name",
        )),
        Err(SteamworksNetworkingSocketsError::InvalidString { field: "name" })
    );
}

#[test]
fn constructors_preserve_inputs() {
    let address = localhost();
    assert_eq!(
        SteamworksNetworkingSocketsCommand::init_authentication(),
        SteamworksNetworkingSocketsCommand::InitAuthentication
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::get_authentication_status(),
        SteamworksNetworkingSocketsCommand::GetAuthenticationStatus
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::create_listen_socket_ip(address),
        SteamworksNetworkingSocketsCommand::CreateListenSocketIp {
            local_address: address,
            options: Vec::new(),
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::connect_by_ip_address(address),
        SteamworksNetworkingSocketsCommand::ConnectByIpAddress {
            address,
            options: Vec::new(),
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::create_listen_socket_ip_with_options(
            address,
            vec![no_auth_config()],
        ),
        SteamworksNetworkingSocketsCommand::CreateListenSocketIp {
            local_address: address,
            options: vec![no_auth_config()],
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::create_listen_socket_p2p_with_options(
            3,
            vec![no_auth_config()],
        ),
        SteamworksNetworkingSocketsCommand::CreateListenSocketP2p {
            local_virtual_port: 3,
            options: vec![no_auth_config()],
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::create_hosted_dedicated_server_listen_socket(27015),
        SteamworksNetworkingSocketsCommand::CreateHostedDedicatedServerListenSocket {
            local_virtual_port: 27015,
            options: Vec::new(),
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::create_hosted_dedicated_server_listen_socket_with_options(
            27015,
            vec![no_auth_config()],
        ),
        SteamworksNetworkingSocketsCommand::CreateHostedDedicatedServerListenSocket {
            local_virtual_port: 27015,
            options: vec![no_auth_config()],
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::connect_by_ip_address_with_options(
            address,
            vec![no_auth_config()],
        ),
        SteamworksNetworkingSocketsCommand::ConnectByIpAddress {
            address,
            options: vec![no_auth_config()],
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::poll_listen_socket_events(
            listen_socket_id(),
            8,
            SteamworksConnectionRequestPolicy::Accept,
        ),
        SteamworksNetworkingSocketsCommand::PollListenSocketEvents {
            listen_socket: listen_socket_id(),
            max_events: 8,
            request_policy: SteamworksConnectionRequestPolicy::Accept,
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::poll_all_listen_socket_events(
            8,
            SteamworksConnectionRequestPolicy::Accept,
        ),
        SteamworksNetworkingSocketsCommand::PollAllListenSocketEvents {
            max_events_per_socket: 8,
            request_policy: SteamworksConnectionRequestPolicy::Accept,
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::poll_all_connection_events(6),
        SteamworksNetworkingSocketsCommand::PollAllConnectionEvents {
            max_events_per_connection: 6,
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::create_poll_group(),
        SteamworksNetworkingSocketsCommand::CreatePollGroup
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::create_server_poll_group(),
        SteamworksNetworkingSocketsCommand::CreateServerPollGroup
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::get_connection_info(connection_id()),
        SteamworksNetworkingSocketsCommand::GetConnectionInfo {
            connection: connection_id(),
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::get_connection_user_data(connection_id()),
        SteamworksNetworkingSocketsCommand::GetConnectionUserData {
            connection: connection_id(),
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::get_realtime_connection_status(connection_id(), 4),
        SteamworksNetworkingSocketsCommand::GetRealtimeConnectionStatus {
            connection: connection_id(),
            lanes: 4,
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::receive_poll_group_messages(poll_group_id(), 16),
        SteamworksNetworkingSocketsCommand::ReceivePollGroupMessages {
            poll_group: poll_group_id(),
            batch_size: 16,
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::receive_all_messages(24),
        SteamworksNetworkingSocketsCommand::ReceiveAllMessages {
            batch_size_per_connection: 24,
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::receive_all_poll_group_messages(12),
        SteamworksNetworkingSocketsCommand::ReceiveAllPollGroupMessages {
            batch_size_per_poll_group: 12,
        }
    );
    let outbound = SteamworksNetworkingSocketsOutboundMessage::new(
        connection_id(),
        steamworks::networking_types::SendFlags::RELIABLE,
        [1, 2, 3],
    )
    .with_channel(1)
    .with_user_data(99);
    assert_eq!(
        SteamworksNetworkingSocketsCommand::send_messages(vec![outbound.clone()]),
        SteamworksNetworkingSocketsCommand::SendMessages {
            messages: vec![outbound],
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::flush_messages(connection_id()),
        SteamworksNetworkingSocketsCommand::FlushMessages {
            connection: connection_id(),
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::flush_all_messages(),
        SteamworksNetworkingSocketsCommand::FlushAllMessages
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::set_connection_poll_group(
            connection_id(),
            poll_group_id(),
        ),
        SteamworksNetworkingSocketsCommand::SetConnectionPollGroup {
            connection: connection_id(),
            poll_group: poll_group_id(),
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::clear_connection_poll_group(connection_id()),
        SteamworksNetworkingSocketsCommand::ClearConnectionPollGroup {
            connection: connection_id(),
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::configure_connection_lanes(
            connection_id(),
            vec![0, 10],
            vec![100, 20],
        ),
        SteamworksNetworkingSocketsCommand::ConfigureConnectionLanes {
            connection: connection_id(),
            lane_priorities: vec![0, 10],
            lane_weights: vec![100, 20],
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::set_connection_user_data(connection_id(), 11),
        SteamworksNetworkingSocketsCommand::SetConnectionUserData {
            connection: connection_id(),
            user_data: 11,
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::set_connection_name(connection_id(), "player-1"),
        SteamworksNetworkingSocketsCommand::SetConnectionName {
            connection: connection_id(),
            name: "player-1".to_owned(),
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::close_listen_socket(listen_socket_id()),
        SteamworksNetworkingSocketsCommand::CloseListenSocket {
            listen_socket: listen_socket_id(),
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::close_poll_group(poll_group_id()),
        SteamworksNetworkingSocketsCommand::ClosePollGroup {
            poll_group: poll_group_id(),
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::close_connection(connection_id()),
        SteamworksNetworkingSocketsCommand::CloseConnection {
            connection: connection_id(),
            reason: steamworks::networking_types::NetConnectionEnd::App(
                steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
            ),
            debug: None,
            enable_linger: false,
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::close_all_connections(),
        SteamworksNetworkingSocketsCommand::CloseAllConnections {
            reason: steamworks::networking_types::NetConnectionEnd::App(
                steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
            ),
            debug: None,
            enable_linger: false,
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::close_all_connections_with_reason(
            steamworks::networking_types::NetConnectionEnd::App(
                steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
            ),
            Some("shutdown".to_owned()),
            true,
        ),
        SteamworksNetworkingSocketsCommand::CloseAllConnections {
            reason: steamworks::networking_types::NetConnectionEnd::App(
                steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
            ),
            debug: Some("shutdown".to_owned()),
            enable_linger: true,
        }
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::close_all_listen_sockets(),
        SteamworksNetworkingSocketsCommand::CloseAllListenSockets
    );
    assert_eq!(
        SteamworksNetworkingSocketsCommand::close_all_poll_groups(),
        SteamworksNetworkingSocketsCommand::CloseAllPollGroups
    );
}

#[test]
fn bulk_commands_return_empty_batches_without_handles() {
    let mut handles = SteamworksNetworkingSocketsHandleStorage::default();

    assert_eq!(
        commands::handle_networking_sockets_command(
            None,
            None,
            &mut handles,
            &SteamworksNetworkingSocketsCommand::poll_all_listen_socket_events(
                4,
                SteamworksConnectionRequestPolicy::Accept,
            ),
        ),
        Ok(
            SteamworksNetworkingSocketsOperation::AllListenSocketEventsPolled {
                listen_sockets: Vec::new(),
            },
        )
    );
    assert_eq!(
        commands::handle_networking_sockets_command(
            None,
            None,
            &mut handles,
            &SteamworksNetworkingSocketsCommand::poll_all_connection_events(4),
        ),
        Ok(
            SteamworksNetworkingSocketsOperation::AllConnectionEventsPolled {
                connections: Vec::new(),
            },
        )
    );
    assert_eq!(
        commands::handle_networking_sockets_command(
            None,
            None,
            &mut handles,
            &SteamworksNetworkingSocketsCommand::receive_all_messages(4),
        ),
        Ok(SteamworksNetworkingSocketsOperation::AllMessagesReceived {
            connections: Vec::new(),
        },)
    );
    assert_eq!(
        commands::handle_networking_sockets_command(
            None,
            None,
            &mut handles,
            &SteamworksNetworkingSocketsCommand::receive_all_poll_group_messages(4),
        ),
        Ok(
            SteamworksNetworkingSocketsOperation::AllPollGroupMessagesReceived {
                poll_groups: Vec::new(),
            },
        )
    );
    assert_eq!(
        commands::handle_networking_sockets_command(
            None,
            None,
            &mut handles,
            &SteamworksNetworkingSocketsCommand::flush_all_messages(),
        ),
        Ok(SteamworksNetworkingSocketsOperation::AllMessagesFlushed {
            connections: Vec::new(),
        },)
    );
    assert_eq!(
        commands::handle_networking_sockets_command(
            None,
            None,
            &mut handles,
            &SteamworksNetworkingSocketsCommand::close_all_connections(),
        ),
        Ok(SteamworksNetworkingSocketsOperation::AllConnectionsClosed {
            connections: Vec::new(),
        },)
    );
    assert_eq!(
        commands::handle_networking_sockets_command(
            None,
            None,
            &mut handles,
            &SteamworksNetworkingSocketsCommand::close_all_listen_sockets(),
        ),
        Ok(
            SteamworksNetworkingSocketsOperation::AllListenSocketsClosed {
                listen_sockets: Vec::new(),
            },
        )
    );
    assert_eq!(
        commands::handle_networking_sockets_command(
            None,
            None,
            &mut handles,
            &SteamworksNetworkingSocketsCommand::close_all_poll_groups(),
        ),
        Ok(SteamworksNetworkingSocketsOperation::AllPollGroupsClosed {
            poll_groups: Vec::new(),
        },)
    );
}

#[test]
fn debug_redacts_config_entry_strings() {
    let entry = SteamworksNetworkingSocketsConfigEntry::string(
        steamworks::networking_types::NetworkingConfigValue::P2PSTUNServerList,
        "secret.stun.example",
    );
    let command = SteamworksNetworkingSocketsCommand::connect_by_ip_address_with_options(
        localhost(),
        vec![entry.clone()],
    );

    let entry_debug = format!("{entry:?}");
    let command_debug = format!("{command:?}");

    assert!(entry_debug.contains("data_len: 19"));
    assert!(!entry_debug.contains("secret.stun.example"));
    assert!(command_debug.contains("data_len: 19"));
    assert!(!command_debug.contains("secret.stun.example"));
}

#[test]
fn debug_redacts_connection_names() {
    let command =
        SteamworksNetworkingSocketsCommand::set_connection_name(connection_id(), "secret-player");
    let operation = SteamworksNetworkingSocketsOperation::ConnectionNameSet {
        connection: connection_id(),
        name: "secret-player".to_owned(),
    };
    let snapshot = SteamworksNetworkingSocketsConnectionName {
        connection: connection_id(),
        name: "secret-player".to_owned(),
    };
    let result = SteamworksNetworkingSocketsResult::Ok(operation.clone());

    for debug in [
        format!("{command:?}"),
        format!("{operation:?}"),
        format!("{snapshot:?}"),
        format!("{result:?}"),
    ] {
        assert!(debug.contains("name_len: 13"));
        assert!(!debug.contains("secret-player"));
    }
}

#[test]
fn debug_redacts_message_payload_bytes() {
    let peer = steamworks::networking_types::NetworkingIdentity::new_ip(localhost());
    let command = SteamworksNetworkingSocketsCommand::send_message(
        connection_id(),
        steamworks::networking_types::SendFlags::RELIABLE,
        vec![1, 2, 3],
    );
    let outbound = SteamworksNetworkingSocketsOutboundMessage::new(
        connection_id(),
        steamworks::networking_types::SendFlags::RELIABLE,
        vec![10, 11, 12],
    )
    .with_channel(2)
    .with_user_data(5);
    let batch_command = SteamworksNetworkingSocketsCommand::send_messages(vec![outbound]);
    let message = SteamworksNetworkingSocketsMessage {
        connection: connection_id(),
        peer: peer.clone(),
        data: vec![4, 5, 6],
        channel: 0,
        send_flags: steamworks::networking_types::SendFlags::RELIABLE,
        message_number: 1,
        connection_user_data: 0,
    };
    let poll_group_message = SteamworksNetworkingSocketsPollGroupMessage {
        poll_group: poll_group_id(),
        peer,
        data: vec![7, 8, 9],
        channel: 1,
        send_flags: steamworks::networking_types::SendFlags::UNRELIABLE,
        message_number: 2,
        connection_user_data: 99,
    };
    let operation = SteamworksNetworkingSocketsOperation::MessagesReceived {
        connection: connection_id(),
        messages: vec![message.clone()],
    };
    let result = SteamworksNetworkingSocketsResult::Ok(
        SteamworksNetworkingSocketsOperation::PollGroupMessagesReceived {
            poll_group: poll_group_id(),
            messages: vec![poll_group_message.clone()],
        },
    );

    let command_debug = format!("{command:?}");
    let batch_command_debug = format!("{batch_command:?}");
    let message_debug = format!("{message:?}");
    let poll_group_debug = format!("{poll_group_message:?}");
    let operation_debug = format!("{operation:?}");
    let result_debug = format!("{result:?}");

    assert!(command_debug.contains("data_len: 3"));
    assert!(!command_debug.contains("[1, 2, 3]"));
    assert!(batch_command_debug.contains("data_len: 3"));
    assert!(!batch_command_debug.contains("[10, 11, 12]"));
    assert!(message_debug.contains("data_len: 3"));
    assert!(!message_debug.contains("[4, 5, 6]"));
    assert!(poll_group_debug.contains("data_len: 3"));
    assert!(!poll_group_debug.contains("[7, 8, 9]"));
    assert!(operation_debug.contains("data_len: 3"));
    assert!(!operation_debug.contains("[4, 5, 6]"));
    assert!(result_debug.contains("data_len: 3"));
    assert!(!result_debug.contains("[7, 8, 9]"));
}

#[test]
fn state_records_bulk_poll_operations() {
    let mut state = SteamworksNetworkingSocketsState::default();
    let listen_socket_events = SteamworksNetworkingSocketsListenSocketEvents {
        listen_socket: listen_socket_id(),
        events: Vec::new(),
    };
    let connection_events = SteamworksNetworkingSocketsConnectionEvents {
        connection: connection_id(),
        events: Vec::new(),
        connection_removed: false,
    };

    state.record_operation(
        &SteamworksNetworkingSocketsOperation::AllListenSocketEventsPolled {
            listen_sockets: vec![listen_socket_events.clone()],
        },
    );
    state.record_operation(
        &SteamworksNetworkingSocketsOperation::AllConnectionEventsPolled {
            connections: vec![connection_events.clone()],
        },
    );

    assert_eq!(
        state.last_listen_socket_events(),
        Some(&listen_socket_events)
    );
    assert_eq!(state.last_connection_events(), Some(&connection_events));
}

#[test]
fn state_records_bulk_receive_operations() {
    let mut state = SteamworksNetworkingSocketsState::default();
    let peer = steamworks::networking_types::NetworkingIdentity::new_ip(localhost());
    let message = SteamworksNetworkingSocketsMessage {
        connection: connection_id(),
        peer: peer.clone(),
        data: vec![1, 2, 3],
        channel: 0,
        send_flags: steamworks::networking_types::SendFlags::RELIABLE,
        message_number: 1,
        connection_user_data: 10,
    };
    let poll_group_message = SteamworksNetworkingSocketsPollGroupMessage {
        poll_group: poll_group_id(),
        peer,
        data: vec![4, 5],
        channel: 1,
        send_flags: steamworks::networking_types::SendFlags::UNRELIABLE,
        message_number: 2,
        connection_user_data: 11,
    };

    state.record_operation(&SteamworksNetworkingSocketsOperation::AllMessagesReceived {
        connections: vec![SteamworksNetworkingSocketsConnectionMessages {
            connection: connection_id(),
            messages: vec![message.clone()],
        }],
    });
    state.record_operation(
        &SteamworksNetworkingSocketsOperation::AllPollGroupMessagesReceived {
            poll_groups: vec![SteamworksNetworkingSocketsPollGroupMessages {
                poll_group: poll_group_id(),
                messages: vec![poll_group_message.clone()],
            }],
        },
    );

    assert_eq!(state.received_count(), 2);
    assert_eq!(state.last_received_messages(), &[message]);
    assert_eq!(state.last_poll_group_messages(), &[poll_group_message]);
}

#[test]
fn state_lookup_caches_are_queryable_by_handle() {
    let mut state = SteamworksNetworkingSocketsState::default();
    let connection = connection_id();
    let listen_socket = listen_socket_id();
    let poll_group = poll_group_id();
    let peer = steamworks::networking_types::NetworkingIdentity::new_ip(localhost());
    let listen_batch = SteamworksNetworkingSocketsListenSocketEvents {
        listen_socket,
        events: vec![SteamworksListenSocketEventInfo::ConnectionRejected {
            listen_socket,
            remote: peer.clone(),
            user_data: 7,
        }],
    };
    let connection_batch = SteamworksNetworkingSocketsConnectionEvents {
        connection,
        events: vec![SteamworksNetworkingSocketsConnectionEventInfo {
            connection,
            new_state: steamworks::networking_types::NetworkingConnectionState::Connected,
            old_state: steamworks::networking_types::NetworkingConnectionState::Connecting,
        }],
        connection_removed: false,
    };
    let info = SteamworksNetworkingSocketsConnectionInfo {
        connection,
        state: steamworks::networking_types::NetworkingConnectionState::Connected,
        remote: Some(peer.clone()),
        user_data: 11,
        end_reason: None,
    };
    let realtime = SteamworksNetworkingSocketsRealtimeStatus {
        connection,
        connection_state: steamworks::networking_types::NetworkingConnectionState::Connected,
        ping: 12,
        connection_quality_local: 0.9,
        connection_quality_remote: 0.8,
        out_packets_per_sec: 1.0,
        out_bytes_per_sec: 2.0,
        in_packets_per_sec: 3.0,
        in_bytes_per_sec: 4.0,
        send_rate_bytes_per_sec: 5,
        pending_unreliable: 6,
        pending_reliable: 7,
        sent_unacked_reliable: 8,
        queued_send_bytes: 9,
        lanes: Vec::new(),
    };
    let sent = SteamworksNetworkingSocketsMessageSendResult {
        connection,
        send_flags: steamworks::networking_types::SendFlags::RELIABLE,
        channel: 1,
        bytes: 3,
        user_data: 4,
        result: Ok(5),
    };
    let received = SteamworksNetworkingSocketsMessage {
        connection,
        peer: peer.clone(),
        data: vec![1, 2, 3],
        channel: 1,
        send_flags: steamworks::networking_types::SendFlags::RELIABLE,
        message_number: 6,
        connection_user_data: 7,
    };
    let poll_group_message = SteamworksNetworkingSocketsPollGroupMessage {
        poll_group,
        peer,
        data: vec![4, 5],
        channel: 2,
        send_flags: steamworks::networking_types::SendFlags::UNRELIABLE,
        message_number: 8,
        connection_user_data: 9,
    };

    state.record_operation(
        &SteamworksNetworkingSocketsOperation::AllListenSocketEventsPolled {
            listen_sockets: vec![listen_batch.clone()],
        },
    );
    state.record_operation(
        &SteamworksNetworkingSocketsOperation::AllConnectionEventsPolled {
            connections: vec![connection_batch.clone()],
        },
    );
    state.record_operation(&SteamworksNetworkingSocketsOperation::ConnectionInfoRead {
        info: info.clone(),
    });
    state.record_operation(
        &SteamworksNetworkingSocketsOperation::RealtimeConnectionStatusRead {
            status: realtime.clone(),
        },
    );
    state.record_operation(&SteamworksNetworkingSocketsOperation::MessagesSent {
        messages: vec![sent.clone()],
    });
    state.record_operation(&SteamworksNetworkingSocketsOperation::AllMessagesReceived {
        connections: vec![SteamworksNetworkingSocketsConnectionMessages {
            connection,
            messages: vec![received.clone()],
        }],
    });
    state.record_operation(
        &SteamworksNetworkingSocketsOperation::AllPollGroupMessagesReceived {
            poll_groups: vec![SteamworksNetworkingSocketsPollGroupMessages {
                poll_group,
                messages: vec![poll_group_message.clone()],
            }],
        },
    );

    assert_eq!(state.listen_socket_events(), &[listen_batch.clone()]);
    assert_eq!(
        state.listen_socket_event_batch(listen_socket),
        Some(&listen_batch)
    );
    assert_eq!(state.connection_events(), &[connection_batch.clone()]);
    assert_eq!(
        state.connection_event_batch(connection),
        Some(&connection_batch)
    );
    assert_eq!(state.connection_infos(), &[info.clone()]);
    assert_eq!(state.connection_info(connection), Some(&info));
    assert_eq!(state.realtime_statuses(), &[realtime.clone()]);
    assert_eq!(state.realtime_status(connection), Some(&realtime));
    assert_eq!(state.recent_sent_messages(), &[sent.clone()]);
    assert_eq!(
        state
            .sent_messages_for_connection(connection)
            .cloned()
            .collect::<Vec<_>>(),
        vec![sent.clone()]
    );
    assert_eq!(
        state.last_sent_message_for_connection(connection),
        Some(&sent)
    );
    assert_eq!(state.recent_received_messages(), &[received.clone()]);
    assert_eq!(
        state
            .received_messages_for_connection(connection)
            .cloned()
            .collect::<Vec<_>>(),
        vec![received.clone()]
    );
    assert_eq!(
        state.last_received_message_for_connection(connection),
        Some(&received)
    );
    assert_eq!(
        state.recent_poll_group_messages(),
        &[poll_group_message.clone()]
    );
    assert_eq!(
        state
            .poll_group_messages(poll_group)
            .cloned()
            .collect::<Vec<_>>(),
        vec![poll_group_message.clone()]
    );
    assert_eq!(
        state.last_poll_group_message(poll_group),
        Some(&poll_group_message)
    );

    state.record_operation(&SteamworksNetworkingSocketsOperation::ConnectionClosed {
        connection,
        close_succeeded: true,
    });
    state.record_operation(&SteamworksNetworkingSocketsOperation::ListenSocketClosed {
        listen_socket,
        closed_connections: Vec::new(),
    });
    state.record_operation(&SteamworksNetworkingSocketsOperation::PollGroupClosed { poll_group });

    assert_eq!(state.connection_info(connection), None);
    assert_eq!(state.realtime_status(connection), None);
    assert_eq!(
        state.connection_event_batch(connection),
        Some(&connection_batch)
    );
    assert_eq!(state.listen_socket_event_batch(listen_socket), None);
    assert_eq!(state.last_poll_group_message(poll_group), None);
}

#[test]
fn state_lookup_caches_are_bounded() {
    let mut state = SteamworksNetworkingSocketsState::default();
    let peer = steamworks::networking_types::NetworkingIdentity::new_ip(localhost());

    for raw in 1..=(super::state::STEAMWORKS_NETWORKING_SOCKETS_STATE_CACHE_LIMIT as u64 + 1) {
        let connection = SteamworksNetworkingSocketsConnectionId::from_raw(raw);
        let listen_socket = SteamworksListenSocketId::from_raw(raw);
        let poll_group = SteamworksNetworkingSocketsPollGroupId::from_raw(raw);
        state.record_operation(
            &SteamworksNetworkingSocketsOperation::ListenSocketEventsPolled {
                listen_socket,
                events: Vec::new(),
            },
        );
        state.record_operation(
            &SteamworksNetworkingSocketsOperation::ConnectionEventsPolled {
                connection,
                events: Vec::new(),
                connection_removed: false,
            },
        );
        state.record_operation(&SteamworksNetworkingSocketsOperation::ConnectionInfoRead {
            info: SteamworksNetworkingSocketsConnectionInfo {
                connection,
                state: steamworks::networking_types::NetworkingConnectionState::Connected,
                remote: Some(peer.clone()),
                user_data: raw as i64,
                end_reason: None,
            },
        });
        state.record_operation(
            &SteamworksNetworkingSocketsOperation::RealtimeConnectionStatusRead {
                status: SteamworksNetworkingSocketsRealtimeStatus {
                    connection,
                    connection_state:
                        steamworks::networking_types::NetworkingConnectionState::Connected,
                    ping: raw as i32,
                    connection_quality_local: 1.0,
                    connection_quality_remote: 1.0,
                    out_packets_per_sec: 0.0,
                    out_bytes_per_sec: 0.0,
                    in_packets_per_sec: 0.0,
                    in_bytes_per_sec: 0.0,
                    send_rate_bytes_per_sec: 0,
                    pending_unreliable: 0,
                    pending_reliable: 0,
                    sent_unacked_reliable: 0,
                    queued_send_bytes: 0,
                    lanes: Vec::new(),
                },
            },
        );
        state.record_operation(&SteamworksNetworkingSocketsOperation::MessagesSent {
            messages: vec![SteamworksNetworkingSocketsMessageSendResult {
                connection,
                send_flags: steamworks::networking_types::SendFlags::RELIABLE,
                channel: 0,
                bytes: raw as usize,
                user_data: 0,
                result: Ok(raw),
            }],
        });
        state.record_operation(&SteamworksNetworkingSocketsOperation::MessagesReceived {
            connection,
            messages: vec![SteamworksNetworkingSocketsMessage {
                connection,
                peer: peer.clone(),
                data: vec![raw as u8],
                channel: 0,
                send_flags: steamworks::networking_types::SendFlags::RELIABLE,
                message_number: raw,
                connection_user_data: 0,
            }],
        });
        state.record_operation(
            &SteamworksNetworkingSocketsOperation::PollGroupMessagesReceived {
                poll_group,
                messages: vec![SteamworksNetworkingSocketsPollGroupMessage {
                    poll_group,
                    peer: peer.clone(),
                    data: vec![raw as u8],
                    channel: 0,
                    send_flags: steamworks::networking_types::SendFlags::UNRELIABLE,
                    message_number: raw,
                    connection_user_data: 0,
                }],
            },
        );
    }

    let first_connection = SteamworksNetworkingSocketsConnectionId::from_raw(1);
    let second_connection = SteamworksNetworkingSocketsConnectionId::from_raw(2);
    let first_listen_socket = SteamworksListenSocketId::from_raw(1);
    let second_listen_socket = SteamworksListenSocketId::from_raw(2);
    let first_poll_group = SteamworksNetworkingSocketsPollGroupId::from_raw(1);
    let second_poll_group = SteamworksNetworkingSocketsPollGroupId::from_raw(2);

    assert_eq!(
        state.listen_socket_events().len(),
        super::state::STEAMWORKS_NETWORKING_SOCKETS_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.connection_events().len(),
        super::state::STEAMWORKS_NETWORKING_SOCKETS_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.connection_infos().len(),
        super::state::STEAMWORKS_NETWORKING_SOCKETS_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.realtime_statuses().len(),
        super::state::STEAMWORKS_NETWORKING_SOCKETS_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.recent_sent_messages().len(),
        super::state::STEAMWORKS_NETWORKING_SOCKETS_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.recent_received_messages().len(),
        super::state::STEAMWORKS_NETWORKING_SOCKETS_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.recent_poll_group_messages().len(),
        super::state::STEAMWORKS_NETWORKING_SOCKETS_STATE_CACHE_LIMIT
    );
    assert_eq!(state.listen_socket_event_batch(first_listen_socket), None);
    assert_eq!(state.connection_event_batch(first_connection), None);
    assert_eq!(state.connection_info(first_connection), None);
    assert_eq!(state.realtime_status(first_connection), None);
    assert_eq!(
        state.last_sent_message_for_connection(first_connection),
        None
    );
    assert_eq!(
        state.last_received_message_for_connection(first_connection),
        None
    );
    assert_eq!(state.last_poll_group_message(first_poll_group), None);
    assert!(state
        .listen_socket_event_batch(second_listen_socket)
        .is_some());
    assert!(state.connection_event_batch(second_connection).is_some());
    assert!(state.connection_info(second_connection).is_some());
    assert!(state.realtime_status(second_connection).is_some());
    assert!(state
        .last_sent_message_for_connection(second_connection)
        .is_some());
    assert!(state
        .last_received_message_for_connection(second_connection)
        .is_some());
    assert!(state.last_poll_group_message(second_poll_group).is_some());
}

#[test]
fn state_records_bulk_flush_operation() {
    let mut state = SteamworksNetworkingSocketsState::default();
    let first = SteamworksNetworkingSocketsConnectionId::from_raw(1);
    let second = SteamworksNetworkingSocketsConnectionId::from_raw(2);

    state.record_operation(&SteamworksNetworkingSocketsOperation::AllMessagesFlushed {
        connections: vec![first, second],
    });

    assert_eq!(state.last_flushed_connection(), Some(second));
}

#[test]
fn state_records_bulk_close_operations() {
    let mut state = SteamworksNetworkingSocketsState::default();
    let first = SteamworksNetworkingSocketsConnectionId::from_raw(1);
    let second = SteamworksNetworkingSocketsConnectionId::from_raw(2);
    let first_listen = SteamworksListenSocketId::from_raw(3);
    let second_listen = SteamworksListenSocketId::from_raw(4);
    let first_poll_group = SteamworksNetworkingSocketsPollGroupId::from_raw(5);
    let second_poll_group = SteamworksNetworkingSocketsPollGroupId::from_raw(6);

    state.record_operation(
        &SteamworksNetworkingSocketsOperation::AllConnectionsClosed {
            connections: vec![
                SteamworksNetworkingSocketsConnectionClosed {
                    connection: first,
                    close_succeeded: true,
                },
                SteamworksNetworkingSocketsConnectionClosed {
                    connection: second,
                    close_succeeded: false,
                },
            ],
        },
    );
    state.record_operation(
        &SteamworksNetworkingSocketsOperation::AllListenSocketsClosed {
            listen_sockets: vec![
                SteamworksNetworkingSocketsListenSocketClosed {
                    listen_socket: first_listen,
                    closed_connections: vec![first],
                },
                SteamworksNetworkingSocketsListenSocketClosed {
                    listen_socket: second_listen,
                    closed_connections: vec![second],
                },
            ],
        },
    );
    state.record_operation(&SteamworksNetworkingSocketsOperation::AllPollGroupsClosed {
        poll_groups: vec![first_poll_group, second_poll_group],
    });

    assert_eq!(
        state.last_closed_connection(),
        Some(&SteamworksNetworkingSocketsConnectionClosed {
            connection: second,
            close_succeeded: false,
        })
    );
    assert_eq!(
        state.last_closed_listen_socket(),
        Some(&SteamworksNetworkingSocketsListenSocketClosed {
            listen_socket: second_listen,
            closed_connections: vec![second],
        })
    );
    assert_eq!(state.last_closed_poll_group(), Some(second_poll_group));
}

#[test]
fn state_records_operations_without_unbounded_message_history() {
    let mut state = SteamworksNetworkingSocketsState::default();
    let endpoint = SteamworksNetworkingSocketsListenEndpoint::Ip(localhost());
    let target = SteamworksNetworkingSocketsConnectionTarget::Ip(localhost());
    let peer = steamworks::networking_types::NetworkingIdentity::new_ip(localhost());
    let listen_event = SteamworksListenSocketEventInfo::ConnectionRejected {
        listen_socket: listen_socket_id(),
        remote: peer.clone(),
        user_data: 17,
    };
    let connection_event = SteamworksNetworkingSocketsConnectionEventInfo {
        connection: connection_id(),
        new_state: steamworks::networking_types::NetworkingConnectionState::ClosedByPeer,
        old_state: steamworks::networking_types::NetworkingConnectionState::Connecting,
    };
    let info = SteamworksNetworkingSocketsConnectionInfo {
        connection: connection_id(),
        state: steamworks::networking_types::NetworkingConnectionState::Connected,
        remote: Some(peer.clone()),
        user_data: 21,
        end_reason: None,
    };
    let realtime_status = SteamworksNetworkingSocketsRealtimeStatus {
        connection: connection_id(),
        connection_state: steamworks::networking_types::NetworkingConnectionState::Connected,
        ping: 42,
        connection_quality_local: 0.95,
        connection_quality_remote: 0.9,
        out_packets_per_sec: 10.0,
        out_bytes_per_sec: 1000.0,
        in_packets_per_sec: 11.0,
        in_bytes_per_sec: 1100.0,
        send_rate_bytes_per_sec: 1200,
        pending_unreliable: 1,
        pending_reliable: 2,
        sent_unacked_reliable: 3,
        queued_send_bytes: 4,
        lanes: vec![SteamworksNetworkingSocketsRealtimeLaneStatus {
            pending_unreliable: 5,
            pending_reliable: 6,
            sent_unacked_reliable: 7,
            queued_send_bytes: 8,
        }],
    };

    state.record_operation(
        &SteamworksNetworkingSocketsOperation::AuthenticationInitialized {
            availability: Ok(steamworks::networking_types::NetworkingAvailability::Attempting),
        },
    );
    state.record_operation(
        &SteamworksNetworkingSocketsOperation::AuthenticationStatusRead {
            availability: Ok(steamworks::networking_types::NetworkingAvailability::Current),
        },
    );
    state.record_operation(&SteamworksNetworkingSocketsOperation::ListenSocketCreated {
        listen_socket: listen_socket_id(),
        endpoint: endpoint.clone(),
    });
    state.record_operation(&SteamworksNetworkingSocketsOperation::ConnectionCreated {
        connection: connection_id(),
        target: target.clone(),
    });
    state.record_operation(&SteamworksNetworkingSocketsOperation::PollGroupCreated {
        poll_group: poll_group_id(),
    });
    state.record_operation(
        &SteamworksNetworkingSocketsOperation::ListenSocketEventsPolled {
            listen_socket: listen_socket_id(),
            events: vec![listen_event.clone()],
        },
    );
    state.record_operation(
        &SteamworksNetworkingSocketsOperation::ConnectionEventsPolled {
            connection: connection_id(),
            events: vec![connection_event.clone()],
            connection_removed: true,
        },
    );
    state.record_operation(&SteamworksNetworkingSocketsOperation::ConnectionInfoRead {
        info: info.clone(),
    });
    state.record_operation(
        &SteamworksNetworkingSocketsOperation::ConnectionUserDataRead {
            connection: connection_id(),
            user_data: 122,
        },
    );
    state.record_operation(
        &SteamworksNetworkingSocketsOperation::RealtimeConnectionStatusRead {
            status: realtime_status.clone(),
        },
    );
    let first = SteamworksNetworkingSocketsMessage {
        connection: connection_id(),
        peer: peer.clone(),
        data: vec![1],
        channel: 0,
        send_flags: steamworks::networking_types::SendFlags::RELIABLE,
        message_number: 1,
        connection_user_data: 0,
    };
    let second = SteamworksNetworkingSocketsMessage {
        data: vec![2, 3],
        message_number: 2,
        ..first.clone()
    };

    state.record_operation(&SteamworksNetworkingSocketsOperation::MessagesReceived {
        connection: connection_id(),
        messages: vec![first],
    });
    state.record_operation(&SteamworksNetworkingSocketsOperation::MessagesReceived {
        connection: connection_id(),
        messages: vec![second.clone()],
    });
    let poll_group_message = SteamworksNetworkingSocketsPollGroupMessage {
        poll_group: poll_group_id(),
        peer,
        data: vec![9],
        channel: 1,
        send_flags: steamworks::networking_types::SendFlags::UNRELIABLE,
        message_number: 4,
        connection_user_data: 99,
    };
    state.record_operation(
        &SteamworksNetworkingSocketsOperation::PollGroupMessagesReceived {
            poll_group: poll_group_id(),
            messages: vec![poll_group_message.clone()],
        },
    );
    state.record_operation(&SteamworksNetworkingSocketsOperation::MessageSent {
        connection: connection_id(),
        message_number: 3,
        bytes: 2,
    });
    let send_results = vec![
        SteamworksNetworkingSocketsMessageSendResult {
            connection: connection_id(),
            send_flags: steamworks::networking_types::SendFlags::RELIABLE,
            channel: 1,
            bytes: 4,
            user_data: 10,
            result: Ok(5),
        },
        SteamworksNetworkingSocketsMessageSendResult {
            connection: connection_id(),
            send_flags: steamworks::networking_types::SendFlags::UNRELIABLE,
            channel: 2,
            bytes: 6,
            user_data: 11,
            result: Err(steamworks::SteamError::InvalidState),
        },
    ];
    state.record_operation(&SteamworksNetworkingSocketsOperation::MessagesSent {
        messages: send_results.clone(),
    });
    state.record_operation(&SteamworksNetworkingSocketsOperation::MessagesFlushed {
        connection: connection_id(),
    });
    state.record_operation(
        &SteamworksNetworkingSocketsOperation::ConnectionPollGroupSet {
            connection: connection_id(),
            poll_group: poll_group_id(),
        },
    );
    state.record_operation(
        &SteamworksNetworkingSocketsOperation::ConnectionPollGroupCleared {
            connection: connection_id(),
        },
    );
    state.record_operation(
        &SteamworksNetworkingSocketsOperation::ConnectionLanesConfigured {
            connection: connection_id(),
            lanes: 2,
        },
    );
    state.record_operation(
        &SteamworksNetworkingSocketsOperation::ConnectionUserDataSet {
            connection: connection_id(),
            user_data: 123,
        },
    );
    state.record_operation(&SteamworksNetworkingSocketsOperation::ConnectionNameSet {
        connection: connection_id(),
        name: "player-1".to_owned(),
    });
    state.record_operation(&SteamworksNetworkingSocketsOperation::ConnectionClosed {
        connection: connection_id(),
        close_succeeded: false,
    });
    state.record_operation(&SteamworksNetworkingSocketsOperation::ListenSocketClosed {
        listen_socket: listen_socket_id(),
        closed_connections: vec![connection_id()],
    });
    state.record_operation(&SteamworksNetworkingSocketsOperation::PollGroupClosed {
        poll_group: poll_group_id(),
    });

    assert_eq!(
        state.last_authentication_status(),
        Some(&Ok(
            steamworks::networking_types::NetworkingAvailability::Current
        ))
    );
    assert_eq!(
        state.last_created_listen_socket(),
        Some(&SteamworksNetworkingSocketsListenSocketCreated {
            listen_socket: listen_socket_id(),
            endpoint,
        })
    );
    assert_eq!(
        state.last_created_connection(),
        Some(&SteamworksNetworkingSocketsConnectionCreated {
            connection: connection_id(),
            target,
        })
    );
    assert_eq!(state.last_created_poll_group(), Some(poll_group_id()));
    assert_eq!(
        state.last_listen_socket_events(),
        Some(&SteamworksNetworkingSocketsListenSocketEvents {
            listen_socket: listen_socket_id(),
            events: vec![listen_event],
        })
    );
    assert_eq!(
        state.last_connection_events(),
        Some(&SteamworksNetworkingSocketsConnectionEvents {
            connection: connection_id(),
            events: vec![connection_event],
            connection_removed: true,
        })
    );
    assert_eq!(state.last_connection_info(), Some(&info));
    assert_eq!(state.last_realtime_status(), Some(&realtime_status));
    assert_eq!(state.received_count(), 3);
    assert_eq!(state.sent_count(), 2);
    assert_eq!(
        state.last_sent_message(),
        Some(&SteamworksNetworkingSocketsSentMessage {
            connection: connection_id(),
            message_number: 3,
            bytes: 2,
        })
    );
    assert_eq!(state.last_sent_messages(), send_results.as_slice());
    assert_eq!(state.last_received_messages(), &[second]);
    assert_eq!(state.last_poll_group_messages(), &[poll_group_message]);
    assert_eq!(state.last_flushed_connection(), Some(connection_id()));
    assert_eq!(
        state.last_connection_poll_group_set(),
        Some(&SteamworksNetworkingSocketsPollGroupAssignment {
            connection: connection_id(),
            poll_group: poll_group_id(),
        })
    );
    assert_eq!(
        state.last_connection_poll_group_cleared(),
        Some(connection_id())
    );
    assert_eq!(
        state.last_connection_lanes_configured(),
        Some(&SteamworksNetworkingSocketsLaneConfiguration {
            connection: connection_id(),
            lanes: 2,
        })
    );
    assert_eq!(
        state.last_connection_user_data(),
        Some(&SteamworksNetworkingSocketsConnectionUserData {
            connection: connection_id(),
            user_data: 123,
        })
    );
    assert_eq!(
        state.last_connection_name(),
        Some(&SteamworksNetworkingSocketsConnectionName {
            connection: connection_id(),
            name: "player-1".to_owned(),
        })
    );
    assert_eq!(
        state.last_closed_connection(),
        Some(&SteamworksNetworkingSocketsConnectionClosed {
            connection: connection_id(),
            close_succeeded: false,
        })
    );
    assert_eq!(
        state.last_closed_listen_socket(),
        Some(&SteamworksNetworkingSocketsListenSocketClosed {
            listen_socket: listen_socket_id(),
            closed_connections: vec![connection_id()],
        })
    );
    assert_eq!(state.last_closed_poll_group(), Some(poll_group_id()));
}

#[test]
fn id_round_trips_raw_values() {
    assert_eq!(SteamworksListenSocketId::from_raw(5).raw(), 5);
    assert_eq!(
        SteamworksNetworkingSocketsConnectionId::from_raw(6).raw(),
        6
    );
    assert_eq!(SteamworksNetworkingSocketsPollGroupId::from_raw(8).raw(), 8);
}

#[test]
fn handle_storage_starts_ids_at_one() {
    let storage = SteamworksNetworkingSocketsHandleStorage::default();

    assert_eq!(storage.next_listen_socket_id, 1);
    assert_eq!(storage.next_connection_id, 1);
    assert_eq!(storage.next_poll_group_id, 1);
}

#[test]
fn handle_storage_tracks_and_clears_handle_owners() {
    let mut storage = SteamworksNetworkingSocketsHandleStorage::default();

    storage.listen_socket_owners.insert(
        listen_socket_id(),
        SteamworksNetworkingSocketsHandleOwner::Server,
    );
    storage.connection_owners.insert(
        connection_id(),
        SteamworksNetworkingSocketsHandleOwner::Server,
    );
    storage.poll_group_owners.insert(
        poll_group_id(),
        SteamworksNetworkingSocketsHandleOwner::Server,
    );
    storage.connection_metadata.insert(
        connection_id(),
        SteamworksNetworkingSocketsConnectionMetadata::listen_socket(
            listen_socket_id(),
            steamworks::networking_types::NetworkingIdentity::new_ip(localhost()),
            0,
        ),
    );

    assert_eq!(
        storage.listen_socket_owner(listen_socket_id()),
        Some(SteamworksNetworkingSocketsHandleOwner::Server)
    );
    assert_eq!(
        storage.connection_owner(connection_id()),
        Some(SteamworksNetworkingSocketsHandleOwner::Server)
    );
    assert_eq!(
        storage.poll_group_owner(poll_group_id()),
        Some(SteamworksNetworkingSocketsHandleOwner::Server)
    );

    assert!(storage.remove_connection(&connection_id()).is_none());
    assert_eq!(storage.connection_owner(connection_id()), None);
    assert!(!storage.connection_metadata.contains_key(&connection_id()));
}

#[test]
fn connection_user_data_read_preserves_unset_sentinel() {
    assert_eq!(
        super::commands::helpers::connection_user_data_from_info_result(Ok(-1)),
        Ok(-1)
    );
    assert_eq!(
        super::commands::helpers::connection_user_data_from_info_result(Ok(123)),
        Ok(123)
    );
    assert_eq!(
        super::commands::helpers::connection_user_data_from_info_result(Err(
            steamworks::networking_sockets::InvalidHandle,
        )),
        Err(SteamworksNetworkingSocketsError::InvalidHandle {
            operation: "net_connection.info",
        })
    );
}

#[test]
fn batch_send_checks_client_allocator_before_server_owned_connections() {
    let mut storage = SteamworksNetworkingSocketsHandleStorage::default();
    storage.connection_owners.insert(
        connection_id(),
        SteamworksNetworkingSocketsHandleOwner::Server,
    );
    let command = SteamworksNetworkingSocketsCommand::send_messages(vec![
        SteamworksNetworkingSocketsOutboundMessage::new(
            connection_id(),
            steamworks::networking_types::SendFlags::RELIABLE,
            [1, 2, 3],
        ),
    ]);

    assert_eq!(
        super::commands::handle_networking_sockets_command(None, None, &mut storage, &command),
        Err(SteamworksNetworkingSocketsError::ClientUnavailable)
    );
}

#[test]
fn poll_group_assignment_rejects_mismatched_handle_owners() {
    let mut storage = SteamworksNetworkingSocketsHandleStorage::default();
    storage.connection_owners.insert(
        connection_id(),
        SteamworksNetworkingSocketsHandleOwner::Server,
    );
    storage.poll_group_owners.insert(
        poll_group_id(),
        SteamworksNetworkingSocketsHandleOwner::Client,
    );
    let command = SteamworksNetworkingSocketsCommand::set_connection_poll_group(
        connection_id(),
        poll_group_id(),
    );

    assert_eq!(
        super::commands::handle_networking_sockets_command(None, None, &mut storage, &command),
        Err(SteamworksNetworkingSocketsError::HandleOwnerMismatch {
            connection: connection_id(),
            poll_group: poll_group_id(),
        })
    );
}

#[test]
fn connection_metadata_tracks_poll_group_membership() {
    let mut storage = SteamworksNetworkingSocketsHandleStorage::default();

    storage.connection_metadata.insert(
        connection_id(),
        SteamworksNetworkingSocketsConnectionMetadata::independent(),
    );

    storage.set_connection_poll_group(connection_id(), poll_group_id());
    assert_eq!(
        storage
            .connection_metadata
            .get(&connection_id())
            .and_then(|metadata| metadata.poll_group),
        Some(poll_group_id())
    );

    storage.clear_connection_poll_group(connection_id());
    assert_eq!(
        storage
            .connection_metadata
            .get(&connection_id())
            .and_then(|metadata| metadata.poll_group),
        None
    );
}

#[test]
fn poll_group_metadata_cleanup_clears_all_matching_connections() {
    let mut storage = SteamworksNetworkingSocketsHandleStorage::default();
    let first = SteamworksNetworkingSocketsConnectionId::from_raw(1);
    let second = SteamworksNetworkingSocketsConnectionId::from_raw(2);
    let other = SteamworksNetworkingSocketsConnectionId::from_raw(3);
    let other_poll_group = SteamworksNetworkingSocketsPollGroupId::from_raw(10);

    for connection in [first, second, other] {
        storage.connection_metadata.insert(
            connection,
            SteamworksNetworkingSocketsConnectionMetadata::independent(),
        );
    }
    storage.set_connection_poll_group(first, poll_group_id());
    storage.set_connection_poll_group(second, poll_group_id());
    storage.set_connection_poll_group(other, other_poll_group);

    assert_eq!(storage.clear_poll_group_metadata(poll_group_id()), 2);
    assert_eq!(
        storage
            .connection_metadata
            .get(&first)
            .and_then(|metadata| metadata.poll_group),
        None
    );
    assert_eq!(
        storage
            .connection_metadata
            .get(&second)
            .and_then(|metadata| metadata.poll_group),
        None
    );
    assert_eq!(
        storage
            .connection_metadata
            .get(&other)
            .and_then(|metadata| metadata.poll_group),
        Some(other_poll_group)
    );
}

#[test]
fn missing_poll_group_remove_does_not_clear_connection_metadata() {
    let mut storage = SteamworksNetworkingSocketsHandleStorage::default();

    storage.connection_metadata.insert(
        connection_id(),
        SteamworksNetworkingSocketsConnectionMetadata::independent(),
    );
    storage.set_connection_poll_group(connection_id(), poll_group_id());

    assert!(storage.remove_poll_group(&poll_group_id()).is_none());
    assert_eq!(
        storage
            .connection_metadata
            .get(&connection_id())
            .and_then(|metadata| metadata.poll_group),
        Some(poll_group_id())
    );
}

#[test]
fn connection_metadata_matches_listen_socket_disconnects() {
    let mut storage = SteamworksNetworkingSocketsHandleStorage::default();
    let remote = steamworks::networking_types::NetworkingIdentity::new_ip(localhost());
    let end_reason = steamworks::networking_types::NetConnectionEnd::App(
        steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
    );
    let first = SteamworksNetworkingSocketsConnectionId::from_raw(1);
    let second = SteamworksNetworkingSocketsConnectionId::from_raw(2);

    storage.connection_metadata.insert(
        first,
        SteamworksNetworkingSocketsConnectionMetadata::listen_socket(
            listen_socket_id(),
            remote.clone(),
            10,
        ),
    );
    storage.connection_metadata.insert(
        second,
        SteamworksNetworkingSocketsConnectionMetadata::listen_socket(
            listen_socket_id(),
            remote.clone(),
            20,
        ),
    );

    assert_eq!(
        storage.remove_listen_connection_by_event(listen_socket_id(), &remote, 20, end_reason),
        Some(second)
    );
    assert!(storage.connection_metadata.contains_key(&first));
    assert!(!storage.connection_metadata.contains_key(&second));
}

#[test]
fn duplicate_listen_socket_disconnect_metadata_is_not_removed_without_terminal_info() {
    let mut storage = SteamworksNetworkingSocketsHandleStorage::default();
    let remote = steamworks::networking_types::NetworkingIdentity::new_ip(localhost());
    let end_reason = steamworks::networking_types::NetConnectionEnd::App(
        steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
    );
    let first = SteamworksNetworkingSocketsConnectionId::from_raw(1);
    let second = SteamworksNetworkingSocketsConnectionId::from_raw(2);

    for connection in [first, second] {
        storage.connection_metadata.insert(
            connection,
            SteamworksNetworkingSocketsConnectionMetadata::listen_socket(
                listen_socket_id(),
                remote.clone(),
                10,
            ),
        );
    }

    assert_eq!(
        storage.remove_listen_connection_by_event(listen_socket_id(), &remote, 10, end_reason),
        None
    );
    assert!(storage.connection_metadata.contains_key(&first));
    assert!(storage.connection_metadata.contains_key(&second));
}

#[test]
fn p2p_connect_constructor_accepts_steam_id() {
    let command = SteamworksNetworkingSocketsCommand::connect_p2p_steam_id(
        steamworks::SteamId::from_raw(123),
        0,
    );

    let SteamworksNetworkingSocketsCommand::ConnectP2p {
        identity,
        remote_virtual_port,
        options,
    } = command
    else {
        panic!("expected ConnectP2p command");
    };

    assert_eq!(identity.debug_string(), "steamid:123");
    assert_eq!(remote_virtual_port, 0);
    assert!(options.is_empty());
}

#[test]
fn request_policy_default_rejects() {
    assert!(matches!(
        SteamworksConnectionRequestPolicy::default(),
        SteamworksConnectionRequestPolicy::Reject { .. }
    ));
}

#[test]
fn socket_addr_from_str_is_tested_for_ipv6_coverage() {
    assert!(SocketAddr::from_str("[::1]:27015").is_ok());
}
