use crate::user::{
    SteamworksAuthSessionTicketResponse, SteamworksAuthTicketValidation,
    SteamworksSteamServerConnectionEvent,
};

use super::{
    SteamworksServerClientApproval, SteamworksServerClientDenial,
    SteamworksServerClientGroupStatus, SteamworksServerClientKick, SteamworksServerError,
    SteamworksServerIncomingPacket, SteamworksServerIssuedAuthSessionTicket,
    SteamworksServerIssuedAuthSessionTicketForIdentity, SteamworksServerOutgoingPacket,
    SteamworksServerState,
};

impl SteamworksServerState {
    /// Returns the most recent synchronous error observed by the server plugin.
    pub fn last_error(&self) -> Option<&SteamworksServerError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent Steam ID read for this game server.
    pub fn steam_id(&self) -> Option<steamworks::SteamId> {
        self.steam_id
    }

    /// Returns the latest known Steam server connection state.
    ///
    /// This is updated by Steam server connection callbacks.
    pub fn steam_server_connected(&self) -> Option<bool> {
        self.steam_server_connected
    }

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

    /// Returns the most recent game-server client approval callback snapshot.
    pub fn last_client_approval(&self) -> Option<&SteamworksServerClientApproval> {
        self.last_client_approval.as_ref()
    }

    /// Returns bounded game-server client approval callback snapshots by user.
    pub fn client_approvals(&self) -> &[SteamworksServerClientApproval] {
        &self.client_approvals
    }

    /// Returns the latest game-server client approval for a Steam user.
    pub fn client_approval(
        &self,
        user: steamworks::SteamId,
    ) -> Option<&SteamworksServerClientApproval> {
        self.client_approvals
            .iter()
            .find(|approval| approval.user == user)
    }

    /// Returns whether a client approval callback has been cached for a user.
    pub fn has_client_approval(&self, user: steamworks::SteamId) -> bool {
        self.client_approval(user).is_some()
    }

    /// Returns the latest cached game owner from a client approval callback.
    pub fn client_approval_owner(&self, user: steamworks::SteamId) -> Option<steamworks::SteamId> {
        self.client_approval(user).map(|approval| approval.owner)
    }

    /// Returns the most recent game-server client denial callback snapshot.
    pub fn last_client_denial(&self) -> Option<&SteamworksServerClientDenial> {
        self.last_client_denial.as_ref()
    }

    /// Returns bounded game-server client denial callback snapshots by user.
    pub fn client_denials(&self) -> &[SteamworksServerClientDenial] {
        &self.client_denials
    }

    /// Returns the latest game-server client denial for a Steam user.
    pub fn client_denial(
        &self,
        user: steamworks::SteamId,
    ) -> Option<&SteamworksServerClientDenial> {
        self.client_denials
            .iter()
            .find(|denial| denial.user == user)
    }

    /// Returns whether a client denial callback has been cached for a user.
    pub fn has_client_denial(&self, user: steamworks::SteamId) -> bool {
        self.client_denial(user).is_some()
    }

    /// Returns the latest cached client denial reason for a user.
    pub fn client_denial_reason(
        &self,
        user: steamworks::SteamId,
    ) -> Option<steamworks::DenyReason> {
        self.client_denial(user).map(|denial| denial.deny_reason)
    }

    /// Returns the most recent game-server client kick callback snapshot.
    pub fn last_client_kick(&self) -> Option<&SteamworksServerClientKick> {
        self.last_client_kick.as_ref()
    }

    /// Returns bounded game-server client kick callback snapshots by user.
    pub fn client_kicks(&self) -> &[SteamworksServerClientKick] {
        &self.client_kicks
    }

    /// Returns the latest game-server client kick for a Steam user.
    pub fn client_kick(&self, user: steamworks::SteamId) -> Option<&SteamworksServerClientKick> {
        self.client_kicks.iter().find(|kick| kick.user == user)
    }

    /// Returns whether a client kick callback has been cached for a user.
    pub fn has_client_kick(&self, user: steamworks::SteamId) -> bool {
        self.client_kick(user).is_some()
    }

    /// Returns the latest cached client kick reason for a user.
    pub fn client_kick_reason(&self, user: steamworks::SteamId) -> Option<steamworks::DenyReason> {
        self.client_kick(user).map(|kick| kick.deny_reason)
    }

    /// Returns the most recent game-server group status callback snapshot.
    pub fn last_client_group_status(&self) -> Option<&SteamworksServerClientGroupStatus> {
        self.last_client_group_status.as_ref()
    }

    /// Returns bounded game-server group status callback snapshots by user/group.
    pub fn client_group_statuses(&self) -> &[SteamworksServerClientGroupStatus] {
        &self.client_group_statuses
    }

    /// Returns the latest game-server group status for a Steam user/group pair.
    pub fn client_group_status(
        &self,
        user: steamworks::SteamId,
        group: steamworks::SteamId,
    ) -> Option<&SteamworksServerClientGroupStatus> {
        self.client_group_statuses
            .iter()
            .find(|status| status.user == user && status.group == group)
    }

    /// Returns whether the latest cached group status says a user is a group member.
    pub fn client_group_member(
        &self,
        user: steamworks::SteamId,
        group: steamworks::SteamId,
    ) -> Option<bool> {
        self.client_group_status(user, group)
            .map(|status| status.member)
    }

    /// Returns whether the latest cached group status says a user is a group officer.
    pub fn client_group_officer(
        &self,
        user: steamworks::SteamId,
        group: steamworks::SteamId,
    ) -> Option<bool> {
        self.client_group_status(user, group)
            .map(|status| status.officer)
    }

    /// Returns the most recent product string submitted through this command layer.
    pub fn product(&self) -> Option<&str> {
        self.product.as_deref()
    }

    /// Returns the most recent game description submitted through this command layer.
    pub fn game_description(&self) -> Option<&str> {
        self.game_description.as_deref()
    }

    /// Returns the most recent game data string submitted through this command layer.
    pub fn game_data(&self) -> Option<&str> {
        self.game_data.as_deref()
    }

    /// Returns the most recent dedicated/listen server flag submitted through this command layer.
    pub fn dedicated(&self) -> Option<bool> {
        self.dedicated
    }

    /// Returns whether anonymous logon was submitted through this command layer.
    pub fn anonymous_logon_submitted(&self) -> bool {
        self.anonymous_logon_submitted
    }

    /// Returns whether token-based logon was submitted through this command layer.
    pub fn token_logon_submitted(&self) -> bool {
        self.token_logon_submitted
    }

    /// Returns whether any server logon command was submitted through this command layer.
    pub fn logon_submitted(&self) -> bool {
        self.anonymous_logon_submitted || self.token_logon_submitted
    }

    /// Returns the most recent advertise-server-active flag submitted through this command layer.
    pub fn advertise_server_active(&self) -> Option<bool> {
        self.advertise_server_active
    }

    /// Returns the most recent heartbeat-active flag submitted through this command layer.
    pub fn heartbeats_active(&self) -> Option<bool> {
        self.heartbeats_active
    }

    /// Returns the most recent mod dir submitted through this command layer.
    pub fn mod_dir(&self) -> Option<&str> {
        self.mod_dir.as_deref()
    }

    /// Returns the most recent map name submitted through this command layer.
    pub fn map_name(&self) -> Option<&str> {
        self.map_name.as_deref()
    }

    /// Returns the most recent server name submitted through this command layer.
    pub fn server_name(&self) -> Option<&str> {
        self.server_name.as_deref()
    }

    /// Returns the most recent maximum player count submitted through this command layer.
    pub fn max_players(&self) -> Option<i32> {
        self.max_players
    }

    /// Returns the most recent game tags string submitted through this command layer.
    pub fn game_tags(&self) -> Option<&str> {
        self.game_tags.as_deref()
    }

    /// Returns key/value rules submitted through this command layer.
    pub fn key_values(&self) -> &[(String, String)] {
        &self.key_values
    }

    /// Returns a server browser rule value submitted through this command layer.
    pub fn key_value(&self, key: &str) -> Option<&str> {
        self.key_values
            .iter()
            .find_map(|(known_key, value)| (known_key == key).then_some(value.as_str()))
    }

    /// Returns the most recent password-protected flag submitted through this command layer.
    pub fn password_protected(&self) -> Option<bool> {
        self.password_protected
    }

    /// Returns the most recent bot player count submitted through this command layer.
    pub fn bot_player_count(&self) -> Option<i32> {
        self.bot_player_count
    }

    /// Returns the most recent shared-query incoming packet forwarded to Steam.
    pub fn last_incoming_packet(&self) -> Option<&SteamworksServerIncomingPacket> {
        self.last_incoming_packet.as_ref()
    }

    /// Returns how many shared-query incoming packets this command layer forwarded.
    pub fn incoming_packet_count(&self) -> u64 {
        self.incoming_packet_count
    }

    /// Returns packets returned by the most recent outgoing packet drain command.
    pub fn last_outgoing_packets(&self) -> &[SteamworksServerOutgoingPacket] {
        &self.last_outgoing_packets
    }

    /// Returns how many shared-query outgoing packet drain commands this layer processed.
    pub fn outgoing_packet_drain_count(&self) -> u64 {
        self.outgoing_packet_drain_count
    }
}
