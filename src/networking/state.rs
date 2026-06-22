use bevy_ecs::prelude::Resource;

use crate::cache::trim_oldest;

use super::*;

mod accessors;
mod operations;

pub(in crate::networking) const STEAMWORKS_NETWORKING_STATE_CACHE_LIMIT: usize = 1_024;

/// Runtime state for [`crate::SteamworksNetworkingPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksNetworkingState {
    last_error: Option<SteamworksNetworkingError>,
    last_accepted_session: Option<steamworks::SteamId>,
    last_closed_session: Option<steamworks::SteamId>,
    session_states: Vec<SteamworksP2pSessionStateResult>,
    last_session_state: Option<SteamworksP2pSessionStateResult>,
    packet_availabilities: Vec<SteamworksP2pPacketAvailability>,
    last_packet_availability: Option<SteamworksP2pPacketAvailability>,
    last_sent_packet: Option<SteamworksP2pPacketSent>,
    received_packets: Vec<SteamworksP2pPacket>,
    last_packet: Option<SteamworksP2pPacket>,
    sent_count: u64,
    received_count: u64,
    empty_read_count: u64,
    last_empty_read_channel: Option<u32>,
    session_request_count: u64,
    session_requests: Vec<steamworks::SteamId>,
    last_session_request: Option<steamworks::SteamId>,
    session_connect_failure_count: u64,
    session_connect_failures: Vec<SteamworksP2pSessionConnectFailure>,
    last_session_connect_failure: Option<SteamworksP2pSessionConnectFailure>,
}

pub(super) fn upsert_session_state(
    states: &mut Vec<SteamworksP2pSessionStateResult>,
    state: SteamworksP2pSessionStateResult,
) {
    if let Some(existing) = states
        .iter_mut()
        .find(|existing| existing.user == state.user)
    {
        *existing = state;
    } else {
        states.push(state);
        trim_oldest(states, STEAMWORKS_NETWORKING_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_packet_availability(
    availabilities: &mut Vec<SteamworksP2pPacketAvailability>,
    availability: SteamworksP2pPacketAvailability,
) {
    if let Some(existing) = availabilities
        .iter_mut()
        .find(|existing| existing.channel == availability.channel)
    {
        *existing = availability;
    } else {
        availabilities.push(availability);
        trim_oldest(availabilities, STEAMWORKS_NETWORKING_STATE_CACHE_LIMIT);
    }
}

pub(super) fn push_received_packet(
    packets: &mut Vec<SteamworksP2pPacket>,
    packet: SteamworksP2pPacket,
) {
    packets.push(packet);
    trim_oldest(packets, STEAMWORKS_NETWORKING_STATE_CACHE_LIMIT);
}

pub(super) fn upsert_session_request(
    requests: &mut Vec<steamworks::SteamId>,
    remote: steamworks::SteamId,
) {
    if let Some(index) = requests.iter().position(|known| *known == remote) {
        requests.remove(index);
    }
    requests.push(remote);
    trim_oldest(requests, STEAMWORKS_NETWORKING_STATE_CACHE_LIMIT);
}

pub(super) fn upsert_session_connect_failure(
    failures: &mut Vec<SteamworksP2pSessionConnectFailure>,
    failure: SteamworksP2pSessionConnectFailure,
) {
    if let Some(existing) = failures
        .iter_mut()
        .find(|existing| existing.remote == failure.remote)
    {
        *existing = failure;
    } else {
        failures.push(failure);
        trim_oldest(failures, STEAMWORKS_NETWORKING_STATE_CACHE_LIMIT);
    }
}
