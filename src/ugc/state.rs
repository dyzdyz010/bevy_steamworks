use bevy_ecs::prelude::Resource;

use super::*;

mod accessors;
mod operations;

/// Runtime state for [`crate::SteamworksUgcPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksUgcState {
    last_error: Option<SteamworksUgcError>,
    subscribed_items: Vec<steamworks::PublishedFileId>,
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
