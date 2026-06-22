use bevy_ecs::prelude::Resource;

use crate::cache::trim_oldest;

use super::{
    messages::SteamworksRemoteStorageError,
    types::{
        SteamworksRemoteStorageCloudInfo, SteamworksRemoteStorageFileContents,
        SteamworksRemoteStorageFileInfo, SteamworksRemoteStorageFileSummary,
        SteamworksRemoteStorageFileWritten, SteamworksRemoteStorageSharedFile,
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
