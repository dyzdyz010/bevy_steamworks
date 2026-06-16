use bevy_ecs::message::Message;

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
    /// [`crate::SteamworksUserOperation::AuthenticationSessionTicketResponse`].
    GetAuthenticationSessionTicket {
        /// Steam ID for the entity that will verify the ticket.
        steam_id: steamworks::SteamId,
    },
    /// Request an authentication session ticket for a specific networking identity.
    ///
    /// Final ticket creation confirmation arrives later through both
    /// [`crate::SteamworksEvent::AuthSessionTicketResponse`] and
    /// [`crate::SteamworksUserOperation::AuthenticationSessionTicketResponse`].
    GetAuthenticationSessionTicketForIdentity {
        /// Networking identity for the entity that will verify the ticket.
        identity: steamworks::networking_types::NetworkingIdentity,
    },
    /// Request an authentication ticket for Steam Web API verification.
    ///
    /// The ticket bytes arrive later through both
    /// [`crate::SteamworksEvent::TicketForWebApiResponse`] and
    /// [`crate::SteamworksUserOperation::WebApiAuthenticationTicketReceived`].
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
    /// End a session started with [`crate::SteamworksUserCommand::BeginAuthenticationSession`].
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
            Self::GetAuthenticationSessionTicketForIdentity { identity } => f
                .debug_struct("GetAuthenticationSessionTicketForIdentity")
                .field("identity", identity)
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
    /// Creates a [`crate::SteamworksUserCommand::GetCurrentUserInfo`] command.
    pub fn get_current_user_info() -> Self {
        Self::GetCurrentUserInfo
    }

    /// Creates a [`crate::SteamworksUserCommand::GetSteamId`] command.
    pub fn get_steam_id() -> Self {
        Self::GetSteamId
    }

    /// Creates a [`crate::SteamworksUserCommand::GetLevel`] command.
    pub fn get_level() -> Self {
        Self::GetLevel
    }

    /// Creates a [`crate::SteamworksUserCommand::IsLoggedOn`] command.
    pub fn is_logged_on() -> Self {
        Self::IsLoggedOn
    }

    /// Creates a [`crate::SteamworksUserCommand::GetAuthenticationSessionTicket`] command.
    pub fn get_authentication_session_ticket(steam_id: steamworks::SteamId) -> Self {
        Self::GetAuthenticationSessionTicket { steam_id }
    }

    /// Creates a [`crate::SteamworksUserCommand::GetAuthenticationSessionTicketForIdentity`] command.
    pub fn get_authentication_session_ticket_for_identity(
        identity: steamworks::networking_types::NetworkingIdentity,
    ) -> Self {
        Self::GetAuthenticationSessionTicketForIdentity { identity }
    }

    /// Creates a [`crate::SteamworksUserCommand::GetAuthenticationSessionTicketForWebApi`] command.
    pub fn get_authentication_session_ticket_for_web_api(identity: impl Into<String>) -> Self {
        Self::GetAuthenticationSessionTicketForWebApi {
            identity: identity.into(),
        }
    }

    /// Creates a [`crate::SteamworksUserCommand::CancelAuthenticationTicket`] command.
    pub fn cancel_authentication_ticket(ticket: steamworks::AuthTicket) -> Self {
        Self::CancelAuthenticationTicket { ticket }
    }

    /// Creates a [`crate::SteamworksUserCommand::BeginAuthenticationSession`] command.
    pub fn begin_authentication_session(
        user: steamworks::SteamId,
        ticket: impl Into<Vec<u8>>,
    ) -> Self {
        Self::BeginAuthenticationSession {
            user,
            ticket: ticket.into(),
        }
    }

    /// Creates a [`crate::SteamworksUserCommand::EndAuthenticationSession`] command.
    pub fn end_authentication_session(user: steamworks::SteamId) -> Self {
        Self::EndAuthenticationSession { user }
    }

    /// Creates a [`crate::SteamworksUserCommand::UserHasLicenseForApp`] command.
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
