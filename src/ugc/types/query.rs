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
    /// Whether Steam should return additional previews.
    pub return_additional_previews: bool,
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

    /// Includes additional previews.
    ///
    /// The upstream `steamworks` crate currently exposes the request flag but not a safe
    /// accessor for additional preview rows, so this only affects the Steam query request.
    pub fn with_additional_previews(mut self, include: bool) -> Self {
        self.return_additional_previews = include;
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
    pub fn items<I, T>(items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<steamworks::PublishedFileId>,
    {
        Self::Items {
            items: items.into_iter().map(Into::into).collect(),
            options: SteamworksUgcQueryOptions::new(),
        }
    }

    /// Creates a single-item details query.
    pub fn item(item: impl Into<steamworks::PublishedFileId>) -> Self {
        Self::items([item.into()])
    }

    /// Replaces the query options.
    pub fn with_options(mut self, options: SteamworksUgcQueryOptions) -> Self {
        *self.options_mut() = options;
        self
    }

    /// Adds a required tag.
    pub fn with_required_tag(mut self, tag: impl Into<String>) -> Self {
        self.options_mut().required_tags.push(tag.into());
        self
    }

    /// Adds an excluded tag.
    pub fn with_excluded_tag(mut self, tag: impl Into<String>) -> Self {
        self.options_mut().excluded_tags.push(tag.into());
        self
    }

    /// Adds a required key/value tag.
    pub fn with_required_key_value_tag(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.options_mut()
            .required_key_value_tags
            .push((key.into(), value.into()));
        self
    }

    /// Sets whether any required tag is sufficient.
    pub fn with_match_any_tag(mut self, match_any_tag: bool) -> Self {
        self.options_mut().match_any_tag = Some(match_any_tag);
        self
    }

    /// Sets the query language.
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.options_mut().language = Some(language.into());
        self
    }

    /// Allows cached responses up to `seconds` old.
    pub fn with_allow_cached_response(mut self, seconds: u32) -> Self {
        self.options_mut().allow_cached_response_seconds = Some(seconds);
        self
    }

    /// Sets a Cloud file name filter.
    pub fn with_cloud_file_name_filter(mut self, file_name: impl Into<String>) -> Self {
        self.options_mut().cloud_file_name_filter = Some(file_name.into());
        self
    }

    /// Sets the full-text search query.
    pub fn with_search_text(mut self, search_text: impl Into<String>) -> Self {
        self.options_mut().search_text = Some(search_text.into());
        self
    }

    /// Sets ranked-by-trend days.
    pub fn with_ranked_by_trend_days(mut self, days: u32) -> Self {
        self.options_mut().ranked_by_trend_days = Some(days);
        self
    }

    /// Includes long descriptions.
    pub fn with_long_description(mut self, include: bool) -> Self {
        self.options_mut().return_long_description = include;
        self
    }

    /// Includes child item IDs.
    pub fn with_children(mut self, include: bool) -> Self {
        self.options_mut().return_children = include;
        self
    }

    /// Includes developer metadata.
    pub fn with_metadata(mut self, include: bool) -> Self {
        self.options_mut().return_metadata = include;
        self
    }

    /// Includes key/value tags.
    pub fn with_key_value_tags(mut self, include: bool) -> Self {
        self.options_mut().return_key_value_tags = include;
        self
    }

    /// Includes additional preview rows when supported by Steam.
    ///
    /// The upstream `steamworks` crate currently exposes the request flag but
    /// not a safe accessor for additional preview rows, so this only affects
    /// the Steam query request.
    pub fn with_additional_previews(mut self, include: bool) -> Self {
        self.options_mut().return_additional_previews = include;
        self
    }

    /// Returns only item IDs.
    pub fn with_return_only_ids(mut self, enabled: bool) -> Self {
        self.options_mut().return_only_ids = enabled;
        self
    }

    /// Returns only the total result count.
    pub fn with_return_total_only(mut self, enabled: bool) -> Self {
        self.options_mut().return_total_only = enabled;
        self
    }

    /// Adds a statistic to copy from each returned item.
    pub fn with_statistic(mut self, statistic: steamworks::UGCStatisticType) -> Self {
        self.options_mut().statistics.push(statistic);
        self
    }

    pub(in crate::ugc) fn options(&self) -> &SteamworksUgcQueryOptions {
        match self {
            Self::All { options, .. }
            | Self::User { options, .. }
            | Self::Items { options, .. } => options,
        }
    }

    fn options_mut(&mut self) -> &mut SteamworksUgcQueryOptions {
        match self {
            Self::All { options, .. }
            | Self::User { options, .. }
            | Self::Items { options, .. } => options,
        }
    }
}
