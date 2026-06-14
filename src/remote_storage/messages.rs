use bevy_ecs::message::Message;
use thiserror::Error;

use super::types::{
    SteamworksRemoteStorageCloudInfo, SteamworksRemoteStorageFileInfo,
    SteamworksRemoteStorageFileSummary, SteamworksRemoteStorageSharedFile,
};

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
    /// Creates a [`SteamworksRemoteStorageCommand::SetCloudEnabledForApp`] command.
    pub fn set_cloud_enabled_for_app(enabled: bool) -> Self {
        Self::SetCloudEnabledForApp { enabled }
    }

    /// Creates a [`SteamworksRemoteStorageCommand::GetFileInfo`] command.
    pub fn get_file_info(name: impl Into<String>) -> Self {
        Self::GetFileInfo { name: name.into() }
    }

    /// Creates a [`SteamworksRemoteStorageCommand::DeleteFile`] command.
    pub fn delete_file(name: impl Into<String>) -> Self {
        Self::DeleteFile { name: name.into() }
    }

    /// Creates a [`SteamworksRemoteStorageCommand::ForgetFile`] command.
    pub fn forget_file(name: impl Into<String>) -> Self {
        Self::ForgetFile { name: name.into() }
    }

    /// Creates a [`SteamworksRemoteStorageCommand::GetSyncPlatforms`] command.
    pub fn get_sync_platforms(name: impl Into<String>) -> Self {
        Self::GetSyncPlatforms { name: name.into() }
    }

    /// Creates a [`SteamworksRemoteStorageCommand::SetSyncPlatforms`] command.
    pub fn set_sync_platforms(
        name: impl Into<String>,
        platforms: steamworks::RemoteStoragePlatforms,
    ) -> Self {
        Self::SetSyncPlatforms {
            name: name.into(),
            platforms,
        }
    }

    /// Creates a [`SteamworksRemoteStorageCommand::ShareFile`] command.
    pub fn share_file(name: impl Into<String>) -> Self {
        Self::ShareFile { name: name.into() }
    }
}

/// A successfully processed Steam Remote Storage operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksRemoteStorageOperation {
    /// Cloud availability was read.
    CloudInfoRead {
        /// Cloud availability snapshot.
        info: SteamworksRemoteStorageCloudInfo,
    },
    /// App-level Cloud enabled state was read.
    CloudEnabledForAppRead {
        /// Whether Steam Cloud is enabled for this app.
        enabled: bool,
    },
    /// Account-level Cloud enabled state was read.
    CloudEnabledForAccountRead {
        /// Whether Steam Cloud is enabled for this account.
        enabled: bool,
    },
    /// App-level Cloud enabled state was submitted.
    CloudEnabledForAppSet {
        /// Enabled value submitted to Steam.
        enabled: bool,
    },
    /// Files were listed.
    FilesListed {
        /// File summaries returned by Steam.
        files: Vec<SteamworksRemoteStorageFileSummary>,
    },
    /// File metadata was read.
    FileInfoRead {
        /// File metadata snapshot.
        info: SteamworksRemoteStorageFileInfo,
    },
    /// File delete was submitted.
    FileDeleted {
        /// File name submitted.
        name: String,
        /// Whether Steam reported that a file was actually deleted.
        deleted: bool,
    },
    /// File forget was submitted.
    FileForgotten {
        /// File name submitted.
        name: String,
        /// Whether Steam reported that a file was actually forgotten.
        forgotten: bool,
    },
    /// File sync platforms were read.
    FileSyncPlatformsRead {
        /// File name inspected.
        name: String,
        /// Platforms reported by Steam.
        platforms: steamworks::RemoteStoragePlatforms,
    },
    /// File sync platforms were set.
    FileSyncPlatformsSet {
        /// File name submitted.
        name: String,
        /// Platforms submitted to Steam.
        platforms: steamworks::RemoteStoragePlatforms,
    },
    /// File sharing was requested.
    FileShareRequested {
        /// Plugin-assigned request ID.
        request_id: u64,
        /// File name submitted.
        name: String,
    },
    /// File sharing completed successfully.
    FileShared {
        /// Shared file details.
        shared_file: SteamworksRemoteStorageSharedFile,
    },
}

/// Result message emitted by [`super::SteamworksRemoteStoragePlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksRemoteStorageResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksRemoteStorageOperation),
    /// The command failed synchronously or through a Steam async call result.
    Err {
        /// Command that failed.
        command: SteamworksRemoteStorageCommand,
        /// Failure reason.
        error: SteamworksRemoteStorageError,
    },
}

/// Synchronous and async errors from [`super::SteamworksRemoteStoragePlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksRemoteStorageError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks Remote Storage command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// The upstream Steamworks API returned an explicit Steam error.
    #[error("Steamworks Remote Storage operation {operation} failed: {source}")]
    SteamError {
        /// Operation that failed.
        operation: &'static str,
        /// Plugin request ID for async operations.
        request_id: Option<u64>,
        /// Steamworks error.
        source: steamworks::SteamError,
    },
    /// The requested file is not available in Steam Remote Storage.
    #[error("Steamworks Remote Storage file is not available: {name}")]
    FileUnavailable {
        /// File name submitted.
        name: String,
    },
    /// The upstream Steamworks API rejected the operation.
    #[error("Steamworks Remote Storage operation failed: {operation}")]
    OperationFailed {
        /// Operation that failed.
        operation: &'static str,
    },
}

impl SteamworksRemoteStorageError {
    pub(super) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    pub(super) fn steam_error(
        operation: &'static str,
        request_id: Option<u64>,
        source: steamworks::SteamError,
    ) -> Self {
        Self::SteamError {
            operation,
            request_id,
            source,
        }
    }

    pub(super) fn file_unavailable(name: impl Into<String>) -> Self {
        Self::FileUnavailable { name: name.into() }
    }

    pub(super) fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }
}
