use std::net::SocketAddr;

use super::super::super::{
    SteamworksConnectionRequestPolicy, SteamworksListenSocketId,
    SteamworksNetworkingSocketsConfigEntry, SteamworksNetworkingSocketsConnectionId,
    SteamworksNetworkingSocketsOutboundMessage, SteamworksNetworkingSocketsPollGroupId,
};
use super::SteamworksNetworkingSocketsCommand;

impl SteamworksNetworkingSocketsCommand {
    /// Creates a [`SteamworksNetworkingSocketsCommand::InitAuthentication`] command.
    pub fn init_authentication() -> Self {
        Self::InitAuthentication
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::GetAuthenticationStatus`] command.
    pub fn get_authentication_status() -> Self {
        Self::GetAuthenticationStatus
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CreateListenSocketIp`] command.
    pub fn create_listen_socket_ip(local_address: SocketAddr) -> Self {
        Self::create_listen_socket_ip_with_options(local_address, [])
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CreateListenSocketIp`] command with config.
    pub fn create_listen_socket_ip_with_options(
        local_address: SocketAddr,
        options: impl Into<Vec<SteamworksNetworkingSocketsConfigEntry>>,
    ) -> Self {
        Self::CreateListenSocketIp {
            local_address,
            options: options.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CreateListenSocketP2p`] command.
    pub fn create_listen_socket_p2p(local_virtual_port: i32) -> Self {
        Self::create_listen_socket_p2p_with_options(local_virtual_port, [])
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CreateListenSocketP2p`] command with config.
    pub fn create_listen_socket_p2p_with_options(
        local_virtual_port: i32,
        options: impl Into<Vec<SteamworksNetworkingSocketsConfigEntry>>,
    ) -> Self {
        Self::CreateListenSocketP2p {
            local_virtual_port,
            options: options.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CreateHostedDedicatedServerListenSocket`] command.
    ///
    /// This command requires a [`crate::SteamworksServer`] resource.
    pub fn create_hosted_dedicated_server_listen_socket(local_virtual_port: u32) -> Self {
        Self::create_hosted_dedicated_server_listen_socket_with_options(local_virtual_port, [])
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CreateHostedDedicatedServerListenSocket`] command with config.
    ///
    /// This command requires a [`crate::SteamworksServer`] resource.
    pub fn create_hosted_dedicated_server_listen_socket_with_options(
        local_virtual_port: u32,
        options: impl Into<Vec<SteamworksNetworkingSocketsConfigEntry>>,
    ) -> Self {
        Self::CreateHostedDedicatedServerListenSocket {
            local_virtual_port,
            options: options.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ConnectByIpAddress`] command.
    pub fn connect_by_ip_address(address: SocketAddr) -> Self {
        Self::connect_by_ip_address_with_options(address, [])
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ConnectByIpAddress`] command with config.
    pub fn connect_by_ip_address_with_options(
        address: SocketAddr,
        options: impl Into<Vec<SteamworksNetworkingSocketsConfigEntry>>,
    ) -> Self {
        Self::ConnectByIpAddress {
            address,
            options: options.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ConnectP2p`] command.
    pub fn connect_p2p(
        identity: steamworks::networking_types::NetworkingIdentity,
        remote_virtual_port: i32,
    ) -> Self {
        Self::connect_p2p_with_options(identity, remote_virtual_port, [])
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ConnectP2p`] command with config.
    pub fn connect_p2p_with_options(
        identity: steamworks::networking_types::NetworkingIdentity,
        remote_virtual_port: i32,
        options: impl Into<Vec<SteamworksNetworkingSocketsConfigEntry>>,
    ) -> Self {
        Self::ConnectP2p {
            identity,
            remote_virtual_port,
            options: options.into(),
        }
    }

    /// Creates a P2P connect command for a Steam user.
    pub fn connect_p2p_steam_id(steam_id: steamworks::SteamId, remote_virtual_port: i32) -> Self {
        Self::connect_p2p_steam_id_with_options(steam_id, remote_virtual_port, [])
    }

    /// Creates a P2P connect command for a Steam user with config.
    pub fn connect_p2p_steam_id_with_options(
        steam_id: steamworks::SteamId,
        remote_virtual_port: i32,
        options: impl Into<Vec<SteamworksNetworkingSocketsConfigEntry>>,
    ) -> Self {
        Self::connect_p2p_with_options(
            steamworks::networking_types::NetworkingIdentity::new_steam_id(steam_id),
            remote_virtual_port,
            options,
        )
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CreatePollGroup`] command.
    pub fn create_poll_group() -> Self {
        Self::CreatePollGroup
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CreateServerPollGroup`] command.
    ///
    /// This command requires a [`crate::SteamworksServer`] resource.
    pub fn create_server_poll_group() -> Self {
        Self::CreateServerPollGroup
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::PollListenSocketEvents`] command.
    pub fn poll_listen_socket_events(
        listen_socket: SteamworksListenSocketId,
        max_events: usize,
        request_policy: SteamworksConnectionRequestPolicy,
    ) -> Self {
        Self::PollListenSocketEvents {
            listen_socket,
            max_events,
            request_policy,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::PollAllListenSocketEvents`] command.
    pub fn poll_all_listen_socket_events(
        max_events_per_socket: usize,
        request_policy: SteamworksConnectionRequestPolicy,
    ) -> Self {
        Self::PollAllListenSocketEvents {
            max_events_per_socket,
            request_policy,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::PollConnectionEvents`] command.
    pub fn poll_connection_events(
        connection: SteamworksNetworkingSocketsConnectionId,
        max_events: usize,
    ) -> Self {
        Self::PollConnectionEvents {
            connection,
            max_events,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::PollAllConnectionEvents`] command.
    pub fn poll_all_connection_events(max_events_per_connection: usize) -> Self {
        Self::PollAllConnectionEvents {
            max_events_per_connection,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::GetConnectionInfo`] command.
    pub fn get_connection_info(connection: SteamworksNetworkingSocketsConnectionId) -> Self {
        Self::GetConnectionInfo { connection }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::GetConnectionUserData`] command.
    pub fn get_connection_user_data(connection: SteamworksNetworkingSocketsConnectionId) -> Self {
        Self::GetConnectionUserData { connection }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::GetRealtimeConnectionStatus`] command.
    pub fn get_realtime_connection_status(
        connection: SteamworksNetworkingSocketsConnectionId,
        lanes: u32,
    ) -> Self {
        Self::GetRealtimeConnectionStatus { connection, lanes }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::SendMessage`] command.
    pub fn send_message(
        connection: SteamworksNetworkingSocketsConnectionId,
        send_flags: steamworks::networking_types::SendFlags,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self::SendMessage {
            connection,
            send_flags,
            data: data.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::SendMessages`] command.
    pub fn send_messages(
        messages: impl Into<Vec<SteamworksNetworkingSocketsOutboundMessage>>,
    ) -> Self {
        Self::SendMessages {
            messages: messages.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ReceiveMessages`] command.
    pub fn receive_messages(
        connection: SteamworksNetworkingSocketsConnectionId,
        batch_size: usize,
    ) -> Self {
        Self::ReceiveMessages {
            connection,
            batch_size,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ReceiveAllMessages`] command.
    pub fn receive_all_messages(batch_size_per_connection: usize) -> Self {
        Self::ReceiveAllMessages {
            batch_size_per_connection,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ReceivePollGroupMessages`] command.
    pub fn receive_poll_group_messages(
        poll_group: SteamworksNetworkingSocketsPollGroupId,
        batch_size: usize,
    ) -> Self {
        Self::ReceivePollGroupMessages {
            poll_group,
            batch_size,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ReceiveAllPollGroupMessages`] command.
    pub fn receive_all_poll_group_messages(batch_size_per_poll_group: usize) -> Self {
        Self::ReceiveAllPollGroupMessages {
            batch_size_per_poll_group,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::FlushMessages`] command.
    pub fn flush_messages(connection: SteamworksNetworkingSocketsConnectionId) -> Self {
        Self::FlushMessages { connection }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::FlushAllMessages`] command.
    pub fn flush_all_messages() -> Self {
        Self::FlushAllMessages
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::SetConnectionPollGroup`] command.
    pub fn set_connection_poll_group(
        connection: SteamworksNetworkingSocketsConnectionId,
        poll_group: SteamworksNetworkingSocketsPollGroupId,
    ) -> Self {
        Self::SetConnectionPollGroup {
            connection,
            poll_group,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ClearConnectionPollGroup`] command.
    pub fn clear_connection_poll_group(
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Self {
        Self::ClearConnectionPollGroup { connection }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ConfigureConnectionLanes`] command.
    pub fn configure_connection_lanes(
        connection: SteamworksNetworkingSocketsConnectionId,
        lane_priorities: impl Into<Vec<i32>>,
        lane_weights: impl Into<Vec<u16>>,
    ) -> Self {
        Self::ConfigureConnectionLanes {
            connection,
            lane_priorities: lane_priorities.into(),
            lane_weights: lane_weights.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::SetConnectionUserData`] command.
    pub fn set_connection_user_data(
        connection: SteamworksNetworkingSocketsConnectionId,
        user_data: i64,
    ) -> Self {
        Self::SetConnectionUserData {
            connection,
            user_data,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::SetConnectionName`] command.
    pub fn set_connection_name(
        connection: SteamworksNetworkingSocketsConnectionId,
        name: impl Into<String>,
    ) -> Self {
        Self::SetConnectionName {
            connection,
            name: name.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CloseConnection`] command.
    pub fn close_connection(connection: SteamworksNetworkingSocketsConnectionId) -> Self {
        Self::close_connection_with_reason(
            connection,
            steamworks::networking_types::NetConnectionEnd::App(
                steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
            ),
            None,
            false,
        )
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CloseConnection`] command with options.
    pub fn close_connection_with_reason(
        connection: SteamworksNetworkingSocketsConnectionId,
        reason: steamworks::networking_types::NetConnectionEnd,
        debug: Option<String>,
        enable_linger: bool,
    ) -> Self {
        Self::CloseConnection {
            connection,
            reason,
            debug,
            enable_linger,
        }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::CloseListenSocket`] command.
    pub fn close_listen_socket(listen_socket: SteamworksListenSocketId) -> Self {
        Self::CloseListenSocket { listen_socket }
    }

    /// Creates a [`SteamworksNetworkingSocketsCommand::ClosePollGroup`] command.
    pub fn close_poll_group(poll_group: SteamworksNetworkingSocketsPollGroupId) -> Self {
        Self::ClosePollGroup { poll_group }
    }
}
