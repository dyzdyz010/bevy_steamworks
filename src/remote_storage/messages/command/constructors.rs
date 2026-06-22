use super::super::super::SteamworksRemoteStorageFileWrite;
use super::SteamworksRemoteStorageCommand;

impl SteamworksRemoteStorageCommand {
    /// Creates a [`crate::SteamworksRemoteStorageCommand::GetCloudInfo`] command.
    pub fn get_cloud_info() -> Self {
        Self::GetCloudInfo
    }

    /// Creates a [`crate::SteamworksRemoteStorageCommand::IsCloudEnabledForApp`] command.
    pub fn is_cloud_enabled_for_app() -> Self {
        Self::IsCloudEnabledForApp
    }

    /// Creates a [`crate::SteamworksRemoteStorageCommand::IsCloudEnabledForAccount`] command.
    pub fn is_cloud_enabled_for_account() -> Self {
        Self::IsCloudEnabledForAccount
    }

    /// Creates a [`crate::SteamworksRemoteStorageCommand::SetCloudEnabledForApp`] command.
    pub fn set_cloud_enabled_for_app(enabled: bool) -> Self {
        Self::SetCloudEnabledForApp { enabled }
    }

    /// Creates a [`crate::SteamworksRemoteStorageCommand::ListFiles`] command.
    pub fn list_files() -> Self {
        Self::ListFiles
    }

    /// Creates a [`crate::SteamworksRemoteStorageCommand::GetFileInfo`] command.
    pub fn get_file_info(name: impl Into<String>) -> Self {
        Self::GetFileInfo { name: name.into() }
    }

    /// Creates a [`crate::SteamworksRemoteStorageCommand::GetFileExists`] command.
    pub fn get_file_exists(name: impl Into<String>) -> Self {
        Self::GetFileExists { name: name.into() }
    }

    /// Creates a [`crate::SteamworksRemoteStorageCommand::IsFilePersisted`] command.
    pub fn is_file_persisted(name: impl Into<String>) -> Self {
        Self::IsFilePersisted { name: name.into() }
    }

    /// Creates a [`crate::SteamworksRemoteStorageCommand::GetFileTimestamp`] command.
    pub fn get_file_timestamp(name: impl Into<String>) -> Self {
        Self::GetFileTimestamp { name: name.into() }
    }

    /// Creates a [`crate::SteamworksRemoteStorageCommand::ReadFile`] command.
    pub fn read_file(name: impl Into<String>) -> Self {
        Self::ReadFile { name: name.into() }
    }

    /// Creates a [`crate::SteamworksRemoteStorageCommand::WriteFile`] command.
    pub fn write_file(name: impl Into<String>, data: impl Into<Vec<u8>>) -> Self {
        Self::WriteFile {
            write: SteamworksRemoteStorageFileWrite::new(name, data),
        }
    }

    /// Creates a [`crate::SteamworksRemoteStorageCommand::DeleteFile`] command.
    pub fn delete_file(name: impl Into<String>) -> Self {
        Self::DeleteFile { name: name.into() }
    }

    /// Creates a [`crate::SteamworksRemoteStorageCommand::ForgetFile`] command.
    pub fn forget_file(name: impl Into<String>) -> Self {
        Self::ForgetFile { name: name.into() }
    }

    /// Creates a [`crate::SteamworksRemoteStorageCommand::GetSyncPlatforms`] command.
    pub fn get_sync_platforms(name: impl Into<String>) -> Self {
        Self::GetSyncPlatforms { name: name.into() }
    }

    /// Creates a [`crate::SteamworksRemoteStorageCommand::SetSyncPlatforms`] command.
    pub fn set_sync_platforms(
        name: impl Into<String>,
        platforms: steamworks::RemoteStoragePlatforms,
    ) -> Self {
        Self::SetSyncPlatforms {
            name: name.into(),
            platforms,
        }
    }

    /// Creates a [`crate::SteamworksRemoteStorageCommand::ShareFile`] command.
    pub fn share_file(name: impl Into<String>) -> Self {
        Self::ShareFile { name: name.into() }
    }
}
