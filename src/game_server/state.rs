use bevy_ecs::prelude::Resource;

use crate::cache::trim_oldest;
use crate::user::{
    SteamworksAuthSessionTicketResponse, SteamworksAuthTicketValidation,
    SteamworksSteamServerConnectionEvent,
};

use super::*;

mod accessors;
mod operations;

pub(in crate::game_server) const STEAMWORKS_SERVER_STATE_CACHE_LIMIT: usize = 1_024;

/// Runtime state for [`crate::SteamworksServerPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksServerState {
    last_error: Option<SteamworksServerError>,
    steam_id: Option<steamworks::SteamId>,
    steam_server_connected: Option<bool>,
    active_auth_tickets: Vec<steamworks::AuthTicket>,
    authenticated_users: Vec<steamworks::SteamId>,
    last_auth_session_ticket: Option<SteamworksServerIssuedAuthSessionTicket>,
    auth_session_tickets: Vec<SteamworksServerIssuedAuthSessionTicket>,
    last_auth_session_ticket_for_identity:
        Option<SteamworksServerIssuedAuthSessionTicketForIdentity>,
    auth_session_tickets_for_identity: Vec<SteamworksServerIssuedAuthSessionTicketForIdentity>,
    last_cancelled_auth_ticket: Option<steamworks::AuthTicket>,
    last_started_authentication_session: Option<steamworks::SteamId>,
    last_ended_authentication_session: Option<steamworks::SteamId>,
    auth_session_ticket_issue_count: u64,
    auth_ticket_cancel_count: u64,
    authentication_session_start_count: u64,
    authentication_session_end_count: u64,
    last_steam_server_connection_event: Option<SteamworksSteamServerConnectionEvent>,
    steam_server_connection_events: Vec<SteamworksSteamServerConnectionEvent>,
    last_auth_ticket_response: Option<SteamworksAuthSessionTicketResponse>,
    auth_ticket_responses: Vec<SteamworksAuthSessionTicketResponse>,
    last_auth_ticket_validation: Option<SteamworksAuthTicketValidation>,
    auth_ticket_validations: Vec<SteamworksAuthTicketValidation>,
    last_client_approval: Option<SteamworksServerClientApproval>,
    client_approvals: Vec<SteamworksServerClientApproval>,
    last_client_denial: Option<SteamworksServerClientDenial>,
    client_denials: Vec<SteamworksServerClientDenial>,
    last_client_kick: Option<SteamworksServerClientKick>,
    client_kicks: Vec<SteamworksServerClientKick>,
    last_client_group_status: Option<SteamworksServerClientGroupStatus>,
    client_group_statuses: Vec<SteamworksServerClientGroupStatus>,
    product: Option<String>,
    game_description: Option<String>,
    game_data: Option<String>,
    dedicated: Option<bool>,
    anonymous_logon_submitted: bool,
    token_logon_submitted: bool,
    advertise_server_active: Option<bool>,
    heartbeats_active: Option<bool>,
    mod_dir: Option<String>,
    map_name: Option<String>,
    server_name: Option<String>,
    max_players: Option<i32>,
    game_tags: Option<String>,
    key_values: Vec<(String, String)>,
    password_protected: Option<bool>,
    bot_player_count: Option<i32>,
    last_incoming_packet: Option<SteamworksServerIncomingPacket>,
    incoming_packet_count: u64,
    last_outgoing_packets: Vec<SteamworksServerOutgoingPacket>,
    outgoing_packet_count: u64,
    outgoing_packet_drain_count: u64,
}

pub(super) fn upsert_by<T>(
    items: &mut Vec<T>,
    item: T,
    mut matches_existing: impl FnMut(&T) -> bool,
) {
    if let Some(existing) = items.iter_mut().find(|existing| matches_existing(existing)) {
        *existing = item;
    } else {
        items.push(item);
        trim_oldest(items, STEAMWORKS_SERVER_STATE_CACHE_LIMIT);
    }
}

pub(super) fn push_bounded<T>(items: &mut Vec<T>, item: T) {
    items.push(item);
    trim_oldest(items, STEAMWORKS_SERVER_STATE_CACHE_LIMIT);
}
