use bevy_ecs::prelude::Resource;

use crate::user::{
    SteamworksAuthSessionTicketResponse, SteamworksAuthTicketValidation,
    SteamworksSteamServerConnectionEvent,
};

use super::*;

mod accessors;
mod operations;

/// Runtime state for [`crate::SteamworksServerPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksServerState {
    last_error: Option<SteamworksServerError>,
    steam_id: Option<steamworks::SteamId>,
    steam_server_connected: Option<bool>,
    active_auth_tickets: Vec<steamworks::AuthTicket>,
    authenticated_users: Vec<steamworks::SteamId>,
    last_auth_session_ticket: Option<SteamworksServerIssuedAuthSessionTicket>,
    last_auth_session_ticket_for_identity:
        Option<SteamworksServerIssuedAuthSessionTicketForIdentity>,
    last_cancelled_auth_ticket: Option<steamworks::AuthTicket>,
    last_started_authentication_session: Option<steamworks::SteamId>,
    last_ended_authentication_session: Option<steamworks::SteamId>,
    auth_session_ticket_issue_count: u64,
    auth_ticket_cancel_count: u64,
    authentication_session_start_count: u64,
    authentication_session_end_count: u64,
    last_steam_server_connection_event: Option<SteamworksSteamServerConnectionEvent>,
    last_auth_ticket_response: Option<SteamworksAuthSessionTicketResponse>,
    last_auth_ticket_validation: Option<SteamworksAuthTicketValidation>,
    last_client_approval: Option<SteamworksServerClientApproval>,
    last_client_denial: Option<SteamworksServerClientDenial>,
    last_client_kick: Option<SteamworksServerClientKick>,
    last_client_group_status: Option<SteamworksServerClientGroupStatus>,
    product: Option<String>,
    game_description: Option<String>,
    game_data: Option<String>,
    dedicated: Option<bool>,
    anonymous_logon_submitted: bool,
    token_logon_submitted: bool,
    advertise_server_active: Option<bool>,
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
    outgoing_packet_drain_count: u64,
}
