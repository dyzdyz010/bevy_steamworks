use super::{
    SteamworksAchievementGlobalPercentage, SteamworksAchievementIcon,
    SteamworksAchievementIconStatus, SteamworksAchievementInfo, SteamworksStatsError,
    SteamworksStatsOperation,
};

pub(super) fn list_achievement_names(
    stats: &steamworks::UserStats,
    offset: usize,
    limit: usize,
) -> Result<(usize, Vec<String>), SteamworksStatsError> {
    let names = read_all_achievement_names(stats)?;
    let total = names.len();
    let page = names.into_iter().skip(offset).take(limit).collect();

    Ok((total, page))
}

fn read_all_achievement_names(
    stats: &steamworks::UserStats,
) -> Result<Vec<String>, SteamworksStatsError> {
    stats
        .get_num_achievements()
        .map_err(|()| SteamworksStatsError::operation_failed("user_stats.get_num_achievements"))?;

    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        stats.get_achievement_names()
    }))
    .map_err(|_| SteamworksStatsError::operation_failed("user_stats.get_achievement_names"))?
    .ok_or_else(|| SteamworksStatsError::operation_failed("user_stats.get_achievement_names"))
}

pub(super) fn list_achievement_infos(
    stats: &steamworks::UserStats,
    include_display_attributes: bool,
    include_unlock_state: bool,
    offset: usize,
    limit: usize,
) -> Result<(usize, Vec<SteamworksAchievementInfo>), SteamworksStatsError> {
    let (total, names) = list_achievement_names(stats, offset, limit)?;
    let achievements = names
        .into_iter()
        .map(|api_name| {
            let mut info = SteamworksAchievementInfo {
                api_name,
                ..Default::default()
            };

            if include_display_attributes {
                info.display_name = achievement_display_attribute(stats, &info.api_name, "name")?;
                info.description = achievement_display_attribute(stats, &info.api_name, "desc")?;
                info.hidden = achievement_display_attribute(stats, &info.api_name, "hidden")?
                    .map(|value| value != "0");
            }
            if include_unlock_state {
                let (achieved, unlock_time) = stats
                    .achievement(&info.api_name)
                    .get_achievement_and_unlock_time()
                    .map_err(|()| {
                        SteamworksStatsError::operation_failed(
                            "achievement.get_achievement_and_unlock_time",
                        )
                    })?;
                info.achieved = Some(achieved);
                info.unlock_time = Some(unlock_time);
            }

            Ok(info)
        })
        .collect::<Result<Vec<_>, SteamworksStatsError>>()?;

    Ok((total, achievements))
}

pub(super) fn list_achievement_global_percentages(
    stats: &steamworks::UserStats,
    offset: usize,
    limit: usize,
) -> Result<(usize, Vec<SteamworksAchievementGlobalPercentage>), SteamworksStatsError> {
    let (total, names) = list_achievement_names(stats, offset, limit)?;
    let percentages = names
        .into_iter()
        .map(|api_name| {
            let percent = stats
                .achievement(&api_name)
                .get_achievement_achieved_percent()
                .map_err(|()| {
                    SteamworksStatsError::operation_failed(
                        "achievement.get_achievement_achieved_percent",
                    )
                })?;

            Ok(SteamworksAchievementGlobalPercentage { api_name, percent })
        })
        .collect::<Result<Vec<_>, SteamworksStatsError>>()?;

    Ok((total, percentages))
}

fn achievement_display_attribute(
    stats: &steamworks::UserStats,
    achievement: &str,
    key: &str,
) -> Result<Option<String>, SteamworksStatsError> {
    stats
        .achievement(achievement)
        .get_achievement_display_attribute(key)
        .map(|value| (!value.is_empty()).then(|| value.to_owned()))
        .map_err(|()| {
            SteamworksStatsError::operation_failed("achievement.get_achievement_display_attribute")
        })
}

pub(super) fn read_achievement_icon(
    stats: &steamworks::UserStats,
    achievement: &str,
) -> SteamworksAchievementIconStatus {
    stats
        .achievement(achievement)
        .get_achievement_icon()
        .map(|rgba| {
            SteamworksAchievementIconStatus::Available(achievement_icon_from_rgba(
                achievement,
                rgba,
            ))
        })
        .unwrap_or(SteamworksAchievementIconStatus::PendingOrUnavailable)
}

pub(super) fn achievement_icon_from_rgba(
    achievement: &str,
    rgba: Vec<u8>,
) -> SteamworksAchievementIcon {
    SteamworksAchievementIcon {
        api_name: achievement.to_owned(),
        width: 64,
        height: 64,
        rgba,
    }
}

pub(super) fn achievement_icon_fetched_operation(
    event: &steamworks::UserAchievementIconFetched,
    icon: SteamworksAchievementIconStatus,
) -> SteamworksStatsOperation {
    SteamworksStatsOperation::AchievementIconFetched {
        name: event.achievement_name.clone(),
        achieved: event.achieved,
        icon_handle: event.icon_handle,
        icon,
    }
}
