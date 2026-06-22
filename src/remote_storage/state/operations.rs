use super::{
    update_file_info, upsert_file_contents, upsert_file_info, upsert_file_read_request,
    upsert_file_share_request, upsert_file_write_request, upsert_file_written, upsert_shared_file,
    SteamworksRemoteStorageState, STEAMWORKS_REMOTE_STORAGE_STATE_CACHE_LIMIT,
};
use crate::cache::trim_oldest;
use crate::remote_storage::{
    SteamworksRemoteStorageError, SteamworksRemoteStorageFileReadRequest,
    SteamworksRemoteStorageFileShareRequest, SteamworksRemoteStorageFileSummary,
    SteamworksRemoteStorageFileWriteRequest, SteamworksRemoteStorageOperation,
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
                upsert_file_info(&mut self.file_infos, info.clone());
                self.last_file_info = Some(info.clone());
                self.last_file_exists = Some((info.name.clone(), info.exists));
                self.last_file_persisted = Some((info.name.clone(), info.persisted));
                self.last_file_timestamp = Some((info.name.clone(), info.timestamp));
                self.last_file_sync_platforms = Some((info.name.clone(), info.sync_platforms));
            }
            SteamworksRemoteStorageOperation::FileExistsRead { name, exists } => {
                update_file_info(&mut self.file_infos, name, |info| {
                    info.exists = *exists;
                });
                self.last_file_exists = Some((name.clone(), *exists));
            }
            SteamworksRemoteStorageOperation::FilePersistedRead { name, persisted } => {
                update_file_info(&mut self.file_infos, name, |info| {
                    info.persisted = *persisted;
                });
                self.last_file_persisted = Some((name.clone(), *persisted));
            }
            SteamworksRemoteStorageOperation::FileTimestampRead { name, timestamp } => {
                update_file_info(&mut self.file_infos, name, |info| {
                    info.timestamp = *timestamp;
                });
                self.last_file_timestamp = Some((name.clone(), *timestamp));
            }
            SteamworksRemoteStorageOperation::FileRead { contents } => {
                upsert_file_read_request(
                    &mut self.file_read_requests,
                    SteamworksRemoteStorageFileReadRequest {
                        request_id: contents.request_id,
                        name: contents.name.clone(),
                    },
                );
                upsert_file_contents(&mut self.file_contents, contents.clone());
                self.last_file_contents = Some(contents.clone());
                self.read_count = self.read_count.saturating_add(1);
            }
            SteamworksRemoteStorageOperation::FileWritten { written } => {
                upsert_file_write_request(
                    &mut self.file_write_requests,
                    SteamworksRemoteStorageFileWriteRequest {
                        request_id: written.request_id,
                        name: written.name.clone(),
                        bytes: written.bytes,
                    },
                );
                upsert_file_written(&mut self.file_writes, written.clone());
                upsert_file_summary(&mut self.files, &written.name, written.bytes as u64);
                self.file_infos.retain(|info| info.name != written.name);
                if self
                    .last_file_info
                    .as_ref()
                    .is_some_and(|info| info.name == written.name)
                {
                    self.last_file_info = None;
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
                    self.file_infos.retain(|info| info.name != *name);
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
                    clear_matching_file_cache(&mut self.last_file_sync_platforms, name);
                }
            }
            SteamworksRemoteStorageOperation::FileForgotten { name, forgotten } => {
                if *forgotten {
                    update_file_info(&mut self.file_infos, name, |info| {
                        info.persisted = false;
                    });
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
                update_file_info(&mut self.file_infos, name, |info| {
                    info.sync_platforms = *platforms;
                });
                self.last_file_sync_platforms = Some((name.clone(), *platforms));
                if let Some(info) = &mut self.last_file_info {
                    if info.name != *name {
                        return;
                    }
                    info.sync_platforms = *platforms;
                }
            }
            SteamworksRemoteStorageOperation::FileReadRequested { request_id, name } => {
                upsert_file_read_request(
                    &mut self.file_read_requests,
                    SteamworksRemoteStorageFileReadRequest {
                        request_id: *request_id,
                        name: name.clone(),
                    },
                );
            }
            SteamworksRemoteStorageOperation::FileWriteRequested {
                request_id,
                name,
                bytes,
            } => {
                upsert_file_write_request(
                    &mut self.file_write_requests,
                    SteamworksRemoteStorageFileWriteRequest {
                        request_id: *request_id,
                        name: name.clone(),
                        bytes: *bytes,
                    },
                );
            }
            SteamworksRemoteStorageOperation::FileShareRequested { request_id, name } => {
                upsert_file_share_request(
                    &mut self.file_share_requests,
                    SteamworksRemoteStorageFileShareRequest {
                        request_id: *request_id,
                        name: name.clone(),
                    },
                );
            }
            SteamworksRemoteStorageOperation::FileShared { shared_file } => {
                upsert_file_share_request(
                    &mut self.file_share_requests,
                    SteamworksRemoteStorageFileShareRequest {
                        request_id: shared_file.request_id,
                        name: shared_file.name.clone(),
                    },
                );
                upsert_shared_file(&mut self.shared_files, shared_file.clone());
                self.last_shared_file = Some(shared_file.clone());
                self.share_count = self.share_count.saturating_add(1);
            }
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
        trim_oldest(files, STEAMWORKS_REMOTE_STORAGE_STATE_CACHE_LIMIT);
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
