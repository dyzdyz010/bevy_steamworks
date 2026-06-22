use std::fmt;

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

/// Submitted Steam Remote Storage file read request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksRemoteStorageFileReadRequest {
    /// Plugin-assigned request ID.
    pub request_id: u64,
    /// File name submitted to Steam Remote Storage.
    pub name: String,
}

/// Owned Steam Remote Storage file payload to write.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksRemoteStorageFileWrite {
    /// File name in Steam Remote Storage.
    pub name: String,
    /// File bytes to write.
    pub data: Vec<u8>,
}

impl SteamworksRemoteStorageFileWrite {
    /// Creates a file write payload.
    pub fn new(name: impl Into<String>, data: impl Into<Vec<u8>>) -> Self {
        Self {
            name: name.into(),
            data: data.into(),
        }
    }

    /// Returns the number of bytes that will be written.
    pub fn bytes(&self) -> usize {
        self.data.len()
    }
}

impl fmt::Debug for SteamworksRemoteStorageFileWrite {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SteamworksRemoteStorageFileWrite")
            .field("name", &self.name)
            .field("data_len", &self.data.len())
            .finish()
    }
}

/// Submitted Steam Remote Storage file write request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksRemoteStorageFileWriteRequest {
    /// Plugin-assigned request ID.
    pub request_id: u64,
    /// File name submitted to Steam Remote Storage.
    pub name: String,
    /// Number of bytes submitted to the worker.
    pub bytes: usize,
}

/// Owned Steam Remote Storage file payload read by the plugin.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksRemoteStorageFileContents {
    /// Plugin-assigned request ID for correlating submitted and completed work.
    pub request_id: u64,
    /// File name in Steam Remote Storage.
    pub name: String,
    /// File bytes read from Steam Remote Storage.
    pub data: Vec<u8>,
}

impl SteamworksRemoteStorageFileContents {
    /// Returns the number of bytes read.
    pub fn bytes(&self) -> usize {
        self.data.len()
    }
}

impl fmt::Debug for SteamworksRemoteStorageFileContents {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SteamworksRemoteStorageFileContents")
            .field("request_id", &self.request_id)
            .field("name", &self.name)
            .field("data_len", &self.data.len())
            .finish()
    }
}

/// Completed Steam Remote Storage file write submission.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksRemoteStorageFileWritten {
    /// Plugin-assigned request ID for correlating submitted and completed work.
    pub request_id: u64,
    /// File name submitted to Steam Remote Storage.
    pub name: String,
    /// Number of bytes accepted by the upstream writer before stream close.
    pub bytes: usize,
}

/// Submitted Steam Remote Storage file share request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksRemoteStorageFileShareRequest {
    /// Plugin-assigned request ID.
    pub request_id: u64,
    /// File name submitted to Steam Remote Storage.
    pub name: String,
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
