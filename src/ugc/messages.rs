use std::path::PathBuf;

use bevy_ecs::message::Message;
use thiserror::Error;

use super::{
    SteamworksUgcDownloadItemResult, SteamworksUgcItemDownloadInfoResult,
    SteamworksUgcItemInstallInfoResult, SteamworksUgcItemStateInfo, SteamworksUgcItemUpdate,
    SteamworksUgcItemUpdateProgress, SteamworksUgcQuery, SteamworksUgcQueryResults,
};

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
    pub fn get_item_state(item: steamworks::PublishedFileId) -> Self {
        Self::GetItemState { item }
    }

    /// Creates a [`SteamworksUgcCommand::GetItemDownloadInfo`] command.
    pub fn get_item_download_info(item: steamworks::PublishedFileId) -> Self {
        Self::GetItemDownloadInfo { item }
    }

    /// Creates a [`SteamworksUgcCommand::GetItemInstallInfo`] command.
    pub fn get_item_install_info(item: steamworks::PublishedFileId) -> Self {
        Self::GetItemInstallInfo { item }
    }

    /// Creates a [`SteamworksUgcCommand::Query`] command.
    pub fn query(query: SteamworksUgcQuery) -> Self {
        Self::Query { query }
    }

    /// Creates a [`SteamworksUgcCommand::CreateItem`] command.
    pub fn create_item(app_id: steamworks::AppId, file_type: steamworks::FileType) -> Self {
        Self::CreateItem { app_id, file_type }
    }

    /// Creates a [`SteamworksUgcCommand::SubmitItemUpdate`] command.
    pub fn submit_item_update(
        app_id: steamworks::AppId,
        item: steamworks::PublishedFileId,
        update: SteamworksUgcItemUpdate,
    ) -> Self {
        Self::SubmitItemUpdate {
            app_id,
            item,
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
    pub fn download_item(item: steamworks::PublishedFileId, high_priority: bool) -> Self {
        Self::DownloadItem {
            item,
            high_priority,
        }
    }

    /// Creates a [`SteamworksUgcCommand::SubscribeItem`] command.
    pub fn subscribe_item(item: steamworks::PublishedFileId) -> Self {
        Self::SubscribeItem { item }
    }

    /// Creates a [`SteamworksUgcCommand::UnsubscribeItem`] command.
    pub fn unsubscribe_item(item: steamworks::PublishedFileId) -> Self {
        Self::UnsubscribeItem { item }
    }

    /// Creates a [`SteamworksUgcCommand::DeleteItem`] command.
    pub fn delete_item(item: steamworks::PublishedFileId) -> Self {
        Self::DeleteItem { item }
    }

    /// Creates a [`SteamworksUgcCommand::StartPlaytimeTracking`] command.
    pub fn start_playtime_tracking(items: impl Into<Vec<steamworks::PublishedFileId>>) -> Self {
        Self::StartPlaytimeTracking {
            items: items.into(),
        }
    }

    /// Creates a [`SteamworksUgcCommand::StopPlaytimeTracking`] command.
    pub fn stop_playtime_tracking(items: impl Into<Vec<steamworks::PublishedFileId>>) -> Self {
        Self::StopPlaytimeTracking {
            items: items.into(),
        }
    }

    /// Creates a [`SteamworksUgcCommand::StopPlaytimeTrackingForAllItems`] command.
    pub fn stop_playtime_tracking_for_all_items() -> Self {
        Self::StopPlaytimeTrackingForAllItems
    }
}

/// A successfully submitted or completed UGC operation.
#[derive(Clone, Debug, PartialEq)]
pub enum SteamworksUgcOperation {
    /// Downloads were suspended or resumed.
    DownloadsSuspended {
        /// Submitted suspend value.
        suspend: bool,
    },
    /// Subscribed items were listed.
    SubscribedItemsListed {
        /// Whether locally disabled items were included.
        include_locally_disabled: bool,
        /// Subscribed item IDs.
        items: Vec<steamworks::PublishedFileId>,
    },
    /// Item state flags were read.
    ItemStateRead {
        /// State snapshot.
        info: SteamworksUgcItemStateInfo,
    },
    /// Item download info was read.
    ItemDownloadInfoRead {
        /// Download info snapshot.
        info: SteamworksUgcItemDownloadInfoResult,
    },
    /// Item install info was read.
    ItemInstallInfoRead {
        /// Install info snapshot.
        info: SteamworksUgcItemInstallInfoResult,
    },
    /// A download was submitted.
    DownloadItemSubmitted {
        /// Item submitted.
        item: steamworks::PublishedFileId,
        /// Whether high priority was requested.
        high_priority: bool,
    },
    /// A Workshop download completion callback was observed.
    DownloadItemResultReceived {
        /// Callback snapshot.
        result: SteamworksUgcDownloadItemResult,
    },
    /// A query was submitted.
    QueryRequested {
        /// Plugin request ID.
        request_id: u64,
        /// Query submitted.
        query: SteamworksUgcQuery,
    },
    /// A query completed.
    QueryCompleted {
        /// Plugin request ID.
        request_id: u64,
        /// Query submitted.
        query: SteamworksUgcQuery,
        /// Owned query results.
        results: SteamworksUgcQueryResults,
    },
    /// Item creation was submitted.
    ItemCreateRequested {
        /// Plugin request ID.
        request_id: u64,
        /// App ID submitted.
        app_id: steamworks::AppId,
        /// File type submitted.
        file_type: steamworks::FileType,
    },
    /// Item creation completed.
    ItemCreated {
        /// Plugin request ID.
        request_id: u64,
        /// Created item.
        item: steamworks::PublishedFileId,
        /// Whether Steam requires the user to accept the legal agreement.
        user_needs_to_accept_workshop_legal_agreement: bool,
    },
    /// Item update was submitted.
    ItemUpdateSubmitted {
        /// Plugin request ID.
        request_id: u64,
        /// App ID submitted.
        app_id: steamworks::AppId,
        /// Item submitted.
        item: steamworks::PublishedFileId,
        /// Update submitted.
        update: SteamworksUgcItemUpdate,
    },
    /// Item update completed.
    ItemUpdated {
        /// Plugin request ID.
        request_id: u64,
        /// Updated item.
        item: steamworks::PublishedFileId,
        /// Whether Steam requires the user to accept the legal agreement.
        user_needs_to_accept_workshop_legal_agreement: bool,
    },
    /// Item update progress was read.
    ItemUpdateProgressRead {
        /// Progress snapshot.
        progress: SteamworksUgcItemUpdateProgress,
    },
    /// Item update progress tracking was forgotten.
    ItemUpdateForgotten {
        /// Plugin request ID.
        request_id: u64,
    },
    /// Item subscription was submitted.
    ItemSubscribeRequested {
        /// Plugin request ID.
        request_id: u64,
        /// Item submitted.
        item: steamworks::PublishedFileId,
    },
    /// Item subscription completed.
    ItemSubscribed {
        /// Plugin request ID.
        request_id: u64,
        /// Item subscribed to.
        item: steamworks::PublishedFileId,
    },
    /// Item unsubscribe was submitted.
    ItemUnsubscribeRequested {
        /// Plugin request ID.
        request_id: u64,
        /// Item submitted.
        item: steamworks::PublishedFileId,
    },
    /// Item unsubscribe completed.
    ItemUnsubscribed {
        /// Plugin request ID.
        request_id: u64,
        /// Item unsubscribed from.
        item: steamworks::PublishedFileId,
    },
    /// Item delete was submitted.
    ItemDeleteRequested {
        /// Plugin request ID.
        request_id: u64,
        /// Item submitted.
        item: steamworks::PublishedFileId,
    },
    /// Item delete completed.
    ItemDeleted {
        /// Plugin request ID.
        request_id: u64,
        /// Item deleted.
        item: steamworks::PublishedFileId,
    },
    /// Playtime tracking start was submitted.
    PlaytimeTrackingStartRequested {
        /// Plugin request ID.
        request_id: u64,
        /// Items submitted.
        items: Vec<steamworks::PublishedFileId>,
    },
    /// Playtime tracking start completed.
    PlaytimeTrackingStarted {
        /// Plugin request ID.
        request_id: u64,
        /// Items submitted.
        items: Vec<steamworks::PublishedFileId>,
    },
    /// Playtime tracking stop was submitted.
    PlaytimeTrackingStopRequested {
        /// Plugin request ID.
        request_id: u64,
        /// Items submitted.
        items: Vec<steamworks::PublishedFileId>,
    },
    /// Playtime tracking stop completed.
    PlaytimeTrackingStopped {
        /// Plugin request ID.
        request_id: u64,
        /// Items submitted.
        items: Vec<steamworks::PublishedFileId>,
    },
    /// Stop-all playtime tracking was submitted.
    PlaytimeTrackingForAllItemsStopRequested {
        /// Plugin request ID.
        request_id: u64,
    },
    /// Stop-all playtime tracking completed.
    PlaytimeTrackingForAllItemsStopped {
        /// Plugin request ID.
        request_id: u64,
    },
}

/// Result message emitted by [`crate::SteamworksUgcPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksUgcResult {
    /// The command was submitted to Steamworks, completed, or read synchronously.
    Ok(SteamworksUgcOperation),
    /// The command failed synchronously or through an async Steam call result.
    Err {
        /// Command that failed.
        command: SteamworksUgcCommand,
        /// Failure reason.
        error: SteamworksUgcError,
    },
}

/// Synchronous and async errors from [`crate::SteamworksUgcPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksUgcError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks UGC command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A Workshop item ID was zero.
    #[error("Steamworks UGC item id must be non-zero")]
    InvalidItemId,
    /// An item list was empty.
    #[error("Steamworks UGC item list must not be empty")]
    EmptyItemList,
    /// An item list exceeded the supported per-command cap.
    #[error("Steamworks UGC item list length {requested} exceeds max {max_supported}")]
    TooManyItems {
        /// Requested item count.
        requested: usize,
        /// Maximum accepted item count.
        max_supported: usize,
    },
    /// UGC query pages are one-based.
    #[error("Steamworks UGC query page must be greater than zero")]
    InvalidPage,
    /// A Workshop item update had no fields to apply.
    #[error("Steamworks UGC item update must include at least one field or change note")]
    EmptyItemUpdate,
    /// A Workshop item update field exceeded a Steam size limit.
    #[error(
        "Steamworks UGC update field {field} must be <= {max_supported} bytes, got {requested}"
    )]
    StringTooLong {
        /// Field that was too long.
        field: &'static str,
        /// Requested byte length.
        requested: usize,
        /// Maximum byte length supported by Steam.
        max_supported: usize,
    },
    /// A Workshop item tag contained unsupported text.
    #[error("Steamworks UGC update tag contains unsupported text: {tag}")]
    InvalidTagText {
        /// Tag that was rejected.
        tag: String,
    },
    /// A Workshop item key/value tag key contained unsupported text.
    #[error("Steamworks UGC update key/value tag key contains unsupported text: {key}")]
    InvalidKeyValueTagKey {
        /// Key that was rejected.
        key: String,
    },
    /// A Workshop item update path could not be canonicalized before calling Steam.
    #[error("Steamworks UGC update path field {field} could not be canonicalized: {path}")]
    InvalidPath {
        /// Field that contained the invalid path.
        field: &'static str,
        /// Path that failed canonicalization.
        path: PathBuf,
    },
    /// Too many key/value tag removals were requested in one update.
    #[error(
        "Steamworks UGC update key/value tag removal count {requested} exceeds max {max_supported}"
    )]
    TooManyKeyValueTagRemovals {
        /// Requested removal count.
        requested: usize,
        /// Maximum accepted removal count.
        max_supported: usize,
    },
    /// Too many key/value tag additions were requested in one update.
    #[error("Steamworks UGC update key/value tag addition count {requested} exceeds max {max_supported}")]
    TooManyKeyValueTags {
        /// Requested addition count.
        requested: usize,
        /// Maximum accepted addition count.
        max_supported: usize,
    },
    /// No item update watch exists for the request ID.
    #[error("Steamworks UGC item update request {request_id} was not found")]
    ItemUpdateNotFound {
        /// Plugin request ID.
        request_id: u64,
    },
    /// Steam failed to create a UGC query handle.
    #[error("Steamworks UGC failed to create query handle")]
    CreateQueryFailed,
    /// Steam rejected a synchronous operation.
    #[error("Steamworks UGC operation failed: {operation}")]
    OperationFailed {
        /// Operation that failed.
        operation: &'static str,
    },
    /// The upstream Steamworks API returned an explicit Steam error.
    #[error("Steamworks UGC operation {operation} failed: {source}")]
    SteamError {
        /// Operation that failed.
        operation: &'static str,
        /// Plugin request ID for async operations.
        request_id: Option<u64>,
        /// Steamworks error.
        source: steamworks::SteamError,
    },
}

impl SteamworksUgcError {
    pub(super) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    pub(super) fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }

    pub(super) fn steam_error(
        operation: &'static str,
        request_id: Option<u64>,
        source: steamworks::SteamError,
    ) -> Self {
        Self::SteamError {
            operation,
            request_id,
            source,
        }
    }

    pub(super) fn async_request_id(&self) -> Option<u64> {
        match self {
            Self::SteamError { request_id, .. } => *request_id,
            _ => None,
        }
    }
}
