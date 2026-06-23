use crate::ugc::*;

impl SteamworksUgcState {
    /// Returns the most recent UGC query result set.
    pub fn last_query(&self) -> Option<&SteamworksUgcQueryResults> {
        self.last_query.as_ref()
    }

    /// Returns bounded submitted UGC query snapshots by request ID.
    pub fn query_requests(&self) -> &[SteamworksUgcQueryRequest] {
        &self.query_requests
    }

    /// Returns the submitted UGC query snapshot for a request ID.
    pub fn query_request(&self, request_id: u64) -> Option<&SteamworksUgcQueryRequest> {
        self.query_requests
            .iter()
            .find(|request| request.request_id == request_id)
    }

    /// Returns bounded completed full UGC query snapshots by request ID.
    pub fn query_results(&self) -> &[SteamworksUgcQueryResult] {
        &self.query_results
    }

    /// Returns the completed full UGC query snapshot for a request ID.
    pub fn query_result(&self, request_id: u64) -> Option<&SteamworksUgcQueryResult> {
        self.query_results
            .iter()
            .find(|result| result.request_id == request_id)
    }

    /// Returns items from a completed full UGC query snapshot.
    pub fn query_result_items(&self, request_id: u64) -> Option<&[SteamworksUgcItemDetails]> {
        self.query_result(request_id)
            .map(|result| result.results.items.as_slice())
    }

    /// Returns the returned item count from a completed full UGC query snapshot.
    pub fn query_result_item_count(&self, request_id: u64) -> Option<usize> {
        self.query_result(request_id)
            .map(|result| result.results.items.len())
    }

    /// Returns the total matching count from a completed full UGC query snapshot.
    pub fn query_result_total_count(&self, request_id: u64) -> Option<u32> {
        self.query_result(request_id)
            .map(|result| result.results.total_results)
    }

    /// Returns whether a completed full UGC query snapshot was served from Steam's cache.
    pub fn query_result_was_cached(&self, request_id: u64) -> Option<bool> {
        self.query_result(request_id)
            .map(|result| result.results.was_cached)
    }

    /// Returns the returned item count from the most recent full UGC query.
    pub fn last_query_item_count(&self) -> Option<usize> {
        self.last_query.as_ref().map(|results| results.items.len())
    }

    /// Returns the total matching count from the most recent full UGC query.
    pub fn last_query_total_count(&self) -> Option<u32> {
        self.last_query
            .as_ref()
            .map(|results| results.total_results)
    }

    /// Returns whether the most recent full UGC query was served from Steam's cache.
    pub fn last_query_was_cached(&self) -> Option<bool> {
        self.last_query.as_ref().map(|results| results.was_cached)
    }

    /// Returns the most recent UGC total-only query result.
    pub fn last_query_total(&self) -> Option<&SteamworksUgcQueryTotal> {
        self.last_query_total.as_ref()
    }

    /// Returns bounded completed total-only UGC query snapshots by request ID.
    pub fn query_total_results(&self) -> &[SteamworksUgcQueryTotalResult] {
        &self.query_total_results
    }

    /// Returns the completed total-only UGC query snapshot for a request ID.
    pub fn query_total_result(&self, request_id: u64) -> Option<&SteamworksUgcQueryTotalResult> {
        self.query_total_results
            .iter()
            .find(|result| result.request_id == request_id)
    }

    /// Returns the total matching count from a completed total-only UGC query snapshot.
    pub fn query_total_count(&self, request_id: u64) -> Option<u32> {
        self.query_total_result(request_id)
            .map(|result| result.total.total_results)
    }

    /// Returns the most recent UGC ID-only query result.
    pub fn last_query_ids(&self) -> Option<&SteamworksUgcQueryIds> {
        self.last_query_ids.as_ref()
    }

    /// Returns bounded completed ID-only UGC query snapshots by request ID.
    pub fn query_ids_results(&self) -> &[SteamworksUgcQueryIdsResult] {
        &self.query_ids_results
    }

    /// Returns the completed ID-only UGC query snapshot for a request ID.
    pub fn query_ids_result(&self, request_id: u64) -> Option<&SteamworksUgcQueryIdsResult> {
        self.query_ids_results
            .iter()
            .find(|result| result.request_id == request_id)
    }

    /// Returns item IDs from a completed ID-only UGC query snapshot.
    pub fn query_ids_items(&self, request_id: u64) -> Option<&[steamworks::PublishedFileId]> {
        self.query_ids_result(request_id)
            .map(|result| result.ids.items.as_slice())
    }

    /// Returns the item count from a completed ID-only UGC query snapshot.
    pub fn query_ids_item_count(&self, request_id: u64) -> Option<usize> {
        self.query_ids_result(request_id)
            .map(|result| result.ids.items.len())
    }

    /// Returns the item count from the most recent ID-only UGC query.
    pub fn last_query_ids_count(&self) -> Option<usize> {
        self.last_query_ids.as_ref().map(|ids| ids.items.len())
    }
}
