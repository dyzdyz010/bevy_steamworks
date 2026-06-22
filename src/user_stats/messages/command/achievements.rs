use crate::user_stats::STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND;

use super::SteamworksStatsCommand;

impl SteamworksStatsCommand {
    /// Creates a [`SteamworksStatsCommand::GetAchievement`] command.
    pub fn get_achievement(name: impl Into<String>) -> Self {
        Self::GetAchievement { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::GetAchievementCount`] command.
    pub fn get_achievement_count() -> Self {
        Self::GetAchievementCount
    }

    /// Creates a [`SteamworksStatsCommand::ListAchievementNames`] command.
    pub fn list_achievement_names() -> Self {
        Self::list_achievement_names_page(0, STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND)
    }

    /// Creates a paged [`SteamworksStatsCommand::ListAchievementNames`] command.
    pub fn list_achievement_names_page(offset: usize, limit: usize) -> Self {
        Self::ListAchievementNames { offset, limit }
    }

    /// Creates a [`SteamworksStatsCommand::ListAchievements`] command.
    pub fn list_achievements(include_display_attributes: bool, include_unlock_state: bool) -> Self {
        Self::list_achievements_page(
            include_display_attributes,
            include_unlock_state,
            0,
            STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND,
        )
    }

    /// Creates a paged [`SteamworksStatsCommand::ListAchievements`] command.
    pub fn list_achievements_page(
        include_display_attributes: bool,
        include_unlock_state: bool,
        offset: usize,
        limit: usize,
    ) -> Self {
        Self::ListAchievements {
            include_display_attributes,
            include_unlock_state,
            offset,
            limit,
        }
    }

    /// Creates a [`SteamworksStatsCommand::GetAchievementIcon`] command.
    pub fn get_achievement_icon(name: impl Into<String>) -> Self {
        Self::GetAchievementIcon { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::UnlockAchievement`] command.
    pub fn unlock_achievement(name: impl Into<String>) -> Self {
        Self::UnlockAchievement { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::ClearAchievement`] command.
    pub fn clear_achievement(name: impl Into<String>) -> Self {
        Self::ClearAchievement { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::GetAchievementAndUnlockTime`] command.
    pub fn get_achievement_and_unlock_time(name: impl Into<String>) -> Self {
        Self::GetAchievementAndUnlockTime { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::GetAchievementDisplayAttribute`] command.
    pub fn get_achievement_display_attribute(
        name: impl Into<String>,
        key: impl Into<String>,
    ) -> Self {
        Self::GetAchievementDisplayAttribute {
            name: name.into(),
            key: key.into(),
        }
    }

    /// Creates a [`SteamworksStatsCommand::RequestGlobalAchievementPercentages`] command.
    pub fn request_global_achievement_percentages() -> Self {
        Self::RequestGlobalAchievementPercentages
    }

    /// Creates a [`SteamworksStatsCommand::GetAchievementAchievedPercent`] command.
    pub fn get_achievement_achieved_percent(name: impl Into<String>) -> Self {
        Self::GetAchievementAchievedPercent { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::ListAchievementGlobalPercentages`] command.
    pub fn list_achievement_global_percentages() -> Self {
        Self::list_achievement_global_percentages_page(
            0,
            STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND,
        )
    }

    /// Creates a paged [`SteamworksStatsCommand::ListAchievementGlobalPercentages`] command.
    pub fn list_achievement_global_percentages_page(offset: usize, limit: usize) -> Self {
        Self::ListAchievementGlobalPercentages { offset, limit }
    }
}
