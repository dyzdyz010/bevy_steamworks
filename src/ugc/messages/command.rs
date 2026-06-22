use bevy_ecs::message::Message;

use super::super::{SteamworksUgcItemUpdate, SteamworksUgcQuery, SteamworksUgcWorkshopDepotId};

/// A high-level command for Steam Workshop / UGC workflows.
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksUgcCommand {
    /// Suspend or resume Workshop downloads.
    SuspendDownloads {
        /// Whether downloads should be suspended.
        suspend: bool,
    },
    /// List subscribed Workshop items.
    ListSubscribedItems {
        /// Include locally disabled items.
        include_locally_disabled: bool,
    },
    /// Read item state flags.
    GetItemState {
        /// Item to inspect.
        item: steamworks::PublishedFileId,
    },
    /// Read item download progress.
    GetItemDownloadInfo {
        /// Item to inspect.
        item: steamworks::PublishedFileId,
    },
    /// Read item install information.
    GetItemInstallInfo {
        /// Item to inspect.
        item: steamworks::PublishedFileId,
    },
    /// Submit a Workshop item download.
    DownloadItem {
        /// Item to download.
        item: steamworks::PublishedFileId,
        /// Whether the download should be high priority.
        high_priority: bool,
    },
    /// Run a UGC query.
    Query {
        /// Query to run.
        query: SteamworksUgcQuery,
    },
    /// Run a UGC query that only returns the total matching result count.
    ///
    /// The query's payload-shape flags (`return_only_ids` and `return_total_only`) are ignored
    /// because this command uses Steam's total-only query path.
    QueryTotal {
        /// Query to run.
        query: SteamworksUgcQuery,
    },
    /// Run a UGC query that only returns item IDs for the submitted query page/result set.
    ///
    /// The query's payload-shape flags (`return_only_ids` and `return_total_only`) are ignored
    /// because this command uses Steam's ID-only query path.
    QueryIds {
        /// Query to run.
        query: SteamworksUgcQuery,
    },
    /// Create a Workshop item.
    CreateItem {
        /// App ID that owns the item.
        app_id: steamworks::AppId,
        /// Item file type.
        file_type: steamworks::FileType,
    },
    /// Submit an update for an existing Workshop item.
    SubmitItemUpdate {
        /// App ID that owns the item.
        app_id: steamworks::AppId,
        /// Item to update.
        item: steamworks::PublishedFileId,
        /// Update data and change note.
        update: SteamworksUgcItemUpdate,
    },
    /// Read progress for a submitted Workshop item update.
    GetItemUpdateProgress {
        /// Plugin request ID returned by the update submission.
        request_id: u64,
    },
    /// Stop retaining progress state for an item update.
    ForgetItemUpdate {
        /// Plugin request ID returned by the update submission.
        request_id: u64,
    },
    /// Subscribe to a Workshop item.
    SubscribeItem {
        /// Item to subscribe to.
        item: steamworks::PublishedFileId,
    },
    /// Unsubscribe from a Workshop item.
    UnsubscribeItem {
        /// Item to unsubscribe from.
        item: steamworks::PublishedFileId,
    },
    /// Delete a Workshop item owned by the current user.
    DeleteItem {
        /// Item to delete.
        item: steamworks::PublishedFileId,
    },
    /// Start playtime tracking for Workshop items.
    StartPlaytimeTracking {
        /// Items to track.
        items: Vec<steamworks::PublishedFileId>,
    },
    /// Stop playtime tracking for Workshop items.
    StopPlaytimeTracking {
        /// Items to stop tracking.
        items: Vec<steamworks::PublishedFileId>,
    },
    /// Stop playtime tracking for all Workshop items.
    StopPlaytimeTrackingForAllItems,
    /// Initialize Workshop content storage for a Steam Game Server.
    ///
    /// This command uses the [`crate::SteamworksServer`] resource instead of
    /// the client [`crate::SteamworksClient`] resource.
    InitWorkshopForGameServer {
        /// Workshop depot to initialize.
        workshop_depot: SteamworksUgcWorkshopDepotId,
        /// Local folder for game-server Workshop content.
        folder: String,
    },
}

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
