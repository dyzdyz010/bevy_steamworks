use super::{SteamworksUserCommand, SteamworksUserError};

pub(super) fn validate_command(command: &SteamworksUserCommand) -> Result<(), SteamworksUserError> {
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
    use super::*;

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
}
