use bevy_ecs::prelude::Resource;

use crate::cache::trim_oldest;

use super::*;

mod accessors;
mod operations;

pub(in crate::user) const STEAMWORKS_USER_STATE_CACHE_LIMIT: usize = 1_024;

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
    auth_session_tickets: Vec<SteamworksIssuedAuthSessionTicket>,
    last_auth_session_ticket_for_identity: Option<SteamworksIssuedAuthSessionTicketForIdentity>,
    auth_session_tickets_for_identity: Vec<SteamworksIssuedAuthSessionTicketForIdentity>,
    last_web_api_ticket_request: Option<SteamworksWebApiAuthenticationTicketRequest>,
    web_api_ticket_requests: Vec<SteamworksWebApiAuthenticationTicketRequest>,
    last_cancelled_auth_ticket: Option<steamworks::AuthTicket>,
    last_started_authentication_session: Option<steamworks::SteamId>,
    last_ended_authentication_session: Option<steamworks::SteamId>,
    last_user_license_for_app: Option<SteamworksUserLicenseForApp>,
    user_licenses_for_apps: Vec<SteamworksUserLicenseForApp>,
    auth_session_ticket_issue_count: u64,
    web_api_ticket_request_count: u64,
    auth_ticket_cancel_count: u64,
    authentication_session_start_count: u64,
    authentication_session_end_count: u64,
    user_license_check_count: u64,
    last_steam_server_connection_event: Option<SteamworksSteamServerConnectionEvent>,
    steam_server_connection_events: Vec<SteamworksSteamServerConnectionEvent>,
    last_micro_txn_authorization_response: Option<SteamworksMicroTxnAuthorizationResponse>,
    micro_txn_authorization_responses: Vec<SteamworksMicroTxnAuthorizationResponse>,
    last_auth_ticket_response: Option<SteamworksAuthSessionTicketResponse>,
    auth_ticket_responses: Vec<SteamworksAuthSessionTicketResponse>,
    last_web_api_ticket_response: Option<SteamworksWebApiTicketResponse>,
    web_api_ticket_responses: Vec<SteamworksWebApiTicketResponse>,
    last_auth_ticket_validation: Option<SteamworksAuthTicketValidation>,
    auth_ticket_validations: Vec<SteamworksAuthTicketValidation>,
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
        trim_oldest(items, STEAMWORKS_USER_STATE_CACHE_LIMIT);
    }
}
