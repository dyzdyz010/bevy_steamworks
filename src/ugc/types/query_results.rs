use super::SteamworksUgcContentDescriptor;

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

/// Owned result count returned by a total-only UGC query.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUgcQueryTotal {
    /// Total matching result count across all pages.
    pub total_results: u32,
}

/// Owned item IDs returned by an ID-only UGC query for the submitted page/result set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUgcQueryIds {
    /// Matching Workshop item IDs returned by Steam.
    pub items: Vec<steamworks::PublishedFileId>,
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
    /// Mature-content descriptors returned by Steam.
    pub content_descriptors: Vec<SteamworksUgcContentDescriptor>,
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
