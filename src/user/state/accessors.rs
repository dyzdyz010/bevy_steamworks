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

    /// Returns the latest known Steam ID for the current user.
    ///
    /// This prefers the full current-user snapshot when available and falls
    /// back to the latest standalone Steam ID read.
    pub fn steam_id(&self) -> Option<steamworks::SteamId> {
        self.current_user
            .as_ref()
            .map(|info| info.steam_id)
            .or(self.last_steam_id)
    }

    /// Returns the most recent Steam ID read through this plugin.
    pub fn last_steam_id(&self) -> Option<steamworks::SteamId> {
        self.last_steam_id
    }

    /// Returns the latest known Steam user level.
    ///
    /// This prefers the full current-user snapshot when available and falls
    /// back to the latest standalone level read.
    pub fn level(&self) -> Option<u32> {
        self.current_user
            .as_ref()
            .map(|info| info.level)
            .or(self.last_level)
    }

    /// Returns the most recent Steam user level read through this plugin.
    pub fn last_level(&self) -> Option<u32> {
        self.last_level
    }

    /// Returns the latest known Steam server connection state.
    ///
    /// This prefers the full current-user snapshot when available and falls
    /// back to standalone logged-on reads and Steam server connection callbacks.
    pub fn logged_on(&self) -> Option<bool> {
        self.current_user
            .as_ref()
            .map(|info| info.logged_on)
            .or(self.steam_server_connected)
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

    /// Returns how many locally issued auth tickets are still active.
    pub fn active_auth_ticket_count(&self) -> usize {
        self.active_auth_tickets.len()
    }

    /// Returns users with sessions started through this command layer.
    ///
    /// Entries are removed after [`crate::SteamworksUserCommand::EndAuthenticationSession`]
    /// is processed for the same user or after Steam reports validation failure
    /// for the same user.
    pub fn authenticated_users(&self) -> &[steamworks::SteamId] {
        &self.authenticated_users
    }

    /// Returns whether a remote user currently has a tracked authenticated session.
    pub fn is_user_authenticated(&self, user: steamworks::SteamId) -> bool {
        self.authenticated_users.contains(&user)
    }

    /// Returns the most recent auth session ticket issued through this command layer.
    pub fn last_auth_session_ticket(&self) -> Option<&SteamworksIssuedAuthSessionTicket> {
        self.last_auth_session_ticket.as_ref()
    }

    /// Returns currently cached issued auth session tickets by ticket handle.
    ///
    /// Ticket bytes are removed from this cache when the ticket is cancelled or
    /// Steam reports ticket creation failure.
    pub fn auth_session_tickets(&self) -> &[SteamworksIssuedAuthSessionTicket] {
        &self.auth_session_tickets
    }

    /// Returns an issued auth session ticket snapshot for a ticket handle.
    pub fn auth_session_ticket(
        &self,
        ticket: steamworks::AuthTicket,
    ) -> Option<&SteamworksIssuedAuthSessionTicket> {
        self.auth_session_tickets
            .iter()
            .find(|issued| issued.ticket == ticket)
    }

    /// Returns the most recent identity-scoped auth session ticket issued through this command layer.
    pub fn last_auth_session_ticket_for_identity(
        &self,
    ) -> Option<&SteamworksIssuedAuthSessionTicketForIdentity> {
        self.last_auth_session_ticket_for_identity.as_ref()
    }

    /// Returns currently cached identity-scoped auth session tickets by ticket handle.
    ///
    /// Ticket bytes are removed from this cache when the ticket is cancelled or
    /// Steam reports ticket creation failure.
    pub fn auth_session_tickets_for_identity(
        &self,
    ) -> &[SteamworksIssuedAuthSessionTicketForIdentity] {
        &self.auth_session_tickets_for_identity
    }

    /// Returns an identity-scoped auth session ticket snapshot for a ticket handle.
    pub fn auth_session_ticket_for_identity(
        &self,
        ticket: steamworks::AuthTicket,
    ) -> Option<&SteamworksIssuedAuthSessionTicketForIdentity> {
        self.auth_session_tickets_for_identity
            .iter()
            .find(|issued| issued.ticket == ticket)
    }

    /// Returns the most recent Web API auth ticket request submitted through this command layer.
    pub fn last_web_api_ticket_request(
        &self,
    ) -> Option<&SteamworksWebApiAuthenticationTicketRequest> {
        self.last_web_api_ticket_request.as_ref()
    }

    /// Returns currently cached Web API auth ticket requests by ticket handle.
    pub fn web_api_ticket_requests(&self) -> &[SteamworksWebApiAuthenticationTicketRequest] {
        &self.web_api_ticket_requests
    }

    /// Returns a Web API auth ticket request snapshot for a ticket handle.
    pub fn web_api_ticket_request(
        &self,
        ticket: steamworks::AuthTicket,
    ) -> Option<&SteamworksWebApiAuthenticationTicketRequest> {
        self.web_api_ticket_requests
            .iter()
            .find(|request| request.ticket == ticket)
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

    /// Returns bounded app-license check snapshots by user and app.
    pub fn user_licenses_for_apps(&self) -> &[SteamworksUserLicenseForApp] {
        &self.user_licenses_for_apps
    }

    /// Returns the latest app-license check for a user/app pair.
    pub fn user_license_for_app(
        &self,
        user: steamworks::SteamId,
        app_id: steamworks::AppId,
    ) -> Option<&SteamworksUserLicenseForApp> {
        self.user_licenses_for_apps
            .iter()
            .find(|license| license.user == user && license.app_id == app_id)
    }

    /// Returns the latest license value for a user/app pair.
    pub fn user_license(
        &self,
        user: steamworks::SteamId,
        app_id: steamworks::AppId,
    ) -> Option<&steamworks::UserHasLicense> {
        self.user_license_for_app(user, app_id)
            .map(|license| &license.license)
    }

    /// Returns whether the latest license check reported a valid app license.
    ///
    /// Returns `None` when this plugin has not cached a license check for the
    /// user/app pair.
    pub fn user_has_license_for_app(
        &self,
        user: steamworks::SteamId,
        app_id: steamworks::AppId,
    ) -> Option<bool> {
        self.user_license(user, app_id)
            .map(|license| matches!(license, steamworks::UserHasLicense::HasLicense))
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

    /// Returns bounded Steam server connection callback snapshots.
    pub fn steam_server_connection_events(&self) -> &[SteamworksSteamServerConnectionEvent] {
        &self.steam_server_connection_events
    }

    /// Returns the most recent microtransaction authorization callback snapshot.
    pub fn last_micro_txn_authorization_response(
        &self,
    ) -> Option<&SteamworksMicroTxnAuthorizationResponse> {
        self.last_micro_txn_authorization_response.as_ref()
    }

    /// Returns bounded microtransaction authorization callback snapshots by app/order.
    pub fn micro_txn_authorization_responses(&self) -> &[SteamworksMicroTxnAuthorizationResponse] {
        &self.micro_txn_authorization_responses
    }

    /// Returns the latest microtransaction authorization for an app/order pair.
    pub fn micro_txn_authorization_response(
        &self,
        app_id: steamworks::AppId,
        order_id: u64,
    ) -> Option<&SteamworksMicroTxnAuthorizationResponse> {
        self.micro_txn_authorization_responses
            .iter()
            .find(|response| response.app_id == app_id && response.order_id == order_id)
    }

    /// Returns whether the latest microtransaction callback authorized an app/order pair.
    pub fn micro_txn_authorized(&self, app_id: steamworks::AppId, order_id: u64) -> Option<bool> {
        self.micro_txn_authorization_response(app_id, order_id)
            .map(|response| response.authorized)
    }

    /// Returns the most recent auth session ticket response callback snapshot.
    pub fn last_auth_ticket_response(&self) -> Option<&SteamworksAuthSessionTicketResponse> {
        self.last_auth_ticket_response.as_ref()
    }

    /// Returns bounded auth session ticket response snapshots by ticket handle.
    pub fn auth_ticket_responses(&self) -> &[SteamworksAuthSessionTicketResponse] {
        &self.auth_ticket_responses
    }

    /// Returns the latest auth session ticket response for a ticket handle.
    pub fn auth_ticket_response(
        &self,
        ticket: steamworks::AuthTicket,
    ) -> Option<&SteamworksAuthSessionTicketResponse> {
        self.auth_ticket_responses
            .iter()
            .find(|response| response.ticket == ticket)
    }

    /// Returns the most recent Web API ticket response callback snapshot.
    pub fn last_web_api_ticket_response(&self) -> Option<&SteamworksWebApiTicketResponse> {
        self.last_web_api_ticket_response.as_ref()
    }

    /// Returns bounded Web API ticket response snapshots by ticket handle.
    pub fn web_api_ticket_responses(&self) -> &[SteamworksWebApiTicketResponse] {
        &self.web_api_ticket_responses
    }

    /// Returns the latest Web API ticket response for a ticket handle.
    pub fn web_api_ticket_response(
        &self,
        ticket: steamworks::AuthTicket,
    ) -> Option<&SteamworksWebApiTicketResponse> {
        self.web_api_ticket_responses
            .iter()
            .find(|response| response.ticket_handle == ticket)
    }

    /// Returns the most recent auth ticket validation callback snapshot.
    pub fn last_auth_ticket_validation(&self) -> Option<&SteamworksAuthTicketValidation> {
        self.last_auth_ticket_validation.as_ref()
    }

    /// Returns bounded auth ticket validation callback snapshots by Steam user.
    pub fn auth_ticket_validations(&self) -> &[SteamworksAuthTicketValidation] {
        &self.auth_ticket_validations
    }

    /// Returns the latest auth ticket validation for a Steam user.
    pub fn auth_ticket_validation(
        &self,
        user: steamworks::SteamId,
    ) -> Option<&SteamworksAuthTicketValidation> {
        self.auth_ticket_validations
            .iter()
            .find(|validation| validation.steam_id == user)
    }

    /// Returns whether the latest cached auth-ticket validation for a user succeeded.
    pub fn auth_ticket_validation_succeeded(&self, user: steamworks::SteamId) -> Option<bool> {
        self.auth_ticket_validation(user)
            .map(|validation| validation.response.is_ok())
    }
}
