use bevy_ecs::prelude::Resource;

use super::{
    SteamworksAchievementDisplayAttribute, SteamworksAchievementGlobalPercentage,
    SteamworksAchievementIcon, SteamworksAchievementInfo, SteamworksGlobalStatHistory,
    SteamworksGlobalStatValue, SteamworksLeaderboardEntriesDownloadRequest,
    SteamworksLeaderboardEntriesDownloadResult, SteamworksLeaderboardEntry,
    SteamworksLeaderboardFindOrCreateRequest, SteamworksLeaderboardFindOrCreateResult,
    SteamworksLeaderboardFindRequest, SteamworksLeaderboardFindResult, SteamworksLeaderboardId,
    SteamworksLeaderboardInfo, SteamworksLeaderboardScoreUploadRequest,
    SteamworksLeaderboardScoreUploadResult, SteamworksStatsError, SteamworksUserAchievementStored,
    SteamworksUserStatsReceived, SteamworksUserStatsStored,
};

mod accessors;
mod helpers;
mod operations;

pub(in crate::user_stats) const STEAMWORKS_STATS_STATE_CACHE_LIMIT: usize = 1_024;

/// Settings used by [`crate::SteamworksStatsPlugin`].
#[derive(Clone, Debug, PartialEq, Eq, Resource)]
pub struct SteamworksStatsSettings {
    /// Request stats for the current Steam user when the plugin starts.
    ///
    /// Steam stats and achievement reads/writes require user stats to be loaded.
    /// The upstream `steamworks` crate exposes this through
    /// [`steamworks::UserStats::request_user_stats`], so this plugin requests
    /// stats for [`steamworks::User::steam_id`] by default.
    pub request_current_user_stats_on_startup: bool,
    /// Call [`steamworks::UserStats::store_stats`] once after a frame with
    /// successful stat or achievement writes.
    pub auto_store: bool,
}

impl Default for SteamworksStatsSettings {
    fn default() -> Self {
        Self {
            request_current_user_stats_on_startup: true,
            auto_store: true,
        }
    }
}

/// Runtime state for [`crate::SteamworksStatsPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksStatsState {
    pub(super) current_user_stats_requested: bool,
    pub(super) pending_store: bool,
    pub(super) force_store: bool,
    last_error: Option<SteamworksStatsError>,
    local_stat_i32: Vec<(String, i32)>,
    local_stat_f32: Vec<(String, f32)>,
    achievement_count: Option<u32>,
    last_achievements: Vec<SteamworksAchievementInfo>,
    achievements: Vec<SteamworksAchievementInfo>,
    achievement_display_attributes: Vec<SteamworksAchievementDisplayAttribute>,
    last_achievement_icon: Option<SteamworksAchievementIcon>,
    achievement_icon_callback_count: u64,
    last_user_stats_received: Option<SteamworksUserStatsReceived>,
    last_user_stats_stored: Option<SteamworksUserStatsStored>,
    last_user_achievement_stored: Option<SteamworksUserAchievementStored>,
    achievement_global_percentages: Vec<SteamworksAchievementGlobalPercentage>,
    last_global_achievement_percentages: Vec<SteamworksAchievementGlobalPercentage>,
    last_global_stats_game_id: Option<steamworks::GameId>,
    last_global_stat_i64: Option<SteamworksGlobalStatValue<i64>>,
    last_global_stat_f64: Option<SteamworksGlobalStatValue<f64>>,
    last_global_stat_history_i64: Option<SteamworksGlobalStatHistory<i64>>,
    last_global_stat_history_f64: Option<SteamworksGlobalStatHistory<f64>>,
    leaderboard_count: usize,
    leaderboard_ids: Vec<(String, SteamworksLeaderboardId)>,
    leaderboard_infos: Vec<SteamworksLeaderboardInfo>,
    last_leaderboard_find_request: Option<SteamworksLeaderboardFindRequest>,
    last_leaderboard_find_result: Option<SteamworksLeaderboardFindResult>,
    last_leaderboard_find_or_create_request: Option<SteamworksLeaderboardFindOrCreateRequest>,
    last_leaderboard_find_or_create_result: Option<SteamworksLeaderboardFindOrCreateResult>,
    last_leaderboard_info: Option<SteamworksLeaderboardInfo>,
    last_leaderboard_score_upload_request: Option<SteamworksLeaderboardScoreUploadRequest>,
    last_leaderboard_score_upload_result: Option<SteamworksLeaderboardScoreUploadResult>,
    last_leaderboard_entries_download_request: Option<SteamworksLeaderboardEntriesDownloadRequest>,
    last_leaderboard_entries_download_result: Option<SteamworksLeaderboardEntriesDownloadResult>,
    last_leaderboard_entries: Vec<SteamworksLeaderboardEntry>,
    last_forgotten_leaderboard: Option<SteamworksLeaderboardId>,
}
