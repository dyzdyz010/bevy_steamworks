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
            validate_command(&SteamworksNetworkingUtilsCommand::init_relay_network_access()),
            Ok(())
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingUtilsCommand::get_relay_network_status()),
            Ok(())
        );
        assert_eq!(
            validate_command(
                &SteamworksNetworkingUtilsCommand::get_detailed_relay_network_status(),
            ),
            Ok(())
        );
        assert_eq!(
            validate_command(
                &SteamworksNetworkingUtilsCommand::is_relay_ping_measurement_in_progress(),
            ),
            Ok(())
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingUtilsCommand::get_relay_network_config_status()),
            Ok(())
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingUtilsCommand::get_any_relay_status()),
            Ok(())
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingUtilsCommand::get_relay_debug_message()),
            Ok(())
        );
    }
}
