use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::{SteamworksClient, SteamworksEvent, SteamworksServer};

use super::{
    callbacks::process_networking_steam_events,
    messages::{
        SteamworksNetworkingCommand, SteamworksNetworkingError, SteamworksNetworkingOperation,
        SteamworksNetworkingResult,
    },
    state::SteamworksNetworkingState,
    types::{
        SteamworksP2pPacket, SteamworksP2pPacketAvailability, SteamworksP2pSessionState,
        SteamworksP2pSessionStateResult,
    },
    validation::validate_command,
};

pub(super) fn process_networking_commands(
    client: Option<Res<SteamworksClient>>,
    server: Option<Res<SteamworksServer>>,
    mut state: ResMut<SteamworksNetworkingState>,
    mut commands: ResMut<Messages<SteamworksNetworkingCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksNetworkingResult>,
) {
    process_networking_steam_events(&mut state, &mut steam_events, &mut results);

    for command in commands.drain() {
        match handle_networking_command(client.as_deref(), server.as_deref(), &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks legacy networking command"
                );
                results.write(SteamworksNetworkingResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks legacy networking command failed"
                );
                results.write(SteamworksNetworkingResult::Err { command, error });
            }
        }
    }
}

fn handle_networking_command(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
    command: &SteamworksNetworkingCommand,
) -> Result<SteamworksNetworkingOperation, SteamworksNetworkingError> {
    validate_command(command)?;

    let networking = networking(client, server)?;
    Ok(match command {
        SteamworksNetworkingCommand::AcceptP2pSession { user } => {
            if !networking.accept_p2p_session(*user) {
                return Err(SteamworksNetworkingError::operation_failed(
                    "networking.accept_p2p_session",
                ));
            }
            SteamworksNetworkingOperation::SessionAccepted { user: *user }
        }
        SteamworksNetworkingCommand::CloseP2pSession { user } => {
            if !networking.close_p2p_session(*user) {
                return Err(SteamworksNetworkingError::operation_failed(
                    "networking.close_p2p_session",
                ));
            }
            SteamworksNetworkingOperation::SessionClosed { user: *user }
        }
        SteamworksNetworkingCommand::GetP2pSessionState { user } => {
            SteamworksNetworkingOperation::SessionStateRead {
                state: SteamworksP2pSessionStateResult {
                    user: *user,
                    state: networking
                        .get_p2p_session_state(*user)
                        .map(SteamworksP2pSessionState::from),
                },
            }
        }
        SteamworksNetworkingCommand::SendP2pPacket {
            remote,
            send_type,
            channel,
            data,
        } => {
            if !networking.send_p2p_packet_on_channel(
                *remote,
                send_type.to_steam(),
                data,
                *channel as i32,
            ) {
                return Err(SteamworksNetworkingError::operation_failed(
                    "networking.send_p2p_packet_on_channel",
                ));
            }
            SteamworksNetworkingOperation::PacketSent {
                remote: *remote,
                send_type: *send_type,
                channel: *channel,
                bytes: data.len(),
            }
        }
        SteamworksNetworkingCommand::GetAvailablePacketSize { channel } => {
            SteamworksNetworkingOperation::PacketAvailabilityRead {
                availability: SteamworksP2pPacketAvailability {
                    channel: *channel,
                    bytes: networking.is_p2p_packet_available_on_channel(*channel as i32),
                },
            }
        }
        SteamworksNetworkingCommand::ReadP2pPacket { channel, max_bytes } => {
            let Some(available_bytes) =
                networking.is_p2p_packet_available_on_channel(*channel as i32)
            else {
                return Ok(SteamworksNetworkingOperation::PacketRead {
                    channel: *channel,
                    packet: None,
                });
            };

            if available_bytes > *max_bytes {
                return Err(SteamworksNetworkingError::PacketExceedsReadBuffer {
                    available_bytes,
                    max_bytes: *max_bytes,
                });
            }

            let mut buffer = vec![0; *max_bytes];
            let packet = networking
                .read_p2p_packet_from_channel(&mut buffer, *channel as i32)
                .map(|(remote, bytes)| {
                    if bytes > buffer.len() {
                        return Err(SteamworksNetworkingError::PacketExceedsReadBuffer {
                            available_bytes: bytes,
                            max_bytes: buffer.len(),
                        });
                    }
                    buffer.truncate(bytes);
                    Ok(SteamworksP2pPacket {
                        remote,
                        channel: *channel,
                        data: buffer,
                    })
                })
                .transpose()?;
            SteamworksNetworkingOperation::PacketRead {
                channel: *channel,
                packet,
            }
        }
    })
}

fn networking(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
) -> Result<steamworks::Networking, SteamworksNetworkingError> {
    if let Some(client) = client {
        Ok(client.networking())
    } else if let Some(server) = server {
        Ok(server.networking())
    } else {
        Err(SteamworksNetworkingError::ClientUnavailable)
    }
}
