use super::super::{
    async_results::SteamworksUgcAsyncResults, queries::apply_query_options, queries::create_query,
    snapshots::snapshot_query_results, SteamworksUgcCommand, SteamworksUgcError,
    SteamworksUgcOperation, SteamworksUgcQuery, SteamworksUgcQueryIds, SteamworksUgcQueryOptions,
    SteamworksUgcQueryTotal, SteamworksUgcResult,
};

pub(super) fn query(
    ugc: &steamworks::UGC,
    async_results: &SteamworksUgcAsyncResults,
    request_id: u64,
    query: SteamworksUgcQuery,
) -> Result<SteamworksUgcOperation, SteamworksUgcError> {
    let options = query.options().clone();
    let query_handle = create_query(ugc, &query)?;
    let query_handle = apply_query_options(query_handle, &options)?;
    let async_results = async_results.clone();
    let command = SteamworksUgcCommand::Query {
        query: query.clone(),
    };
    let callback_query = query.clone();
    query_handle.fetch(move |result| {
        async_results.push(match result {
            Ok(results) => SteamworksUgcResult::Ok(SteamworksUgcOperation::QueryCompleted {
                request_id,
                query: callback_query,
                results: snapshot_query_results(&results, &options),
            }),
            Err(source) => SteamworksUgcResult::Err {
                command,
                error: SteamworksUgcError::steam_error("ugc.query.fetch", Some(request_id), source),
            },
        });
    });
    Ok(SteamworksUgcOperation::QueryRequested { request_id, query })
}

pub(super) fn query_total(
    ugc: &steamworks::UGC,
    async_results: &SteamworksUgcAsyncResults,
    request_id: u64,
    query: SteamworksUgcQuery,
) -> Result<SteamworksUgcOperation, SteamworksUgcError> {
    let options = query_options_without_payload_shape_flags(&query);
    let query_handle = create_query(ugc, &query)?;
    let query_handle = apply_query_options(query_handle, &options)?;
    let async_results = async_results.clone();
    let command = SteamworksUgcCommand::QueryTotal {
        query: query.clone(),
    };
    let callback_query = query.clone();
    query_handle.fetch_total(move |result| {
        async_results.push(match result {
            Ok(total_results) => {
                SteamworksUgcResult::Ok(SteamworksUgcOperation::QueryTotalCompleted {
                    request_id,
                    query: callback_query.clone(),
                    total: SteamworksUgcQueryTotal { total_results },
                })
            }
            Err(source) => SteamworksUgcResult::Err {
                command: command.clone(),
                error: SteamworksUgcError::steam_error(
                    "ugc.query.fetch_total",
                    Some(request_id),
                    source,
                ),
            },
        });
    });
    Ok(SteamworksUgcOperation::QueryTotalRequested { request_id, query })
}

pub(super) fn query_ids(
    ugc: &steamworks::UGC,
    async_results: &SteamworksUgcAsyncResults,
    request_id: u64,
    query: SteamworksUgcQuery,
) -> Result<SteamworksUgcOperation, SteamworksUgcError> {
    let options = query_options_without_payload_shape_flags(&query);
    let query_handle = create_query(ugc, &query)?;
    let query_handle = apply_query_options(query_handle, &options)?;
    let async_results = async_results.clone();
    let command = SteamworksUgcCommand::QueryIds {
        query: query.clone(),
    };
    let callback_query = query.clone();
    query_handle.fetch_ids(move |result| {
        async_results.push(match result {
            Ok(items) => SteamworksUgcResult::Ok(SteamworksUgcOperation::QueryIdsCompleted {
                request_id,
                query: callback_query.clone(),
                ids: SteamworksUgcQueryIds { items },
            }),
            Err(source) => SteamworksUgcResult::Err {
                command: command.clone(),
                error: SteamworksUgcError::steam_error(
                    "ugc.query.fetch_ids",
                    Some(request_id),
                    source,
                ),
            },
        });
    });
    Ok(SteamworksUgcOperation::QueryIdsRequested { request_id, query })
}

pub(super) fn query_options_without_payload_shape_flags(
    query: &SteamworksUgcQuery,
) -> SteamworksUgcQueryOptions {
    let mut options = query.options().clone();
    options.return_only_ids = false;
    options.return_total_only = false;
    options
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn specialized_query_commands_ignore_payload_shape_option_flags() {
        let query = SteamworksUgcQuery::item(steamworks::PublishedFileId(1)).with_options(
            SteamworksUgcQueryOptions::new()
                .with_return_only_ids(true)
                .with_return_total_only(true),
        );

        let options = query_options_without_payload_shape_flags(&query);

        assert!(!options.return_only_ids);
        assert!(!options.return_total_only);
    }
}
