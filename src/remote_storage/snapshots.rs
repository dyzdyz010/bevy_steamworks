use super::{
    SteamworksRemoteStorageCloudInfo, SteamworksRemoteStorageFileInfo,
    SteamworksRemoteStorageFileSummary,
};

pub(super) fn snapshot_cloud_info(
    remote_storage: &steamworks::RemoteStorage,
) -> SteamworksRemoteStorageCloudInfo {
    SteamworksRemoteStorageCloudInfo {
        app_enabled: remote_storage.is_cloud_enabled_for_app(),
        account_enabled: remote_storage.is_cloud_enabled_for_account(),
    }
}

pub(super) fn snapshot_file_summary(
    file: steamworks::SteamFileInfo,
) -> SteamworksRemoteStorageFileSummary {
    SteamworksRemoteStorageFileSummary {
        name: file.name,
        size_bytes: file.size,
    }
}

pub(super) fn snapshot_file_info(
    name: String,
    file: &steamworks::SteamFile,
) -> SteamworksRemoteStorageFileInfo {
    SteamworksRemoteStorageFileInfo {
        name,
        exists: file.exists(),
        persisted: file.is_persisted(),
        timestamp: file.timestamp(),
        sync_platforms: file.get_sync_platforms(),
    }
}
