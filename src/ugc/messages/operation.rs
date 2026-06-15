use super::super::{
    SteamworksUgcDownloadItemResult, SteamworksUgcItemDownloadInfoResult,
    SteamworksUgcItemInstallInfoResult, SteamworksUgcItemStateInfo, SteamworksUgcItemUpdate,
    SteamworksUgcItemUpdateProgress, SteamworksUgcQuery, SteamworksUgcQueryIds,
    SteamworksUgcQueryResults, SteamworksUgcQueryTotal,
};

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
    /// A total-only query was submitted.
    QueryTotalRequested {
        /// Plugin request ID.
        request_id: u64,
        /// Query submitted.
        query: SteamworksUgcQuery,
    },
    /// A total-only query completed.
    QueryTotalCompleted {
        /// Plugin request ID.
        request_id: u64,
        /// Query submitted.
        query: SteamworksUgcQuery,
        /// Total result count.
        total: SteamworksUgcQueryTotal,
    },
    /// An ID-only query was submitted.
    QueryIdsRequested {
        /// Plugin request ID.
        request_id: u64,
        /// Query submitted.
        query: SteamworksUgcQuery,
    },
    /// An ID-only query completed.
    QueryIdsCompleted {
        /// Plugin request ID.
        request_id: u64,
        /// Query submitted.
        query: SteamworksUgcQuery,
        /// Returned item IDs.
        ids: SteamworksUgcQueryIds,
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
