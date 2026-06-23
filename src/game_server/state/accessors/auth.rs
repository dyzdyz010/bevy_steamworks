use crate::game_server::*;

impl SteamworksServerState {
    /// Returns authentication ticket handles issued through this command layer.
    ///
    /// Handles are removed after [`crate::SteamworksServerCommand::CancelAuthenticationTicket`]
    /// is processed for the same ticket or after Steam reports ticket creation failure
    /// for the same ticket.
    pub fn active_auth_tickets(&self) -> &[steamworks::AuthTicket] {
        &self.active_auth_tickets
    }

    /// Returns how many locally issued auth tickets are still active.
    pub fn active_auth_ticket_count(&self) -> usize {
        self.active_auth_tickets.len()
    }

    /// Returns users currently considered authenticated or approved by this layer.
    ///
    /// Entries are removed after [`crate::SteamworksServerCommand::EndAuthenticationSession`]
    /// is processed for the same user or after Steam reports validation failure,
    /// denial, or kick for the same user.
    pub fn authenticated_users(&self) -> &[steamworks::SteamId] {
        &self.authenticated_users
    }

    /// Returns whether a user is currently considered authenticated or approved by this layer.
    pub fn is_user_authenticated(&self, user: steamworks::SteamId) -> bool {
        self.authenticated_users.contains(&user)
    }

    /// Returns the most recent auth session ticket issued through this command layer.
    pub fn last_auth_session_ticket(&self) -> Option<&SteamworksServerIssuedAuthSessionTicket> {
        self.last_auth_session_ticket.as_ref()
    }

    /// Returns currently cached issued auth session tickets by ticket handle.
    ///
    /// Ticket bytes are removed from this cache when the ticket is cancelled or
    /// Steam reports ticket creation failure.
    pub fn auth_session_tickets(&self) -> &[SteamworksServerIssuedAuthSessionTicket] {
        &self.auth_session_tickets
    }

    /// Returns an issued auth session ticket snapshot for a ticket handle.
    pub fn auth_session_ticket(
        &self,
        ticket: steamworks::AuthTicket,
    ) -> Option<&SteamworksServerIssuedAuthSessionTicket> {
        self.auth_session_tickets
            .iter()
            .find(|issued| issued.ticket == ticket)
    }

    /// Returns the most recent identity-scoped auth session ticket issued through this command layer.
    pub fn last_auth_session_ticket_for_identity(
        &self,
    ) -> Option<&SteamworksServerIssuedAuthSessionTicketForIdentity> {
        self.last_auth_session_ticket_for_identity.as_ref()
    }

    /// Returns currently cached identity-scoped auth session tickets by ticket handle.
    ///
    /// Ticket bytes are removed from this cache when the ticket is cancelled or
    /// Steam reports ticket creation failure.
    pub fn auth_session_tickets_for_identity(
        &self,
    ) -> &[SteamworksServerIssuedAuthSessionTicketForIdentity] {
        &self.auth_session_tickets_for_identity
    }

    /// Returns an identity-scoped auth session ticket snapshot for a ticket handle.
    pub fn auth_session_ticket_for_identity(
        &self,
        ticket: steamworks::AuthTicket,
    ) -> Option<&SteamworksServerIssuedAuthSessionTicketForIdentity> {
        self.auth_session_tickets_for_identity
            .iter()
            .find(|issued| issued.ticket == ticket)
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

    /// Returns how many auth session tickets this plugin issued.
    pub fn auth_session_ticket_issue_count(&self) -> u64 {
        self.auth_session_ticket_issue_count
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
}
