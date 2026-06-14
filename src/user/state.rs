use bevy_ecs::prelude::Resource;

use super::*;

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

impl SteamworksUserState {
    /// Returns the most recent synchronous error observed by the user plugin.
    pub fn last_error(&self) -> Option<&SteamworksUserError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent current-user snapshot read through the plugin.
    pub fn current_user(&self) -> Option<&SteamworksUserInfo> {
        self.current_user.as_ref()
    }

    /// Returns the most recent Steam ID read through this plugin.
    pub fn last_steam_id(&self) -> Option<steamworks::SteamId> {
        self.last_steam_id
    }

    /// Returns the most recent Steam user level read through this plugin.
    pub fn last_level(&self) -> Option<u32> {
        self.last_level
    }

    /// Returns the latest known Steam server connection state.
    ///
    /// This is updated by [`SteamworksUserCommand::IsLoggedOn`],
    /// [`SteamworksUserCommand::GetCurrentUserInfo`], and Steam server
    /// connection callbacks.
    pub fn steam_server_connected(&self) -> Option<bool> {
        self.steam_server_connected
    }

    /// Returns authentication ticket handles issued through this command layer.
    ///
    /// Handles are removed after [`SteamworksUserCommand::CancelAuthenticationTicket`]
    /// is processed for the same ticket.
    pub fn active_auth_tickets(&self) -> &[steamworks::AuthTicket] {
        &self.active_auth_tickets
    }

    /// Returns users with sessions started through this command layer.
    ///
    /// Entries are removed after [`SteamworksUserCommand::EndAuthenticationSession`]
    /// is processed for the same user or after Steam reports validation failure
    /// for the same user.
    pub fn authenticated_users(&self) -> &[steamworks::SteamId] {
        &self.authenticated_users
    }

    /// Returns the most recent auth session ticket issued through this command layer.
    pub fn last_auth_session_ticket(&self) -> Option<&SteamworksIssuedAuthSessionTicket> {
        self.last_auth_session_ticket.as_ref()
    }

    /// Returns the most recent Web API auth ticket request submitted through this command layer.
    pub fn last_web_api_ticket_request(
        &self,
    ) -> Option<&SteamworksWebApiAuthenticationTicketRequest> {
        self.last_web_api_ticket_request.as_ref()
    }

    /// Returns the most recent auth ticket cancelled through this command layer.
    pub fn last_cancelled_auth_ticket(&self) -> Option<steamworks::AuthTicket> {
        self.last_cancelled_auth_ticket
    }

    /// Returns the most recent remote authentication session started through this command layer.
    pub fn last_started_authentication_session(&self) -> Option<steamworks::SteamId> {
        self.last_started_authentication_session
    }

    /// Returns the most recent remote authentication session ended through this command layer.
    pub fn last_ended_authentication_session(&self) -> Option<steamworks::SteamId> {
        self.last_ended_authentication_session
    }

    /// Returns the most recent app-license check submitted through this command layer.
    pub fn last_user_license_for_app(&self) -> Option<&SteamworksUserLicenseForApp> {
        self.last_user_license_for_app.as_ref()
    }

    /// Returns how many auth session tickets this plugin issued.
    pub fn auth_session_ticket_issue_count(&self) -> u64 {
        self.auth_session_ticket_issue_count
    }

    /// Returns how many Web API auth ticket requests this plugin submitted.
    pub fn web_api_ticket_request_count(&self) -> u64 {
        self.web_api_ticket_request_count
    }

    /// Returns how many auth tickets this plugin cancelled.
    pub fn auth_ticket_cancel_count(&self) -> u64 {
        self.auth_ticket_cancel_count
    }

    /// Returns how many remote authentication sessions this plugin started.
    pub fn authentication_session_start_count(&self) -> u64 {
        self.authentication_session_start_count
    }

    /// Returns how many remote authentication sessions this plugin ended.
    pub fn authentication_session_end_count(&self) -> u64 {
        self.authentication_session_end_count
    }

    /// Returns how many app-license checks this plugin performed.
    pub fn user_license_check_count(&self) -> u64 {
        self.user_license_check_count
    }

    /// Returns the most recent Steam server connection callback snapshot.
    pub fn last_steam_server_connection_event(
        &self,
    ) -> Option<&SteamworksSteamServerConnectionEvent> {
        self.last_steam_server_connection_event.as_ref()
    }

    /// Returns the most recent microtransaction authorization callback snapshot.
    pub fn last_micro_txn_authorization_response(
        &self,
    ) -> Option<&SteamworksMicroTxnAuthorizationResponse> {
        self.last_micro_txn_authorization_response.as_ref()
    }

    /// Returns the most recent auth session ticket response callback snapshot.
    pub fn last_auth_ticket_response(&self) -> Option<&SteamworksAuthSessionTicketResponse> {
        self.last_auth_ticket_response.as_ref()
    }

    /// Returns the most recent Web API ticket response callback snapshot.
    pub fn last_web_api_ticket_response(&self) -> Option<&SteamworksWebApiTicketResponse> {
        self.last_web_api_ticket_response.as_ref()
    }

    /// Returns the most recent auth ticket validation callback snapshot.
    pub fn last_auth_ticket_validation(&self) -> Option<&SteamworksAuthTicketValidation> {
        self.last_auth_ticket_validation.as_ref()
    }

    pub(super) fn record_error(&mut self, error: SteamworksUserError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksUserOperation) {
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
                self.last_auth_session_ticket = Some(SteamworksIssuedAuthSessionTicket {
                    ticket: *ticket,
                    ticket_bytes: ticket_bytes.clone(),
                    steam_id: *steam_id,
                });
                self.auth_session_ticket_issue_count =
                    self.auth_session_ticket_issue_count.saturating_add(1);
            }
            SteamworksUserOperation::WebApiAuthenticationTicketRequested { ticket, identity } => {
                if !self.active_auth_tickets.contains(ticket) {
                    self.active_auth_tickets.push(*ticket);
                }
                self.last_web_api_ticket_request =
                    Some(SteamworksWebApiAuthenticationTicketRequest {
                        ticket: *ticket,
                        identity: identity.clone(),
                    });
                self.web_api_ticket_request_count =
                    self.web_api_ticket_request_count.saturating_add(1);
            }
            SteamworksUserOperation::AuthenticationTicketCancelled { ticket } => {
                self.active_auth_tickets.retain(|known| known != ticket);
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
                self.last_user_license_for_app = Some(SteamworksUserLicenseForApp {
                    user: *user,
                    app_id: *app_id,
                    license: license.clone(),
                });
                self.user_license_check_count = self.user_license_check_count.saturating_add(1);
            }
            SteamworksUserOperation::AuthenticationSessionTicketResponse { response } => {
                if response.result.is_err() {
                    self.active_auth_tickets
                        .retain(|known| *known != response.ticket);
                }
                self.last_auth_ticket_response = Some(response.clone());
            }
            SteamworksUserOperation::WebApiAuthenticationTicketReceived { response } => {
                if response.result.is_err() {
                    self.active_auth_tickets
                        .retain(|known| *known != response.ticket_handle);
                }
                self.last_web_api_ticket_response = Some(response.clone());
            }
            SteamworksUserOperation::AuthenticationTicketValidationReceived { validation } => {
                if validation.response.is_err() {
                    self.authenticated_users
                        .retain(|known| *known != validation.steam_id);
                }
                self.last_auth_ticket_validation = Some(validation.clone());
            }
            SteamworksUserOperation::SteamServerConnectionEventReceived { event } => {
                let connected = matches!(event, SteamworksSteamServerConnectionEvent::Connected);
                self.steam_server_connected = Some(connected);
                if let Some(info) = &mut self.current_user {
                    info.logged_on = connected;
                }
                self.last_steam_server_connection_event = Some(event.clone());
            }
            SteamworksUserOperation::MicroTxnAuthorizationResponseReceived { response } => {
                self.last_micro_txn_authorization_response = Some(response.clone());
            }
        }
    }
}
