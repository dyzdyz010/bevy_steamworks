use super::{SteamworksRemoteStorageCommand, SteamworksRemoteStorageError};

pub(super) fn validate_command(
    command: &SteamworksRemoteStorageCommand,
) -> Result<(), SteamworksRemoteStorageError> {
    match command {
        SteamworksRemoteStorageCommand::GetFileInfo { name }
        | SteamworksRemoteStorageCommand::GetFileExists { name }
        | SteamworksRemoteStorageCommand::IsFilePersisted { name }
        | SteamworksRemoteStorageCommand::GetFileTimestamp { name }
        | SteamworksRemoteStorageCommand::ReadFile { name }
        | SteamworksRemoteStorageCommand::DeleteFile { name }
        | SteamworksRemoteStorageCommand::ForgetFile { name }
        | SteamworksRemoteStorageCommand::GetSyncPlatforms { name }
        | SteamworksRemoteStorageCommand::SetSyncPlatforms { name, .. }
        | SteamworksRemoteStorageCommand::ShareFile { name } => validate_steam_string("name", name),
        SteamworksRemoteStorageCommand::WriteFile { write } => {
            validate_steam_string("name", &write.name)
        }
        SteamworksRemoteStorageCommand::GetCloudInfo
        | SteamworksRemoteStorageCommand::IsCloudEnabledForApp
        | SteamworksRemoteStorageCommand::IsCloudEnabledForAccount
        | SteamworksRemoteStorageCommand::SetCloudEnabledForApp { .. }
        | SteamworksRemoteStorageCommand::ListFiles => Ok(()),
    }
}

fn validate_steam_string(
    field: &'static str,
    value: &str,
) -> Result<(), SteamworksRemoteStorageError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksRemoteStorageError::invalid_string(field))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_rejects_file_metadata_commands_with_interior_nul() {
        for command in [
            SteamworksRemoteStorageCommand::get_file_info("save\0bad.dat"),
            SteamworksRemoteStorageCommand::get_file_exists("save\0bad.dat"),
            SteamworksRemoteStorageCommand::is_file_persisted("save\0bad.dat"),
            SteamworksRemoteStorageCommand::get_file_timestamp("save\0bad.dat"),
            SteamworksRemoteStorageCommand::get_sync_platforms("save\0bad.dat"),
            SteamworksRemoteStorageCommand::share_file("save\0bad.dat"),
            SteamworksRemoteStorageCommand::read_file("save\0bad.dat"),
        ] {
            assert_eq!(
                validate_command(&command),
                Err(SteamworksRemoteStorageError::InvalidString { field: "name" })
            );
        }
    }

    #[test]
    fn validation_rejects_write_file_with_interior_nul_name() {
        let command = SteamworksRemoteStorageCommand::write_file("save\0bad.dat", b"payload");

        assert_eq!(
            validate_command(&command),
            Err(SteamworksRemoteStorageError::InvalidString { field: "name" })
        );
    }
}
