use super::SteamworksUserCommand;

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
        identity: impl Into<steamworks::networking_types::NetworkingIdentity>,
    ) -> Self {
        Self::GetAuthenticationSessionTicketForIdentity {
            identity: identity.into(),
        }
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
