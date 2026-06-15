use bevy_ecs::prelude::Resource;

use super::{
    messages::{SteamworksRemoteStorageError, SteamworksRemoteStorageOperation},
    types::{
        SteamworksRemoteStorageCloudInfo, SteamworksRemoteStorageFileContents,
        SteamworksRemoteStorageFileInfo, SteamworksRemoteStorageFileSummary,
        SteamworksRemoteStorageFileWritten, SteamworksRemoteStorageSharedFile,
    },
};

/// Runtime state for [`super::SteamworksRemoteStoragePlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksRemoteStorageState {
    last_error: Option<SteamworksRemoteStorageError>,
    cloud_info: Option<SteamworksRemoteStorageCloudInfo>,
    files: Vec<SteamworksRemoteStorageFileSummary>,
    last_file_info: Option<SteamworksRemoteStorageFileInfo>,
    last_file_contents: Option<SteamworksRemoteStorageFileContents>,
    last_file_written: Option<SteamworksRemoteStorageFileWritten>,
    last_shared_file: Option<SteamworksRemoteStorageSharedFile>,
    read_count: u64,
    write_count: u64,
    share_count: u64,
    next_request_id: u64,
}

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

    pub(super) fn record_error(&mut self, error: SteamworksRemoteStorageError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksRemoteStorageOperation) {
        match operation {
            SteamworksRemoteStorageOperation::CloudInfoRead { info } => {
                self.cloud_info = Some(info.clone());
            }
            SteamworksRemoteStorageOperation::CloudEnabledForAppRead { enabled } => {
                if let Some(info) = &mut self.cloud_info {
                    info.app_enabled = *enabled;
                }
            }
            SteamworksRemoteStorageOperation::CloudEnabledForAccountRead { enabled } => {
                if let Some(info) = &mut self.cloud_info {
                    info.account_enabled = *enabled;
                }
            }
            SteamworksRemoteStorageOperation::CloudEnabledForAppSet { enabled } => {
                if let Some(info) = &mut self.cloud_info {
                    info.app_enabled = *enabled;
                }
            }
            SteamworksRemoteStorageOperation::FilesListed { files } => {
                self.files.clone_from(files);
            }
            SteamworksRemoteStorageOperation::FileInfoRead { info } => {
                self.last_file_info = Some(info.clone());
            }
            SteamworksRemoteStorageOperation::FileRead { contents } => {
                self.last_file_contents = Some(contents.clone());
                self.read_count = self.read_count.saturating_add(1);
            }
            SteamworksRemoteStorageOperation::FileWritten { written } => {
                upsert_file_summary(&mut self.files, &written.name, written.bytes as u64);
                self.last_file_written = Some(written.clone());
                self.write_count = self.write_count.saturating_add(1);
            }
            SteamworksRemoteStorageOperation::FileDeleted { name, deleted } => {
                if *deleted {
                    self.files.retain(|file| file.name != *name);
                    if self
                        .last_file_info
                        .as_ref()
                        .is_some_and(|info| info.name == *name)
                    {
                        self.last_file_info = None;
                    }
                }
            }
            SteamworksRemoteStorageOperation::FileForgotten { name, forgotten } => {
                if *forgotten {
                    if let Some(info) = &mut self.last_file_info {
                        if info.name == *name {
                            info.persisted = false;
                        }
                    }
                }
            }
            SteamworksRemoteStorageOperation::FileSyncPlatformsRead { name, platforms }
            | SteamworksRemoteStorageOperation::FileSyncPlatformsSet { name, platforms } => {
                if let Some(info) = &mut self.last_file_info {
                    if info.name != *name {
                        return;
                    }
                    info.sync_platforms = *platforms;
                }
            }
            SteamworksRemoteStorageOperation::FileShared { shared_file } => {
                self.last_shared_file = Some(shared_file.clone());
                self.share_count = self.share_count.saturating_add(1);
            }
            SteamworksRemoteStorageOperation::FileReadRequested { .. }
            | SteamworksRemoteStorageOperation::FileWriteRequested { .. }
            | SteamworksRemoteStorageOperation::FileShareRequested { .. } => {}
        }
    }

    pub(super) fn next_request_id(&mut self) -> u64 {
        let request_id = self.next_request_id;
        self.next_request_id = self.next_request_id.wrapping_add(1);
        request_id
    }
}

fn upsert_file_summary(
    files: &mut Vec<SteamworksRemoteStorageFileSummary>,
    name: &str,
    size_bytes: u64,
) {
    if let Some(file) = files.iter_mut().find(|file| file.name == name) {
        file.size_bytes = size_bytes;
    } else {
        files.push(SteamworksRemoteStorageFileSummary {
            name: name.to_owned(),
            size_bytes,
        });
    }
}
