use bevy_ecs::message::Message;

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
    /// Creates a [`crate::SteamworksRemoteStorageCommand::SetCloudEnabledForApp`] command.
    pub fn set_cloud_enabled_for_app(enabled: bool) -> Self {
        Self::SetCloudEnabledForApp { enabled }
    }

    /// Creates a [`crate::SteamworksRemoteStorageCommand::GetFileInfo`] command.
    pub fn get_file_info(name: impl Into<String>) -> Self {
        Self::GetFileInfo { name: name.into() }
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
