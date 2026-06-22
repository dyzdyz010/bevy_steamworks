use super::SteamworksRemoteStorageState;
use crate::remote_storage::{
    SteamworksRemoteStorageError, SteamworksRemoteStorageFileSummary,
    SteamworksRemoteStorageOperation,
};

impl SteamworksRemoteStorageState {
    pub(in crate::remote_storage) fn record_error(&mut self, error: SteamworksRemoteStorageError) {
        self.last_error = Some(error);
    }

    pub(in crate::remote_storage) fn record_operation(
        &mut self,
        operation: &SteamworksRemoteStorageOperation,
    ) {
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
                self.last_file_exists = Some((info.name.clone(), info.exists));
                self.last_file_persisted = Some((info.name.clone(), info.persisted));
                self.last_file_timestamp = Some((info.name.clone(), info.timestamp));
            }
            SteamworksRemoteStorageOperation::FileExistsRead { name, exists } => {
                self.last_file_exists = Some((name.clone(), *exists));
            }
            SteamworksRemoteStorageOperation::FilePersistedRead { name, persisted } => {
                self.last_file_persisted = Some((name.clone(), *persisted));
            }
            SteamworksRemoteStorageOperation::FileTimestampRead { name, timestamp } => {
                self.last_file_timestamp = Some((name.clone(), *timestamp));
            }
            SteamworksRemoteStorageOperation::FileRead { contents } => {
                self.last_file_contents = Some(contents.clone());
                self.read_count = self.read_count.saturating_add(1);
            }
            SteamworksRemoteStorageOperation::FileWritten { written } => {
                upsert_file_summary(&mut self.files, &written.name, written.bytes as u64);
                if let Some(info) = &mut self.last_file_info {
                    if info.name == written.name {
                        info.exists = true;
                    }
                }
                self.last_file_exists = Some((written.name.clone(), true));
                clear_matching_file_cache(&mut self.last_file_persisted, &written.name);
                clear_matching_file_cache(&mut self.last_file_timestamp, &written.name);
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
                    self.last_file_exists = Some((name.clone(), false));
                    clear_matching_file_cache(&mut self.last_file_persisted, name);
                    clear_matching_file_cache(&mut self.last_file_timestamp, name);
                }
            }
            SteamworksRemoteStorageOperation::FileForgotten { name, forgotten } => {
                if *forgotten {
                    if let Some(info) = &mut self.last_file_info {
                        if info.name == *name {
                            info.persisted = false;
                        }
                    }
                    self.last_file_persisted = Some((name.clone(), false));
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

    pub(in crate::remote_storage) fn next_request_id(&mut self) -> u64 {
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

fn clear_matching_file_cache<T>(cache: &mut Option<(String, T)>, name: &str) {
    if cache
        .as_ref()
        .is_some_and(|(cached_name, _)| cached_name == name)
    {
        *cache = None;
    }
}
