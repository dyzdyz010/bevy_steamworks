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

    /// Returns a listed file summary by name, if present in the latest file list or write cache.
    pub fn file_summary(&self, name: &str) -> Option<&SteamworksRemoteStorageFileSummary> {
        self.files.iter().find(|file| file.name == name)
    }

    /// Returns cached full metadata for one file.
    ///
    /// This is populated by [`crate::SteamworksRemoteStorageCommand::get_file_info`]
    /// and updated by later file status commands when the file is already known.
    pub fn file_info(&self, name: &str) -> Option<&SteamworksRemoteStorageFileInfo> {
        self.file_infos.iter().find(|info| info.name == name)
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

    /// Returns the latest known existence value for one file.
    pub fn file_exists(&self, name: &str) -> Option<bool> {
        self.file_info(name)
            .map(|info| info.exists)
            .or_else(|| matching_last_value(&self.last_file_exists, name))
            .or_else(|| self.file_summary(name).map(|_| true))
    }

    /// Returns the most recent file persisted-state result read through the plugin.
    pub fn last_file_persisted(&self) -> Option<(&str, bool)> {
        self.last_file_persisted
            .as_ref()
            .map(|(name, persisted)| (name.as_str(), *persisted))
    }

    /// Returns the latest known Cloud persisted-state value for one file.
    pub fn file_persisted(&self, name: &str) -> Option<bool> {
        self.file_info(name)
            .map(|info| info.persisted)
            .or_else(|| matching_last_value(&self.last_file_persisted, name))
    }

    /// Returns the most recent file timestamp read through the plugin.
    pub fn last_file_timestamp(&self) -> Option<(&str, i64)> {
        self.last_file_timestamp
            .as_ref()
            .map(|(name, timestamp)| (name.as_str(), *timestamp))
    }

    /// Returns the latest known Steam timestamp for one file.
    pub fn file_timestamp(&self, name: &str) -> Option<i64> {
        self.file_info(name)
            .map(|info| info.timestamp)
            .or_else(|| matching_last_value(&self.last_file_timestamp, name))
    }

    /// Returns the most recent sync-platforms result read or set through the plugin.
    pub fn last_file_sync_platforms(&self) -> Option<(&str, steamworks::RemoteStoragePlatforms)> {
        self.last_file_sync_platforms
            .as_ref()
            .map(|(name, platforms)| (name.as_str(), *platforms))
    }

    /// Returns the latest known sync-platforms value for one file.
    pub fn file_sync_platforms(&self, name: &str) -> Option<steamworks::RemoteStoragePlatforms> {
        self.file_info(name)
            .map(|info| info.sync_platforms)
            .or_else(|| matching_last_value(&self.last_file_sync_platforms, name))
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

fn matching_last_value<T: Copy>(cache: &Option<(String, T)>, name: &str) -> Option<T> {
    cache
        .as_ref()
        .and_then(|(cached_name, value)| (cached_name == name).then_some(*value))
}
