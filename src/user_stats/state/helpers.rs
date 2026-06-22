use super::{
    SteamworksAchievementDisplayAttribute, SteamworksAchievementGlobalPercentage,
    SteamworksAchievementInfo, SteamworksGlobalStatHistory, SteamworksGlobalStatValue,
    SteamworksLeaderboardEntriesDownloadRequest, SteamworksLeaderboardEntriesDownloadResult,
    SteamworksLeaderboardId, SteamworksLeaderboardInfo, SteamworksLeaderboardScoreUploadRequest,
    SteamworksLeaderboardScoreUploadResult, STEAMWORKS_STATS_STATE_CACHE_LIMIT,
};
use crate::cache::trim_oldest;

pub(super) fn named_value<'a, T>(values: &'a [(String, T)], name: &str) -> Option<&'a T> {
    values
        .iter()
        .find_map(|(known_name, value)| (known_name == name).then_some(value))
}

pub(super) fn upsert_named_value<T>(values: &mut Vec<(String, T)>, name: String, value: T) {
    if let Some((_, known_value)) = values
        .iter_mut()
        .find(|(known_name, _)| known_name == &name)
    {
        *known_value = value;
    } else {
        values.push((name, value));
        trim_oldest(values, STEAMWORKS_STATS_STATE_CACHE_LIMIT);
    }
}

pub(super) fn update_achievement<F>(
    achievements: &mut Vec<SteamworksAchievementInfo>,
    api_name: &str,
    update: F,
) where
    F: FnOnce(&mut SteamworksAchievementInfo),
{
    if let Some(achievement) = achievements
        .iter_mut()
        .find(|achievement| achievement.api_name == api_name)
    {
        update(achievement);
    } else {
        let mut achievement = SteamworksAchievementInfo {
            api_name: api_name.to_owned(),
            ..Default::default()
        };
        update(&mut achievement);
        achievements.push(achievement);
    }
}

pub(super) fn merge_achievement_info(
    achievements: &mut Vec<SteamworksAchievementInfo>,
    info: &SteamworksAchievementInfo,
) {
    update_achievement(achievements, &info.api_name, |achievement| {
        if info.display_name.is_some() {
            achievement.display_name.clone_from(&info.display_name);
        }
        if info.description.is_some() {
            achievement.description.clone_from(&info.description);
        }
        if info.hidden.is_some() {
            achievement.hidden = info.hidden;
        }
        if info.achieved.is_some() {
            achievement.achieved = info.achieved;
        }
        if info.unlock_time.is_some() {
            achievement.unlock_time = info.unlock_time;
        }
    });
}

pub(super) fn merge_achievement_display_attributes(
    attributes: &mut Vec<SteamworksAchievementDisplayAttribute>,
    info: &SteamworksAchievementInfo,
) {
    if let Some(display_name) = &info.display_name {
        upsert_achievement_display_attribute(
            attributes,
            info.api_name.clone(),
            "name".to_owned(),
            display_name.clone(),
        );
    }
    if let Some(description) = &info.description {
        upsert_achievement_display_attribute(
            attributes,
            info.api_name.clone(),
            "desc".to_owned(),
            description.clone(),
        );
    }
    if let Some(hidden) = info.hidden {
        upsert_achievement_display_attribute(
            attributes,
            info.api_name.clone(),
            "hidden".to_owned(),
            if hidden { "1" } else { "0" }.to_owned(),
        );
    }
}

pub(super) fn upsert_achievement_display_attribute(
    attributes: &mut Vec<SteamworksAchievementDisplayAttribute>,
    api_name: String,
    key: String,
    value: String,
) {
    if let Some(attribute) = attributes
        .iter_mut()
        .find(|attribute| attribute.api_name == api_name && attribute.key == key)
    {
        attribute.value = value;
    } else {
        attributes.push(SteamworksAchievementDisplayAttribute {
            api_name,
            key,
            value,
        });
    }
}

pub(super) fn upsert_global_achievement_percentage(
    percentages: &mut Vec<SteamworksAchievementGlobalPercentage>,
    api_name: String,
    percent: f32,
) {
    if let Some(known_percentage) = percentages
        .iter_mut()
        .find(|known_percentage| known_percentage.api_name == api_name)
    {
        known_percentage.percent = percent;
    } else {
        percentages.push(SteamworksAchievementGlobalPercentage { api_name, percent });
    }
}

pub(super) fn upsert_global_stat_value<T>(
    values: &mut Vec<SteamworksGlobalStatValue<T>>,
    value: SteamworksGlobalStatValue<T>,
) {
    if let Some(known_value) = values
        .iter_mut()
        .find(|known_value| known_value.name == value.name)
    {
        *known_value = value;
    } else {
        values.push(value);
        trim_oldest(values, STEAMWORKS_STATS_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_global_stat_history<T>(
    histories: &mut Vec<SteamworksGlobalStatHistory<T>>,
    history: SteamworksGlobalStatHistory<T>,
) {
    if let Some(known_history) = histories
        .iter_mut()
        .find(|known_history| known_history.name == history.name)
    {
        *known_history = history;
    } else {
        histories.push(history);
        trim_oldest(histories, STEAMWORKS_STATS_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_leaderboard_id(
    leaderboards: &mut Vec<(String, SteamworksLeaderboardId)>,
    name: String,
    leaderboard: SteamworksLeaderboardId,
) {
    if let Some((_, known_leaderboard)) = leaderboards
        .iter_mut()
        .find(|(known_name, _)| known_name == &name)
    {
        *known_leaderboard = leaderboard;
    } else {
        leaderboards.push((name, leaderboard));
        trim_oldest(leaderboards, STEAMWORKS_STATS_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_leaderboard_info(
    leaderboards: &mut Vec<SteamworksLeaderboardInfo>,
    info: SteamworksLeaderboardInfo,
) {
    if let Some(known_info) = leaderboards
        .iter_mut()
        .find(|known_info| known_info.leaderboard == info.leaderboard)
    {
        *known_info = info;
    } else {
        leaderboards.push(info);
        trim_oldest(leaderboards, STEAMWORKS_STATS_STATE_CACHE_LIMIT);
    }
}

pub(super) fn remove_leaderboard_id(
    leaderboards: &mut Vec<(String, SteamworksLeaderboardId)>,
    leaderboard: SteamworksLeaderboardId,
) {
    leaderboards.retain(|(_, known_leaderboard)| *known_leaderboard != leaderboard);
}

pub(super) fn remove_leaderboard_info(
    leaderboards: &mut Vec<SteamworksLeaderboardInfo>,
    leaderboard: SteamworksLeaderboardId,
) {
    leaderboards.retain(|info| info.leaderboard != leaderboard);
}

pub(super) fn upsert_leaderboard_score_upload_request(
    requests: &mut Vec<SteamworksLeaderboardScoreUploadRequest>,
    request: SteamworksLeaderboardScoreUploadRequest,
) {
    if let Some(known_request) = requests
        .iter_mut()
        .find(|known_request| known_request.leaderboard == request.leaderboard)
    {
        *known_request = request;
    } else {
        requests.push(request);
        trim_oldest(requests, STEAMWORKS_STATS_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_leaderboard_score_upload_result(
    results: &mut Vec<SteamworksLeaderboardScoreUploadResult>,
    result: SteamworksLeaderboardScoreUploadResult,
) {
    if let Some(known_result) = results
        .iter_mut()
        .find(|known_result| known_result.leaderboard == result.leaderboard)
    {
        *known_result = result;
    } else {
        results.push(result);
        trim_oldest(results, STEAMWORKS_STATS_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_leaderboard_entries_download_request(
    requests: &mut Vec<SteamworksLeaderboardEntriesDownloadRequest>,
    request: SteamworksLeaderboardEntriesDownloadRequest,
) {
    if let Some(known_request) = requests
        .iter_mut()
        .find(|known_request| known_request.leaderboard == request.leaderboard)
    {
        *known_request = request;
    } else {
        requests.push(request);
        trim_oldest(requests, STEAMWORKS_STATS_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_leaderboard_entries_download_result(
    results: &mut Vec<SteamworksLeaderboardEntriesDownloadResult>,
    result: SteamworksLeaderboardEntriesDownloadResult,
) {
    if let Some(known_result) = results
        .iter_mut()
        .find(|known_result| known_result.leaderboard == result.leaderboard)
    {
        *known_result = result;
    } else {
        results.push(result);
        trim_oldest(results, STEAMWORKS_STATS_STATE_CACHE_LIMIT);
    }
}

pub(super) fn remove_leaderboard_result_caches(
    score_upload_requests: &mut Vec<SteamworksLeaderboardScoreUploadRequest>,
    score_upload_results: &mut Vec<SteamworksLeaderboardScoreUploadResult>,
    entries_download_requests: &mut Vec<SteamworksLeaderboardEntriesDownloadRequest>,
    entries_download_results: &mut Vec<SteamworksLeaderboardEntriesDownloadResult>,
    leaderboard: SteamworksLeaderboardId,
) {
    score_upload_requests.retain(|request| request.leaderboard != leaderboard);
    score_upload_results.retain(|result| result.leaderboard != leaderboard);
    entries_download_requests.retain(|request| request.leaderboard != leaderboard);
    entries_download_results.retain(|result| result.leaderboard != leaderboard);
}
