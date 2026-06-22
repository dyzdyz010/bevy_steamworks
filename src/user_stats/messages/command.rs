use bevy_ecs::message::Message;

use super::super::{
    SteamworksLeaderboardDataRequest, SteamworksLeaderboardDisplayType, SteamworksLeaderboardId,
    SteamworksLeaderboardSortMethod, SteamworksLeaderboardUploadScoreMethod,
};

mod achievements;
mod global_stats;
mod leaderboards;
mod local_stats;

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
    /// Read the number of achievements defined for the current app.
    GetAchievementCount,
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
