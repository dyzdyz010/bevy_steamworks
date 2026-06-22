use bevy_ecs::prelude::Resource;

use super::*;

mod accessors;
mod operations;

pub(in crate::ugc) const STEAMWORKS_UGC_STATE_ITEM_CACHE_LIMIT: usize = 1_024;

/// Runtime state for [`crate::SteamworksUgcPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksUgcState {
    last_error: Option<SteamworksUgcError>,
    subscribed_items: Vec<steamworks::PublishedFileId>,
    item_details: Vec<SteamworksUgcItemDetails>,
    item_states: Vec<SteamworksUgcItemStateInfo>,
    item_download_infos: Vec<SteamworksUgcItemDownloadInfoResult>,
    item_install_infos: Vec<SteamworksUgcItemInstallInfoResult>,
    last_query: Option<SteamworksUgcQueryResults>,
    last_query_total: Option<SteamworksUgcQueryTotal>,
    last_query_ids: Option<SteamworksUgcQueryIds>,
    last_item_state: Option<SteamworksUgcItemStateInfo>,
    last_item_download_info: Option<SteamworksUgcItemDownloadInfoResult>,
    last_item_install_info: Option<SteamworksUgcItemInstallInfoResult>,
    last_item_update_progress: Option<SteamworksUgcItemUpdateProgress>,
    last_download_item_result: Option<SteamworksUgcDownloadItemResult>,
    last_game_server_workshop_init: Option<SteamworksUgcGameServerWorkshopInit>,
    active_item_updates: usize,
    submitted_downloads: u64,
    successful_async_operations: u64,
    failed_async_operations: u64,
    next_request_id: u64,
}

pub(super) fn upsert_item_details(
    items: &mut Vec<SteamworksUgcItemDetails>,
    details: SteamworksUgcItemDetails,
) {
    if let Some(existing) = items
        .iter_mut()
        .find(|existing| existing.published_file_id == details.published_file_id)
    {
        *existing = details;
    } else {
        items.push(details);
        trim_cache(items);
    }
}

pub(super) fn upsert_item_state(
    states: &mut Vec<SteamworksUgcItemStateInfo>,
    info: SteamworksUgcItemStateInfo,
) {
    if let Some(existing) = states
        .iter_mut()
        .find(|existing| existing.item == info.item)
    {
        *existing = info;
    } else {
        states.push(info);
        trim_cache(states);
    }
}

pub(super) fn upsert_item_download_info(
    infos: &mut Vec<SteamworksUgcItemDownloadInfoResult>,
    info: SteamworksUgcItemDownloadInfoResult,
) {
    if let Some(existing) = infos.iter_mut().find(|existing| existing.item == info.item) {
        *existing = info;
    } else {
        infos.push(info);
        trim_cache(infos);
    }
}

pub(super) fn upsert_item_install_info(
    infos: &mut Vec<SteamworksUgcItemInstallInfoResult>,
    info: SteamworksUgcItemInstallInfoResult,
) {
    if let Some(existing) = infos.iter_mut().find(|existing| existing.item == info.item) {
        *existing = info;
    } else {
        infos.push(info);
        trim_cache(infos);
    }
}

pub(super) fn remove_item_cache(state: &mut SteamworksUgcState, item: steamworks::PublishedFileId) {
    state
        .item_details
        .retain(|details| details.published_file_id != item);
    state.item_states.retain(|info| info.item != item);
    state.item_download_infos.retain(|info| info.item != item);
    state.item_install_infos.retain(|info| info.item != item);
}

fn trim_cache<T>(items: &mut Vec<T>) {
    if items.len() > STEAMWORKS_UGC_STATE_ITEM_CACHE_LIMIT {
        let overflow = items.len() - STEAMWORKS_UGC_STATE_ITEM_CACHE_LIMIT;
        items.drain(0..overflow);
    }
}
