use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::{SteamworksClient, SteamworksEvent};

use super::{
    callbacks::process_user_steam_events,
    messages::{
        SteamworksUserCommand, SteamworksUserError, SteamworksUserOperation, SteamworksUserResult,
    },
    state::SteamworksUserState,
    types::SteamworksUserInfo,
    validation::validate_command,
};

pub(super) fn process_user_commands(
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
