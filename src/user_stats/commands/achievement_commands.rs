use crate::SteamworksClient;

use super::super::{
    achievements::{list_achievement_infos, list_achievement_names, read_achievement_icon},
    SteamworksStatsCommand, SteamworksStatsError, SteamworksStatsOperation,
};

pub(super) fn handle_achievement_command(
    client: &SteamworksClient,
    command: SteamworksStatsCommand,
) -> Result<SteamworksStatsOperation, SteamworksStatsError> {
    match command {
        SteamworksStatsCommand::GetAchievement { name } => client
            .user_stats()
            .achievement(&name)
            .get()
            .map(|achieved| SteamworksStatsOperation::AchievementRead { name, achieved })
            .map_err(|()| SteamworksStatsError::operation_failed("achievement.get")),
        SteamworksStatsCommand::ListAchievementNames { offset, limit } => {
            list_achievement_names(&client.user_stats(), offset, limit).map(|(total, names)| {
                SteamworksStatsOperation::AchievementNamesListed {
                    offset,
                    total,
                    names,
                }
            })
        }
        SteamworksStatsCommand::ListAchievements {
            include_display_attributes,
            include_unlock_state,
            offset,
            limit,
        } => list_achievement_infos(
            &client.user_stats(),
            include_display_attributes,
            include_unlock_state,
            offset,
            limit,
        )
        .map(
            |(total, achievements)| SteamworksStatsOperation::AchievementsListed {
                offset,
                total,
                achievements,
            },
        ),
        SteamworksStatsCommand::GetAchievementIcon { name } => {
            Ok(SteamworksStatsOperation::AchievementIconRead {
                icon: read_achievement_icon(&client.user_stats(), &name),
                name,
            })
        }
        SteamworksStatsCommand::UnlockAchievement { name } => client
            .user_stats()
            .achievement(&name)
            .set()
            .map(|()| SteamworksStatsOperation::AchievementUnlocked { name })
            .map_err(|()| SteamworksStatsError::operation_failed("achievement.set")),
        SteamworksStatsCommand::ClearAchievement { name } => client
            .user_stats()
            .achievement(&name)
            .clear()
            .map(|()| SteamworksStatsOperation::AchievementCleared { name })
            .map_err(|()| SteamworksStatsError::operation_failed("achievement.clear")),
        SteamworksStatsCommand::GetAchievementAndUnlockTime { name } => client
            .user_stats()
            .achievement(&name)
            .get_achievement_and_unlock_time()
            .map(
                |(achieved, unlock_time)| SteamworksStatsOperation::AchievementAndUnlockTimeRead {
                    name,
                    achieved,
                    unlock_time,
                },
            )
            .map_err(|()| {
                SteamworksStatsError::operation_failed(
                    "achievement.get_achievement_and_unlock_time",
                )
            }),
        SteamworksStatsCommand::GetAchievementDisplayAttribute { name, key } => client
            .user_stats()
            .achievement(&name)
            .get_achievement_display_attribute(&key)
            .map(
                |value| SteamworksStatsOperation::AchievementDisplayAttributeRead {
                    name,
                    key,
                    value: value.to_owned(),
                },
            )
            .map_err(|()| {
                SteamworksStatsError::operation_failed(
                    "achievement.get_achievement_display_attribute",
                )
            }),
        _ => unreachable!("non-achievement command routed to achievement handler"),
    }
}
