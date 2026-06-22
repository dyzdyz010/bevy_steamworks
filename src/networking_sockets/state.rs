use bevy_ecs::prelude::Resource;

use crate::cache::trim_oldest;

use super::*;

mod accessors;
mod operations;

pub(in crate::networking_sockets) const STEAMWORKS_NETWORKING_SOCKETS_STATE_CACHE_LIMIT: usize =
    1_024;

/// Runtime state for [`crate::SteamworksNetworkingSocketsPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksNetworkingSocketsState {
    last_error: Option<SteamworksNetworkingSocketsError>,
    last_authentication_status: Option<steamworks::networking_types::NetworkingAvailabilityResult>,
    last_created_listen_socket: Option<SteamworksNetworkingSocketsListenSocketCreated>,
    last_created_connection: Option<SteamworksNetworkingSocketsConnectionCreated>,
    last_created_poll_group: Option<SteamworksNetworkingSocketsPollGroupId>,
    listen_socket_events: Vec<SteamworksNetworkingSocketsListenSocketEvents>,
    last_listen_socket_events: Option<SteamworksNetworkingSocketsListenSocketEvents>,
    connection_events: Vec<SteamworksNetworkingSocketsConnectionEvents>,
    last_connection_events: Option<SteamworksNetworkingSocketsConnectionEvents>,
    connection_infos: Vec<SteamworksNetworkingSocketsConnectionInfo>,
    last_connection_info: Option<SteamworksNetworkingSocketsConnectionInfo>,
    realtime_statuses: Vec<SteamworksNetworkingSocketsRealtimeStatus>,
    last_realtime_status: Option<SteamworksNetworkingSocketsRealtimeStatus>,
    last_sent_message: Option<SteamworksNetworkingSocketsSentMessage>,
    recent_sent_messages: Vec<SteamworksNetworkingSocketsMessageSendResult>,
    last_sent_messages: Vec<SteamworksNetworkingSocketsMessageSendResult>,
    recent_received_messages: Vec<SteamworksNetworkingSocketsMessage>,
    last_received_messages: Vec<SteamworksNetworkingSocketsMessage>,
    recent_poll_group_messages: Vec<SteamworksNetworkingSocketsPollGroupMessage>,
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

pub(super) fn upsert_listen_socket_events(
    events: &mut Vec<SteamworksNetworkingSocketsListenSocketEvents>,
    value: SteamworksNetworkingSocketsListenSocketEvents,
) {
    if let Some(existing) = events
        .iter_mut()
        .find(|existing| existing.listen_socket == value.listen_socket)
    {
        *existing = value;
    } else {
        events.push(value);
        trim_oldest(events, STEAMWORKS_NETWORKING_SOCKETS_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_connection_events(
    events: &mut Vec<SteamworksNetworkingSocketsConnectionEvents>,
    value: SteamworksNetworkingSocketsConnectionEvents,
) {
    if let Some(existing) = events
        .iter_mut()
        .find(|existing| existing.connection == value.connection)
    {
        *existing = value;
    } else {
        events.push(value);
        trim_oldest(events, STEAMWORKS_NETWORKING_SOCKETS_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_connection_info(
    infos: &mut Vec<SteamworksNetworkingSocketsConnectionInfo>,
    info: SteamworksNetworkingSocketsConnectionInfo,
) {
    if let Some(existing) = infos
        .iter_mut()
        .find(|existing| existing.connection == info.connection)
    {
        *existing = info;
    } else {
        infos.push(info);
        trim_oldest(infos, STEAMWORKS_NETWORKING_SOCKETS_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_realtime_status(
    statuses: &mut Vec<SteamworksNetworkingSocketsRealtimeStatus>,
    status: SteamworksNetworkingSocketsRealtimeStatus,
) {
    if let Some(existing) = statuses
        .iter_mut()
        .find(|existing| existing.connection == status.connection)
    {
        *existing = status;
    } else {
        statuses.push(status);
        trim_oldest(statuses, STEAMWORKS_NETWORKING_SOCKETS_STATE_CACHE_LIMIT);
    }
}

pub(super) fn push_sent_message_results(
    recent: &mut Vec<SteamworksNetworkingSocketsMessageSendResult>,
    messages: &[SteamworksNetworkingSocketsMessageSendResult],
) {
    recent.extend_from_slice(messages);
    trim_oldest(recent, STEAMWORKS_NETWORKING_SOCKETS_STATE_CACHE_LIMIT);
}

pub(super) fn push_received_messages(
    recent: &mut Vec<SteamworksNetworkingSocketsMessage>,
    messages: &[SteamworksNetworkingSocketsMessage],
) {
    recent.extend_from_slice(messages);
    trim_oldest(recent, STEAMWORKS_NETWORKING_SOCKETS_STATE_CACHE_LIMIT);
}

pub(super) fn push_poll_group_messages(
    recent: &mut Vec<SteamworksNetworkingSocketsPollGroupMessage>,
    messages: &[SteamworksNetworkingSocketsPollGroupMessage],
) {
    recent.extend_from_slice(messages);
    trim_oldest(recent, STEAMWORKS_NETWORKING_SOCKETS_STATE_CACHE_LIMIT);
}

pub(super) fn remove_connection_state(
    state: &mut SteamworksNetworkingSocketsState,
    connection: SteamworksNetworkingSocketsConnectionId,
) {
    state
        .connection_infos
        .retain(|info| info.connection != connection);
    state
        .realtime_statuses
        .retain(|status| status.connection != connection);
}
