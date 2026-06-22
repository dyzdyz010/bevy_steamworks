use super::super::super::{
    SteamworksUgcItemUpdate, SteamworksUgcQuery, SteamworksUgcWorkshopDepotId,
};
use super::SteamworksUgcCommand;

impl SteamworksUgcCommand {
    /// Creates a [`SteamworksUgcCommand::SuspendDownloads`] command.
    pub fn suspend_downloads(suspend: bool) -> Self {
        Self::SuspendDownloads { suspend }
    }

    /// Creates a [`SteamworksUgcCommand::ListSubscribedItems`] command.
    pub fn list_subscribed_items(include_locally_disabled: bool) -> Self {
        Self::ListSubscribedItems {
            include_locally_disabled,
        }
    }

    /// Creates a [`SteamworksUgcCommand::GetItemState`] command.
    pub fn get_item_state(item: impl Into<steamworks::PublishedFileId>) -> Self {
        Self::GetItemState { item: item.into() }
    }

    /// Creates a [`SteamworksUgcCommand::GetItemDownloadInfo`] command.
    pub fn get_item_download_info(item: impl Into<steamworks::PublishedFileId>) -> Self {
        Self::GetItemDownloadInfo { item: item.into() }
    }

    /// Creates a [`SteamworksUgcCommand::GetItemInstallInfo`] command.
    pub fn get_item_install_info(item: impl Into<steamworks::PublishedFileId>) -> Self {
        Self::GetItemInstallInfo { item: item.into() }
    }

    /// Creates a [`SteamworksUgcCommand::Query`] command.
    pub fn query(query: SteamworksUgcQuery) -> Self {
        Self::Query { query }
    }

    /// Creates a [`SteamworksUgcCommand::QueryTotal`] command.
    pub fn query_total(query: SteamworksUgcQuery) -> Self {
        Self::QueryTotal { query }
    }

    /// Creates a [`SteamworksUgcCommand::QueryIds`] command.
    pub fn query_ids(query: SteamworksUgcQuery) -> Self {
        Self::QueryIds { query }
    }

    /// Creates a [`SteamworksUgcCommand::CreateItem`] command.
    pub fn create_item(
        app_id: impl Into<steamworks::AppId>,
        file_type: steamworks::FileType,
    ) -> Self {
        Self::CreateItem {
            app_id: app_id.into(),
            file_type,
        }
    }

    /// Creates a [`SteamworksUgcCommand::SubmitItemUpdate`] command.
    pub fn submit_item_update(
        app_id: impl Into<steamworks::AppId>,
        item: impl Into<steamworks::PublishedFileId>,
        update: SteamworksUgcItemUpdate,
    ) -> Self {
        Self::SubmitItemUpdate {
            app_id: app_id.into(),
            item: item.into(),
            update,
        }
    }

    /// Creates a [`SteamworksUgcCommand::GetItemUpdateProgress`] command.
    pub fn get_item_update_progress(request_id: u64) -> Self {
        Self::GetItemUpdateProgress { request_id }
    }

    /// Creates a [`SteamworksUgcCommand::ForgetItemUpdate`] command.
    pub fn forget_item_update(request_id: u64) -> Self {
        Self::ForgetItemUpdate { request_id }
    }

    /// Creates a [`SteamworksUgcCommand::DownloadItem`] command.
    pub fn download_item(
        item: impl Into<steamworks::PublishedFileId>,
        high_priority: bool,
    ) -> Self {
        Self::DownloadItem {
            item: item.into(),
            high_priority,
        }
    }

    /// Creates a [`SteamworksUgcCommand::SubscribeItem`] command.
    pub fn subscribe_item(item: impl Into<steamworks::PublishedFileId>) -> Self {
        Self::SubscribeItem { item: item.into() }
    }

    /// Creates a [`SteamworksUgcCommand::UnsubscribeItem`] command.
    pub fn unsubscribe_item(item: impl Into<steamworks::PublishedFileId>) -> Self {
        Self::UnsubscribeItem { item: item.into() }
    }

    /// Creates a [`SteamworksUgcCommand::DeleteItem`] command.
    pub fn delete_item(item: impl Into<steamworks::PublishedFileId>) -> Self {
        Self::DeleteItem { item: item.into() }
    }

    /// Creates a [`SteamworksUgcCommand::StartPlaytimeTracking`] command.
    pub fn start_playtime_tracking<I, T>(items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<steamworks::PublishedFileId>,
    {
        Self::StartPlaytimeTracking {
            items: items.into_iter().map(Into::into).collect(),
        }
    }

    /// Creates a [`SteamworksUgcCommand::StopPlaytimeTracking`] command.
    pub fn stop_playtime_tracking<I, T>(items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<steamworks::PublishedFileId>,
    {
        Self::StopPlaytimeTracking {
            items: items.into_iter().map(Into::into).collect(),
        }
    }

    /// Creates a [`SteamworksUgcCommand::StopPlaytimeTrackingForAllItems`] command.
    pub fn stop_playtime_tracking_for_all_items() -> Self {
        Self::StopPlaytimeTrackingForAllItems
    }

    /// Creates a [`SteamworksUgcCommand::InitWorkshopForGameServer`] command.
    pub fn init_workshop_for_game_server(
        workshop_depot: impl Into<SteamworksUgcWorkshopDepotId>,
        folder: impl Into<String>,
    ) -> Self {
        Self::InitWorkshopForGameServer {
            workshop_depot: workshop_depot.into(),
            folder: folder.into(),
        }
    }
}
