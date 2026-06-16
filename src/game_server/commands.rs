use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::SteamworksEvent;

use super::{
    callbacks::process_server_steam_events, packets::drain_outgoing_packets,
    validation::validate_server_command_for_state, SteamworksServer, SteamworksServerCommand,
    SteamworksServerError, SteamworksServerOperation, SteamworksServerResult,
    SteamworksServerState,
};

pub(super) fn process_server_commands(
    server: Option<Res<SteamworksServer>>,
    mut state: ResMut<SteamworksServerState>,
    mut commands: ResMut<Messages<SteamworksServerCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksServerResult>,
) {
    process_server_steam_events(&mut state, &mut steam_events, &mut results);

    let Some(server) = server else {
        let error = SteamworksServerError::ServerUnavailable;
        for command in commands.drain() {
            state.record_error(error.clone());
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steam Game Server command failed"
            );
            results.write(SteamworksServerResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        match handle_server_command(&server, &state, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steam Game Server command"
                );
                results.write(SteamworksServerResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steam Game Server command failed"
                );
                results.write(SteamworksServerResult::Err { command, error });
            }
        }
    }
}

fn handle_server_command(
    server: &SteamworksServer,
    state: &SteamworksServerState,
    command: &SteamworksServerCommand,
) -> Result<SteamworksServerOperation, SteamworksServerError> {
    validate_server_command_for_state(command, state)?;

    Ok(match command {
        SteamworksServerCommand::GetSteamId => SteamworksServerOperation::SteamIdRead {
            steam_id: server.steam_id(),
        },
        SteamworksServerCommand::GetAuthenticationSessionTicket { steam_id } => {
            let (ticket, ticket_bytes) =
                server.authentication_session_ticket_with_steam_id(*steam_id);
            SteamworksServerOperation::AuthenticationSessionTicketIssued {
                ticket,
                ticket_bytes,
                steam_id: *steam_id,
            }
        }
        SteamworksServerCommand::GetAuthenticationSessionTicketForIdentity { identity } => {
            let (ticket, ticket_bytes) = server.authentication_session_ticket(identity.clone());
            SteamworksServerOperation::AuthenticationSessionTicketForIdentityIssued {
                ticket,
                ticket_bytes,
                identity: identity.clone(),
            }
        }
        SteamworksServerCommand::CancelAuthenticationTicket { ticket } => {
            server.cancel_authentication_ticket(*ticket);
            SteamworksServerOperation::AuthenticationTicketCancelled { ticket: *ticket }
        }
        SteamworksServerCommand::BeginAuthenticationSession { user, ticket } => {
            server
                .begin_authentication_session(*user, ticket)
                .map_err(SteamworksServerError::auth_session)?;
            SteamworksServerOperation::AuthenticationSessionStarted { user: *user }
        }
        SteamworksServerCommand::EndAuthenticationSession { user } => {
            server.end_authentication_session(*user);
            SteamworksServerOperation::AuthenticationSessionEnded { user: *user }
        }
        SteamworksServerCommand::HandleIncomingPacket { data, addr } => {
            SteamworksServerOperation::IncomingPacketHandled {
                addr: *addr,
                bytes: data.len(),
                accepted: server.handle_incoming_packet(data, *addr),
            }
        }
        SteamworksServerCommand::SetProduct { product } => {
            server.set_product(product);
            SteamworksServerOperation::ProductSet {
                product: product.clone(),
            }
        }
        SteamworksServerCommand::SetGameDescription { description } => {
            server.set_game_description(description);
            SteamworksServerOperation::GameDescriptionSet {
                description: description.clone(),
            }
        }
        SteamworksServerCommand::SetGameData { data } => {
            server.set_game_data(data);
            SteamworksServerOperation::GameDataSet { data: data.clone() }
        }
        SteamworksServerCommand::SetDedicatedServer { dedicated } => {
            server.set_dedicated_server(*dedicated);
            SteamworksServerOperation::DedicatedServerSet {
                dedicated: *dedicated,
            }
        }
        SteamworksServerCommand::LogOnAnonymous => {
            server.log_on_anonymous();
            SteamworksServerOperation::AnonymousLogonSubmitted
        }
        SteamworksServerCommand::LogOn { token } => {
            server.log_on(token.as_str());
            SteamworksServerOperation::TokenLogonSubmitted
        }
        SteamworksServerCommand::SetAdvertiseServerActive { active } => {
            server.set_advertise_server_active(*active);
            SteamworksServerOperation::AdvertiseServerActiveSet { active: *active }
        }
        SteamworksServerCommand::SetModDir { mod_dir } => {
            server.set_mod_dir(mod_dir);
            SteamworksServerOperation::ModDirSet {
                mod_dir: mod_dir.clone(),
            }
        }
        SteamworksServerCommand::SetMapName { map_name } => {
            server.set_map_name(map_name);
            SteamworksServerOperation::MapNameSet {
                map_name: map_name.clone(),
            }
        }
        SteamworksServerCommand::SetServerName { server_name } => {
            server.set_server_name(server_name);
            SteamworksServerOperation::ServerNameSet {
                server_name: server_name.clone(),
            }
        }
        SteamworksServerCommand::SetMaxPlayers { count } => {
            server.set_max_players(*count);
            SteamworksServerOperation::MaxPlayersSet { count: *count }
        }
        SteamworksServerCommand::SetGameTags { tags } => {
            server.set_game_tags(tags);
            SteamworksServerOperation::GameTagsSet { tags: tags.clone() }
        }
        SteamworksServerCommand::SetKeyValue { key, value } => {
            server.set_key_value(key, value);
            SteamworksServerOperation::KeyValueSet {
                key: key.clone(),
                value: value.clone(),
            }
        }
        SteamworksServerCommand::ClearAllKeyValues => {
            server.clear_all_key_values();
            SteamworksServerOperation::AllKeyValuesCleared
        }
        SteamworksServerCommand::SetPasswordProtected { protected } => {
            server.set_password_protected(*protected);
            SteamworksServerOperation::PasswordProtectedSet {
                protected: *protected,
            }
        }
        SteamworksServerCommand::SetBotPlayerCount { count } => {
            server.set_bot_player_count(*count);
            SteamworksServerOperation::BotPlayerCountSet { count: *count }
        }
        SteamworksServerCommand::DrainOutgoingPackets => {
            SteamworksServerOperation::OutgoingPacketsDrained {
                packets: drain_outgoing_packets(server),
            }
        }
    })
}
