use bevy_ecs::message::MessageWriter;

use crate::SteamworksClient;

use super::{
    SteamworksStatsOperation, SteamworksStatsResult, SteamworksStatsSettings, SteamworksStatsState,
};

pub(super) fn should_submit_store(
    settings: &SteamworksStatsSettings,
    state: &SteamworksStatsState,
) -> bool {
    state.pending_store && (settings.auto_store || state.force_store)
}

pub(super) fn request_current_user_stats(
    client: &SteamworksClient,
    state: &mut SteamworksStatsState,
    results: &mut MessageWriter<SteamworksStatsResult>,
) {
    let steam_id = client.user().steam_id();
    client.user_stats().request_user_stats(steam_id.raw());
    state.current_user_stats_requested = true;
    results.write(SteamworksStatsResult::Ok(
        SteamworksStatsOperation::CurrentUserStatsRequested { steam_id },
    ));
    tracing::debug!(
        target: "bevy_steamworks",
        steam_id = steam_id.raw(),
        "requested Steamworks stats for current user"
    );
}
