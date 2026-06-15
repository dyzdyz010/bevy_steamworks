use super::{
    SteamworksUgcContentDescriptor, SteamworksUgcItemDetails, SteamworksUgcQueryOptions,
    SteamworksUgcQueryResults, SteamworksUgcStatistic,
};

pub(super) fn snapshot_query_results(
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
        content_descriptors: results
            .content_descriptor(index)
            .into_iter()
            .map(SteamworksUgcContentDescriptor::from)
            .collect(),
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
