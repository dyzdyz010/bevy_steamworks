use bevy_ecs::prelude::Resource;

use super::{
    messages::{SteamworksAppsError, SteamworksAppsOperation},
    types::SteamworksCurrentAppInfo,
};

mod accessors;
mod operations;

pub(in crate::apps) const STEAMWORKS_APPS_STATE_CACHE_LIMIT: usize = 1_024;

/// Runtime state for [`super::SteamworksAppsPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksAppsState {
    last_error: Option<SteamworksAppsError>,
    current_app_info: Option<SteamworksCurrentAppInfo>,
    subscribed: Option<bool>,
    installed_apps: Vec<(steamworks::AppId, bool)>,
    installed_dlcs: Vec<(steamworks::AppId, bool)>,
    subscribed_apps: Vec<(steamworks::AppId, bool)>,
    subscribed_from_free_weekend: Option<bool>,
    vac_banned: Option<bool>,
    cybercafe: Option<bool>,
    low_violence: Option<bool>,
    app_build_id: Option<i32>,
    app_install_dirs: Vec<(steamworks::AppId, String)>,
    app_owner: Option<steamworks::SteamId>,
    available_game_languages: Option<Vec<String>>,
    current_game_language: Option<String>,
    current_beta_name: Option<Option<String>>,
    launch_command_line: Option<String>,
    launch_query_params: Vec<(String, String)>,
    new_url_launch_parameters_count: u64,
}

pub(super) fn app_value<T>(
    values: &[(steamworks::AppId, T)],
    app_id: steamworks::AppId,
) -> Option<&T> {
    values
        .iter()
        .find_map(|(known_app_id, value)| (*known_app_id == app_id).then_some(value))
}

pub(super) fn upsert_app_value<T>(
    values: &mut Vec<(steamworks::AppId, T)>,
    app_id: steamworks::AppId,
    value: T,
) {
    if let Some((_, known_value)) = values
        .iter_mut()
        .find(|(known_app_id, _)| *known_app_id == app_id)
    {
        *known_value = value;
    } else {
        values.push((app_id, value));
        trim_cache(values);
    }
}

pub(super) fn trim_cache<T>(values: &mut Vec<T>) {
    if values.len() > STEAMWORKS_APPS_STATE_CACHE_LIMIT {
        let overflow = values.len() - STEAMWORKS_APPS_STATE_CACHE_LIMIT;
        values.drain(0..overflow);
    }
}
