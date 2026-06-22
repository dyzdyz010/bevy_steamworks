use super::SteamworksUgcState;
use crate::ugc::{
    SteamworksUgcDownloadItemResult, SteamworksUgcError, SteamworksUgcGameServerWorkshopInit,
    SteamworksUgcItemDownloadInfoResult, SteamworksUgcItemInstallInfoResult,
    SteamworksUgcItemStateInfo, SteamworksUgcItemUpdateProgress, SteamworksUgcQueryIds,
    SteamworksUgcQueryResults, SteamworksUgcQueryTotal,
};

impl SteamworksUgcState {
    /// Returns the most recent synchronous or async error observed by the plugin.
    pub fn last_error(&self) -> Option<&SteamworksUgcError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent subscribed Workshop item list.
    pub fn subscribed_items(&self) -> &[steamworks::PublishedFileId] {
        &self.subscribed_items
    }

    /// Returns the most recent UGC query result set.
    pub fn last_query(&self) -> Option<&SteamworksUgcQueryResults> {
        self.last_query.as_ref()
    }

    /// Returns the most recent UGC total-only query result.
    pub fn last_query_total(&self) -> Option<&SteamworksUgcQueryTotal> {
        self.last_query_total.as_ref()
    }

    /// Returns the most recent UGC ID-only query result.
    pub fn last_query_ids(&self) -> Option<&SteamworksUgcQueryIds> {
        self.last_query_ids.as_ref()
    }

    /// Returns the most recent item state snapshot.
    pub fn last_item_state(&self) -> Option<&SteamworksUgcItemStateInfo> {
        self.last_item_state.as_ref()
    }

    /// Returns the most recent item download info snapshot.
    pub fn last_item_download_info(&self) -> Option<&SteamworksUgcItemDownloadInfoResult> {
        self.last_item_download_info.as_ref()
    }

    /// Returns the most recent item install info snapshot.
    pub fn last_item_install_info(&self) -> Option<&SteamworksUgcItemInstallInfoResult> {
        self.last_item_install_info.as_ref()
    }

    /// Returns the most recent item update progress snapshot.
    pub fn last_item_update_progress(&self) -> Option<&SteamworksUgcItemUpdateProgress> {
        self.last_item_update_progress.as_ref()
    }

    /// Returns the most recent Workshop download completion callback snapshot.
    pub fn last_download_item_result(&self) -> Option<&SteamworksUgcDownloadItemResult> {
        self.last_download_item_result.as_ref()
    }

    /// Returns the most recent Steam Game Server Workshop initialization.
    pub fn last_game_server_workshop_init(&self) -> Option<&SteamworksUgcGameServerWorkshopInit> {
        self.last_game_server_workshop_init.as_ref()
    }

    /// Returns the number of item update progress handles currently owned by the plugin.
    pub fn active_item_updates(&self) -> usize {
        self.active_item_updates
    }

    /// Returns the number of `DownloadItem` submissions accepted by Steam.
    pub fn submitted_downloads(&self) -> u64 {
        self.submitted_downloads
    }

    /// Returns the number of successful async operations completed through this plugin.
    pub fn successful_async_operations(&self) -> u64 {
        self.successful_async_operations
    }

    /// Returns the number of failed async operations completed through this plugin.
    pub fn failed_async_operations(&self) -> u64 {
        self.failed_async_operations
    }

    /// Returns the total number of async operations completed through this plugin.
    pub fn completed_async_operations(&self) -> u64 {
        self.successful_async_operations
            .saturating_add(self.failed_async_operations)
    }
}
