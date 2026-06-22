use super::SteamworksUgcState;
use crate::ugc::{
    update_watches::SteamworksUgcUpdateWatches, SteamworksUgcError,
    SteamworksUgcGameServerWorkshopInit, SteamworksUgcOperation,
};

impl SteamworksUgcState {
    pub(in crate::ugc) fn record_error(&mut self, error: SteamworksUgcError) {
        self.last_error = Some(error);
    }

    pub(in crate::ugc) fn record_operation(&mut self, operation: &SteamworksUgcOperation) {
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

    pub(in crate::ugc) fn record_failed_async_operation(&mut self) {
        self.failed_async_operations = self.failed_async_operations.saturating_add(1);
    }

    pub(in crate::ugc) fn sync_active_item_updates(
        &mut self,
        watches: &SteamworksUgcUpdateWatches,
    ) {
        self.active_item_updates = watches.len();
    }

    pub(in crate::ugc) fn next_request_id(&mut self) -> u64 {
        let request_id = self.next_request_id;
        self.next_request_id = self.next_request_id.wrapping_add(1);
        request_id
    }
}
