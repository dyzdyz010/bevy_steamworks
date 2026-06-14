//! High-level Bevy ECS integration for Steam's legacy P2P Networking API.
//!
//! This module builds on top of the upstream [`steamworks::Networking`] API. It
//! exists for older Steam P2P workflows; new projects should prefer
//! [`crate::SteamworksNetworkingMessagesPlugin`].

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
    schedule::IntoScheduleConfigs,
};

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

mod messages;
mod state;
#[cfg(test)]
mod tests;
mod types;

pub use messages::*;
pub use state::SteamworksNetworkingState;
pub use types::*;

/// Maximum unreliable legacy P2P packet size accepted by Steam.
pub const STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES: usize = 1_200;

/// Maximum reliable legacy P2P packet size accepted by Steam.
pub const STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES: usize = 1_048_576;

/// Maximum receive buffer this command layer will allocate in one frame.
pub const STEAMWORKS_P2P_MAX_READ_PACKET_BYTES: usize = STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES;

/// Bevy plugin for high-level legacy Steam P2P Networking commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksNetworkingCommand`] and [`SteamworksNetworkingResult`] messages,
/// observes legacy P2P callbacks from [`crate::SteamworksEvent`], and processes
/// commands in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksNetworkingPlugin;

impl SteamworksNetworkingPlugin {
    /// Creates a legacy P2P Networking plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksNetworkingState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksNetworkingCommand>()
            .add_message::<SteamworksNetworkingResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessNetworkingCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_networking_commands.in_set(SteamworksSystem::ProcessNetworkingCommands),
            );
    }
}

fn process_networking_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksNetworkingState>,
    mut commands: ResMut<Messages<SteamworksNetworkingCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksNetworkingResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::P2PSessionRequest(event) => {
                Some(SteamworksNetworkingOperation::SessionRequestReceived {
                    remote: event.remote,
                })
            }
            SteamworksEvent::P2PSessionConnectFail(event) => {
                Some(SteamworksNetworkingOperation::SessionConnectFailed {
                    remote: event.remote,
                    error: steamworks::P2PSessionError::from(event.error),
                })
            }
            _ => None,
        };

        if let Some(operation) = operation {
            state.record_operation(&operation);
            tracing::debug!(
                target: "bevy_steamworks",
                operation = ?operation,
                "processed Steamworks legacy P2P callback"
            );
            results.write(SteamworksNetworkingResult::Ok(operation));
        }
    }

    let Some(client) = client else {
        let error = SteamworksNetworkingError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks legacy networking command failed"
            );
            results.write(SteamworksNetworkingResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        match handle_networking_command(&client, &command) {
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
    client: &SteamworksClient,
    command: &SteamworksNetworkingCommand,
) -> Result<SteamworksNetworkingOperation, SteamworksNetworkingError> {
    validate_command(command)?;

    let networking = client.networking();
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

fn validate_command(
    command: &SteamworksNetworkingCommand,
) -> Result<(), SteamworksNetworkingError> {
    match command {
        SteamworksNetworkingCommand::AcceptP2pSession { user }
        | SteamworksNetworkingCommand::CloseP2pSession { user }
        | SteamworksNetworkingCommand::GetP2pSessionState { user } => validate_steam_id(*user),
        SteamworksNetworkingCommand::SendP2pPacket {
            remote,
            send_type,
            channel,
            data,
        } => {
            validate_steam_id(*remote)?;
            validate_channel(*channel)?;
            let max_bytes = send_type.max_packet_bytes();
            if data.len() > max_bytes {
                return Err(SteamworksNetworkingError::PacketTooLarge {
                    bytes: data.len(),
                    max_bytes,
                });
            }
            Ok(())
        }
        SteamworksNetworkingCommand::GetAvailablePacketSize { channel } => {
            validate_channel(*channel)
        }
        SteamworksNetworkingCommand::ReadP2pPacket { channel, max_bytes } => {
            validate_channel(*channel)?;
            if *max_bytes == 0 {
                return Err(SteamworksNetworkingError::InvalidReadBufferSize);
            }
            if *max_bytes > STEAMWORKS_P2P_MAX_READ_PACKET_BYTES {
                return Err(SteamworksNetworkingError::ReadBufferTooLarge {
                    max_bytes: *max_bytes,
                    max_supported: STEAMWORKS_P2P_MAX_READ_PACKET_BYTES,
                });
            }
            Ok(())
        }
    }
}

fn validate_steam_id(user: steamworks::SteamId) -> Result<(), SteamworksNetworkingError> {
    if user.raw() == 0 {
        return Err(SteamworksNetworkingError::InvalidSteamId);
    }
    Ok(())
}

fn validate_channel(channel: u32) -> Result<(), SteamworksNetworkingError> {
    if channel > i32::MAX as u32 {
        return Err(SteamworksNetworkingError::InvalidChannel { channel });
    }
    Ok(())
}
