use super::{SteamworksFriendsCommand, SteamworksFriendsError};

pub(super) fn validate_command_strings(
    command: &SteamworksFriendsCommand,
) -> Result<(), SteamworksFriendsError> {
    match command {
        SteamworksFriendsCommand::SetRichPresence { key, value } => {
            validate_steam_string("key", key)?;
            if let Some(value) = value {
                validate_steam_string("value", value)?;
            }
        }
        SteamworksFriendsCommand::GetFriendRichPresence { key, .. } => {
            validate_steam_string("key", key)?;
        }
        SteamworksFriendsCommand::ActivateGameOverlay { dialog } => {
            validate_steam_string("dialog", dialog)?;
        }
        SteamworksFriendsCommand::ActivateGameOverlayToWebPage { url } => {
            validate_steam_string("url", url)?;
        }
        SteamworksFriendsCommand::ActivateGameOverlayToUser { dialog, .. } => {
            validate_steam_string("dialog", dialog)?;
        }
        SteamworksFriendsCommand::ActivateInviteDialogConnectString { connect }
        | SteamworksFriendsCommand::InviteUserToGame { connect, .. } => {
            validate_steam_string("connect", connect)?;
        }
        _ => {}
    }

    Ok(())
}

fn validate_steam_string(field: &'static str, value: &str) -> Result<(), SteamworksFriendsError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksFriendsError::invalid_string(field))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_validation_rejects_interior_nul() {
        let command = SteamworksFriendsCommand::SetRichPresence {
            key: "status\0bad".to_owned(),
            value: Some("ok".to_owned()),
        };

        assert_eq!(
            validate_command_strings(&command),
            Err(SteamworksFriendsError::InvalidString { field: "key" })
        );

        let command = SteamworksFriendsCommand::invite_user_to_game(
            steamworks::SteamId::from_raw(1),
            "join\0bad",
        );

        assert_eq!(
            validate_command_strings(&command),
            Err(SteamworksFriendsError::InvalidString { field: "connect" })
        );
    }
}
