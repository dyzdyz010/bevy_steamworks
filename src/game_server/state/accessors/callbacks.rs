use crate::{
    game_server::*, SteamworksAuthSessionTicketResponse, SteamworksAuthTicketValidation,
    SteamworksSteamServerConnectionEvent,
};

impl SteamworksServerState {
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
}
