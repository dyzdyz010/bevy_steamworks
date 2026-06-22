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

    /// Returns users currently considered authenticated or approved by this layer.
    ///
    /// Entries are removed after [`crate::SteamworksServerCommand::EndAuthenticationSession`]
    /// is processed for the same user or after Steam reports validation failure,
    /// denial, or kick for the same user.
    pub fn authenticated_users(&self) -> &[steamworks::SteamId] {
        &self.authenticated_users
    }

    /// Returns the most recent auth session ticket issued through this command layer.
    pub fn last_auth_session_ticket(&self) -> Option<&SteamworksServerIssuedAuthSessionTicket> {
        self.last_auth_session_ticket.as_ref()
    }

    /// Returns the most recent identity-scoped auth session ticket issued through this command layer.
    pub fn last_auth_session_ticket_for_identity(
        &self,
    ) -> Option<&SteamworksServerIssuedAuthSessionTicketForIdentity> {
        self.last_auth_session_ticket_for_identity.as_ref()
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

    /// Returns the most recent auth session ticket response callback snapshot.
    pub fn last_auth_ticket_response(&self) -> Option<&SteamworksAuthSessionTicketResponse> {
        self.last_auth_ticket_response.as_ref()
    }

    /// Returns the most recent auth ticket validation callback snapshot.
    pub fn last_auth_ticket_validation(&self) -> Option<&SteamworksAuthTicketValidation> {
        self.last_auth_ticket_validation.as_ref()
    }

    /// Returns the most recent game-server client approval callback snapshot.
    pub fn last_client_approval(&self) -> Option<&SteamworksServerClientApproval> {
        self.last_client_approval.as_ref()
    }

    /// Returns the most recent game-server client denial callback snapshot.
    pub fn last_client_denial(&self) -> Option<&SteamworksServerClientDenial> {
        self.last_client_denial.as_ref()
    }

    /// Returns the most recent game-server client kick callback snapshot.
    pub fn last_client_kick(&self) -> Option<&SteamworksServerClientKick> {
        self.last_client_kick.as_ref()
    }

    /// Returns the most recent game-server group status callback snapshot.
    pub fn last_client_group_status(&self) -> Option<&SteamworksServerClientGroupStatus> {
        self.last_client_group_status.as_ref()
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
