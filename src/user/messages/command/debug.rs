use std::fmt;

use super::SteamworksUserCommand;

impl fmt::Debug for SteamworksUserCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
