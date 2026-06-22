use bevy_ecs::message::Message;

use super::super::{SteamworksUgcItemUpdate, SteamworksUgcQuery, SteamworksUgcWorkshopDepotId};

mod constructors;

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
