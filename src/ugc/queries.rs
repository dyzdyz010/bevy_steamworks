use super::{
    validation::validate_query_options, SteamworksUgcError, SteamworksUgcQuery,
    SteamworksUgcQueryOptions,
};

pub(super) fn create_query(
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

pub(super) fn apply_query_options(
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
    query = query.set_return_additional_previews(options.return_additional_previews);
    query = query.set_return_only_ids(options.return_only_ids);
    query = query.set_return_total_only(options.return_total_only);

    Ok(query)
}
