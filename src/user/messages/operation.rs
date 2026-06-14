use super::super::{
    SteamworksAuthSessionTicketResponse, SteamworksAuthTicketValidation,
    SteamworksMicroTxnAuthorizationResponse, SteamworksSteamServerConnectionEvent,
    SteamworksUserInfo, SteamworksWebApiTicketResponse,
};

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
    /// [`crate::SteamworksUserOperation::AuthenticationSessionTicketResponse`].
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
    /// [`crate::SteamworksUserOperation::WebApiAuthenticationTicketReceived`].
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
    /// [`crate::SteamworksUserOperation::AuthenticationTicketValidationReceived`].
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
