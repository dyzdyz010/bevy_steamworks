use bevy_ecs::prelude::Resource;

use crate::cache::trim_oldest;

use super::{
    messages::SteamworksRemoteStorageError,
    types::{
        SteamworksRemoteStorageCloudInfo, SteamworksRemoteStorageFileContents,
        SteamworksRemoteStorageFileInfo, SteamworksRemoteStorageFileReadRequest,
        SteamworksRemoteStorageFileShareRequest, SteamworksRemoteStorageFileSummary,
        SteamworksRemoteStorageFileWriteRequest, SteamworksRemoteStorageFileWritten,
        SteamworksRemoteStorageSharedFile,
    },
};

mod accessors;
mod operations;

pub(in crate::remote_storage) const STEAMWORKS_REMOTE_STORAGE_STATE_CACHE_LIMIT: usize = 1_024;

/// Runtime state for [`super::SteamworksRemoteStoragePlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksRemoteStorageState {
    last_error: Option<SteamworksRemoteStorageError>,
    cloud_info: Option<SteamworksRemoteStorageCloudInfo>,
    files: Vec<SteamworksRemoteStorageFileSummary>,
    file_infos: Vec<SteamworksRemoteStorageFileInfo>,
    last_file_info: Option<SteamworksRemoteStorageFileInfo>,
    last_file_exists: Option<(String, bool)>,
    last_file_persisted: Option<(String, bool)>,
    last_file_timestamp: Option<(String, i64)>,
    last_file_sync_platforms: Option<(String, steamworks::RemoteStoragePlatforms)>,
    file_read_requests: Vec<SteamworksRemoteStorageFileReadRequest>,
    file_write_requests: Vec<SteamworksRemoteStorageFileWriteRequest>,
    file_share_requests: Vec<SteamworksRemoteStorageFileShareRequest>,
    file_contents: Vec<SteamworksRemoteStorageFileContents>,
    file_writes: Vec<SteamworksRemoteStorageFileWritten>,
    shared_files: Vec<SteamworksRemoteStorageSharedFile>,
    last_file_contents: Option<SteamworksRemoteStorageFileContents>,
    last_file_written: Option<SteamworksRemoteStorageFileWritten>,
    last_shared_file: Option<SteamworksRemoteStorageSharedFile>,
    read_count: u64,
    write_count: u64,
    share_count: u64,
    next_request_id: u64,
}

pub(super) fn upsert_file_info(
    infos: &mut Vec<SteamworksRemoteStorageFileInfo>,
    info: SteamworksRemoteStorageFileInfo,
) {
    if let Some(existing) = infos.iter_mut().find(|existing| existing.name == info.name) {
        *existing = info;
    } else {
        infos.push(info);
        trim_oldest(infos, STEAMWORKS_REMOTE_STORAGE_STATE_CACHE_LIMIT);
    }
}

pub(super) fn update_file_info(
    infos: &mut [SteamworksRemoteStorageFileInfo],
    name: &str,
    mut update: impl FnMut(&mut SteamworksRemoteStorageFileInfo),
) {
    if let Some(info) = infos.iter_mut().find(|info| info.name == name) {
        update(info);
    }
}

pub(super) fn upsert_file_read_request(
    requests: &mut Vec<SteamworksRemoteStorageFileReadRequest>,
    request: SteamworksRemoteStorageFileReadRequest,
) {
    if let Some(existing) = requests
        .iter_mut()
        .find(|existing| existing.request_id == request.request_id)
    {
        *existing = request;
    } else {
        requests.push(request);
        trim_oldest(requests, STEAMWORKS_REMOTE_STORAGE_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_file_write_request(
    requests: &mut Vec<SteamworksRemoteStorageFileWriteRequest>,
    request: SteamworksRemoteStorageFileWriteRequest,
) {
    if let Some(existing) = requests
        .iter_mut()
        .find(|existing| existing.request_id == request.request_id)
    {
        *existing = request;
    } else {
        requests.push(request);
        trim_oldest(requests, STEAMWORKS_REMOTE_STORAGE_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_file_share_request(
    requests: &mut Vec<SteamworksRemoteStorageFileShareRequest>,
    request: SteamworksRemoteStorageFileShareRequest,
) {
    if let Some(existing) = requests
        .iter_mut()
        .find(|existing| existing.request_id == request.request_id)
    {
        *existing = request;
    } else {
        requests.push(request);
        trim_oldest(requests, STEAMWORKS_REMOTE_STORAGE_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_file_contents(
    contents: &mut Vec<SteamworksRemoteStorageFileContents>,
    file_contents: SteamworksRemoteStorageFileContents,
) {
    if let Some(existing) = contents
        .iter_mut()
        .find(|existing| existing.request_id == file_contents.request_id)
    {
        *existing = file_contents;
    } else {
        contents.push(file_contents);
        trim_oldest(contents, STEAMWORKS_REMOTE_STORAGE_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_file_written(
    writes: &mut Vec<SteamworksRemoteStorageFileWritten>,
    written: SteamworksRemoteStorageFileWritten,
) {
    if let Some(existing) = writes
        .iter_mut()
        .find(|existing| existing.request_id == written.request_id)
    {
        *existing = written;
    } else {
        writes.push(written);
        trim_oldest(writes, STEAMWORKS_REMOTE_STORAGE_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_shared_file(
    shared_files: &mut Vec<SteamworksRemoteStorageSharedFile>,
    shared_file: SteamworksRemoteStorageSharedFile,
) {
    if let Some(existing) = shared_files
        .iter_mut()
        .find(|existing| existing.request_id == shared_file.request_id)
    {
        *existing = shared_file;
    } else {
        shared_files.push(shared_file);
        trim_oldest(shared_files, STEAMWORKS_REMOTE_STORAGE_STATE_CACHE_LIMIT);
    }
}
