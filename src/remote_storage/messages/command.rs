use bevy_ecs::message::Message;

use super::super::SteamworksRemoteStorageFileWrite;

mod constructors;

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
