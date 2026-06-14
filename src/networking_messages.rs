//! High-level Bevy ECS integration for Steam Networking Messages.
//!
//! This module builds on top of the upstream
//! [`steamworks::networking_messages::NetworkingMessages`] API. It exposes the
//! UDP-like Steam P2P message interface through Bevy commands/results while
//! copying received payloads into owned `Vec<u8>` values that are safe to keep
//! in ECS state.

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
    schedule::IntoScheduleConfigs,
};

use crate::{SteamworksClient, SteamworksSystem};

mod messages;
mod snapshots;
mod state;
#[cfg(test)]
mod tests;
mod types;

use snapshots::{snapshot_networking_message, snapshot_session_connection_info};

pub use messages::*;
pub use state::SteamworksNetworkingMessagesState;
pub use types::*;

/// Maximum number of messages one receive command will pull in a single frame.
///
/// Steam's upstream wrapper allocates a temporary pointer buffer with the
/// requested batch size before calling the C API. Keeping this bounded prevents
/// one malformed command from forcing a huge frame-loop allocation.
pub const STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE: usize = 1024;

/// Bevy plugin for high-level Steam Networking Messages commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksNetworkingMessagesCommand`] and
/// [`SteamworksNetworkingMessagesResult`] messages, installs the upstream
/// session callbacks once a [`SteamworksClient`] exists, and processes commands
/// in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug)]
pub struct SteamworksNetworkingMessagesPlugin {
    auto_accept_session_requests: bool,
}

impl Default for SteamworksNetworkingMessagesPlugin {
    fn default() -> Self {
        Self {
            auto_accept_session_requests: true,
        }
    }
}

impl SteamworksNetworkingMessagesPlugin {
    /// Creates a Networking Messages plugin with default behavior.
    ///
    /// Incoming session requests are accepted by default. Use
    /// [`Self::auto_accept_session_requests`] to opt out.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether incoming session requests are accepted in the Steam callback.
    ///
    /// The upstream safe API only allows accepting a session while handling the
    /// callback; it cannot defer the accept/reject decision to a later ECS frame.
    pub fn auto_accept_session_requests(mut self, enabled: bool) -> Self {
        self.auto_accept_session_requests = enabled;
        self
    }
}

impl Plugin for SteamworksNetworkingMessagesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SteamworksNetworkingMessagesState::new(
            self.auto_accept_session_requests,
        ))
        .add_message::<SteamworksNetworkingMessagesCommand>()
        .add_message::<SteamworksNetworkingMessagesResult>()
        .configure_sets(
            First,
            SteamworksSystem::ProcessNetworkingMessagesCommands
                .after(SteamworksSystem::RunCallbacks)
                .before(bevy_ecs::message::MessageUpdateSystems),
        )
        .add_systems(
            First,
            (
                ensure_networking_messages_callbacks,
                apply_networking_messages_policy_commands,
            )
                .chain()
                .before(SteamworksSystem::RunCallbacks),
        )
        .add_systems(
            First,
            process_networking_messages_commands
                .in_set(SteamworksSystem::ProcessNetworkingMessagesCommands),
        );
    }
}

fn ensure_networking_messages_callbacks(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksNetworkingMessagesState>,
) {
    if state.callbacks_registered() {
        return;
    }

    let Some(client) = client else {
        return;
    };

    let request_queue = state.callback_results_queue();
    let auto_accept = state.auto_accept_session_requests_policy();
    client
        .networking_messages()
        .session_request_callback(move |request| {
            let remote = request.remote().clone();
            let should_accept = *auto_accept
                .lock()
                .expect("Steamworks Networking Messages policy mutex was poisoned");
            let accepted = should_accept && request.accept();
            let result = SteamworksNetworkingMessagesResult::Ok(
                SteamworksNetworkingMessagesOperation::SessionRequestReceived {
                    request: SteamworksNetworkingMessagesSessionRequestInfo { remote, accepted },
                },
            );
            request_queue
                .lock()
                .expect("Steamworks Networking Messages callback queue mutex was poisoned")
                .push(result);
        });

    let failure_queue = state.callback_results_queue();
    client
        .networking_messages()
        .session_failed_callback(move |info| {
            let result = SteamworksNetworkingMessagesResult::Ok(
                SteamworksNetworkingMessagesOperation::SessionFailed {
                    info: snapshot_session_connection_info(
                        info.state().unwrap_or(
                            steamworks::networking_types::NetworkingConnectionState::None,
                        ),
                        Some(&info),
                        None,
                    ),
                },
            );
            failure_queue
                .lock()
                .expect("Steamworks Networking Messages callback queue mutex was poisoned")
                .push(result);
        });

    state.mark_callbacks_registered();
    tracing::debug!(
        target: "bevy_steamworks",
        auto_accept_session_requests = state.auto_accept_session_requests(),
        "registered Steamworks Networking Messages callbacks"
    );
}

fn apply_networking_messages_policy_commands(
    state: Res<SteamworksNetworkingMessagesState>,
    mut commands: MessageReader<SteamworksNetworkingMessagesCommand>,
) {
    for command in commands.read() {
        if let SteamworksNetworkingMessagesCommand::SetAutoAcceptSessionRequests { enabled } =
            command
        {
            state.set_auto_accept_session_requests(*enabled);
        }
    }
}

fn process_networking_messages_commands(
    client: Option<Res<SteamworksClient>>,
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

        if let SteamworksNetworkingMessagesCommand::SetAutoAcceptSessionRequests { enabled } =
            command
        {
            let operation =
                SteamworksNetworkingMessagesOperation::AutoAcceptSessionRequestsSet { enabled };
            state.record_operation(&operation);
            tracing::debug!(
                target: "bevy_steamworks",
                operation = ?operation,
                "processed Steamworks Networking Messages command"
            );
            results.write(SteamworksNetworkingMessagesResult::Ok(operation));
            continue;
        }

        let Some(client) = client.as_ref() else {
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

        match handle_networking_messages_command(client, command.clone()) {
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

fn handle_networking_messages_command(
    client: &SteamworksClient,
    command: SteamworksNetworkingMessagesCommand,
) -> Result<SteamworksNetworkingMessagesOperation, SteamworksNetworkingMessagesError> {
    let networking_messages = client.networking_messages();
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
    })
}

fn validate_command(
    command: &SteamworksNetworkingMessagesCommand,
) -> Result<(), SteamworksNetworkingMessagesError> {
    match command {
        SteamworksNetworkingMessagesCommand::SendMessage { peer, channel, .. } => {
            validate_peer(peer)?;
            validate_channel(*channel)
        }
        SteamworksNetworkingMessagesCommand::ReceiveMessages {
            channel,
            batch_size,
        } => {
            validate_channel(*channel)?;
            if *batch_size == 0 {
                return Err(SteamworksNetworkingMessagesError::InvalidBatchSize);
            }
            if *batch_size > STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE {
                return Err(SteamworksNetworkingMessagesError::BatchSizeTooLarge {
                    batch_size: *batch_size,
                    max_batch_size: STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE,
                });
            }
            Ok(())
        }
        SteamworksNetworkingMessagesCommand::GetSessionConnectionInfo { peer } => {
            validate_peer(peer)
        }
        SteamworksNetworkingMessagesCommand::SetAutoAcceptSessionRequests { .. } => Ok(()),
    }
}

fn validate_peer(peer: &SteamworksNetworkingPeer) -> Result<(), SteamworksNetworkingMessagesError> {
    match peer {
        SteamworksNetworkingPeer::SteamId(id) if id.is_invalid() => {
            Err(SteamworksNetworkingMessagesError::InvalidIdentity)
        }
        SteamworksNetworkingPeer::Identity(identity) if identity.is_invalid() => {
            Err(SteamworksNetworkingMessagesError::InvalidIdentity)
        }
        _ => Ok(()),
    }
}

fn validate_channel(channel: u32) -> Result<(), SteamworksNetworkingMessagesError> {
    if channel > i32::MAX as u32 {
        return Err(SteamworksNetworkingMessagesError::InvalidChannel { channel });
    }
    Ok(())
}
