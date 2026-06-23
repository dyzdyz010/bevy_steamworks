use crate::ugc::*;

impl SteamworksUgcState {
    /// Returns the most recent item download info snapshot.
    pub fn last_item_download_info(&self) -> Option<&SteamworksUgcItemDownloadInfoResult> {
        self.last_item_download_info.as_ref()
    }

    /// Returns cached download progress info for one Workshop item.
    pub fn item_download_info(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<&SteamworksUgcItemDownloadInfoResult> {
        self.item_download_infos
            .iter()
            .find(|info| info.item == item)
    }

    /// Returns whether a completed download-info read had progress data.
    pub fn item_download_info_available(&self, item: steamworks::PublishedFileId) -> Option<bool> {
        self.item_download_info(item)
            .map(|result| result.info.is_some())
    }

    /// Returns downloaded bytes for one item, preserving a completed read with no progress as `Some(None)`.
    pub fn item_downloaded_bytes(&self, item: steamworks::PublishedFileId) -> Option<Option<u64>> {
        self.item_download_info(item)
            .map(|result| result.info.as_ref().map(|info| info.downloaded_bytes))
    }

    /// Returns total download bytes for one item, preserving a completed read with no progress as `Some(None)`.
    pub fn item_download_total_bytes(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<Option<u64>> {
        self.item_download_info(item)
            .map(|result| result.info.as_ref().map(|info| info.total_bytes))
    }

    /// Returns download progress from 0.0 to 1.0, preserving a completed read with no progress as `Some(None)`.
    pub fn item_download_progress(&self, item: steamworks::PublishedFileId) -> Option<Option<f32>> {
        self.item_download_info(item).map(|result| {
            result.info.as_ref().map(|info| {
                if info.total_bytes == 0 {
                    0.0
                } else {
                    info.downloaded_bytes as f32 / info.total_bytes as f32
                }
            })
        })
    }

    /// Returns whether cached download info says the download reached total bytes.
    pub fn item_download_complete(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<Option<bool>> {
        self.item_download_info(item).map(|result| {
            result
                .info
                .as_ref()
                .map(|info| info.total_bytes > 0 && info.downloaded_bytes >= info.total_bytes)
        })
    }

    /// Returns the most recent item install info snapshot.
    pub fn last_item_install_info(&self) -> Option<&SteamworksUgcItemInstallInfoResult> {
        self.last_item_install_info.as_ref()
    }

    /// Returns cached install info for one Workshop item.
    pub fn item_install_info(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<&SteamworksUgcItemInstallInfoResult> {
        self.item_install_infos
            .iter()
            .find(|info| info.item == item)
    }

    /// Returns whether a completed install-info read had install data.
    pub fn item_install_info_available(&self, item: steamworks::PublishedFileId) -> Option<bool> {
        self.item_install_info(item)
            .map(|result| result.info.is_some())
    }

    /// Returns the install folder for one item, preserving a completed read with no install info as `Some(None)`.
    pub fn item_install_folder(&self, item: steamworks::PublishedFileId) -> Option<Option<&str>> {
        self.item_install_info(item)
            .map(|result| result.info.as_ref().map(|info| info.folder.as_str()))
    }

    /// Returns size on disk for one item, preserving a completed read with no install info as `Some(None)`.
    pub fn item_size_on_disk(&self, item: steamworks::PublishedFileId) -> Option<Option<u64>> {
        self.item_install_info(item)
            .map(|result| result.info.as_ref().map(|info| info.size_on_disk))
    }

    /// Returns the Steam install timestamp for one item, preserving a completed read with no install info as `Some(None)`.
    pub fn item_install_timestamp(&self, item: steamworks::PublishedFileId) -> Option<Option<u32>> {
        self.item_install_info(item)
            .map(|result| result.info.as_ref().map(|info| info.timestamp))
    }

    /// Returns the most recent item update progress snapshot.
    pub fn last_item_update_progress(&self) -> Option<&SteamworksUgcItemUpdateProgress> {
        self.last_item_update_progress.as_ref()
    }

    /// Returns the most recent Workshop download completion callback snapshot.
    pub fn last_download_item_result(&self) -> Option<&SteamworksUgcDownloadItemResult> {
        self.last_download_item_result.as_ref()
    }

    /// Returns bounded Workshop download completion callback snapshots by item.
    pub fn download_item_results(&self) -> &[SteamworksUgcDownloadItemResult] {
        &self.download_item_results
    }

    /// Returns the most recent Workshop download completion callback for one item.
    pub fn download_item_result(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<&SteamworksUgcDownloadItemResult> {
        self.download_item_results
            .iter()
            .find(|result| result.item == item)
    }

    /// Returns whether the most recent download completion for one item failed.
    pub fn download_item_failed(&self, item: steamworks::PublishedFileId) -> Option<bool> {
        self.download_item_result(item)
            .map(|result| result.error.is_some())
    }
}
