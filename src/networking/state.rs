use bevy_ecs::prelude::Resource;

use super::*;

mod accessors;
mod operations;

/// Runtime state for [`crate::SteamworksNetworkingPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksNetworkingState {
    last_error: Option<SteamworksNetworkingError>,
    last_accepted_session: Option<steamworks::SteamId>,
    last_closed_session: Option<steamworks::SteamId>,
    last_session_state: Option<SteamworksP2pSessionStateResult>,
    last_packet_availability: Option<SteamworksP2pPacketAvailability>,
    last_sent_packet: Option<SteamworksP2pPacketSent>,
    last_packet: Option<SteamworksP2pPacket>,
    sent_count: u64,
    received_count: u64,
    empty_read_count: u64,
    last_empty_read_channel: Option<u32>,
    session_request_count: u64,
    last_session_request: Option<steamworks::SteamId>,
    session_connect_failure_count: u64,
    last_session_connect_failure: Option<SteamworksP2pSessionConnectFailure>,
}
