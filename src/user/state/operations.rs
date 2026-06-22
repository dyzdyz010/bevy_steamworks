use super::{upsert_by, SteamworksUserState};
use crate::user::{
    SteamworksIssuedAuthSessionTicket, SteamworksIssuedAuthSessionTicketForIdentity,
    SteamworksSteamServerConnectionEvent, SteamworksUserError, SteamworksUserLicenseForApp,
    SteamworksUserOperation, SteamworksWebApiAuthenticationTicketRequest,
};

impl SteamworksUserState {
    pub(in crate::user) fn record_error(&mut self, error: SteamworksUserError) {
        self.last_error = Some(error);
    }

    pub(in crate::user) fn record_operation(&mut self, operation: &SteamworksUserOperation) {
        match operation {
            SteamworksUserOperation::CurrentUserInfoRead { info } => {
                self.current_user = Some(info.clone());
                self.last_steam_id = Some(info.steam_id);
                self.last_level = Some(info.level);
                self.steam_server_connected = Some(info.logged_on);
            }
            SteamworksUserOperation::SteamIdRead { steam_id } => {
                self.last_steam_id = Some(*steam_id);
                if let Some(info) = &mut self.current_user {
                    info.steam_id = *steam_id;
                }
            }
            SteamworksUserOperation::LevelRead { level } => {
                self.last_level = Some(*level);
                if let Some(info) = &mut self.current_user {
                    info.level = *level;
                }
            }
            SteamworksUserOperation::LoggedOnRead { logged_on } => {
                self.steam_server_connected = Some(*logged_on);
                if let Some(info) = &mut self.current_user {
                    info.logged_on = *logged_on;
                }
            }
            SteamworksUserOperation::AuthenticationSessionTicketIssued {
                ticket,
                ticket_bytes,
                steam_id,
            } => {
                if !self.active_auth_tickets.contains(ticket) {
                    self.active_auth_tickets.push(*ticket);
                }
                let issued = SteamworksIssuedAuthSessionTicket {
                    ticket: *ticket,
                    ticket_bytes: ticket_bytes.clone(),
                    steam_id: *steam_id,
                };
                upsert_by(&mut self.auth_session_tickets, issued.clone(), |existing| {
                    existing.ticket == *ticket
                });
                self.last_auth_session_ticket = Some(issued);
                self.auth_session_ticket_issue_count =
                    self.auth_session_ticket_issue_count.saturating_add(1);
            }
            SteamworksUserOperation::AuthenticationSessionTicketForIdentityIssued {
                ticket,
                ticket_bytes,
                identity,
            } => {
                if !self.active_auth_tickets.contains(ticket) {
                    self.active_auth_tickets.push(*ticket);
                }
                let issued = SteamworksIssuedAuthSessionTicketForIdentity {
                    ticket: *ticket,
                    ticket_bytes: ticket_bytes.clone(),
                    identity: identity.clone(),
                };
                upsert_by(
                    &mut self.auth_session_tickets_for_identity,
                    issued.clone(),
                    |existing| existing.ticket == *ticket,
                );
                self.last_auth_session_ticket_for_identity = Some(issued);
                self.auth_session_ticket_issue_count =
                    self.auth_session_ticket_issue_count.saturating_add(1);
            }
            SteamworksUserOperation::WebApiAuthenticationTicketRequested { ticket, identity } => {
                if !self.active_auth_tickets.contains(ticket) {
                    self.active_auth_tickets.push(*ticket);
                }
                let request = SteamworksWebApiAuthenticationTicketRequest {
                    ticket: *ticket,
                    identity: identity.clone(),
                };
                upsert_by(
                    &mut self.web_api_ticket_requests,
                    request.clone(),
                    |existing| existing.ticket == *ticket,
                );
                self.last_web_api_ticket_request = Some(request);
                self.web_api_ticket_request_count =
                    self.web_api_ticket_request_count.saturating_add(1);
            }
            SteamworksUserOperation::AuthenticationTicketCancelled { ticket } => {
                self.active_auth_tickets.retain(|known| known != ticket);
                remove_ticket_caches(self, *ticket);
                self.last_cancelled_auth_ticket = Some(*ticket);
                self.auth_ticket_cancel_count = self.auth_ticket_cancel_count.saturating_add(1);
            }
            SteamworksUserOperation::AuthenticationSessionStarted { user } => {
                if !self.authenticated_users.contains(user) {
                    self.authenticated_users.push(*user);
                }
                self.last_started_authentication_session = Some(*user);
                self.authentication_session_start_count =
                    self.authentication_session_start_count.saturating_add(1);
            }
            SteamworksUserOperation::AuthenticationSessionEnded { user } => {
                self.authenticated_users.retain(|known| known != user);
                self.last_ended_authentication_session = Some(*user);
                self.authentication_session_end_count =
                    self.authentication_session_end_count.saturating_add(1);
            }
            SteamworksUserOperation::UserLicenseForAppRead {
                user,
                app_id,
                license,
            } => {
                let license = SteamworksUserLicenseForApp {
                    user: *user,
                    app_id: *app_id,
                    license: license.clone(),
                };
                upsert_by(
                    &mut self.user_licenses_for_apps,
                    license.clone(),
                    |existing| existing.user == *user && existing.app_id == *app_id,
                );
                self.last_user_license_for_app = Some(license);
                self.user_license_check_count = self.user_license_check_count.saturating_add(1);
            }
            SteamworksUserOperation::AuthenticationSessionTicketResponse { response } => {
                if response.result.is_err() {
                    self.active_auth_tickets
                        .retain(|known| *known != response.ticket);
                    remove_ticket_caches(self, response.ticket);
                }
                upsert_by(
                    &mut self.auth_ticket_responses,
                    response.clone(),
                    |existing| existing.ticket == response.ticket,
                );
                self.last_auth_ticket_response = Some(response.clone());
            }
            SteamworksUserOperation::WebApiAuthenticationTicketReceived { response } => {
                if response.result.is_err() {
                    self.active_auth_tickets
                        .retain(|known| *known != response.ticket_handle);
                    remove_ticket_caches(self, response.ticket_handle);
                }
                upsert_by(
                    &mut self.web_api_ticket_responses,
                    response.clone(),
                    |existing| existing.ticket_handle == response.ticket_handle,
                );
                self.last_web_api_ticket_response = Some(response.clone());
            }
            SteamworksUserOperation::AuthenticationTicketValidationReceived { validation } => {
                if validation.response.is_err() {
                    self.authenticated_users
                        .retain(|known| *known != validation.steam_id);
                }
                upsert_by(
                    &mut self.auth_ticket_validations,
                    validation.clone(),
                    |existing| existing.steam_id == validation.steam_id,
                );
                self.last_auth_ticket_validation = Some(validation.clone());
            }
            SteamworksUserOperation::SteamServerConnectionEventReceived { event } => {
                let connected = matches!(event, SteamworksSteamServerConnectionEvent::Connected);
                self.steam_server_connected = Some(connected);
                if let Some(info) = &mut self.current_user {
                    info.logged_on = connected;
                }
                self.steam_server_connection_events.push(event.clone());
                crate::cache::trim_oldest(
                    &mut self.steam_server_connection_events,
                    super::STEAMWORKS_USER_STATE_CACHE_LIMIT,
                );
                self.last_steam_server_connection_event = Some(event.clone());
            }
            SteamworksUserOperation::MicroTxnAuthorizationResponseReceived { response } => {
                upsert_by(
                    &mut self.micro_txn_authorization_responses,
                    response.clone(),
                    |existing| {
                        existing.app_id == response.app_id && existing.order_id == response.order_id
                    },
                );
                self.last_micro_txn_authorization_response = Some(response.clone());
            }
        }
    }
}

fn remove_ticket_caches(state: &mut SteamworksUserState, ticket: steamworks::AuthTicket) {
    state
        .auth_session_tickets
        .retain(|issued| issued.ticket != ticket);
    state
        .auth_session_tickets_for_identity
        .retain(|issued| issued.ticket != ticket);
    state
        .web_api_ticket_requests
        .retain(|request| request.ticket != ticket);
    state
        .auth_ticket_responses
        .retain(|response| response.ticket != ticket);
    state
        .web_api_ticket_responses
        .retain(|response| response.ticket_handle != ticket);
}
