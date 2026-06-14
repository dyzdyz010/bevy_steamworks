use super::messages::{SteamworksNetworkingUtilsCommand, SteamworksNetworkingUtilsError};

pub(super) fn validate_command(
    _command: &SteamworksNetworkingUtilsCommand,
) -> Result<(), SteamworksNetworkingUtilsError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_and_validation_cover_current_commands() {
        assert_eq!(
            SteamworksNetworkingUtilsCommand::init_relay_network_access(),
            SteamworksNetworkingUtilsCommand::InitRelayNetworkAccess
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingUtilsCommand::InitRelayNetworkAccess),
            Ok(())
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingUtilsCommand::GetRelayNetworkStatus),
            Ok(())
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingUtilsCommand::GetDetailedRelayNetworkStatus),
            Ok(())
        );
    }
}
