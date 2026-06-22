use super::SteamworksUserState;
use crate::user::{
    SteamworksAuthSessionTicketResponse, SteamworksAuthTicketValidation,
    SteamworksIssuedAuthSessionTicket, SteamworksIssuedAuthSessionTicketForIdentity,
    SteamworksMicroTxnAuthorizationResponse, SteamworksSteamServerConnectionEvent,
    SteamworksUserError, SteamworksUserInfo, SteamworksUserLicenseForApp,
    SteamworksWebApiAuthenticationTicketRequest, SteamworksWebApiTicketResponse,
};

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
    /// This is updated by [`crate::SteamworksUserCommand::IsLoggedOn`],
    /// [`crate::SteamworksUserCommand::GetCurrentUserInfo`], and Steam server
    /// connection callbacks.
    pub fn steam_server_connected(&self) -> Option<bool> {
        self.steam_server_connected
    }

    /// Returns authentication ticket handles issued through this command layer.
    ///
    /// Handles are removed after [`crate::SteamworksUserCommand::CancelAuthenticationTicket`]
    /// is processed for the same ticket.
    pub fn active_auth_tickets(&self) -> &[steamworks::AuthTicket] {
        &self.active_auth_tickets
    }

    /// Returns users with sessions started through this command layer.
    ///
    /// Entries are removed after [`crate::SteamworksUserCommand::EndAuthenticationSession`]
    /// is processed for the same user or after Steam reports validation failure
    /// for the same user.
    pub fn authenticated_users(&self) -> &[steamworks::SteamId] {
        &self.authenticated_users
    }

    /// Returns the most recent auth session ticket issued through this command layer.
    pub fn last_auth_session_ticket(&self) -> Option<&SteamworksIssuedAuthSessionTicket> {
        self.last_auth_session_ticket.as_ref()
    }

    /// Returns the most recent identity-scoped auth session ticket issued through this command layer.
    pub fn last_auth_session_ticket_for_identity(
        &self,
    ) -> Option<&SteamworksIssuedAuthSessionTicketForIdentity> {
        self.last_auth_session_ticket_for_identity.as_ref()
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
}
