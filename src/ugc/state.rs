use bevy_ecs::prelude::Resource;

use super::update_watches::SteamworksUgcUpdateWatches;
use super::*;

/// Runtime state for [`crate::SteamworksUgcPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksUgcState {
    last_error: Option<SteamworksUgcError>,
    subscribed_items: Vec<steamworks::PublishedFileId>,
    last_query: Option<SteamworksUgcQueryResults>,
    last_query_total: Option<SteamworksUgcQueryTotal>,
    last_query_ids: Option<SteamworksUgcQueryIds>,
    last_item_state: Option<SteamworksUgcItemStateInfo>,
    last_item_download_info: Option<SteamworksUgcItemDownloadInfoResult>,
    last_item_install_info: Option<SteamworksUgcItemInstallInfoResult>,
    last_item_update_progress: Option<SteamworksUgcItemUpdateProgress>,
    last_download_item_result: Option<SteamworksUgcDownloadItemResult>,
    last_game_server_workshop_init: Option<SteamworksUgcGameServerWorkshopInit>,
    active_item_updates: usize,
    submitted_downloads: u64,
    successful_async_operations: u64,
    failed_async_operations: u64,
    next_request_id: u64,
}

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

    pub(super) fn record_error(&mut self, error: SteamworksUgcError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksUgcOperation) {
        match operation {
            SteamworksUgcOperation::SubscribedItemsListed { items, .. } => {
                self.subscribed_items.clone_from(items);
            }
            SteamworksUgcOperation::QueryCompleted { results, .. } => {
                self.last_query = Some(results.clone());
                self.record_successful_async_operation();
            }
            SteamworksUgcOperation::QueryTotalCompleted { total, .. } => {
                self.last_query_total = Some(total.clone());
                self.record_successful_async_operation();
            }
            SteamworksUgcOperation::QueryIdsCompleted { ids, .. } => {
                self.last_query_ids = Some(ids.clone());
                self.record_successful_async_operation();
            }
            SteamworksUgcOperation::ItemStateRead { info } => {
                self.last_item_state = Some(info.clone());
            }
            SteamworksUgcOperation::ItemDownloadInfoRead { info } => {
                self.last_item_download_info = Some(info.clone());
            }
            SteamworksUgcOperation::ItemInstallInfoRead { info } => {
                self.last_item_install_info = Some(info.clone());
            }
            SteamworksUgcOperation::ItemUpdateProgressRead { progress } => {
                self.last_item_update_progress = Some(progress.clone());
            }
            SteamworksUgcOperation::DownloadItemSubmitted { .. } => {
                self.submitted_downloads = self.submitted_downloads.saturating_add(1);
            }
            SteamworksUgcOperation::DownloadItemResultReceived { result } => {
                self.last_download_item_result = Some(result.clone());
            }
            SteamworksUgcOperation::GameServerWorkshopInitialized {
                workshop_depot,
                folder,
            } => {
                self.last_game_server_workshop_init = Some(SteamworksUgcGameServerWorkshopInit {
                    workshop_depot: *workshop_depot,
                    folder: folder.clone(),
                });
            }
            SteamworksUgcOperation::ItemCreated { .. }
            | SteamworksUgcOperation::ItemUpdated { .. }
            | SteamworksUgcOperation::PlaytimeTrackingStarted { .. }
            | SteamworksUgcOperation::PlaytimeTrackingStopped { .. }
            | SteamworksUgcOperation::PlaytimeTrackingForAllItemsStopped { .. } => {
                self.record_successful_async_operation();
            }
            SteamworksUgcOperation::ItemSubscribed { item, .. } => {
                if !self.subscribed_items.contains(item) {
                    self.subscribed_items.push(*item);
                }
                self.record_successful_async_operation();
            }
            SteamworksUgcOperation::ItemUnsubscribed { item, .. }
            | SteamworksUgcOperation::ItemDeleted { item, .. } => {
                self.subscribed_items.retain(|known| known != item);
                self.record_successful_async_operation();
            }
            SteamworksUgcOperation::DownloadsSuspended { .. }
            | SteamworksUgcOperation::QueryRequested { .. }
            | SteamworksUgcOperation::QueryTotalRequested { .. }
            | SteamworksUgcOperation::QueryIdsRequested { .. }
            | SteamworksUgcOperation::ItemCreateRequested { .. }
            | SteamworksUgcOperation::ItemUpdateSubmitted { .. }
            | SteamworksUgcOperation::ItemUpdateForgotten { .. }
            | SteamworksUgcOperation::ItemSubscribeRequested { .. }
            | SteamworksUgcOperation::ItemUnsubscribeRequested { .. }
            | SteamworksUgcOperation::ItemDeleteRequested { .. }
            | SteamworksUgcOperation::PlaytimeTrackingStartRequested { .. }
            | SteamworksUgcOperation::PlaytimeTrackingStopRequested { .. }
            | SteamworksUgcOperation::PlaytimeTrackingForAllItemsStopRequested { .. } => {}
        }
    }

    fn record_successful_async_operation(&mut self) {
        self.successful_async_operations = self.successful_async_operations.saturating_add(1);
    }

    pub(super) fn record_failed_async_operation(&mut self) {
        self.failed_async_operations = self.failed_async_operations.saturating_add(1);
    }

    pub(super) fn sync_active_item_updates(&mut self, watches: &SteamworksUgcUpdateWatches) {
        self.active_item_updates = watches.len();
    }

    pub(super) fn next_request_id(&mut self) -> u64 {
        let request_id = self.next_request_id;
        self.next_request_id = self.next_request_id.wrapping_add(1);
        request_id
    }
}
