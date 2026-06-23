use super::SteamworksUgcState;
use crate::ugc::{
    SteamworksUgcDownloadItemResult, SteamworksUgcError, SteamworksUgcGameServerWorkshopInit,
    SteamworksUgcItemDetails, SteamworksUgcItemDownloadInfoResult,
    SteamworksUgcItemInstallInfoResult, SteamworksUgcItemStateInfo,
    SteamworksUgcItemUpdateProgress, SteamworksUgcQueryIds, SteamworksUgcQueryIdsResult,
    SteamworksUgcQueryRequest, SteamworksUgcQueryResult, SteamworksUgcQueryResults,
    SteamworksUgcQueryTotal, SteamworksUgcQueryTotalResult, SteamworksUgcStatistic,
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

    /// Returns the number of items in the most recent subscribed Workshop item list.
    pub fn subscribed_item_count(&self) -> usize {
        self.subscribed_items.len()
    }

    /// Returns whether the most recent subscribed item list contains an item.
    pub fn is_item_subscribed(&self, item: steamworks::PublishedFileId) -> bool {
        self.subscribed_items.contains(&item)
    }

    /// Returns Workshop item detail snapshots cached from completed full queries.
    pub fn item_details(&self) -> &[SteamworksUgcItemDetails] {
        &self.item_details
    }

    /// Returns cached Workshop item details for one item.
    pub fn item_detail(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<&SteamworksUgcItemDetails> {
        self.item_details
            .iter()
            .find(|details| details.published_file_id == item)
    }

    /// Returns a cached Workshop item's creator app ID, preserving a read with no value as `Some(None)`.
    pub fn item_creator_app_id(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<Option<steamworks::AppId>> {
        self.item_detail(item).map(|details| details.creator_app_id)
    }

    /// Returns a cached Workshop item's consumer app ID, preserving a read with no value as `Some(None)`.
    pub fn item_consumer_app_id(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<Option<steamworks::AppId>> {
        self.item_detail(item)
            .map(|details| details.consumer_app_id)
    }

    /// Returns a cached Workshop item's title.
    pub fn item_title(&self, item: steamworks::PublishedFileId) -> Option<&str> {
        self.item_detail(item).map(|details| details.title.as_str())
    }

    /// Returns a cached Workshop item's description.
    pub fn item_description(&self, item: steamworks::PublishedFileId) -> Option<&str> {
        self.item_detail(item)
            .map(|details| details.description.as_str())
    }

    /// Returns a cached Workshop item's tags.
    pub fn item_tags(&self, item: steamworks::PublishedFileId) -> Option<&[String]> {
        self.item_detail(item)
            .map(|details| details.tags.as_slice())
    }

    /// Returns a cached Workshop item's preview URL, preserving a read with no URL as `Some(None)`.
    pub fn item_preview_url(&self, item: steamworks::PublishedFileId) -> Option<Option<&str>> {
        self.item_detail(item)
            .map(|details| details.preview_url.as_deref())
    }

    /// Returns a cached Workshop item's content descriptors.
    pub fn item_content_descriptors(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<&[crate::ugc::SteamworksUgcContentDescriptor]> {
        self.item_detail(item)
            .map(|details| details.content_descriptors.as_slice())
    }

    /// Returns cached Workshop item statistics.
    pub fn item_statistics(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<&[SteamworksUgcStatistic]> {
        self.item_detail(item)
            .map(|details| details.statistics.as_slice())
    }

    /// Returns one cached Workshop item statistic, preserving a cached item without that statistic as `Some(None)`.
    pub fn item_statistic(
        &self,
        item: steamworks::PublishedFileId,
        statistic: steamworks::UGCStatisticType,
    ) -> Option<Option<u64>> {
        self.item_detail(item).map(|details| {
            details
                .statistics
                .iter()
                .find(|entry| entry.statistic == statistic)
                .map(|entry| entry.value)
        })
    }

    /// Returns cached Workshop item metadata, preserving a read with no metadata as `Some(None)`.
    pub fn item_metadata(&self, item: steamworks::PublishedFileId) -> Option<Option<&[u8]>> {
        self.item_detail(item)
            .map(|details| details.metadata.as_deref())
    }

    /// Returns cached Workshop child item IDs, preserving a read with no child list as `Some(None)`.
    pub fn item_children(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<Option<&[steamworks::PublishedFileId]>> {
        self.item_detail(item)
            .map(|details| details.children.as_deref())
    }

    /// Returns cached Workshop item key/value tags.
    pub fn item_key_value_tags(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<&[(String, String)]> {
        self.item_detail(item)
            .map(|details| details.key_value_tags.as_slice())
    }

    /// Returns one cached Workshop item key/value tag, preserving a cached item without that key as `Some(None)`.
    pub fn item_key_value_tag(
        &self,
        item: steamworks::PublishedFileId,
        key: &str,
    ) -> Option<Option<&str>> {
        self.item_detail(item).map(|details| {
            details
                .key_value_tags
                .iter()
                .find(|(entry_key, _)| entry_key == key)
                .map(|(_, value)| value.as_str())
        })
    }

    /// Returns the most recent UGC query result set.
    pub fn last_query(&self) -> Option<&SteamworksUgcQueryResults> {
        self.last_query.as_ref()
    }

    /// Returns bounded submitted UGC query snapshots by request ID.
    pub fn query_requests(&self) -> &[SteamworksUgcQueryRequest] {
        &self.query_requests
    }

    /// Returns the submitted UGC query snapshot for a request ID.
    pub fn query_request(&self, request_id: u64) -> Option<&SteamworksUgcQueryRequest> {
        self.query_requests
            .iter()
            .find(|request| request.request_id == request_id)
    }

    /// Returns bounded completed full UGC query snapshots by request ID.
    pub fn query_results(&self) -> &[SteamworksUgcQueryResult] {
        &self.query_results
    }

    /// Returns the completed full UGC query snapshot for a request ID.
    pub fn query_result(&self, request_id: u64) -> Option<&SteamworksUgcQueryResult> {
        self.query_results
            .iter()
            .find(|result| result.request_id == request_id)
    }

    /// Returns items from a completed full UGC query snapshot.
    pub fn query_result_items(&self, request_id: u64) -> Option<&[SteamworksUgcItemDetails]> {
        self.query_result(request_id)
            .map(|result| result.results.items.as_slice())
    }

    /// Returns the returned item count from a completed full UGC query snapshot.
    pub fn query_result_item_count(&self, request_id: u64) -> Option<usize> {
        self.query_result(request_id)
            .map(|result| result.results.items.len())
    }

    /// Returns the total matching count from a completed full UGC query snapshot.
    pub fn query_result_total_count(&self, request_id: u64) -> Option<u32> {
        self.query_result(request_id)
            .map(|result| result.results.total_results)
    }

    /// Returns whether a completed full UGC query snapshot was served from Steam's cache.
    pub fn query_result_was_cached(&self, request_id: u64) -> Option<bool> {
        self.query_result(request_id)
            .map(|result| result.results.was_cached)
    }

    /// Returns the returned item count from the most recent full UGC query.
    pub fn last_query_item_count(&self) -> Option<usize> {
        self.last_query.as_ref().map(|results| results.items.len())
    }

    /// Returns the total matching count from the most recent full UGC query.
    pub fn last_query_total_count(&self) -> Option<u32> {
        self.last_query
            .as_ref()
            .map(|results| results.total_results)
    }

    /// Returns whether the most recent full UGC query was served from Steam's cache.
    pub fn last_query_was_cached(&self) -> Option<bool> {
        self.last_query.as_ref().map(|results| results.was_cached)
    }

    /// Returns the most recent UGC total-only query result.
    pub fn last_query_total(&self) -> Option<&SteamworksUgcQueryTotal> {
        self.last_query_total.as_ref()
    }

    /// Returns bounded completed total-only UGC query snapshots by request ID.
    pub fn query_total_results(&self) -> &[SteamworksUgcQueryTotalResult] {
        &self.query_total_results
    }

    /// Returns the completed total-only UGC query snapshot for a request ID.
    pub fn query_total_result(&self, request_id: u64) -> Option<&SteamworksUgcQueryTotalResult> {
        self.query_total_results
            .iter()
            .find(|result| result.request_id == request_id)
    }

    /// Returns the total matching count from a completed total-only UGC query snapshot.
    pub fn query_total_count(&self, request_id: u64) -> Option<u32> {
        self.query_total_result(request_id)
            .map(|result| result.total.total_results)
    }

    /// Returns the most recent UGC ID-only query result.
    pub fn last_query_ids(&self) -> Option<&SteamworksUgcQueryIds> {
        self.last_query_ids.as_ref()
    }

    /// Returns bounded completed ID-only UGC query snapshots by request ID.
    pub fn query_ids_results(&self) -> &[SteamworksUgcQueryIdsResult] {
        &self.query_ids_results
    }

    /// Returns the completed ID-only UGC query snapshot for a request ID.
    pub fn query_ids_result(&self, request_id: u64) -> Option<&SteamworksUgcQueryIdsResult> {
        self.query_ids_results
            .iter()
            .find(|result| result.request_id == request_id)
    }

    /// Returns item IDs from a completed ID-only UGC query snapshot.
    pub fn query_ids_items(&self, request_id: u64) -> Option<&[steamworks::PublishedFileId]> {
        self.query_ids_result(request_id)
            .map(|result| result.ids.items.as_slice())
    }

    /// Returns the item count from a completed ID-only UGC query snapshot.
    pub fn query_ids_item_count(&self, request_id: u64) -> Option<usize> {
        self.query_ids_result(request_id)
            .map(|result| result.ids.items.len())
    }

    /// Returns the item count from the most recent ID-only UGC query.
    pub fn last_query_ids_count(&self) -> Option<usize> {
        self.last_query_ids.as_ref().map(|ids| ids.items.len())
    }

    /// Returns the most recent item state snapshot.
    pub fn last_item_state(&self) -> Option<&SteamworksUgcItemStateInfo> {
        self.last_item_state.as_ref()
    }

    /// Returns cached state flags for one Workshop item.
    pub fn item_state(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<&SteamworksUgcItemStateInfo> {
        self.item_states.iter().find(|info| info.item == item)
    }

    /// Returns cached state flags for one Workshop item.
    pub fn item_state_flags(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<steamworks::ItemState> {
        self.item_state(item).map(|info| info.state)
    }

    /// Returns whether cached item state flags contain a flag.
    pub fn item_state_contains(
        &self,
        item: steamworks::PublishedFileId,
        flag: steamworks::ItemState,
    ) -> Option<bool> {
        self.item_state_flags(item)
            .map(|state| state.contains(flag))
    }

    /// Returns whether cached item state says the item is subscribed.
    pub fn item_state_subscribed(&self, item: steamworks::PublishedFileId) -> Option<bool> {
        self.item_state_contains(item, steamworks::ItemState::SUBSCRIBED)
    }

    /// Returns whether cached item state says the item is installed.
    pub fn item_state_installed(&self, item: steamworks::PublishedFileId) -> Option<bool> {
        self.item_state_contains(item, steamworks::ItemState::INSTALLED)
    }

    /// Returns whether cached item state says the item needs an update.
    pub fn item_state_needs_update(&self, item: steamworks::PublishedFileId) -> Option<bool> {
        self.item_state_contains(item, steamworks::ItemState::NEEDS_UPDATE)
    }

    /// Returns whether cached item state says the item is downloading.
    pub fn item_state_downloading(&self, item: steamworks::PublishedFileId) -> Option<bool> {
        self.item_state_contains(item, steamworks::ItemState::DOWNLOADING)
    }

    /// Returns whether cached item state says the item download is pending.
    pub fn item_state_download_pending(&self, item: steamworks::PublishedFileId) -> Option<bool> {
        self.item_state_contains(item, steamworks::ItemState::DOWNLOAD_PENDING)
    }

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
