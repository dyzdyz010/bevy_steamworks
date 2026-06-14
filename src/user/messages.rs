use bevy_ecs::message::Message;
use thiserror::Error;

use super::*;

/// A high-level command for Steam user identity and authentication workflows.
#[derive(Clone, Message, PartialEq, Eq)]
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
    /// Final ticket creation confirmation arrives later through both
    /// [`crate::SteamworksEvent::AuthSessionTicketResponse`] and
    /// [`SteamworksUserOperation::AuthenticationSessionTicketResponse`].
    GetAuthenticationSessionTicket {
        /// Steam ID for the entity that will verify the ticket.
        steam_id: steamworks::SteamId,
    },
    /// Request an authentication ticket for Steam Web API verification.
    ///
    /// The ticket bytes arrive later through both
    /// [`crate::SteamworksEvent::TicketForWebApiResponse`] and
    /// [`SteamworksUserOperation::WebApiAuthenticationTicketReceived`].
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

impl std::fmt::Debug for SteamworksUserCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GetCurrentUserInfo => f.write_str("GetCurrentUserInfo"),
            Self::GetSteamId => f.write_str("GetSteamId"),
            Self::GetLevel => f.write_str("GetLevel"),
            Self::IsLoggedOn => f.write_str("IsLoggedOn"),
            Self::GetAuthenticationSessionTicket { steam_id } => f
                .debug_struct("GetAuthenticationSessionTicket")
                .field("steam_id", steam_id)
                .finish(),
            Self::GetAuthenticationSessionTicketForWebApi { identity } => f
                .debug_struct("GetAuthenticationSessionTicketForWebApi")
                .field("identity", identity)
                .finish(),
            Self::CancelAuthenticationTicket { ticket } => f
                .debug_struct("CancelAuthenticationTicket")
                .field("ticket", ticket)
                .finish(),
            Self::BeginAuthenticationSession { user, ticket } => f
                .debug_struct("BeginAuthenticationSession")
                .field("user", user)
                .field("ticket_len", &ticket.len())
                .finish(),
            Self::EndAuthenticationSession { user } => f
                .debug_struct("EndAuthenticationSession")
                .field("user", user)
                .finish(),
            Self::UserHasLicenseForApp { user, app_id } => f
                .debug_struct("UserHasLicenseForApp")
                .field("user", user)
                .field("app_id", app_id)
                .finish(),
        }
    }
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
#[derive(Clone, PartialEq, Eq)]
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
    /// Final creation confirmation arrives later through both
    /// [`crate::SteamworksEvent::AuthSessionTicketResponse`] and
    /// [`SteamworksUserOperation::AuthenticationSessionTicketResponse`].
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
    /// Ticket bytes arrive later through both
    /// [`crate::SteamworksEvent::TicketForWebApiResponse`] and
    /// [`SteamworksUserOperation::WebApiAuthenticationTicketReceived`].
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
    /// Later validation callbacks arrive through both
    /// [`crate::SteamworksEvent::ValidateAuthTicketResponse`] and
    /// [`SteamworksUserOperation::AuthenticationTicketValidationReceived`].
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
    /// Auth session ticket creation callback was observed.
    AuthenticationSessionTicketResponse {
        /// Callback snapshot.
        response: SteamworksAuthSessionTicketResponse,
    },
    /// Web API auth ticket creation callback was observed.
    WebApiAuthenticationTicketReceived {
        /// Callback snapshot.
        response: SteamworksWebApiTicketResponse,
    },
    /// Auth ticket validation callback was observed.
    AuthenticationTicketValidationReceived {
        /// Callback snapshot.
        validation: SteamworksAuthTicketValidation,
    },
    /// Steam server connection state callback was observed.
    SteamServerConnectionEventReceived {
        /// Callback snapshot.
        event: SteamworksSteamServerConnectionEvent,
    },
    /// Microtransaction authorization callback was observed.
    MicroTxnAuthorizationResponseReceived {
        /// Callback snapshot.
        response: SteamworksMicroTxnAuthorizationResponse,
    },
}

impl std::fmt::Debug for SteamworksUserOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CurrentUserInfoRead { info } => f
                .debug_struct("CurrentUserInfoRead")
                .field("info", info)
                .finish(),
            Self::SteamIdRead { steam_id } => f
                .debug_struct("SteamIdRead")
                .field("steam_id", steam_id)
                .finish(),
            Self::LevelRead { level } => f.debug_struct("LevelRead").field("level", level).finish(),
            Self::LoggedOnRead { logged_on } => f
                .debug_struct("LoggedOnRead")
                .field("logged_on", logged_on)
                .finish(),
            Self::AuthenticationSessionTicketIssued {
                ticket,
                ticket_bytes,
                steam_id,
            } => f
                .debug_struct("AuthenticationSessionTicketIssued")
                .field("ticket", ticket)
                .field("ticket_bytes_len", &ticket_bytes.len())
                .field("steam_id", steam_id)
                .finish(),
            Self::WebApiAuthenticationTicketRequested { ticket, identity } => f
                .debug_struct("WebApiAuthenticationTicketRequested")
                .field("ticket", ticket)
                .field("identity", identity)
                .finish(),
            Self::AuthenticationTicketCancelled { ticket } => f
                .debug_struct("AuthenticationTicketCancelled")
                .field("ticket", ticket)
                .finish(),
            Self::AuthenticationSessionStarted { user } => f
                .debug_struct("AuthenticationSessionStarted")
                .field("user", user)
                .finish(),
            Self::AuthenticationSessionEnded { user } => f
                .debug_struct("AuthenticationSessionEnded")
                .field("user", user)
                .finish(),
            Self::UserLicenseForAppRead {
                user,
                app_id,
                license,
            } => f
                .debug_struct("UserLicenseForAppRead")
                .field("user", user)
                .field("app_id", app_id)
                .field("license", license)
                .finish(),
            Self::AuthenticationSessionTicketResponse { response } => f
                .debug_struct("AuthenticationSessionTicketResponse")
                .field("response", response)
                .finish(),
            Self::WebApiAuthenticationTicketReceived { response } => f
                .debug_struct("WebApiAuthenticationTicketReceived")
                .field("response", response)
                .finish(),
            Self::AuthenticationTicketValidationReceived { validation } => f
                .debug_struct("AuthenticationTicketValidationReceived")
                .field("validation", validation)
                .finish(),
            Self::SteamServerConnectionEventReceived { event } => f
                .debug_struct("SteamServerConnectionEventReceived")
                .field("event", event)
                .finish(),
            Self::MicroTxnAuthorizationResponseReceived { response } => f
                .debug_struct("MicroTxnAuthorizationResponseReceived")
                .field("response", response)
                .finish(),
        }
    }
}

/// Result message emitted by [`crate::SteamworksUserPlugin`].
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

/// Synchronous errors from [`crate::SteamworksUserPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksUserError {
    /// No [`crate::SteamworksClient`] resource exists.
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
    pub(super) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    pub(super) fn auth_session(source: steamworks::AuthSessionError) -> Self {
        Self::AuthSession {
            source: source.into(),
        }
    }
}
