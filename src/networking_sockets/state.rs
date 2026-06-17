use bevy_ecs::prelude::Resource;

use super::*;

mod accessors;
mod operations;

/// Runtime state for [`crate::SteamworksNetworkingSocketsPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksNetworkingSocketsState {
    last_error: Option<SteamworksNetworkingSocketsError>,
    last_authentication_status: Option<steamworks::networking_types::NetworkingAvailabilityResult>,
    last_created_listen_socket: Option<SteamworksNetworkingSocketsListenSocketCreated>,
    last_created_connection: Option<SteamworksNetworkingSocketsConnectionCreated>,
    last_created_poll_group: Option<SteamworksNetworkingSocketsPollGroupId>,
    last_listen_socket_events: Option<SteamworksNetworkingSocketsListenSocketEvents>,
    last_connection_events: Option<SteamworksNetworkingSocketsConnectionEvents>,
    last_connection_info: Option<SteamworksNetworkingSocketsConnectionInfo>,
    last_realtime_status: Option<SteamworksNetworkingSocketsRealtimeStatus>,
    last_sent_message: Option<SteamworksNetworkingSocketsSentMessage>,
    last_sent_messages: Vec<SteamworksNetworkingSocketsMessageSendResult>,
    last_received_messages: Vec<SteamworksNetworkingSocketsMessage>,
    last_poll_group_messages: Vec<SteamworksNetworkingSocketsPollGroupMessage>,
    last_flushed_connection: Option<SteamworksNetworkingSocketsConnectionId>,
    last_connection_poll_group_set: Option<SteamworksNetworkingSocketsPollGroupAssignment>,
    last_connection_poll_group_cleared: Option<SteamworksNetworkingSocketsConnectionId>,
    last_connection_lanes_configured: Option<SteamworksNetworkingSocketsLaneConfiguration>,
    last_connection_user_data: Option<SteamworksNetworkingSocketsConnectionUserData>,
    last_connection_name: Option<SteamworksNetworkingSocketsConnectionName>,
    last_closed_connection: Option<SteamworksNetworkingSocketsConnectionClosed>,
    last_closed_listen_socket: Option<SteamworksNetworkingSocketsListenSocketClosed>,
    last_closed_poll_group: Option<SteamworksNetworkingSocketsPollGroupId>,
    listen_socket_count: usize,
    connection_count: usize,
    poll_group_count: usize,
    sent_count: u64,
    received_count: u64,
}
