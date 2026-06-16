use super::super::{
    SteamworksRemoteStorageCloudInfo, SteamworksRemoteStorageFileContents,
    SteamworksRemoteStorageFileInfo, SteamworksRemoteStorageFileSummary,
    SteamworksRemoteStorageFileWritten, SteamworksRemoteStorageSharedFile,
};

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
    /// File existence was read.
    FileExistsRead {
        /// File name inspected.
        name: String,
        /// Whether Steam reports the file currently exists.
        exists: bool,
    },
    /// File persisted state was read.
    FilePersistedRead {
        /// File name inspected.
        name: String,
        /// Whether Steam reports the file is persisted in Cloud storage.
        persisted: bool,
    },
    /// File timestamp was read.
    FileTimestampRead {
        /// File name inspected.
        name: String,
        /// Steam file timestamp, as Unix epoch seconds.
        timestamp: i64,
    },
    /// File read was requested.
    FileReadRequested {
        /// Plugin-assigned request ID.
        request_id: u64,
        /// File name submitted.
        name: String,
    },
    /// File bytes were read.
    FileRead {
        /// Read file contents.
        contents: SteamworksRemoteStorageFileContents,
    },
    /// File write was requested.
    FileWriteRequested {
        /// Plugin-assigned request ID.
        request_id: u64,
        /// File name submitted.
        name: String,
        /// Number of bytes submitted to the worker.
        bytes: usize,
    },
    /// File bytes were accepted by the upstream writer.
    FileWritten {
        /// Completed write submission snapshot.
        written: SteamworksRemoteStorageFileWritten,
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
