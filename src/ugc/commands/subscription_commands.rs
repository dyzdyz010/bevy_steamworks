use super::super::{
    async_results::SteamworksUgcAsyncResults, SteamworksUgcCommand, SteamworksUgcError,
    SteamworksUgcOperation, SteamworksUgcResult,
};

pub(super) fn subscribe_item(
    ugc: &steamworks::UGC,
    async_results: &SteamworksUgcAsyncResults,
    request_id: u64,
    item: steamworks::PublishedFileId,
) -> SteamworksUgcOperation {
    let async_results = async_results.clone();
    let command = SteamworksUgcCommand::SubscribeItem { item };
    ugc.subscribe_item(item, move |result| {
        async_results.push(match result {
            Ok(()) => {
                SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemSubscribed { request_id, item })
            }
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
    SteamworksUgcOperation::ItemSubscribeRequested { request_id, item }
}

pub(super) fn unsubscribe_item(
    ugc: &steamworks::UGC,
    async_results: &SteamworksUgcAsyncResults,
    request_id: u64,
    item: steamworks::PublishedFileId,
) -> SteamworksUgcOperation {
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
    SteamworksUgcOperation::ItemUnsubscribeRequested { request_id, item }
}

pub(super) fn delete_item(
    ugc: &steamworks::UGC,
    async_results: &SteamworksUgcAsyncResults,
    request_id: u64,
    item: steamworks::PublishedFileId,
) -> SteamworksUgcOperation {
    let async_results = async_results.clone();
    let command = SteamworksUgcCommand::DeleteItem { item };
    ugc.delete_item(item, move |result| {
        async_results.push(match result {
            Ok(()) => {
                SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemDeleted { request_id, item })
            }
            Err(source) => SteamworksUgcResult::Err {
                command,
                error: SteamworksUgcError::steam_error("ugc.delete_item", Some(request_id), source),
            },
        });
    });
    SteamworksUgcOperation::ItemDeleteRequested { request_id, item }
}

pub(super) fn start_playtime_tracking(
    ugc: &steamworks::UGC,
    async_results: &SteamworksUgcAsyncResults,
    request_id: u64,
    items: Vec<steamworks::PublishedFileId>,
) -> SteamworksUgcOperation {
    let async_results = async_results.clone();
    let command = SteamworksUgcCommand::StartPlaytimeTracking {
        items: items.clone(),
    };
    let callback_items = items.clone();
    ugc.start_playtime_tracking(&items, move |result| {
        async_results.push(match result {
            Ok(()) => SteamworksUgcResult::Ok(SteamworksUgcOperation::PlaytimeTrackingStarted {
                request_id,
                items: callback_items,
            }),
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
    SteamworksUgcOperation::PlaytimeTrackingStartRequested { request_id, items }
}

pub(super) fn stop_playtime_tracking(
    ugc: &steamworks::UGC,
    async_results: &SteamworksUgcAsyncResults,
    request_id: u64,
    items: Vec<steamworks::PublishedFileId>,
) -> SteamworksUgcOperation {
    let async_results = async_results.clone();
    let command = SteamworksUgcCommand::StopPlaytimeTracking {
        items: items.clone(),
    };
    let callback_items = items.clone();
    ugc.stop_playtime_tracking(&items, move |result| {
        async_results.push(match result {
            Ok(()) => SteamworksUgcResult::Ok(SteamworksUgcOperation::PlaytimeTrackingStopped {
                request_id,
                items: callback_items,
            }),
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
    SteamworksUgcOperation::PlaytimeTrackingStopRequested { request_id, items }
}

pub(super) fn stop_playtime_tracking_for_all_items(
    ugc: &steamworks::UGC,
    async_results: &SteamworksUgcAsyncResults,
    request_id: u64,
) -> SteamworksUgcOperation {
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
    SteamworksUgcOperation::PlaytimeTrackingForAllItemsStopRequested { request_id }
}
