use crate::user::SteamworksSteamServerConnectionEvent;

use super::{
    push_bounded, upsert_by, SteamworksServerIncomingPacket,
    SteamworksServerIssuedAuthSessionTicket, SteamworksServerIssuedAuthSessionTicketForIdentity,
    SteamworksServerOperation, SteamworksServerState,
};

impl SteamworksServerState {
    pub(in crate::game_server) fn record_error(&mut self, error: super::SteamworksServerError) {
        self.last_error = Some(error);
    }

    pub(in crate::game_server) fn record_operation(
        &mut self,
        operation: &SteamworksServerOperation,
    ) {
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
                let issued = SteamworksServerIssuedAuthSessionTicket {
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
            SteamworksServerOperation::AuthenticationSessionTicketForIdentityIssued {
                ticket,
                ticket_bytes,
                identity,
            } => {
                if !self.active_auth_tickets.contains(ticket) {
                    self.active_auth_tickets.push(*ticket);
                }
                let issued = SteamworksServerIssuedAuthSessionTicketForIdentity {
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
            SteamworksServerOperation::AuthenticationTicketCancelled { ticket } => {
                self.active_auth_tickets.retain(|known| known != ticket);
                remove_ticket_caches(self, *ticket);
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
                    remove_ticket_caches(self, response.ticket);
                }
                upsert_by(
                    &mut self.auth_ticket_responses,
                    response.clone(),
                    |existing| existing.ticket == response.ticket,
                );
                self.last_auth_ticket_response = Some(response.clone());
            }
            SteamworksServerOperation::AuthenticationTicketValidationReceived { validation } => {
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
            SteamworksServerOperation::SteamServerConnectionEventReceived { event } => {
                self.steam_server_connected = Some(matches!(
                    event,
                    SteamworksSteamServerConnectionEvent::Connected
                ));
                push_bounded(&mut self.steam_server_connection_events, event.clone());
                self.last_steam_server_connection_event = Some(event.clone());
            }
            SteamworksServerOperation::ClientApproved { approval } => {
                if !self.authenticated_users.contains(&approval.user) {
                    self.authenticated_users.push(approval.user);
                }
                upsert_by(&mut self.client_approvals, approval.clone(), |existing| {
                    existing.user == approval.user
                });
                self.last_client_approval = Some(approval.clone());
            }
            SteamworksServerOperation::ClientDenied { denial } => {
                self.authenticated_users
                    .retain(|known| *known != denial.user);
                upsert_by(&mut self.client_denials, denial.clone(), |existing| {
                    existing.user == denial.user
                });
                self.last_client_denial = Some(denial.clone());
            }
            SteamworksServerOperation::ClientKicked { kick } => {
                self.authenticated_users.retain(|known| *known != kick.user);
                upsert_by(&mut self.client_kicks, kick.clone(), |existing| {
                    existing.user == kick.user
                });
                self.last_client_kick = Some(kick.clone());
            }
            SteamworksServerOperation::ClientGroupStatusReceived { status } => {
                upsert_by(
                    &mut self.client_group_statuses,
                    status.clone(),
                    |existing| existing.user == status.user && existing.group == status.group,
                );
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
            SteamworksServerOperation::HeartbeatsEnabled { active } => {
                self.heartbeats_active = Some(*active);
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

fn remove_ticket_caches(state: &mut SteamworksServerState, ticket: steamworks::AuthTicket) {
    state
        .auth_session_tickets
        .retain(|issued| issued.ticket != ticket);
    state
        .auth_session_tickets_for_identity
        .retain(|issued| issued.ticket != ticket);
    state
        .auth_ticket_responses
        .retain(|response| response.ticket != ticket);
}
