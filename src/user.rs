//! High-level Bevy ECS integration for Steam user identity and authentication.
//!
//! This module builds on top of the upstream [`steamworks::User`] API. It keeps
//! common authentication flows in Bevy messages while mirroring relevant
//! low-level callback confirmations from [`crate::SteamworksEvent`] into
//! [`SteamworksUserResult`].

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
pub use state::SteamworksUserState;
pub use types::*;

/// Bevy plugin for high-level Steam user identity and authentication commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksUserCommand`] and [`SteamworksUserResult`] messages and runs its
/// command processor in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksUserPlugin;

impl SteamworksUserPlugin {
    /// Creates a user plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksUserPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksUserState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksUserCommand>()
            .add_message::<SteamworksUserResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessUserCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_user_commands.in_set(SteamworksSystem::ProcessUserCommands),
            );
    }
}

fn process_user_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksUserState>,
    mut commands: ResMut<Messages<SteamworksUserCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksUserResult>,
) {
    process_user_steam_events(&mut state, &mut steam_events, &mut results);

    let Some(client) = client else {
        let error = SteamworksUserError::ClientUnavailable;
        for command in commands.drain() {
            state.record_error(error.clone());
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks user command failed"
            );
            results.write(SteamworksUserResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        match handle_user_command(&client, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks user command"
                );
                results.write(SteamworksUserResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks user command failed"
                );
                results.write(SteamworksUserResult::Err { command, error });
            }
        }
    }
}

fn process_user_steam_events(
    state: &mut SteamworksUserState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksUserResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::AuthSessionTicketResponse(event) => {
                SteamworksUserOperation::AuthenticationSessionTicketResponse {
                    response: SteamworksAuthSessionTicketResponse {
                        ticket: event.ticket,
                        result: event.result,
                    },
                }
            }
            SteamworksEvent::TicketForWebApiResponse(event) => {
                SteamworksUserOperation::WebApiAuthenticationTicketReceived {
                    response: snapshot_web_api_ticket_response(event),
                }
            }
            SteamworksEvent::ValidateAuthTicketResponse(event) => {
                SteamworksUserOperation::AuthenticationTicketValidationReceived {
                    validation: SteamworksAuthTicketValidation {
                        steam_id: event.steam_id,
                        owner_steam_id: event.owner_steam_id,
                        response: event.response.clone().map_err(Into::into),
                    },
                }
            }
            SteamworksEvent::SteamServersConnected(_) => {
                SteamworksUserOperation::SteamServerConnectionEventReceived {
                    event: SteamworksSteamServerConnectionEvent::Connected,
                }
            }
            SteamworksEvent::SteamServersDisconnected(event) => {
                SteamworksUserOperation::SteamServerConnectionEventReceived {
                    event: SteamworksSteamServerConnectionEvent::Disconnected {
                        reason: event.reason,
                    },
                }
            }
            SteamworksEvent::SteamServerConnectFailure(event) => {
                SteamworksUserOperation::SteamServerConnectionEventReceived {
                    event: SteamworksSteamServerConnectionEvent::ConnectFailure {
                        reason: event.reason,
                        still_retrying: event.still_retrying,
                    },
                }
            }
            SteamworksEvent::MicroTxnAuthorizationResponse(event) => {
                SteamworksUserOperation::MicroTxnAuthorizationResponseReceived {
                    response: SteamworksMicroTxnAuthorizationResponse {
                        app_id: event.app_id,
                        order_id: event.order_id,
                        authorized: event.authorized,
                    },
                }
            }
            _ => continue,
        };
        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks user callback"
        );
        results.write(SteamworksUserResult::Ok(operation));
    }
}

fn handle_user_command(
    client: &SteamworksClient,
    command: &SteamworksUserCommand,
) -> Result<SteamworksUserOperation, SteamworksUserError> {
    validate_command(command)?;

    let user = client.user();
    Ok(match command {
        SteamworksUserCommand::GetCurrentUserInfo => SteamworksUserOperation::CurrentUserInfoRead {
            info: snapshot_current_user(client),
        },
        SteamworksUserCommand::GetSteamId => SteamworksUserOperation::SteamIdRead {
            steam_id: user.steam_id(),
        },
        SteamworksUserCommand::GetLevel => SteamworksUserOperation::LevelRead {
            level: user.level(),
        },
        SteamworksUserCommand::IsLoggedOn => SteamworksUserOperation::LoggedOnRead {
            logged_on: user.logged_on(),
        },
        SteamworksUserCommand::GetAuthenticationSessionTicket { steam_id } => {
            let (ticket, ticket_bytes) =
                user.authentication_session_ticket_with_steam_id(*steam_id);
            SteamworksUserOperation::AuthenticationSessionTicketIssued {
                ticket,
                ticket_bytes,
                steam_id: *steam_id,
            }
        }
        SteamworksUserCommand::GetAuthenticationSessionTicketForWebApi { identity } => {
            let ticket = user.authentication_session_ticket_for_webapi(identity);
            SteamworksUserOperation::WebApiAuthenticationTicketRequested {
                ticket,
                identity: identity.clone(),
            }
        }
        SteamworksUserCommand::CancelAuthenticationTicket { ticket } => {
            user.cancel_authentication_ticket(*ticket);
            SteamworksUserOperation::AuthenticationTicketCancelled { ticket: *ticket }
        }
        SteamworksUserCommand::BeginAuthenticationSession {
            user: steam_id,
            ticket,
        } => {
            user.begin_authentication_session(*steam_id, ticket)
                .map_err(SteamworksUserError::auth_session)?;
            SteamworksUserOperation::AuthenticationSessionStarted { user: *steam_id }
        }
        SteamworksUserCommand::EndAuthenticationSession { user: steam_id } => {
            user.end_authentication_session(*steam_id);
            SteamworksUserOperation::AuthenticationSessionEnded { user: *steam_id }
        }
        SteamworksUserCommand::UserHasLicenseForApp {
            user: steam_id,
            app_id,
        } => SteamworksUserOperation::UserLicenseForAppRead {
            user: *steam_id,
            app_id: *app_id,
            license: user.user_has_license_for_app(*steam_id, *app_id),
        },
    })
}

fn snapshot_current_user(client: &SteamworksClient) -> SteamworksUserInfo {
    let user = client.user();
    SteamworksUserInfo {
        steam_id: user.steam_id(),
        level: user.level(),
        logged_on: user.logged_on(),
    }
}

fn snapshot_web_api_ticket_response(
    response: &steamworks::TicketForWebApiResponse,
) -> SteamworksWebApiTicketResponse {
    let ticket_len = usize::try_from(response.ticket_len).unwrap_or(0);
    let mut ticket_bytes = response.ticket.clone();
    ticket_bytes.truncate(ticket_len.min(ticket_bytes.len()));
    SteamworksWebApiTicketResponse {
        ticket_handle: response.ticket_handle,
        result: response.result,
        ticket_bytes,
    }
}

fn validate_command(command: &SteamworksUserCommand) -> Result<(), SteamworksUserError> {
    match command {
        SteamworksUserCommand::GetAuthenticationSessionTicketForWebApi { identity } => {
            validate_steam_string("identity", identity)
        }
        SteamworksUserCommand::BeginAuthenticationSession { ticket, .. } => {
            if ticket.is_empty() {
                Err(SteamworksUserError::EmptyTicket)
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

fn validate_steam_string(field: &'static str, value: &str) -> Result<(), SteamworksUserError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksUserError::invalid_string(field))
    } else {
        Ok(())
    }
}
