use bevy_ecs::prelude::Resource;

use super::*;

mod accessors;
mod operations;

/// Runtime state for [`crate::SteamworksUserPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksUserState {
    last_error: Option<SteamworksUserError>,
    current_user: Option<SteamworksUserInfo>,
    last_steam_id: Option<steamworks::SteamId>,
    last_level: Option<u32>,
    steam_server_connected: Option<bool>,
    active_auth_tickets: Vec<steamworks::AuthTicket>,
    authenticated_users: Vec<steamworks::SteamId>,
    last_auth_session_ticket: Option<SteamworksIssuedAuthSessionTicket>,
    last_auth_session_ticket_for_identity: Option<SteamworksIssuedAuthSessionTicketForIdentity>,
    last_web_api_ticket_request: Option<SteamworksWebApiAuthenticationTicketRequest>,
    last_cancelled_auth_ticket: Option<steamworks::AuthTicket>,
    last_started_authentication_session: Option<steamworks::SteamId>,
    last_ended_authentication_session: Option<steamworks::SteamId>,
    last_user_license_for_app: Option<SteamworksUserLicenseForApp>,
    auth_session_ticket_issue_count: u64,
    web_api_ticket_request_count: u64,
    auth_ticket_cancel_count: u64,
    authentication_session_start_count: u64,
    authentication_session_end_count: u64,
    user_license_check_count: u64,
    last_steam_server_connection_event: Option<SteamworksSteamServerConnectionEvent>,
    last_micro_txn_authorization_response: Option<SteamworksMicroTxnAuthorizationResponse>,
    last_auth_ticket_response: Option<SteamworksAuthSessionTicketResponse>,
    last_web_api_ticket_response: Option<SteamworksWebApiTicketResponse>,
    last_auth_ticket_validation: Option<SteamworksAuthTicketValidation>,
}
