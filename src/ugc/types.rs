use std::path::PathBuf;

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

    pub(super) fn options(&self) -> &SteamworksUgcQueryOptions {
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

/// Workshop download completion callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUgcDownloadItemResult {
    /// App ID reported by Steam.
    pub app_id: steamworks::AppId,
    /// Workshop item whose download completed or failed.
    pub item: steamworks::PublishedFileId,
    /// Steam error when the download failed.
    pub error: Option<steamworks::SteamError>,
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

/// Tags to set on a Workshop item update.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SteamworksUgcItemUpdateTags {
    /// Tags to set.
    pub tags: Vec<String>,
    /// Whether Steam may apply admin-only tags.
    pub allow_admin_tags: bool,
}

/// Mature-content descriptor used by Workshop item updates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SteamworksUgcContentDescriptor {
    /// Some nudity or sexual content.
    NudityOrSexualContent,
    /// Frequent violence or gore.
    FrequentViolenceOrGore,
    /// Adult-only sexual content.
    AdultOnlySexualContent,
    /// Frequent nudity or sexual content.
    GratuitousSexualContent,
    /// General mature content.
    AnyMatureContent,
}

impl From<SteamworksUgcContentDescriptor> for steamworks::UGCContentDescriptorID {
    fn from(value: SteamworksUgcContentDescriptor) -> Self {
        match value {
            SteamworksUgcContentDescriptor::NudityOrSexualContent => Self::NudityOrSexualContent,
            SteamworksUgcContentDescriptor::FrequentViolenceOrGore => Self::FrequentViolenceOrGore,
            SteamworksUgcContentDescriptor::AdultOnlySexualContent => Self::AdultOnlySexualContent,
            SteamworksUgcContentDescriptor::GratuitousSexualContent => {
                Self::GratuitousSexualContent
            }
            SteamworksUgcContentDescriptor::AnyMatureContent => Self::AnyMatureContent,
        }
    }
}

/// Options applied to one Workshop item update before submission.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SteamworksUgcItemUpdate {
    /// New title.
    pub title: Option<String>,
    /// New description.
    pub description: Option<String>,
    /// Update language.
    pub language: Option<String>,
    /// Preview image path.
    pub preview_path: Option<PathBuf>,
    /// Content directory path.
    pub content_path: Option<PathBuf>,
    /// Developer metadata.
    pub metadata: Option<String>,
    /// Item visibility.
    pub visibility: Option<steamworks::PublishedFileVisibility>,
    /// Replacement tag list.
    pub tags: Option<SteamworksUgcItemUpdateTags>,
    /// Key/value tags to add.
    pub add_key_value_tags: Vec<(String, String)>,
    /// Key/value tag keys to remove.
    pub remove_key_value_tags: Vec<String>,
    /// Whether all key/value tags should be removed before adding requested tags.
    pub remove_all_key_value_tags: bool,
    /// Content descriptors to add.
    pub add_content_descriptors: Vec<SteamworksUgcContentDescriptor>,
    /// Content descriptors to remove.
    pub remove_content_descriptors: Vec<SteamworksUgcContentDescriptor>,
    /// Optional change note sent with the update submission.
    pub change_note: Option<String>,
}

impl SteamworksUgcItemUpdate {
    /// Creates an empty item update.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the item title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the item description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the update language.
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Sets the preview image path.
    pub fn with_preview_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.preview_path = Some(path.into());
        self
    }

    /// Sets the content directory path.
    pub fn with_content_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.content_path = Some(path.into());
        self
    }

    /// Sets developer metadata.
    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }

    /// Sets item visibility.
    pub fn with_visibility(mut self, visibility: steamworks::PublishedFileVisibility) -> Self {
        self.visibility = Some(visibility);
        self
    }

    /// Replaces item tags.
    pub fn with_tags<I, S>(mut self, tags: I, allow_admin_tags: bool) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tags = Some(SteamworksUgcItemUpdateTags {
            tags: tags.into_iter().map(Into::into).collect(),
            allow_admin_tags,
        });
        self
    }

    /// Adds one key/value tag.
    pub fn with_key_value_tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.add_key_value_tags.push((key.into(), value.into()));
        self
    }

    /// Removes all key/value tags with the given key.
    pub fn with_removed_key_value_tag(mut self, key: impl Into<String>) -> Self {
        self.remove_key_value_tags.push(key.into());
        self
    }

    /// Removes all key/value tags before applying added key/value tags.
    pub fn with_remove_all_key_value_tags(mut self) -> Self {
        self.remove_all_key_value_tags = true;
        self
    }

    /// Adds one content descriptor.
    pub fn with_added_content_descriptor(
        mut self,
        descriptor: SteamworksUgcContentDescriptor,
    ) -> Self {
        self.add_content_descriptors.push(descriptor);
        self
    }

    /// Removes one content descriptor.
    pub fn with_removed_content_descriptor(
        mut self,
        descriptor: SteamworksUgcContentDescriptor,
    ) -> Self {
        self.remove_content_descriptors.push(descriptor);
        self
    }

    /// Sets the change note submitted with this update.
    pub fn with_change_note(mut self, change_note: impl Into<String>) -> Self {
        self.change_note = Some(change_note.into());
        self
    }
}

/// Progress snapshot for a submitted Workshop item update.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUgcItemUpdateProgress {
    /// Plugin request ID returned by the update submission.
    pub request_id: u64,
    /// Current update status.
    pub status: steamworks::UpdateStatus,
    /// Bytes processed so far.
    pub processed_bytes: u64,
    /// Total bytes expected by Steam.
    pub total_bytes: u64,
}
