use bevy_ecs::prelude::Resource;

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

/// Runtime state for [`super::SteamworksRemoteStoragePlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksRemoteStorageState {
    last_error: Option<SteamworksRemoteStorageError>,
    cloud_info: Option<SteamworksRemoteStorageCloudInfo>,
    files: Vec<SteamworksRemoteStorageFileSummary>,
    last_file_info: Option<SteamworksRemoteStorageFileInfo>,
    last_file_exists: Option<(String, bool)>,
    last_file_persisted: Option<(String, bool)>,
    last_file_timestamp: Option<(String, i64)>,
    last_file_contents: Option<SteamworksRemoteStorageFileContents>,
    last_file_written: Option<SteamworksRemoteStorageFileWritten>,
    last_shared_file: Option<SteamworksRemoteStorageSharedFile>,
    read_count: u64,
    write_count: u64,
    share_count: u64,
    next_request_id: u64,
}
