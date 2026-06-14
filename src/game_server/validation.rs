use super::{
    SteamworksServerCommand, SteamworksServerConfig, SteamworksServerError, SteamworksServerState,
    SteamworksServerUnavailable,
};

pub(super) fn validate_server_command(
    command: &SteamworksServerCommand,
) -> Result<(), SteamworksServerError> {
    match command {
        SteamworksServerCommand::BeginAuthenticationSession { ticket, .. } => {
            if ticket.is_empty() {
                Err(SteamworksServerError::EmptyTicket)
            } else {
                Ok(())
            }
        }
        SteamworksServerCommand::SetProduct { product } => {
            validate_steam_string("product", product)
        }
        SteamworksServerCommand::SetGameDescription { description } => {
            validate_steam_string("description", description)
        }
        SteamworksServerCommand::SetGameData { data } => validate_steam_string("data", data),
        SteamworksServerCommand::LogOn { token } => {
            if token.as_str().is_empty() {
                Err(SteamworksServerError::EmptyLogonToken)
            } else {
                validate_steam_string("token", token.as_str())
            }
        }
        SteamworksServerCommand::SetModDir { mod_dir } => validate_steam_string("mod_dir", mod_dir),
        SteamworksServerCommand::SetMapName { map_name } => {
            validate_steam_string("map_name", map_name)
        }
        SteamworksServerCommand::SetServerName { server_name } => {
            validate_steam_string("server_name", server_name)
        }
        SteamworksServerCommand::SetGameTags { tags } => {
            validate_steam_string("tags", tags)?;
            if tags.is_empty() || tags.len() >= 128 {
                Err(SteamworksServerError::InvalidGameTags)
            } else {
                Ok(())
            }
        }
        SteamworksServerCommand::SetKeyValue { key, value } => {
            validate_steam_string("key", key)?;
            validate_steam_string("value", value)
        }
        SteamworksServerCommand::SetMaxPlayers { count } => {
            validate_non_negative_count("count", *count)
        }
        SteamworksServerCommand::SetBotPlayerCount { count } => {
            validate_non_negative_count("count", *count)
        }
        _ => Ok(()),
    }
}

pub(super) fn validate_server_command_for_state(
    command: &SteamworksServerCommand,
    state: &SteamworksServerState,
) -> Result<(), SteamworksServerError> {
    validate_server_command(command)?;

    if state.logon_submitted() {
        if matches!(
            command,
            SteamworksServerCommand::LogOnAnonymous | SteamworksServerCommand::LogOn { .. }
        ) {
            return Err(SteamworksServerError::LogonAlreadySubmitted);
        }

        if let Some(command_name) = pre_logon_only_command_name(command) {
            return Err(SteamworksServerError::command_requires_pre_logon(
                command_name,
            ));
        }
    }

    Ok(())
}

fn pre_logon_only_command_name(command: &SteamworksServerCommand) -> Option<&'static str> {
    match command {
        SteamworksServerCommand::SetProduct { .. } => Some("SetProduct"),
        SteamworksServerCommand::SetGameDescription { .. } => Some("SetGameDescription"),
        _ => None,
    }
}

pub(super) fn validate_server_config(
    config: &SteamworksServerConfig,
) -> Result<(), SteamworksServerUnavailable> {
    if config.version.as_bytes().contains(&0) {
        Err(SteamworksServerUnavailable::invalid_string("version"))
    } else {
        Ok(())
    }
}

fn validate_steam_string(field: &'static str, value: &str) -> Result<(), SteamworksServerError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksServerError::invalid_string(field))
    } else {
        Ok(())
    }
}

fn validate_non_negative_count(
    field: &'static str,
    value: i32,
) -> Result<(), SteamworksServerError> {
    if value < 0 {
        Err(SteamworksServerError::invalid_count(field, value))
    } else {
        Ok(())
    }
}
