//! High-level Bevy ECS integration for Steam Workshop / UGC.
//!
//! This module builds on top of the upstream [`steamworks::UGC`] API. It keeps
//! common Workshop queries, subscriptions, downloads, and playtime tracking in
//! Bevy messages, while converting asynchronous Steam call results into owned
//! ECS-safe result messages.

use std::sync::{Arc, Mutex};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksSystem};

/// Maximum number of item IDs accepted by one UGC details or playtime command.
///
/// The raw Steam call takes a `u32` count and is not intended for unbounded
/// frame-loop payloads. Larger batches should be split by the caller.
pub const STEAMWORKS_UGC_MAX_ITEMS_PER_COMMAND: usize = 1_000;

/// Bevy plugin for high-level Steam Workshop / UGC commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksUgcCommand`] and [`SteamworksUgcResult`] messages and runs its
/// command processor in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksUgcPlugin;

impl SteamworksUgcPlugin {
    /// Creates a UGC plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksUgcPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksUgcState>()
            .init_resource::<SteamworksUgcAsyncResults>()
            .add_message::<SteamworksUgcCommand>()
            .add_message::<SteamworksUgcResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessUgcCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_ugc_commands.in_set(SteamworksSystem::ProcessUgcCommands),
            );
    }
}

/// Runtime state for [`SteamworksUgcPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksUgcState {
    last_error: Option<SteamworksUgcError>,
    subscribed_items: Vec<steamworks::PublishedFileId>,
    last_query: Option<SteamworksUgcQueryResults>,
    last_item_state: Option<SteamworksUgcItemStateInfo>,
    last_item_download_info: Option<SteamworksUgcItemDownloadInfoResult>,
    last_item_install_info: Option<SteamworksUgcItemInstallInfoResult>,
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

    fn record_error(&mut self, error: SteamworksUgcError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksUgcOperation) {
        match operation {
            SteamworksUgcOperation::SubscribedItemsListed { items, .. } => {
                self.subscribed_items.clone_from(items);
            }
            SteamworksUgcOperation::QueryCompleted { results, .. } => {
                self.last_query = Some(results.clone());
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
            SteamworksUgcOperation::DownloadItemSubmitted { .. } => {
                self.submitted_downloads = self.submitted_downloads.saturating_add(1);
            }
            SteamworksUgcOperation::ItemCreated { .. }
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
            | SteamworksUgcOperation::ItemCreateRequested { .. }
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

    fn record_failed_async_operation(&mut self) {
        self.failed_async_operations = self.failed_async_operations.saturating_add(1);
    }

    fn next_request_id(&mut self) -> u64 {
        let request_id = self.next_request_id;
        self.next_request_id = self.next_request_id.wrapping_add(1);
        request_id
    }
}

#[derive(Clone, Debug, Default, Resource)]
struct SteamworksUgcAsyncResults {
    queue: Arc<Mutex<Vec<SteamworksUgcResult>>>,
}

impl SteamworksUgcAsyncResults {
    fn push(&self, result: SteamworksUgcResult) {
        self.queue
            .lock()
            .expect("Steamworks UGC async result mutex was poisoned")
            .push(result);
    }

    fn drain(&self) -> Vec<SteamworksUgcResult> {
        self.queue
            .lock()
            .expect("Steamworks UGC async result mutex was poisoned")
            .drain(..)
            .collect()
    }
}

/// Options applied to UGC queries before they are sent to Steam.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SteamworksUgcQueryOptions {
    /// Tags that must be present.
    pub required_tags: Vec<String>,
    /// Tags that must not be present.
    pub excluded_tags: Vec<String>,
    /// Required key/value tags.
    pub required_key_value_tags: Vec<(String, String)>,
    /// Whether any required tag is sufficient.
    pub match_any_tag: Option<bool>,
    /// Language used for localized title/description fields.
    pub language: Option<String>,
    /// Cache max age in seconds.
    pub allow_cached_response_seconds: Option<u32>,
    /// Filter by Cloud file name.
    pub cloud_file_name_filter: Option<String>,
    /// Full-text search string.
    pub search_text: Option<String>,
    /// Number of trend days for ranked-by-trend queries.
    pub ranked_by_trend_days: Option<u32>,
    /// Whether Steam should return long descriptions.
    pub return_long_description: bool,
    /// Whether Steam should return child item IDs.
    pub return_children: bool,
    /// Whether Steam should return developer metadata.
    pub return_metadata: bool,
    /// Whether Steam should return key/value tags.
    pub return_key_value_tags: bool,
    /// Whether Steam should return only IDs.
    pub return_only_ids: bool,
    /// Whether Steam should return only the total result count.
    pub return_total_only: bool,
    /// Per-item statistics to copy into query result snapshots.
    pub statistics: Vec<steamworks::UGCStatisticType>,
}

impl SteamworksUgcQueryOptions {
    /// Creates default UGC query options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a required tag.
    pub fn with_required_tag(mut self, tag: impl Into<String>) -> Self {
        self.required_tags.push(tag.into());
        self
    }

    /// Adds an excluded tag.
    pub fn with_excluded_tag(mut self, tag: impl Into<String>) -> Self {
        self.excluded_tags.push(tag.into());
        self
    }

    /// Adds a required key/value tag.
    pub fn with_required_key_value_tag(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.required_key_value_tags
            .push((key.into(), value.into()));
        self
    }

    /// Sets whether any required tag is sufficient.
    pub fn with_match_any_tag(mut self, match_any_tag: bool) -> Self {
        self.match_any_tag = Some(match_any_tag);
        self
    }

    /// Sets the query language.
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Allows cached responses up to `seconds` old.
    pub fn with_allow_cached_response(mut self, seconds: u32) -> Self {
        self.allow_cached_response_seconds = Some(seconds);
        self
    }

    /// Sets a Cloud file name filter.
    pub fn with_cloud_file_name_filter(mut self, file_name: impl Into<String>) -> Self {
        self.cloud_file_name_filter = Some(file_name.into());
        self
    }

    /// Sets the full-text search query.
    pub fn with_search_text(mut self, search_text: impl Into<String>) -> Self {
        self.search_text = Some(search_text.into());
        self
    }

    /// Sets ranked-by-trend days.
    pub fn with_ranked_by_trend_days(mut self, days: u32) -> Self {
        self.ranked_by_trend_days = Some(days);
        self
    }

    /// Includes long descriptions.
    pub fn with_long_description(mut self, include: bool) -> Self {
        self.return_long_description = include;
        self
    }

    /// Includes child item IDs.
    pub fn with_children(mut self, include: bool) -> Self {
        self.return_children = include;
        self
    }

    /// Includes developer metadata.
    pub fn with_metadata(mut self, include: bool) -> Self {
        self.return_metadata = include;
        self
    }

    /// Includes key/value tags.
    pub fn with_key_value_tags(mut self, include: bool) -> Self {
        self.return_key_value_tags = include;
        self
    }

    /// Returns only item IDs.
    pub fn with_return_only_ids(mut self, enabled: bool) -> Self {
        self.return_only_ids = enabled;
        self
    }

    /// Returns only the total result count.
    pub fn with_return_total_only(mut self, enabled: bool) -> Self {
        self.return_total_only = enabled;
        self
    }

    /// Adds a statistic to copy from each returned item.
    pub fn with_statistic(mut self, statistic: steamworks::UGCStatisticType) -> Self {
        self.statistics.push(statistic);
        self
    }
}

/// A high-level UGC query.
#[derive(Clone, Debug, PartialEq)]
pub enum SteamworksUgcQuery {
    /// Query a page of all Workshop items.
    All {
        /// Query ranking mode.
        query_type: steamworks::UGCQueryType,
        /// Item type filter.
        item_type: steamworks::UGCType,
        /// Creator/consumer app filters.
        app_ids: steamworks::AppIDs,
        /// One-based result page.
        page: u32,
        /// Query options.
        options: SteamworksUgcQueryOptions,
    },
    /// Query a user-specific Workshop list.
    User {
        /// User account to query.
        account: steamworks::AccountId,
        /// User list type.
        list_type: steamworks::UserList,
        /// Item type filter.
        item_type: steamworks::UGCType,
        /// Sort order.
        sort_order: steamworks::UserListOrder,
        /// Creator/consumer app filters.
        app_ids: steamworks::AppIDs,
        /// One-based result page.
        page: u32,
        /// Query options.
        options: SteamworksUgcQueryOptions,
    },
    /// Query details for explicit item IDs.
    Items {
        /// Items to query.
        items: Vec<steamworks::PublishedFileId>,
        /// Query options.
        options: SteamworksUgcQueryOptions,
    },
}

impl SteamworksUgcQuery {
    /// Creates a query for all Workshop items.
    pub fn all(
        query_type: steamworks::UGCQueryType,
        item_type: steamworks::UGCType,
        app_ids: steamworks::AppIDs,
        page: u32,
    ) -> Self {
        Self::All {
            query_type,
            item_type,
            app_ids,
            page,
            options: SteamworksUgcQueryOptions::new(),
        }
    }

    /// Creates a user-list query.
    pub fn user(
        account: steamworks::AccountId,
        list_type: steamworks::UserList,
        item_type: steamworks::UGCType,
        sort_order: steamworks::UserListOrder,
        app_ids: steamworks::AppIDs,
        page: u32,
    ) -> Self {
        Self::User {
            account,
            list_type,
            item_type,
            sort_order,
            app_ids,
            page,
            options: SteamworksUgcQueryOptions::new(),
        }
    }

    /// Creates an explicit item-details query.
    pub fn items(items: impl Into<Vec<steamworks::PublishedFileId>>) -> Self {
        Self::Items {
            items: items.into(),
            options: SteamworksUgcQueryOptions::new(),
        }
    }

    /// Creates a single-item details query.
    pub fn item(item: steamworks::PublishedFileId) -> Self {
        Self::items(vec![item])
    }

    /// Replaces the query options.
    pub fn with_options(mut self, options: SteamworksUgcQueryOptions) -> Self {
        match &mut self {
            Self::All {
                options: current, ..
            }
            | Self::User {
                options: current, ..
            }
            | Self::Items {
                options: current, ..
            } => *current = options,
        }
        self
    }

    fn options(&self) -> &SteamworksUgcQueryOptions {
        match self {
            Self::All { options, .. }
            | Self::User { options, .. }
            | Self::Items { options, .. } => options,
        }
    }
}

/// Owned result set for a UGC query.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksUgcQueryResults {
    /// Whether Steam served cached data.
    pub was_cached: bool,
    /// Total matching result count across all pages.
    pub total_results: u32,
    /// Number of results returned in this result set.
    pub returned_results: u32,
    /// Item snapshots copied from the query result handle.
    pub items: Vec<SteamworksUgcItemDetails>,
}

/// Owned UGC item details copied from one query result row.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksUgcItemDetails {
    /// Published Workshop file ID.
    pub published_file_id: steamworks::PublishedFileId,
    /// Creator app ID, if present.
    pub creator_app_id: Option<steamworks::AppId>,
    /// Consumer app ID, if present.
    pub consumer_app_id: Option<steamworks::AppId>,
    /// Item title.
    pub title: String,
    /// Item description.
    pub description: String,
    /// Owner Steam ID.
    pub owner: steamworks::SteamId,
    /// Unix epoch seconds when the item was created.
    pub time_created: u32,
    /// Unix epoch seconds when the item was updated.
    pub time_updated: u32,
    /// Unix epoch seconds when the item was added to the relevant user list.
    pub time_added_to_user_list: u32,
    /// Item visibility.
    pub visibility: steamworks::PublishedFileVisibility,
    /// Whether Steam reports the item is banned.
    pub banned: bool,
    /// Whether Steam reports the item is accepted for use.
    pub accepted_for_use: bool,
    /// Tags returned by Steam.
    pub tags: Vec<String>,
    /// Whether tags were truncated.
    pub tags_truncated: bool,
    /// Original file name.
    pub file_name: String,
    /// Workshop file type.
    pub file_type: steamworks::FileType,
    /// File size in bytes.
    pub file_size: u32,
    /// URL returned by Steam.
    pub url: String,
    /// Upvote count.
    pub num_upvotes: u32,
    /// Downvote count.
    pub num_downvotes: u32,
    /// Bayesian vote score, 0.0 to 1.0.
    pub score: f32,
    /// Number of child items.
    pub num_children: u32,
    /// Preview URL, if Steam returned one.
    pub preview_url: Option<String>,
    /// Requested statistics.
    pub statistics: Vec<SteamworksUgcStatistic>,
    /// Developer metadata, if requested and present.
    pub metadata: Option<Vec<u8>>,
    /// Child item IDs, if requested and present.
    pub children: Option<Vec<steamworks::PublishedFileId>>,
    /// Key/value tags, if requested.
    pub key_value_tags: Vec<(String, String)>,
}

/// One UGC statistic copied from a query result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUgcStatistic {
    /// Statistic type.
    pub statistic: steamworks::UGCStatisticType,
    /// Statistic value.
    pub value: u64,
}

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

/// Result message emitted by [`SteamworksUgcPlugin`].
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

/// Synchronous and async errors from [`SteamworksUgcPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksUgcError {
    /// No [`SteamworksClient`] resource exists.
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
    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }

    fn steam_error(
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

    fn async_request_id(&self) -> Option<u64> {
        match self {
            Self::SteamError { request_id, .. } => *request_id,
            _ => None,
        }
    }
}

fn process_ugc_commands(
    client: Option<Res<SteamworksClient>>,
    async_results: Res<SteamworksUgcAsyncResults>,
    mut state: ResMut<SteamworksUgcState>,
    mut commands: ResMut<Messages<SteamworksUgcCommand>>,
    mut results: MessageWriter<SteamworksUgcResult>,
) {
    for result in async_results.drain() {
        record_ugc_result(&mut state, &result);
        results.write(result);
    }

    let Some(client) = client else {
        let error = SteamworksUgcError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks UGC command failed"
            );
            results.write(SteamworksUgcResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        if let Err(error) = validate_command(&command) {
            state.record_error(error.clone());
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks UGC command failed"
            );
            results.write(SteamworksUgcResult::Err { command, error });
            continue;
        }

        let request_id = async_command_request_id(&command, &mut state);
        match handle_ugc_command(&client, &async_results, command.clone(), request_id) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks UGC command"
                );
                results.write(SteamworksUgcResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks UGC command failed"
                );
                results.write(SteamworksUgcResult::Err { command, error });
            }
        }
    }
}

fn record_ugc_result(state: &mut SteamworksUgcState, result: &SteamworksUgcResult) {
    match result {
        SteamworksUgcResult::Ok(operation) => state.record_operation(operation),
        SteamworksUgcResult::Err { error, .. } => {
            if error.async_request_id().is_some() {
                state.record_failed_async_operation();
            }
            state.record_error(error.clone());
        }
    }
}

fn async_command_request_id(
    command: &SteamworksUgcCommand,
    state: &mut SteamworksUgcState,
) -> Option<u64> {
    matches!(
        command,
        SteamworksUgcCommand::Query { .. }
            | SteamworksUgcCommand::CreateItem { .. }
            | SteamworksUgcCommand::SubscribeItem { .. }
            | SteamworksUgcCommand::UnsubscribeItem { .. }
            | SteamworksUgcCommand::DeleteItem { .. }
            | SteamworksUgcCommand::StartPlaytimeTracking { .. }
            | SteamworksUgcCommand::StopPlaytimeTracking { .. }
            | SteamworksUgcCommand::StopPlaytimeTrackingForAllItems
    )
    .then(|| state.next_request_id())
}

fn handle_ugc_command(
    client: &SteamworksClient,
    async_results: &SteamworksUgcAsyncResults,
    command: SteamworksUgcCommand,
    request_id: Option<u64>,
) -> Result<SteamworksUgcOperation, SteamworksUgcError> {
    let ugc = client.ugc();
    match command {
        SteamworksUgcCommand::SuspendDownloads { suspend } => {
            ugc.suspend_downloads(suspend);
            Ok(SteamworksUgcOperation::DownloadsSuspended { suspend })
        }
        SteamworksUgcCommand::ListSubscribedItems {
            include_locally_disabled,
        } => {
            let items = ugc.subscribed_items(include_locally_disabled);
            Ok(SteamworksUgcOperation::SubscribedItemsListed {
                include_locally_disabled,
                items,
            })
        }
        SteamworksUgcCommand::GetItemState { item } => Ok(SteamworksUgcOperation::ItemStateRead {
            info: SteamworksUgcItemStateInfo {
                item,
                state: ugc.item_state(item),
            },
        }),
        SteamworksUgcCommand::GetItemDownloadInfo { item } => {
            Ok(SteamworksUgcOperation::ItemDownloadInfoRead {
                info: SteamworksUgcItemDownloadInfoResult {
                    item,
                    info: ugc
                        .item_download_info(item)
                        .map(
                            |(downloaded_bytes, total_bytes)| SteamworksUgcItemDownloadInfo {
                                downloaded_bytes,
                                total_bytes,
                            },
                        ),
                },
            })
        }
        SteamworksUgcCommand::GetItemInstallInfo { item } => {
            Ok(SteamworksUgcOperation::ItemInstallInfoRead {
                info: SteamworksUgcItemInstallInfoResult {
                    item,
                    info: ugc
                        .item_install_info(item)
                        .map(|info| SteamworksUgcItemInstallInfo {
                            folder: info.folder,
                            size_on_disk: info.size_on_disk,
                            timestamp: info.timestamp,
                        }),
                },
            })
        }
        SteamworksUgcCommand::DownloadItem {
            item,
            high_priority,
        } => {
            if !ugc.download_item(item, high_priority) {
                return Err(SteamworksUgcError::operation_failed("ugc.download_item"));
            }
            Ok(SteamworksUgcOperation::DownloadItemSubmitted {
                item,
                high_priority,
            })
        }
        SteamworksUgcCommand::Query { query } => {
            let request_id = request_id.expect("async UGC query command missing request id");
            let options = query.options().clone();
            let query_handle = create_query(&ugc, &query)?;
            let query_handle = apply_query_options(query_handle, &options)?;
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::Query {
                query: query.clone(),
            };
            let callback_query = query.clone();
            query_handle.fetch(move |result| {
                async_results.push(match result {
                    Ok(results) => {
                        SteamworksUgcResult::Ok(SteamworksUgcOperation::QueryCompleted {
                            request_id,
                            query: callback_query,
                            results: snapshot_query_results(&results, &options),
                        })
                    }
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.query.fetch",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::QueryRequested { request_id, query })
        }
        SteamworksUgcCommand::CreateItem { app_id, file_type } => {
            let request_id = request_id.expect("async UGC create command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::CreateItem { app_id, file_type };
            ugc.create_item(app_id, file_type, move |result| {
                async_results.push(match result {
                    Ok((item, legal)) => {
                        SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemCreated {
                            request_id,
                            item,
                            user_needs_to_accept_workshop_legal_agreement: legal,
                        })
                    }
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.create_item",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::ItemCreateRequested {
                request_id,
                app_id,
                file_type,
            })
        }
        SteamworksUgcCommand::SubscribeItem { item } => {
            let request_id = request_id.expect("async UGC subscribe command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::SubscribeItem { item };
            ugc.subscribe_item(item, move |result| {
                async_results.push(match result {
                    Ok(()) => SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemSubscribed {
                        request_id,
                        item,
                    }),
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.subscribe_item",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::ItemSubscribeRequested { request_id, item })
        }
        SteamworksUgcCommand::UnsubscribeItem { item } => {
            let request_id = request_id.expect("async UGC unsubscribe command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::UnsubscribeItem { item };
            ugc.unsubscribe_item(item, move |result| {
                async_results.push(match result {
                    Ok(()) => SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemUnsubscribed {
                        request_id,
                        item,
                    }),
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.unsubscribe_item",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::ItemUnsubscribeRequested { request_id, item })
        }
        SteamworksUgcCommand::DeleteItem { item } => {
            let request_id = request_id.expect("async UGC delete command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::DeleteItem { item };
            ugc.delete_item(item, move |result| {
                async_results.push(match result {
                    Ok(()) => SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemDeleted {
                        request_id,
                        item,
                    }),
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.delete_item",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::ItemDeleteRequested { request_id, item })
        }
        SteamworksUgcCommand::StartPlaytimeTracking { items } => {
            let request_id =
                request_id.expect("async UGC start playtime command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::StartPlaytimeTracking {
                items: items.clone(),
            };
            let callback_items = items.clone();
            ugc.start_playtime_tracking(&items, move |result| {
                async_results.push(match result {
                    Ok(()) => {
                        SteamworksUgcResult::Ok(SteamworksUgcOperation::PlaytimeTrackingStarted {
                            request_id,
                            items: callback_items,
                        })
                    }
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.start_playtime_tracking",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::PlaytimeTrackingStartRequested { request_id, items })
        }
        SteamworksUgcCommand::StopPlaytimeTracking { items } => {
            let request_id =
                request_id.expect("async UGC stop playtime command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::StopPlaytimeTracking {
                items: items.clone(),
            };
            let callback_items = items.clone();
            ugc.stop_playtime_tracking(&items, move |result| {
                async_results.push(match result {
                    Ok(()) => {
                        SteamworksUgcResult::Ok(SteamworksUgcOperation::PlaytimeTrackingStopped {
                            request_id,
                            items: callback_items,
                        })
                    }
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.stop_playtime_tracking",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::PlaytimeTrackingStopRequested { request_id, items })
        }
        SteamworksUgcCommand::StopPlaytimeTrackingForAllItems => {
            let request_id =
                request_id.expect("async UGC stop all playtime command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::StopPlaytimeTrackingForAllItems;
            ugc.stop_playtime_tracking_for_all_items(move |result| {
                async_results.push(match result {
                    Ok(()) => SteamworksUgcResult::Ok(
                        SteamworksUgcOperation::PlaytimeTrackingForAllItemsStopped { request_id },
                    ),
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.stop_playtime_tracking_for_all_items",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::PlaytimeTrackingForAllItemsStopRequested { request_id })
        }
    }
}

fn create_query(
    ugc: &steamworks::UGC,
    query: &SteamworksUgcQuery,
) -> Result<steamworks::QueryHandle, SteamworksUgcError> {
    match query {
        SteamworksUgcQuery::All {
            query_type,
            item_type,
            app_ids,
            page,
            ..
        } => ugc.query_all(*query_type, *item_type, *app_ids, *page),
        SteamworksUgcQuery::User {
            account,
            list_type,
            item_type,
            sort_order,
            app_ids,
            page,
            ..
        } => ugc.query_user(
            *account,
            *list_type,
            *item_type,
            *sort_order,
            *app_ids,
            *page,
        ),
        SteamworksUgcQuery::Items { items, .. } => ugc.query_items(items.clone()),
    }
    .map_err(|_| SteamworksUgcError::CreateQueryFailed)
}

fn apply_query_options(
    mut query: steamworks::QueryHandle,
    options: &SteamworksUgcQueryOptions,
) -> Result<steamworks::QueryHandle, SteamworksUgcError> {
    validate_query_options(options)?;

    for tag in &options.required_tags {
        query = query.add_required_tag(tag);
    }
    for tag in &options.excluded_tags {
        query = query.add_excluded_tag(tag);
    }
    for (key, value) in &options.required_key_value_tags {
        query = query.add_required_key_value_tag(key, value);
    }
    if let Some(match_any_tag) = options.match_any_tag {
        query = query.set_match_any_tag(match_any_tag);
    }
    if let Some(language) = &options.language {
        query = query.set_language(language);
    }
    if let Some(seconds) = options.allow_cached_response_seconds {
        query = query.set_allow_cached_response(seconds);
    }
    if let Some(file_name) = &options.cloud_file_name_filter {
        query = query.set_cloud_file_name_filter(file_name);
    }
    if let Some(search_text) = &options.search_text {
        query = query.set_search_text(search_text);
    }
    if let Some(days) = options.ranked_by_trend_days {
        query = query.set_ranked_by_trend_days(days);
    }
    query = query.set_return_long_description(options.return_long_description);
    query = query.set_return_children(options.return_children);
    query = query.set_return_metadata(options.return_metadata);
    query = query.set_return_key_value_tags(options.return_key_value_tags);
    query = query.set_return_only_ids(options.return_only_ids);
    query = query.set_return_total_only(options.return_total_only);

    Ok(query)
}

fn snapshot_query_results(
    results: &steamworks::QueryResults<'_>,
    options: &SteamworksUgcQueryOptions,
) -> SteamworksUgcQueryResults {
    let items = (0..results.returned_results())
        .filter_map(|index| {
            results
                .get(index)
                .map(|result| snapshot_query_item(results, options, index, result))
        })
        .collect();

    SteamworksUgcQueryResults {
        was_cached: results.was_cached(),
        total_results: results.total_results(),
        returned_results: results.returned_results(),
        items,
    }
}

fn snapshot_query_item(
    results: &steamworks::QueryResults<'_>,
    options: &SteamworksUgcQueryOptions,
    index: u32,
    result: steamworks::QueryResult,
) -> SteamworksUgcItemDetails {
    SteamworksUgcItemDetails {
        published_file_id: result.published_file_id,
        creator_app_id: result.creator_app_id,
        consumer_app_id: result.consumer_app_id,
        title: result.title,
        description: result.description,
        owner: result.owner,
        time_created: result.time_created,
        time_updated: result.time_updated,
        time_added_to_user_list: result.time_added_to_user_list,
        visibility: result.visibility,
        banned: result.banned,
        accepted_for_use: result.accepted_for_use,
        tags: result.tags,
        tags_truncated: result.tags_truncated,
        file_name: result.file_name,
        file_type: result.file_type,
        file_size: result.file_size,
        url: result.url,
        num_upvotes: result.num_upvotes,
        num_downvotes: result.num_downvotes,
        score: result.score,
        num_children: result.num_children,
        preview_url: results.preview_url(index),
        statistics: options
            .statistics
            .iter()
            .filter_map(|statistic| {
                results
                    .statistic(index, *statistic)
                    .map(|value| SteamworksUgcStatistic {
                        statistic: *statistic,
                        value,
                    })
            })
            .collect(),
        metadata: options
            .return_metadata
            .then(|| results.get_metadata(index))
            .flatten(),
        children: options
            .return_children
            .then(|| results.get_children(index))
            .flatten(),
        key_value_tags: if options.return_key_value_tags {
            (0..results.key_value_tags(index))
                .filter_map(|tag_index| results.get_key_value_tag(index, tag_index))
                .collect()
        } else {
            Vec::new()
        },
    }
}

fn validate_command(command: &SteamworksUgcCommand) -> Result<(), SteamworksUgcError> {
    match command {
        SteamworksUgcCommand::SuspendDownloads { .. }
        | SteamworksUgcCommand::ListSubscribedItems { .. }
        | SteamworksUgcCommand::CreateItem { .. }
        | SteamworksUgcCommand::StopPlaytimeTrackingForAllItems => Ok(()),
        SteamworksUgcCommand::GetItemState { item }
        | SteamworksUgcCommand::GetItemDownloadInfo { item }
        | SteamworksUgcCommand::GetItemInstallInfo { item }
        | SteamworksUgcCommand::DownloadItem { item, .. }
        | SteamworksUgcCommand::SubscribeItem { item }
        | SteamworksUgcCommand::UnsubscribeItem { item }
        | SteamworksUgcCommand::DeleteItem { item } => validate_item(*item),
        SteamworksUgcCommand::Query { query } => validate_query(query),
        SteamworksUgcCommand::StartPlaytimeTracking { items }
        | SteamworksUgcCommand::StopPlaytimeTracking { items } => validate_items(items),
    }
}

fn validate_query(query: &SteamworksUgcQuery) -> Result<(), SteamworksUgcError> {
    match query {
        SteamworksUgcQuery::All { page, options, .. }
        | SteamworksUgcQuery::User { page, options, .. } => {
            if *page == 0 {
                return Err(SteamworksUgcError::InvalidPage);
            }
            validate_query_options(options)
        }
        SteamworksUgcQuery::Items { items, options } => {
            validate_items(items)?;
            validate_query_options(options)
        }
    }
}

fn validate_query_options(options: &SteamworksUgcQueryOptions) -> Result<(), SteamworksUgcError> {
    for tag in &options.required_tags {
        validate_steam_string("required_tag", tag)?;
    }
    for tag in &options.excluded_tags {
        validate_steam_string("excluded_tag", tag)?;
    }
    for (key, value) in &options.required_key_value_tags {
        validate_steam_string("required_key_value_tag.key", key)?;
        validate_steam_string("required_key_value_tag.value", value)?;
    }
    if let Some(language) = &options.language {
        validate_steam_string("language", language)?;
    }
    if let Some(file_name) = &options.cloud_file_name_filter {
        validate_steam_string("cloud_file_name_filter", file_name)?;
    }
    if let Some(search_text) = &options.search_text {
        validate_steam_string("search_text", search_text)?;
    }
    Ok(())
}

fn validate_items(items: &[steamworks::PublishedFileId]) -> Result<(), SteamworksUgcError> {
    if items.is_empty() {
        return Err(SteamworksUgcError::EmptyItemList);
    }
    if items.len() > STEAMWORKS_UGC_MAX_ITEMS_PER_COMMAND {
        return Err(SteamworksUgcError::TooManyItems {
            requested: items.len(),
            max_supported: STEAMWORKS_UGC_MAX_ITEMS_PER_COMMAND,
        });
    }
    for item in items {
        validate_item(*item)?;
    }
    Ok(())
}

fn validate_item(item: steamworks::PublishedFileId) -> Result<(), SteamworksUgcError> {
    if item.0 == 0 {
        Err(SteamworksUgcError::InvalidItemId)
    } else {
        Ok(())
    }
}

fn validate_steam_string(field: &'static str, value: &str) -> Result<(), SteamworksUgcError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksUgcError::invalid_string(field))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    #[test]
    fn ugc_plugin_registers_resources_and_messages() {
        let mut app = App::new();

        app.add_plugins(SteamworksUgcPlugin::new());

        assert!(app.world().contains_resource::<SteamworksUgcState>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksUgcCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksUgcResult>>());
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksUgcPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksUgcCommand>>()
            .write(SteamworksUgcCommand::list_subscribed_items(false));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksUgcResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksUgcResult::Err {
                command: SteamworksUgcCommand::list_subscribed_items(false),
                error: SteamworksUgcError::ClientUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksUgcState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksUgcError::ClientUnavailable)
        );
    }

    #[test]
    fn validation_rejects_invalid_inputs() {
        assert_eq!(
            validate_command(&SteamworksUgcCommand::GetItemState {
                item: steamworks::PublishedFileId(0),
            }),
            Err(SteamworksUgcError::InvalidItemId)
        );

        assert_eq!(
            validate_command(&SteamworksUgcCommand::query(SteamworksUgcQuery::items(
                Vec::new()
            ))),
            Err(SteamworksUgcError::EmptyItemList)
        );

        let too_many =
            vec![steamworks::PublishedFileId(1); STEAMWORKS_UGC_MAX_ITEMS_PER_COMMAND + 1];
        assert_eq!(
            validate_command(&SteamworksUgcCommand::query(SteamworksUgcQuery::items(
                too_many
            ))),
            Err(SteamworksUgcError::TooManyItems {
                requested: STEAMWORKS_UGC_MAX_ITEMS_PER_COMMAND + 1,
                max_supported: STEAMWORKS_UGC_MAX_ITEMS_PER_COMMAND,
            })
        );

        assert_eq!(
            validate_query(&SteamworksUgcQuery::all(
                steamworks::UGCQueryType::RankedByVote,
                steamworks::UGCType::Items,
                steamworks::AppIDs::ConsumerAppId(steamworks::AppId(480)),
                0,
            )),
            Err(SteamworksUgcError::InvalidPage)
        );

        let invalid_options = [
            (
                SteamworksUgcQueryOptions::new().with_required_tag("bad\0tag"),
                "required_tag",
            ),
            (
                SteamworksUgcQueryOptions::new().with_excluded_tag("bad\0tag"),
                "excluded_tag",
            ),
            (
                SteamworksUgcQueryOptions::new().with_required_key_value_tag("bad\0key", "value"),
                "required_key_value_tag.key",
            ),
            (
                SteamworksUgcQueryOptions::new().with_required_key_value_tag("key", "bad\0value"),
                "required_key_value_tag.value",
            ),
            (
                SteamworksUgcQueryOptions::new().with_language("en\0bad"),
                "language",
            ),
            (
                SteamworksUgcQueryOptions::new().with_cloud_file_name_filter("save\0bad.dat"),
                "cloud_file_name_filter",
            ),
            (
                SteamworksUgcQueryOptions::new().with_search_text("bad\0search"),
                "search_text",
            ),
        ];

        for (options, field) in invalid_options {
            assert_eq!(
                validate_query_options(&options),
                Err(SteamworksUgcError::InvalidString { field })
            );
        }
    }

    #[test]
    fn async_commands_get_unique_request_ids() {
        let mut state = SteamworksUgcState::default();
        let query =
            SteamworksUgcCommand::query(SteamworksUgcQuery::item(steamworks::PublishedFileId(1)));

        assert_eq!(async_command_request_id(&query, &mut state), Some(0));
        assert_eq!(async_command_request_id(&query, &mut state), Some(1));
        assert_eq!(
            async_command_request_id(
                &SteamworksUgcCommand::download_item(steamworks::PublishedFileId(1), false),
                &mut state,
            ),
            None
        );
        assert_eq!(
            async_command_request_id(
                &SteamworksUgcCommand::subscribe_item(steamworks::PublishedFileId(1)),
                &mut state,
            ),
            Some(2)
        );
    }

    #[test]
    fn constructors_preserve_inputs() {
        let item = steamworks::PublishedFileId(42);
        let query = SteamworksUgcQuery::item(item).with_options(
            SteamworksUgcQueryOptions::new()
                .with_metadata(true)
                .with_key_value_tags(true),
        );

        assert_eq!(
            SteamworksUgcCommand::query(query.clone()),
            SteamworksUgcCommand::Query { query }
        );
        assert_eq!(
            SteamworksUgcCommand::download_item(item, true),
            SteamworksUgcCommand::DownloadItem {
                item,
                high_priority: true,
            }
        );
        assert_eq!(
            SteamworksUgcCommand::start_playtime_tracking(vec![item]),
            SteamworksUgcCommand::StartPlaytimeTracking { items: vec![item] }
        );
        assert_eq!(
            SteamworksUgcCommand::get_item_state(item),
            SteamworksUgcCommand::GetItemState { item }
        );
        assert_eq!(
            SteamworksUgcCommand::get_item_download_info(item),
            SteamworksUgcCommand::GetItemDownloadInfo { item }
        );
        assert_eq!(
            SteamworksUgcCommand::get_item_install_info(item),
            SteamworksUgcCommand::GetItemInstallInfo { item }
        );
        assert_eq!(
            SteamworksUgcCommand::create_item(
                steamworks::AppId(480),
                steamworks::FileType::Community
            ),
            SteamworksUgcCommand::CreateItem {
                app_id: steamworks::AppId(480),
                file_type: steamworks::FileType::Community,
            }
        );
        assert_eq!(
            SteamworksUgcCommand::delete_item(item),
            SteamworksUgcCommand::DeleteItem { item }
        );
        assert_eq!(
            SteamworksUgcCommand::stop_playtime_tracking(vec![item]),
            SteamworksUgcCommand::StopPlaytimeTracking { items: vec![item] }
        );
        assert_eq!(
            SteamworksUgcCommand::stop_playtime_tracking_for_all_items(),
            SteamworksUgcCommand::StopPlaytimeTrackingForAllItems
        );
    }

    #[test]
    fn state_records_operations_without_unbounded_query_history() {
        let mut state = SteamworksUgcState::default();
        let item = steamworks::PublishedFileId(42);
        let first = SteamworksUgcQueryResults {
            was_cached: false,
            total_results: 1,
            returned_results: 1,
            items: Vec::new(),
        };
        let second = SteamworksUgcQueryResults {
            was_cached: true,
            total_results: 2,
            returned_results: 0,
            items: Vec::new(),
        };

        state.record_operation(&SteamworksUgcOperation::SubscribedItemsListed {
            include_locally_disabled: false,
            items: vec![item],
        });
        state.record_operation(&SteamworksUgcOperation::QueryCompleted {
            request_id: 0,
            query: SteamworksUgcQuery::item(item),
            results: first,
        });
        state.record_operation(&SteamworksUgcOperation::QueryCompleted {
            request_id: 1,
            query: SteamworksUgcQuery::item(item),
            results: second.clone(),
        });
        state.record_operation(&SteamworksUgcOperation::ItemStateRead {
            info: SteamworksUgcItemStateInfo {
                item,
                state: steamworks::ItemState::SUBSCRIBED,
            },
        });
        state.record_operation(&SteamworksUgcOperation::DownloadItemSubmitted {
            item,
            high_priority: false,
        });
        state.record_operation(&SteamworksUgcOperation::ItemSubscribed {
            request_id: 2,
            item,
        });
        state.record_operation(&SteamworksUgcOperation::ItemUnsubscribed {
            request_id: 3,
            item,
        });

        assert!(state.subscribed_items().is_empty());
        assert_eq!(state.last_query(), Some(&second));
        assert_eq!(
            state.last_item_state(),
            Some(&SteamworksUgcItemStateInfo {
                item,
                state: steamworks::ItemState::SUBSCRIBED,
            })
        );
        assert_eq!(state.submitted_downloads(), 1);
        assert_eq!(state.successful_async_operations(), 4);
        assert_eq!(state.failed_async_operations(), 0);
        assert_eq!(state.completed_async_operations(), 4);
    }

    #[test]
    fn state_counts_async_failures_as_completed() {
        let mut state = SteamworksUgcState::default();
        let result = SteamworksUgcResult::Err {
            command: SteamworksUgcCommand::subscribe_item(steamworks::PublishedFileId(1)),
            error: SteamworksUgcError::steam_error(
                "ugc.subscribe_item",
                Some(7),
                steamworks::SteamError::IOFailure,
            ),
        };

        record_ugc_result(&mut state, &result);

        assert_eq!(state.successful_async_operations(), 0);
        assert_eq!(state.failed_async_operations(), 1);
        assert_eq!(state.completed_async_operations(), 1);
        assert_eq!(
            state.last_error(),
            Some(&SteamworksUgcError::steam_error(
                "ugc.subscribe_item",
                Some(7),
                steamworks::SteamError::IOFailure,
            ))
        );
    }
}
