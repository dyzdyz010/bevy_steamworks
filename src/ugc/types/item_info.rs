/// Download progress snapshot for one Workshop item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUgcItemDownloadInfo {
    /// Bytes currently downloaded.
    pub downloaded_bytes: u64,
    /// Total bytes Steam expects to download.
    pub total_bytes: u64,
}

/// Result of reading one item's download progress.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUgcItemDownloadInfoResult {
    /// Item inspected.
    pub item: steamworks::PublishedFileId,
    /// Progress if Steam had download info for the item.
    pub info: Option<SteamworksUgcItemDownloadInfo>,
}

/// Workshop download completion callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUgcDownloadItemResult {
    /// App ID reported by Steam.
    pub app_id: steamworks::AppId,
    /// Workshop item whose download completed or failed.
    pub item: steamworks::PublishedFileId,
    /// Steam error when the download failed.
    pub error: Option<steamworks::SteamError>,
}

/// Install information snapshot for one Workshop item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUgcItemInstallInfo {
    /// Folder where the item is installed.
    pub folder: String,
    /// Size on disk in bytes.
    pub size_on_disk: u64,
    /// Steam install timestamp.
    pub timestamp: u32,
}

/// Result of reading one item's install information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUgcItemInstallInfoResult {
    /// Item inspected.
    pub item: steamworks::PublishedFileId,
    /// Install info if Steam had it for the item.
    pub info: Option<SteamworksUgcItemInstallInfo>,
}

/// State snapshot for one Workshop item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUgcItemStateInfo {
    /// Item inspected.
    pub item: steamworks::PublishedFileId,
    /// State flags reported by Steam.
    pub state: steamworks::ItemState,
}
