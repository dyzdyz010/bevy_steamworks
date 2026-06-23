use bevy_ecs::{
    message::{MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::{SteamworksClient, SteamworksServer};

use super::{
    messages::{
        SteamworksNetworkingMessagesCommand, SteamworksNetworkingMessagesError,
        SteamworksNetworkingMessagesOperation, SteamworksNetworkingMessagesResult,
    },
    snapshots::{snapshot_networking_message, snapshot_session_connection_info},
    state::SteamworksNetworkingMessagesState,
    validation::validate_command,
};

pub(super) fn process_networking_messages_commands(
    client: Option<Res<SteamworksClient>>,
    server: Option<Res<SteamworksServer>>,
    mut state: ResMut<SteamworksNetworkingMessagesState>,
    mut commands: ResMut<Messages<SteamworksNetworkingMessagesCommand>>,
    mut results: MessageWriter<SteamworksNetworkingMessagesResult>,
) {
    for result in state.drain_callback_results() {
        record_networking_messages_result(&mut state, &result);
        results.write(result);
    }

    for command in commands.drain() {
        if let Err(error) = validate_command(&command) {
            state.record_error(error.clone());
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks Networking Messages command failed"
            );
            results.write(SteamworksNetworkingMessagesResult::Err { command, error });
            continue;
        }

        if let Some(operation) = local_networking_messages_command_operation(&command) {
            state.record_operation(&operation);
            tracing::debug!(
                target: "bevy_steamworks",
                operation = ?operation,
                "processed Steamworks Networking Messages command"
            );
            results.write(SteamworksNetworkingMessagesResult::Ok(operation));
            continue;
        }

        let Some(networking_messages) = networking_messages(client.as_deref(), server.as_deref())
        else {
            let error = SteamworksNetworkingMessagesError::ClientUnavailable;
            state.record_error(error.clone());
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks Networking Messages command failed"
            );
            results.write(SteamworksNetworkingMessagesResult::Err { command, error });
            continue;
        };

        match handle_networking_messages_command(&networking_messages, command.clone()) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks Networking Messages command"
                );
                results.write(SteamworksNetworkingMessagesResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks Networking Messages command failed"
                );
                results.write(SteamworksNetworkingMessagesResult::Err { command, error });
            }
        }
    }
}

fn record_networking_messages_result(
    state: &mut SteamworksNetworkingMessagesState,
    result: &SteamworksNetworkingMessagesResult,
) {
    match result {
        SteamworksNetworkingMessagesResult::Ok(operation) => state.record_operation(operation),
        SteamworksNetworkingMessagesResult::Err { error, .. } => {
            state.record_error(error.clone());
        }
    }
}

fn local_networking_messages_command_operation(
    command: &SteamworksNetworkingMessagesCommand,
) -> Option<SteamworksNetworkingMessagesOperation> {
    match command {
        SteamworksNetworkingMessagesCommand::SetAutoAcceptSessionRequests { enabled } => Some(
            SteamworksNetworkingMessagesOperation::AutoAcceptSessionRequestsSet {
                enabled: *enabled,
            },
        ),
        SteamworksNetworkingMessagesCommand::SetSessionRequestDecision { decision } => Some(
            SteamworksNetworkingMessagesOperation::SessionRequestDecisionSet {
                decision: decision.clone(),
            },
        ),
        SteamworksNetworkingMessagesCommand::ClearSessionRequestDecision { peer } => Some(
            SteamworksNetworkingMessagesOperation::SessionRequestDecisionCleared {
                peer: peer.clone(),
            },
        ),
        _ => None,
    }
}

fn handle_networking_messages_command(
    networking_messages: &steamworks::networking_messages::NetworkingMessages,
    command: SteamworksNetworkingMessagesCommand,
) -> Result<SteamworksNetworkingMessagesOperation, SteamworksNetworkingMessagesError> {
    Ok(match command {
        SteamworksNetworkingMessagesCommand::SendMessage {
            peer,
            send_flags,
            channel,
            data,
        } => {
            let bytes = data.len();
            networking_messages
                .send_message_to_user(peer.to_identity(), send_flags, &data, channel)
                .map_err(|source| {
                    SteamworksNetworkingMessagesError::steam_error(
                        "networking_messages.send_message_to_user",
                        source,
                    )
                })?;
            SteamworksNetworkingMessagesOperation::MessageSent {
                peer,
                channel,
                send_flags,
                bytes,
            }
        }
        SteamworksNetworkingMessagesCommand::ReceiveMessages {
            channel,
            batch_size,
        } => {
            let messages = networking_messages
                .receive_messages_on_channel(channel, batch_size)
                .into_iter()
                .map(snapshot_networking_message)
                .collect();
            SteamworksNetworkingMessagesOperation::MessagesReceived { channel, messages }
        }
        SteamworksNetworkingMessagesCommand::GetSessionConnectionInfo { peer } => {
            let (state, info, realtime) =
                networking_messages.get_session_connection_info(&peer.to_identity());
            SteamworksNetworkingMessagesOperation::SessionConnectionInfoRead {
                peer,
                info: snapshot_session_connection_info(state, info.as_ref(), realtime.as_ref()),
            }
        }
        SteamworksNetworkingMessagesCommand::SetAutoAcceptSessionRequests { enabled } => {
            SteamworksNetworkingMessagesOperation::AutoAcceptSessionRequestsSet { enabled }
        }
        SteamworksNetworkingMessagesCommand::SetSessionRequestDecision { decision } => {
            SteamworksNetworkingMessagesOperation::SessionRequestDecisionSet { decision }
        }
        SteamworksNetworkingMessagesCommand::ClearSessionRequestDecision { peer } => {
            SteamworksNetworkingMessagesOperation::SessionRequestDecisionCleared { peer }
        }
    })
}

fn networking_messages(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
) -> Option<steamworks::networking_messages::NetworkingMessages> {
    if let Some(client) = client {
        Some(client.networking_messages())
    } else {
        server.map(|server| server.networking_messages())
    }
}
