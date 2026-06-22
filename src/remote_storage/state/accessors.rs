use super::SteamworksRemoteStorageState;
use crate::remote_storage::{
    SteamworksRemoteStorageCloudInfo, SteamworksRemoteStorageError,
    SteamworksRemoteStorageFileContents, SteamworksRemoteStorageFileInfo,
    SteamworksRemoteStorageFileSummary, SteamworksRemoteStorageFileWritten,
    SteamworksRemoteStorageSharedFile,
};

impl SteamworksRemoteStorageState {
    /// Returns the most recent synchronous or async error observed by the plugin.
    pub fn last_error(&self) -> Option<&SteamworksRemoteStorageError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent Cloud availability snapshot.
    pub fn cloud_info(&self) -> Option<&SteamworksRemoteStorageCloudInfo> {
        self.cloud_info.as_ref()
    }

    /// Returns the most recent Steam Cloud file list.
    pub fn files(&self) -> &[SteamworksRemoteStorageFileSummary] {
        &self.files
    }

    /// Returns the most recent file metadata snapshot read through the plugin.
    pub fn last_file_info(&self) -> Option<&SteamworksRemoteStorageFileInfo> {
        self.last_file_info.as_ref()
    }

    /// Returns the most recent file existence result read through the plugin.
    pub fn last_file_exists(&self) -> Option<(&str, bool)> {
        self.last_file_exists
            .as_ref()
            .map(|(name, exists)| (name.as_str(), *exists))
    }

    /// Returns the most recent file persisted-state result read through the plugin.
    pub fn last_file_persisted(&self) -> Option<(&str, bool)> {
        self.last_file_persisted
            .as_ref()
            .map(|(name, persisted)| (name.as_str(), *persisted))
    }

    /// Returns the most recent file timestamp read through the plugin.
    pub fn last_file_timestamp(&self) -> Option<(&str, i64)> {
        self.last_file_timestamp
            .as_ref()
            .map(|(name, timestamp)| (name.as_str(), *timestamp))
    }

    /// Returns the most recent file contents read through the plugin.
    pub fn last_file_contents(&self) -> Option<&SteamworksRemoteStorageFileContents> {
        self.last_file_contents.as_ref()
    }

    /// Returns the most recent file write completed through the plugin.
    pub fn last_file_written(&self) -> Option<&SteamworksRemoteStorageFileWritten> {
        self.last_file_written.as_ref()
    }

    /// Returns the most recent file share completed through the plugin.
    pub fn last_shared_file(&self) -> Option<&SteamworksRemoteStorageSharedFile> {
        self.last_shared_file.as_ref()
    }

    /// Returns the number of completed file reads observed through the plugin.
    pub fn read_count(&self) -> u64 {
        self.read_count
    }

    /// Returns the number of completed file writes observed through the plugin.
    pub fn write_count(&self) -> u64 {
        self.write_count
    }

    /// Returns the number of completed file shares observed through the plugin.
    pub fn share_count(&self) -> u64 {
        self.share_count
    }
}
