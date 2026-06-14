use bevy_ecs::prelude::Resource;

use super::*;

/// Runtime state for [`crate::SteamworksServerPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksServerState {
    last_error: Option<SteamworksServerError>,
    steam_id: Option<steamworks::SteamId>,
    steam_server_connected: Option<bool>,
    active_auth_tickets: Vec<steamworks::AuthTicket>,
    authenticated_users: Vec<steamworks::SteamId>,
    last_auth_session_ticket: Option<SteamworksServerIssuedAuthSessionTicket>,
    last_cancelled_auth_ticket: Option<steamworks::AuthTicket>,
    last_started_authentication_session: Option<steamworks::SteamId>,
    last_ended_authentication_session: Option<steamworks::SteamId>,
    auth_session_ticket_issue_count: u64,
    auth_ticket_cancel_count: u64,
    authentication_session_start_count: u64,
    authentication_session_end_count: u64,
    last_steam_server_connection_event: Option<SteamworksSteamServerConnectionEvent>,
    last_auth_ticket_response: Option<SteamworksAuthSessionTicketResponse>,
    last_auth_ticket_validation: Option<SteamworksAuthTicketValidation>,
    last_client_approval: Option<SteamworksServerClientApproval>,
    last_client_denial: Option<SteamworksServerClientDenial>,
    last_client_kick: Option<SteamworksServerClientKick>,
    last_client_group_status: Option<SteamworksServerClientGroupStatus>,
    product: Option<String>,
    game_description: Option<String>,
    game_data: Option<String>,
    dedicated: Option<bool>,
    anonymous_logon_submitted: bool,
    token_logon_submitted: bool,
    advertise_server_active: Option<bool>,
    mod_dir: Option<String>,
    map_name: Option<String>,
    server_name: Option<String>,
    max_players: Option<i32>,
    game_tags: Option<String>,
    key_values: Vec<(String, String)>,
    password_protected: Option<bool>,
    bot_player_count: Option<i32>,
    last_incoming_packet: Option<SteamworksServerIncomingPacket>,
    incoming_packet_count: u64,
    last_outgoing_packets: Vec<SteamworksServerOutgoingPacket>,
    outgoing_packet_drain_count: u64,
}

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
    /// Handles are removed after [`SteamworksServerCommand::CancelAuthenticationTicket`]
    /// is processed for the same ticket or after Steam reports ticket creation failure
    /// for the same ticket.
    pub fn active_auth_tickets(&self) -> &[steamworks::AuthTicket] {
        &self.active_auth_tickets
    }

    /// Returns users currently considered authenticated or approved by this layer.
    ///
    /// Entries are removed after [`SteamworksServerCommand::EndAuthenticationSession`]
    /// is processed for the same user or after Steam reports validation failure,
    /// denial, or kick for the same user.
    pub fn authenticated_users(&self) -> &[steamworks::SteamId] {
        &self.authenticated_users
    }

    /// Returns the most recent auth session ticket issued through this command layer.
    pub fn last_auth_session_ticket(&self) -> Option<&SteamworksServerIssuedAuthSessionTicket> {
        self.last_auth_session_ticket.as_ref()
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

    pub(super) fn record_error(&mut self, error: SteamworksServerError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksServerOperation) {
        match operation {
            SteamworksServerOperation::SteamIdRead { steam_id } => {
                self.steam_id = Some(*steam_id);
            }
            SteamworksServerOperation::AuthenticationSessionTicketIssued {
                ticket,
                ticket_bytes,
                steam_id,
            } => {
                if !self.active_auth_tickets.contains(ticket) {
                    self.active_auth_tickets.push(*ticket);
                }
                self.last_auth_session_ticket = Some(SteamworksServerIssuedAuthSessionTicket {
                    ticket: *ticket,
                    ticket_bytes: ticket_bytes.clone(),
                    steam_id: *steam_id,
                });
                self.auth_session_ticket_issue_count =
                    self.auth_session_ticket_issue_count.saturating_add(1);
            }
            SteamworksServerOperation::AuthenticationTicketCancelled { ticket } => {
                self.active_auth_tickets.retain(|known| known != ticket);
                self.last_cancelled_auth_ticket = Some(*ticket);
                self.auth_ticket_cancel_count = self.auth_ticket_cancel_count.saturating_add(1);
            }
            SteamworksServerOperation::AuthenticationSessionStarted { user } => {
                if !self.authenticated_users.contains(user) {
                    self.authenticated_users.push(*user);
                }
                self.last_started_authentication_session = Some(*user);
                self.authentication_session_start_count =
                    self.authentication_session_start_count.saturating_add(1);
            }
            SteamworksServerOperation::AuthenticationSessionEnded { user } => {
                self.authenticated_users.retain(|known| known != user);
                self.last_ended_authentication_session = Some(*user);
                self.authentication_session_end_count =
                    self.authentication_session_end_count.saturating_add(1);
            }
            SteamworksServerOperation::AuthenticationSessionTicketResponse { response } => {
                if response.result.is_err() {
                    self.active_auth_tickets
                        .retain(|known| *known != response.ticket);
                }
                self.last_auth_ticket_response = Some(response.clone());
            }
            SteamworksServerOperation::AuthenticationTicketValidationReceived { validation } => {
                if validation.response.is_err() {
                    self.authenticated_users
                        .retain(|known| *known != validation.steam_id);
                }
                self.last_auth_ticket_validation = Some(validation.clone());
            }
            SteamworksServerOperation::SteamServerConnectionEventReceived { event } => {
                self.steam_server_connected = Some(matches!(
                    event,
                    SteamworksSteamServerConnectionEvent::Connected
                ));
                self.last_steam_server_connection_event = Some(event.clone());
            }
            SteamworksServerOperation::ClientApproved { approval } => {
                if !self.authenticated_users.contains(&approval.user) {
                    self.authenticated_users.push(approval.user);
                }
                self.last_client_approval = Some(approval.clone());
            }
            SteamworksServerOperation::ClientDenied { denial } => {
                self.authenticated_users
                    .retain(|known| *known != denial.user);
                self.last_client_denial = Some(denial.clone());
            }
            SteamworksServerOperation::ClientKicked { kick } => {
                self.authenticated_users.retain(|known| *known != kick.user);
                self.last_client_kick = Some(kick.clone());
            }
            SteamworksServerOperation::ClientGroupStatusReceived { status } => {
                self.last_client_group_status = Some(status.clone());
            }
            SteamworksServerOperation::ProductSet { product } => {
                self.product = Some(product.clone());
            }
            SteamworksServerOperation::GameDescriptionSet { description } => {
                self.game_description = Some(description.clone());
            }
            SteamworksServerOperation::GameDataSet { data } => {
                self.game_data = Some(data.clone());
            }
            SteamworksServerOperation::DedicatedServerSet { dedicated } => {
                self.dedicated = Some(*dedicated);
            }
            SteamworksServerOperation::AnonymousLogonSubmitted => {
                self.anonymous_logon_submitted = true;
            }
            SteamworksServerOperation::TokenLogonSubmitted => {
                self.token_logon_submitted = true;
            }
            SteamworksServerOperation::AdvertiseServerActiveSet { active } => {
                self.advertise_server_active = Some(*active);
            }
            SteamworksServerOperation::ModDirSet { mod_dir } => {
                self.mod_dir = Some(mod_dir.clone());
            }
            SteamworksServerOperation::MapNameSet { map_name } => {
                self.map_name = Some(map_name.clone());
            }
            SteamworksServerOperation::ServerNameSet { server_name } => {
                self.server_name = Some(server_name.clone());
            }
            SteamworksServerOperation::MaxPlayersSet { count } => {
                self.max_players = Some(*count);
            }
            SteamworksServerOperation::GameTagsSet { tags } => {
                self.game_tags = Some(tags.clone());
            }
            SteamworksServerOperation::KeyValueSet { key, value } => {
                if let Some((_, known_value)) = self
                    .key_values
                    .iter_mut()
                    .find(|(known_key, _)| known_key == key)
                {
                    *known_value = value.clone();
                } else {
                    self.key_values.push((key.clone(), value.clone()));
                }
            }
            SteamworksServerOperation::AllKeyValuesCleared => {
                self.key_values.clear();
            }
            SteamworksServerOperation::PasswordProtectedSet { protected } => {
                self.password_protected = Some(*protected);
            }
            SteamworksServerOperation::BotPlayerCountSet { count } => {
                self.bot_player_count = Some(*count);
            }
            SteamworksServerOperation::IncomingPacketHandled {
                addr,
                bytes,
                accepted,
            } => {
                self.last_incoming_packet = Some(SteamworksServerIncomingPacket {
                    addr: *addr,
                    bytes: *bytes,
                    accepted: *accepted,
                });
                self.incoming_packet_count = self.incoming_packet_count.saturating_add(1);
            }
            SteamworksServerOperation::OutgoingPacketsDrained { packets } => {
                self.last_outgoing_packets = packets.clone();
                self.outgoing_packet_drain_count =
                    self.outgoing_packet_drain_count.saturating_add(1);
            }
        }
    }
}
