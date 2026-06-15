use bevy_ecs::message::Message;

use super::super::{
    SteamworksLeaderboardDataRequest, SteamworksLeaderboardDisplayType, SteamworksLeaderboardId,
    SteamworksLeaderboardSortMethod, SteamworksLeaderboardUploadScoreMethod,
    STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND,
};

/// A high-level command for Steam user stats and achievements.
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksStatsCommand {
    /// Request stats for the current Steam user.
    RequestCurrentUserStats,
    /// Request stats for a specific Steam user.
    RequestUserStats {
        /// Steam user to request stats for.
        steam_id: steamworks::SteamId,
    },
    /// Read the current user's integer stat.
    GetStatI32 {
        /// Steamworks stat API name.
        name: String,
    },
    /// Set the current user's integer stat.
    SetStatI32 {
        /// Steamworks stat API name.
        name: String,
        /// New stat value.
        value: i32,
    },
    /// Read the current user's floating-point stat.
    GetStatF32 {
        /// Steamworks stat API name.
        name: String,
    },
    /// Set the current user's floating-point stat.
    SetStatF32 {
        /// Steamworks stat API name.
        name: String,
        /// New stat value.
        value: f32,
    },
    /// Read whether an achievement is unlocked for the current user.
    GetAchievement {
        /// Steamworks achievement API name.
        name: String,
    },
    /// List achievement API names for the current app.
    ///
    /// The upstream safe wrapper enumerates the catalog names internally. Keep
    /// this as a startup or tooling command, and use pages instead of doing
    /// repeated full catalog work every frame.
    ListAchievementNames {
        /// Zero-based achievement index to start from.
        offset: usize,
        /// Maximum names returned by this command.
        limit: usize,
    },
    /// List achievement snapshots for the current app.
    ListAchievements {
        /// Include localized display name, description, and hidden flag.
        include_display_attributes: bool,
        /// Include current-user unlock state and unlock time.
        include_unlock_state: bool,
        /// Zero-based achievement index to start from.
        offset: usize,
        /// Maximum achievement snapshots returned by this command.
        limit: usize,
    },
    /// Read a 64x64 RGBA icon for an achievement.
    GetAchievementIcon {
        /// Steamworks achievement API name.
        name: String,
    },
    /// Unlock an achievement for the current user.
    UnlockAchievement {
        /// Steamworks achievement API name.
        name: String,
    },
    /// Clear an achievement for the current user.
    ClearAchievement {
        /// Steamworks achievement API name.
        name: String,
    },
    /// Read achievement unlock state and unlock time.
    GetAchievementAndUnlockTime {
        /// Steamworks achievement API name.
        name: String,
    },
    /// Read a localized achievement display attribute.
    GetAchievementDisplayAttribute {
        /// Steamworks achievement API name.
        name: String,
        /// Attribute key such as `"name"`, `"desc"`, or `"hidden"`.
        key: String,
    },
    /// Request global achievement percentages.
    RequestGlobalAchievementPercentages,
    /// Read the global unlock percentage for one achievement.
    GetAchievementAchievedPercent {
        /// Steamworks achievement API name.
        name: String,
    },
    /// List global achievement unlock percentages for the current app.
    ///
    /// Call [`SteamworksStatsCommand::RequestGlobalAchievementPercentages`] and
    /// wait for
    /// [`SteamworksStatsOperation::GlobalAchievementPercentagesReceived`][super::SteamworksStatsOperation::GlobalAchievementPercentagesReceived]
    /// before reading percentages.
    ListAchievementGlobalPercentages {
        /// Zero-based achievement index to start from.
        offset: usize,
        /// Maximum percentages returned by this command.
        limit: usize,
    },
    /// Request global stat data for stats marked as aggregated.
    RequestGlobalStats {
        /// Number of history days to request, up to Steam's supported limit.
        history_days: i32,
    },
    /// Read an aggregated global stat as an integer.
    GetGlobalStatI64 {
        /// Steamworks stat API name.
        name: String,
    },
    /// Read an aggregated global stat as a floating-point value.
    GetGlobalStatF64 {
        /// Steamworks stat API name.
        name: String,
    },
    /// Read daily history for an aggregated global integer stat.
    GetGlobalStatHistoryI64 {
        /// Steamworks stat API name.
        name: String,
        /// Maximum number of days to read.
        max_days: usize,
    },
    /// Read daily history for an aggregated global floating-point stat.
    GetGlobalStatHistoryF64 {
        /// Steamworks stat API name.
        name: String,
        /// Maximum number of days to read.
        max_days: usize,
    },
    /// Store any changed stats and achievements on Steam.
    StoreStats,
    /// Reset the current user's stats.
    ResetAllStats {
        /// Whether achievements should also be reset.
        achievements_too: bool,
        /// Whether to store the reset immediately if it succeeds.
        store_after_reset: bool,
    },
    /// Find an existing leaderboard by API name.
    FindLeaderboard {
        /// Steamworks leaderboard API name.
        name: String,
    },
    /// Find or create a leaderboard by API name.
    FindOrCreateLeaderboard {
        /// Steamworks leaderboard API name.
        name: String,
        /// Sort method used if Steam creates the leaderboard.
        sort_method: SteamworksLeaderboardSortMethod,
        /// Display type used if Steam creates the leaderboard.
        display_type: SteamworksLeaderboardDisplayType,
    },
    /// Read metadata for a known leaderboard.
    GetLeaderboardInfo {
        /// Plugin-owned leaderboard ID.
        leaderboard: SteamworksLeaderboardId,
    },
    /// Upload a score to a known leaderboard.
    UploadLeaderboardScore {
        /// Plugin-owned leaderboard ID.
        leaderboard: SteamworksLeaderboardId,
        /// Upload behavior.
        method: SteamworksLeaderboardUploadScoreMethod,
        /// Score to submit.
        score: i32,
        /// Optional detail integers.
        details: Vec<i32>,
    },
    /// Download entries from a known leaderboard.
    DownloadLeaderboardEntries {
        /// Plugin-owned leaderboard ID.
        leaderboard: SteamworksLeaderboardId,
        /// Entry scope to download.
        request: SteamworksLeaderboardDataRequest,
        /// Maximum detail integers to read per entry.
        max_details: usize,
    },
    /// Forget a leaderboard handle owned by the plugin.
    ForgetLeaderboard {
        /// Plugin-owned leaderboard ID.
        leaderboard: SteamworksLeaderboardId,
    },
}

impl SteamworksStatsCommand {
    /// Creates a [`SteamworksStatsCommand::RequestCurrentUserStats`] command.
    pub fn request_current_user_stats() -> Self {
        Self::RequestCurrentUserStats
    }

    /// Creates a [`SteamworksStatsCommand::RequestUserStats`] command.
    pub fn request_user_stats(steam_id: steamworks::SteamId) -> Self {
        Self::RequestUserStats { steam_id }
    }

    /// Creates a [`SteamworksStatsCommand::GetStatI32`] command.
    pub fn get_stat_i32(name: impl Into<String>) -> Self {
        Self::GetStatI32 { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::SetStatI32`] command.
    pub fn set_stat_i32(name: impl Into<String>, value: i32) -> Self {
        Self::SetStatI32 {
            name: name.into(),
            value,
        }
    }

    /// Creates a [`SteamworksStatsCommand::GetStatF32`] command.
    pub fn get_stat_f32(name: impl Into<String>) -> Self {
        Self::GetStatF32 { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::SetStatF32`] command.
    pub fn set_stat_f32(name: impl Into<String>, value: f32) -> Self {
        Self::SetStatF32 {
            name: name.into(),
            value,
        }
    }

    /// Creates a [`SteamworksStatsCommand::GetAchievement`] command.
    pub fn get_achievement(name: impl Into<String>) -> Self {
        Self::GetAchievement { name: name.into() }
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

    /// Creates a [`SteamworksStatsCommand::RequestGlobalStats`] command.
    pub fn request_global_stats(history_days: i32) -> Self {
        Self::RequestGlobalStats { history_days }
    }

    /// Creates a [`SteamworksStatsCommand::GetGlobalStatI64`] command.
    pub fn get_global_stat_i64(name: impl Into<String>) -> Self {
        Self::GetGlobalStatI64 { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::GetGlobalStatF64`] command.
    pub fn get_global_stat_f64(name: impl Into<String>) -> Self {
        Self::GetGlobalStatF64 { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::GetGlobalStatHistoryI64`] command.
    pub fn get_global_stat_history_i64(name: impl Into<String>, max_days: usize) -> Self {
        Self::GetGlobalStatHistoryI64 {
            name: name.into(),
            max_days,
        }
    }

    /// Creates a [`SteamworksStatsCommand::GetGlobalStatHistoryF64`] command.
    pub fn get_global_stat_history_f64(name: impl Into<String>, max_days: usize) -> Self {
        Self::GetGlobalStatHistoryF64 {
            name: name.into(),
            max_days,
        }
    }

    /// Creates a [`SteamworksStatsCommand::StoreStats`] command.
    pub fn store_stats() -> Self {
        Self::StoreStats
    }

    /// Creates a [`SteamworksStatsCommand::ResetAllStats`] command.
    pub fn reset_all_stats(achievements_too: bool, store_after_reset: bool) -> Self {
        Self::ResetAllStats {
            achievements_too,
            store_after_reset,
        }
    }

    /// Creates a [`SteamworksStatsCommand::FindLeaderboard`] command.
    pub fn find_leaderboard(name: impl Into<String>) -> Self {
        Self::FindLeaderboard { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::FindOrCreateLeaderboard`] command.
    pub fn find_or_create_leaderboard(
        name: impl Into<String>,
        sort_method: SteamworksLeaderboardSortMethod,
        display_type: SteamworksLeaderboardDisplayType,
    ) -> Self {
        Self::FindOrCreateLeaderboard {
            name: name.into(),
            sort_method,
            display_type,
        }
    }

    /// Creates a [`SteamworksStatsCommand::GetLeaderboardInfo`] command.
    pub fn get_leaderboard_info(leaderboard: SteamworksLeaderboardId) -> Self {
        Self::GetLeaderboardInfo { leaderboard }
    }

    /// Creates a [`SteamworksStatsCommand::UploadLeaderboardScore`] command.
    pub fn upload_leaderboard_score(
        leaderboard: SteamworksLeaderboardId,
        method: SteamworksLeaderboardUploadScoreMethod,
        score: i32,
        details: impl Into<Vec<i32>>,
    ) -> Self {
        Self::UploadLeaderboardScore {
            leaderboard,
            method,
            score,
            details: details.into(),
        }
    }

    /// Creates a [`SteamworksStatsCommand::DownloadLeaderboardEntries`] command.
    pub fn download_leaderboard_entries(
        leaderboard: SteamworksLeaderboardId,
        request: SteamworksLeaderboardDataRequest,
        max_details: usize,
    ) -> Self {
        Self::DownloadLeaderboardEntries {
            leaderboard,
            request,
            max_details,
        }
    }

    /// Creates a global [`SteamworksStatsCommand::DownloadLeaderboardEntries`] command.
    pub fn download_global_leaderboard_entries(
        leaderboard: SteamworksLeaderboardId,
        start: i32,
        end: i32,
        max_details: usize,
    ) -> Self {
        Self::download_leaderboard_entries(
            leaderboard,
            SteamworksLeaderboardDataRequest::Global { start, end },
            max_details,
        )
    }

    /// Creates a user-relative [`SteamworksStatsCommand::DownloadLeaderboardEntries`] command.
    pub fn download_leaderboard_entries_around_user(
        leaderboard: SteamworksLeaderboardId,
        start: i32,
        end: i32,
        max_details: usize,
    ) -> Self {
        Self::download_leaderboard_entries(
            leaderboard,
            SteamworksLeaderboardDataRequest::GlobalAroundUser { start, end },
            max_details,
        )
    }

    /// Creates a friends [`SteamworksStatsCommand::DownloadLeaderboardEntries`] command.
    pub fn download_friends_leaderboard_entries(
        leaderboard: SteamworksLeaderboardId,
        max_details: usize,
    ) -> Self {
        Self::download_leaderboard_entries(
            leaderboard,
            SteamworksLeaderboardDataRequest::Friends,
            max_details,
        )
    }

    /// Creates a [`SteamworksStatsCommand::ForgetLeaderboard`] command.
    pub fn forget_leaderboard(leaderboard: SteamworksLeaderboardId) -> Self {
        Self::ForgetLeaderboard { leaderboard }
    }
}
