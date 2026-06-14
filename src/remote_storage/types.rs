/// Steam Cloud availability for the current app and account.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SteamworksRemoteStorageCloudInfo {
    /// Whether Steam Cloud is enabled for this app.
    pub app_enabled: bool,
    /// Whether Steam Cloud is enabled for this Steam account.
    pub account_enabled: bool,
}

/// Name and size of a file in Steam Remote Storage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksRemoteStorageFileSummary {
    /// File name in Steam Remote Storage.
    pub name: String,
    /// File size in bytes.
    pub size_bytes: u64,
}

/// Metadata for one Steam Remote Storage file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksRemoteStorageFileInfo {
    /// File name in Steam Remote Storage.
    pub name: String,
    /// Whether Steam reports the file currently exists.
    pub exists: bool,
    /// Whether Steam reports the file is persisted in Cloud storage.
    pub persisted: bool,
    /// Steam file timestamp, as Unix epoch seconds.
    pub timestamp: i64,
    /// Platforms the file is configured to sync to.
    pub sync_platforms: steamworks::RemoteStoragePlatforms,
}

/// Opaque handle returned by Steam after a successful file share.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SteamworksRemoteStorageFileShareHandle(u64);

impl SteamworksRemoteStorageFileShareHandle {
    /// Creates a file share handle from a raw Steam handle value.
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw Steam handle value.
    pub const fn raw(self) -> u64 {
        self.0
    }
}

/// A file share completed through Steam Remote Storage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksRemoteStorageSharedFile {
    /// Plugin-assigned request ID for correlating submitted and completed work.
    pub request_id: u64,
    /// File name that was shared.
    pub name: String,
    /// Steam handle for the shared file.
    pub handle: SteamworksRemoteStorageFileShareHandle,
}
