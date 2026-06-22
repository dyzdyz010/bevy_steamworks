use bevy_ecs::message::Message;

mod constructors;
mod debug;

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
