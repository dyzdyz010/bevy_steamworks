//! High-level Bevy ECS integration for Steam user identity and authentication.
//!
//! This module builds on top of the upstream [`steamworks::User`] API. It keeps
//! common authentication flows in Bevy messages while leaving low-level callback
//! confirmation in [`crate::SteamworksEvent`].

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksSystem};

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

/// Runtime state for [`SteamworksUserPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksUserState {
    last_error: Option<SteamworksUserError>,
    current_user: Option<SteamworksUserInfo>,
    active_auth_tickets: Vec<steamworks::AuthTicket>,
    authenticated_users: Vec<steamworks::SteamId>,
}

impl SteamworksUserState {
    /// Returns the most recent synchronous error observed by the user plugin.
    pub fn last_error(&self) -> Option<&SteamworksUserError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent current-user snapshot read through the plugin.
    pub fn current_user(&self) -> Option<&SteamworksUserInfo> {
        self.current_user.as_ref()
    }

    /// Returns authentication ticket handles issued through this command layer.
    ///
    /// Handles are removed after [`SteamworksUserCommand::CancelAuthenticationTicket`]
    /// is processed for the same ticket.
    pub fn active_auth_tickets(&self) -> &[steamworks::AuthTicket] {
        &self.active_auth_tickets
    }

    /// Returns users with sessions started through this command layer.
    ///
    /// Entries are removed after [`SteamworksUserCommand::EndAuthenticationSession`]
    /// is processed for the same user.
    pub fn authenticated_users(&self) -> &[steamworks::SteamId] {
        &self.authenticated_users
    }

    fn record_error(&mut self, error: SteamworksUserError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksUserOperation) {
        match operation {
            SteamworksUserOperation::CurrentUserInfoRead { info } => {
                self.current_user = Some(info.clone());
            }
            SteamworksUserOperation::AuthenticationSessionTicketIssued { ticket, .. }
            | SteamworksUserOperation::WebApiAuthenticationTicketRequested { ticket, .. }
                if !self.active_auth_tickets.contains(ticket) =>
            {
                self.active_auth_tickets.push(*ticket);
            }
            SteamworksUserOperation::AuthenticationTicketCancelled { ticket } => {
                self.active_auth_tickets.retain(|known| known != ticket);
            }
            SteamworksUserOperation::AuthenticationSessionStarted { user }
                if !self.authenticated_users.contains(user) =>
            {
                self.authenticated_users.push(*user);
            }
            SteamworksUserOperation::AuthenticationSessionEnded { user } => {
                self.authenticated_users.retain(|known| known != user);
            }
            _ => {}
        }
    }
}

/// Snapshot of common information about the current Steam user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUserInfo {
    /// Current Steam user ID.
    pub steam_id: steamworks::SteamId,
    /// Current Steam user level.
    pub level: u32,
    /// Whether the local Steam client is connected to Steam servers.
    pub logged_on: bool,
}

/// A high-level command for Steam user identity and authentication workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksUserCommand {
    /// Read a snapshot of common current-user information.
    GetCurrentUserInfo,
    /// Read the current Steam user ID.
    GetSteamId,
    /// Read the current Steam user level.
    GetLevel,
    /// Read whether the local Steam client is connected to Steam servers.
    IsLoggedOn,
    /// Request an authentication session ticket for an entity identified by Steam ID.
    ///
    /// Final ticket creation confirmation arrives later through
    /// [`crate::SteamworksEvent::AuthSessionTicketResponse`].
    GetAuthenticationSessionTicket {
        /// Steam ID for the entity that will verify the ticket.
        steam_id: steamworks::SteamId,
    },
    /// Request an authentication ticket for Steam Web API verification.
    ///
    /// The ticket bytes arrive later through
    /// [`crate::SteamworksEvent::TicketForWebApiResponse`].
    GetAuthenticationSessionTicketForWebApi {
        /// Identity string for the service that will consume the ticket.
        identity: String,
    },
    /// Cancel a locally issued authentication ticket.
    CancelAuthenticationTicket {
        /// Ticket handle to cancel.
        ticket: steamworks::AuthTicket,
    },
    /// Begin validating a ticket received from another Steam user.
    BeginAuthenticationSession {
        /// Steam user that provided the ticket.
        user: steamworks::SteamId,
        /// Raw authentication ticket bytes.
        ticket: Vec<u8>,
    },
    /// End a session started with [`SteamworksUserCommand::BeginAuthenticationSession`].
    EndAuthenticationSession {
        /// Steam user whose authentication session should end.
        user: steamworks::SteamId,
    },
    /// Check whether an authenticated user owns a specific app or DLC.
    UserHasLicenseForApp {
        /// Steam user to check.
        user: steamworks::SteamId,
        /// Steam app ID to check.
        app_id: steamworks::AppId,
    },
}

impl SteamworksUserCommand {
    /// Creates a [`SteamworksUserCommand::GetAuthenticationSessionTicket`] command.
    pub fn get_authentication_session_ticket(steam_id: steamworks::SteamId) -> Self {
        Self::GetAuthenticationSessionTicket { steam_id }
    }

    /// Creates a [`SteamworksUserCommand::GetAuthenticationSessionTicketForWebApi`] command.
    pub fn get_authentication_session_ticket_for_web_api(identity: impl Into<String>) -> Self {
        Self::GetAuthenticationSessionTicketForWebApi {
            identity: identity.into(),
        }
    }

    /// Creates a [`SteamworksUserCommand::CancelAuthenticationTicket`] command.
    pub fn cancel_authentication_ticket(ticket: steamworks::AuthTicket) -> Self {
        Self::CancelAuthenticationTicket { ticket }
    }

    /// Creates a [`SteamworksUserCommand::BeginAuthenticationSession`] command.
    pub fn begin_authentication_session(
        user: steamworks::SteamId,
        ticket: impl Into<Vec<u8>>,
    ) -> Self {
        Self::BeginAuthenticationSession {
            user,
            ticket: ticket.into(),
        }
    }

    /// Creates a [`SteamworksUserCommand::EndAuthenticationSession`] command.
    pub fn end_authentication_session(user: steamworks::SteamId) -> Self {
        Self::EndAuthenticationSession { user }
    }

    /// Creates a [`SteamworksUserCommand::UserHasLicenseForApp`] command.
    pub fn user_has_license_for_app(
        user: steamworks::SteamId,
        app_id: impl Into<steamworks::AppId>,
    ) -> Self {
        Self::UserHasLicenseForApp {
            user,
            app_id: app_id.into(),
        }
    }
}

/// A successfully submitted Steam user operation or synchronous read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksUserOperation {
    /// Common current-user information was read.
    CurrentUserInfoRead {
        /// Current-user snapshot.
        info: SteamworksUserInfo,
    },
    /// Current Steam user ID was read.
    SteamIdRead {
        /// Current Steam user ID.
        steam_id: steamworks::SteamId,
    },
    /// Current Steam user level was read.
    LevelRead {
        /// Current Steam user level.
        level: u32,
    },
    /// Steam server connection state was read.
    LoggedOnRead {
        /// Whether the local Steam client is connected to Steam servers.
        logged_on: bool,
    },
    /// Authentication session ticket bytes were issued.
    ///
    /// Final creation confirmation arrives later through
    /// [`crate::SteamworksEvent::AuthSessionTicketResponse`].
    AuthenticationSessionTicketIssued {
        /// Ticket handle that should be cancelled when no longer needed.
        ticket: steamworks::AuthTicket,
        /// Raw ticket bytes to send to the verifying entity.
        ticket_bytes: Vec<u8>,
        /// Steam ID used as the network identity for the verifier.
        steam_id: steamworks::SteamId,
    },
    /// A Steam Web API authentication ticket request was submitted.
    ///
    /// Ticket bytes arrive later through
    /// [`crate::SteamworksEvent::TicketForWebApiResponse`].
    WebApiAuthenticationTicketRequested {
        /// Ticket handle that should be cancelled when no longer needed.
        ticket: steamworks::AuthTicket,
        /// Identity string submitted for the consuming service.
        identity: String,
    },
    /// A locally issued authentication ticket was cancelled.
    AuthenticationTicketCancelled {
        /// Ticket handle that was cancelled.
        ticket: steamworks::AuthTicket,
    },
    /// Authentication began for a remote user ticket.
    ///
    /// Later invalidation can arrive through
    /// [`crate::SteamworksEvent::ValidateAuthTicketResponse`].
    AuthenticationSessionStarted {
        /// Steam user whose ticket was accepted for validation.
        user: steamworks::SteamId,
    },
    /// Authentication ended for a remote user.
    AuthenticationSessionEnded {
        /// Steam user whose authentication session ended.
        user: steamworks::SteamId,
    },
    /// App license state was read for an authenticated user.
    UserLicenseForAppRead {
        /// Steam user that was checked.
        user: steamworks::SteamId,
        /// Steam app ID that was checked.
        app_id: steamworks::AppId,
        /// License state reported by Steam.
        license: steamworks::UserHasLicense,
    },
}

/// Result message emitted by [`SteamworksUserPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksUserResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksUserOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksUserCommand,
        /// Failure reason.
        error: SteamworksUserError,
    },
}

/// Synchronous errors from [`SteamworksUserPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksUserError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks user command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A remote authentication session was requested with no ticket bytes.
    #[error("Steamworks user command requires a non-empty authentication ticket")]
    EmptyTicket,
    /// The upstream Steamworks API rejected an authentication session.
    #[error("Steamworks authentication session failed: {source}")]
    AuthSession {
        /// Authentication session failure reason.
        #[source]
        source: SteamworksAuthSessionError,
    },
}

impl SteamworksUserError {
    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    fn auth_session(source: steamworks::AuthSessionError) -> Self {
        Self::AuthSession {
            source: source.into(),
        }
    }
}

/// Cloneable, comparable mirror of upstream [`steamworks::AuthSessionError`].
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum SteamworksAuthSessionError {
    /// The ticket is invalid.
    #[error("invalid ticket")]
    InvalidTicket,
    /// A ticket has already been submitted for this Steam ID.
    #[error("duplicate ticket request")]
    DuplicateRequest,
    /// The ticket is from an incompatible interface version.
    #[error("incompatible interface version")]
    InvalidVersion,
    /// The ticket is not for this game.
    #[error("incorrect game for ticket")]
    GameMismatch,
    /// The ticket has expired.
    #[error("ticket has expired")]
    ExpiredTicket,
}

impl From<steamworks::AuthSessionError> for SteamworksAuthSessionError {
    fn from(error: steamworks::AuthSessionError) -> Self {
        match error {
            steamworks::AuthSessionError::InvalidTicket => Self::InvalidTicket,
            steamworks::AuthSessionError::DuplicateRequest => Self::DuplicateRequest,
            steamworks::AuthSessionError::InvalidVersion => Self::InvalidVersion,
            steamworks::AuthSessionError::GameMismatch => Self::GameMismatch,
            steamworks::AuthSessionError::ExpiredTicket => Self::ExpiredTicket,
        }
    }
}

fn process_user_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksUserState>,
    mut commands: ResMut<Messages<SteamworksUserCommand>>,
    mut results: MessageWriter<SteamworksUserResult>,
) {
    let Some(client) = client else {
        let error = SteamworksUserError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
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

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    #[test]
    fn user_plugin_registers_resources_and_messages() {
        let mut app = App::new();

        app.add_plugins(SteamworksUserPlugin::new());

        assert!(app.world().contains_resource::<SteamworksUserState>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksUserCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksUserResult>>());
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksUserPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksUserCommand>>()
            .write(SteamworksUserCommand::GetCurrentUserInfo);

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksUserResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksUserResult::Err {
                command: SteamworksUserCommand::GetCurrentUserInfo,
                error: SteamworksUserError::ClientUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksUserState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksUserError::ClientUnavailable)
        );
    }

    #[test]
    fn validation_rejects_interior_nul_for_web_api_identity() {
        let command =
            SteamworksUserCommand::get_authentication_session_ticket_for_web_api("web\0bad");

        assert_eq!(
            validate_command(&command),
            Err(SteamworksUserError::InvalidString { field: "identity" })
        );
    }

    #[test]
    fn validation_rejects_empty_auth_ticket() {
        let command = SteamworksUserCommand::begin_authentication_session(
            steamworks::SteamId::from_raw(1),
            Vec::new(),
        );

        assert_eq!(
            validate_command(&command),
            Err(SteamworksUserError::EmptyTicket)
        );
    }

    #[test]
    fn auth_session_errors_are_cloneable_and_comparable() {
        assert_eq!(
            SteamworksAuthSessionError::from(steamworks::AuthSessionError::InvalidTicket),
            SteamworksAuthSessionError::InvalidTicket
        );
        assert_eq!(
            SteamworksAuthSessionError::from(steamworks::AuthSessionError::DuplicateRequest),
            SteamworksAuthSessionError::DuplicateRequest
        );
        assert_eq!(
            SteamworksAuthSessionError::from(steamworks::AuthSessionError::InvalidVersion),
            SteamworksAuthSessionError::InvalidVersion
        );
        assert_eq!(
            SteamworksAuthSessionError::from(steamworks::AuthSessionError::GameMismatch),
            SteamworksAuthSessionError::GameMismatch
        );
        assert_eq!(
            SteamworksAuthSessionError::from(steamworks::AuthSessionError::ExpiredTicket),
            SteamworksAuthSessionError::ExpiredTicket
        );
    }
}
