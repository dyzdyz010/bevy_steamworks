use super::{
    SteamworksAchievementDisplayAttribute, SteamworksAchievementGlobalPercentage,
    SteamworksAchievementInfo, SteamworksLeaderboardId, SteamworksLeaderboardInfo,
    STEAMWORKS_STATS_STATE_CACHE_LIMIT,
};

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
        trim_cache(values);
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
        trim_cache(leaderboards);
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
        trim_cache(leaderboards);
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

fn trim_cache<T>(items: &mut Vec<T>) {
    if items.len() > STEAMWORKS_STATS_STATE_CACHE_LIMIT {
        let overflow = items.len() - STEAMWORKS_STATS_STATE_CACHE_LIMIT;
        items.drain(0..overflow);
    }
}
