use bevy_ecs::message::Message;

use super::super::SteamworksRemoteStorageFileWrite;

/// A high-level command for Steam Remote Storage metadata and sharing workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksRemoteStorageCommand {
    /// Read a snapshot of Cloud availability.
    GetCloudInfo,
    /// Read whether Steam Cloud is enabled for this app.
    IsCloudEnabledForApp,
    /// Read whether Steam Cloud is enabled for this account.
    IsCloudEnabledForAccount,
    /// Set whether Steam Cloud is enabled for this app.
    SetCloudEnabledForApp {
        /// Enabled value to submit.
        enabled: bool,
    },
    /// List Steam Cloud files.
    ListFiles,
    /// Read metadata for one file.
    GetFileInfo {
        /// File name in Steam Remote Storage.
        name: String,
    },
    /// Read whether one file exists in Steam Remote Storage.
    GetFileExists {
        /// File name in Steam Remote Storage.
        name: String,
    },
    /// Read whether one file is persisted in Cloud storage.
    IsFilePersisted {
        /// File name in Steam Remote Storage.
        name: String,
    },
    /// Read one file's Steam timestamp as Unix epoch seconds.
    GetFileTimestamp {
        /// File name in Steam Remote Storage.
        name: String,
    },
    /// Read one file's bytes on a background worker.
    ReadFile {
        /// File name in Steam Remote Storage.
        name: String,
    },
    /// Write one file's bytes on a background worker.
    WriteFile {
        /// File write payload.
        write: SteamworksRemoteStorageFileWrite,
    },
    /// Delete one file locally and remotely.
    DeleteFile {
        /// File name in Steam Remote Storage.
        name: String,
    },
    /// Forget one file remotely while keeping it locally.
    ForgetFile {
        /// File name in Steam Remote Storage.
        name: String,
    },
    /// Read the platforms a file syncs to.
    GetSyncPlatforms {
        /// File name in Steam Remote Storage.
        name: String,
    },
    /// Set the platforms a file syncs to.
    SetSyncPlatforms {
        /// File name in Steam Remote Storage.
        name: String,
        /// Platforms to submit to Steam.
        platforms: steamworks::RemoteStoragePlatforms,
    },
    /// Share one file through Steam Remote Storage.
    ShareFile {
        /// File name in Steam Remote Storage.
        name: String,
    },
}

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
