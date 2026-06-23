use super::{
    SteamworksAchievementDisplayAttribute, SteamworksAchievementGlobalPercentage,
    SteamworksAchievementIcon, SteamworksAchievementInfo, SteamworksGlobalStatHistory,
    SteamworksGlobalStatValue, SteamworksLeaderboardEntriesDownloadRequest,
    SteamworksLeaderboardEntriesDownloadResult, SteamworksLeaderboardEntry,
    SteamworksLeaderboardFindOrCreateRequest, SteamworksLeaderboardFindOrCreateResult,
    SteamworksLeaderboardFindRequest, SteamworksLeaderboardFindResult, SteamworksLeaderboardId,
    SteamworksLeaderboardInfo, SteamworksLeaderboardScoreUploadRequest,
    SteamworksLeaderboardScoreUploadResult, SteamworksStatsError, SteamworksStatsState,
    SteamworksUserAchievementStored, SteamworksUserStatsReceived, SteamworksUserStatsStored,
};

use super::helpers::named_value;

impl SteamworksStatsState {
    /// Returns whether this plugin has requested stats for the current user.
    pub fn current_user_stats_requested(&self) -> bool {
        self.current_user_stats_requested
    }

    /// Returns whether successful writes are waiting for `store_stats`.
    pub fn pending_store(&self) -> bool {
        self.pending_store
    }

    /// Returns the most recent command or asynchronous callback error observed by the stats plugin.
    pub fn last_error(&self) -> Option<&SteamworksStatsError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent current-user integer stat value read or set through this plugin.
    pub fn stat_i32(&self, name: &str) -> Option<i32> {
        named_value(&self.local_stat_i32, name).copied()
    }

    /// Returns the most recent current-user floating-point stat value read or set through this plugin.
    pub fn stat_f32(&self, name: &str) -> Option<f32> {
        named_value(&self.local_stat_f32, name).copied()
    }

    /// Returns the most recent achievement count read through this plugin.
    pub fn achievement_count(&self) -> Option<u32> {
        self.achievement_count
    }

    /// Returns the most recent achievement catalog snapshot.
    pub fn last_achievements(&self) -> &[SteamworksAchievementInfo] {
        &self.last_achievements
    }

    /// Returns the cached achievement snapshot for an API name.
    pub fn achievement(&self, api_name: &str) -> Option<&SteamworksAchievementInfo> {
        self.achievements
            .iter()
            .find(|achievement| achievement.api_name == api_name)
    }

    /// Returns the most recent current-user unlock state for an achievement.
    pub fn achievement_unlocked(&self, api_name: &str) -> Option<bool> {
        self.achievement(api_name)
            .and_then(|achievement| achievement.achieved)
    }

    /// Returns the most recent current-user unlock time for an achievement.
    pub fn achievement_unlock_time(&self, api_name: &str) -> Option<u32> {
        self.achievement(api_name)
            .and_then(|achievement| achievement.unlock_time)
    }

    /// Returns the most recent display attribute read for an achievement and key.
    pub fn achievement_display_attribute(&self, api_name: &str, key: &str) -> Option<&str> {
        self.achievement_display_attributes
            .iter()
            .find_map(|attribute| {
                (attribute.api_name == api_name && attribute.key == key)
                    .then_some(attribute.value.as_str())
            })
    }

    /// Returns all achievement display attributes cached by this plugin.
    pub fn achievement_display_attributes(&self) -> &[SteamworksAchievementDisplayAttribute] {
        &self.achievement_display_attributes
    }

    /// Returns the most recent achievement icon snapshot read through this plugin.
    pub fn last_achievement_icon(&self) -> Option<&SteamworksAchievementIcon> {
        self.last_achievement_icon.as_ref()
    }

    /// Returns how many achievement icon fetched callbacks this plugin observed.
    pub fn achievement_icon_callback_count(&self) -> u64 {
        self.achievement_icon_callback_count
    }

    /// Returns the most recent user stats received callback snapshot.
    pub fn last_user_stats_received(&self) -> Option<&SteamworksUserStatsReceived> {
        self.last_user_stats_received.as_ref()
    }

    /// Returns the most recent user stats stored callback snapshot.
    pub fn last_user_stats_stored(&self) -> Option<&SteamworksUserStatsStored> {
        self.last_user_stats_stored.as_ref()
    }

    /// Returns the most recent achievement stored callback snapshot.
    pub fn last_user_achievement_stored(&self) -> Option<&SteamworksUserAchievementStored> {
        self.last_user_achievement_stored.as_ref()
    }

    /// Returns the most recent global achievement percentage page.
    pub fn last_global_achievement_percentages(&self) -> &[SteamworksAchievementGlobalPercentage] {
        &self.last_global_achievement_percentages
    }

    /// Returns the most recent global unlock percentage for one achievement.
    pub fn achievement_global_percent(&self, api_name: &str) -> Option<f32> {
        self.achievement_global_percentages
            .iter()
            .find_map(|percentage| (percentage.api_name == api_name).then_some(percentage.percent))
    }

    /// Returns the most recent global stats received callback game ID.
    pub fn last_global_stats_game_id(&self) -> Option<steamworks::GameId> {
        self.last_global_stats_game_id
    }

    /// Returns the most recent aggregated global integer stat read through this plugin.
    pub fn last_global_stat_i64(&self) -> Option<&SteamworksGlobalStatValue<i64>> {
        self.last_global_stat_i64.as_ref()
    }

    /// Returns bounded aggregated global integer stat snapshots keyed by stat name.
    pub fn global_stat_i64_values(&self) -> &[SteamworksGlobalStatValue<i64>] {
        &self.global_stat_i64
    }

    /// Returns the cached aggregated global integer stat value for one stat name.
    pub fn global_stat_i64(&self, name: &str) -> Option<i64> {
        self.global_stat_i64
            .iter()
            .find_map(|value| (value.name == name).then_some(value.value))
    }

    /// Returns the most recent aggregated global floating-point stat read through this plugin.
    pub fn last_global_stat_f64(&self) -> Option<&SteamworksGlobalStatValue<f64>> {
        self.last_global_stat_f64.as_ref()
    }

    /// Returns bounded aggregated global floating-point stat snapshots keyed by stat name.
    pub fn global_stat_f64_values(&self) -> &[SteamworksGlobalStatValue<f64>] {
        &self.global_stat_f64
    }

    /// Returns the cached aggregated global floating-point stat value for one stat name.
    pub fn global_stat_f64(&self, name: &str) -> Option<f64> {
        self.global_stat_f64
            .iter()
            .find_map(|value| (value.name == name).then_some(value.value))
    }

    /// Returns the most recent aggregated global integer stat history read through this plugin.
    pub fn last_global_stat_history_i64(&self) -> Option<&SteamworksGlobalStatHistory<i64>> {
        self.last_global_stat_history_i64.as_ref()
    }

    /// Returns bounded aggregated global integer stat history snapshots keyed by stat name.
    pub fn global_stat_history_i64_values(&self) -> &[SteamworksGlobalStatHistory<i64>] {
        &self.global_stat_history_i64
    }

    /// Returns the cached aggregated global integer stat history for one stat name.
    pub fn global_stat_history_i64(&self, name: &str) -> Option<&SteamworksGlobalStatHistory<i64>> {
        self.global_stat_history_i64
            .iter()
            .find(|history| history.name == name)
    }

    /// Returns the most recent aggregated global floating-point stat history read through this plugin.
    pub fn last_global_stat_history_f64(&self) -> Option<&SteamworksGlobalStatHistory<f64>> {
        self.last_global_stat_history_f64.as_ref()
    }

    /// Returns bounded aggregated global floating-point stat history snapshots keyed by stat name.
    pub fn global_stat_history_f64_values(&self) -> &[SteamworksGlobalStatHistory<f64>] {
        &self.global_stat_history_f64
    }

    /// Returns the cached aggregated global floating-point stat history for one stat name.
    pub fn global_stat_history_f64(&self, name: &str) -> Option<&SteamworksGlobalStatHistory<f64>> {
        self.global_stat_history_f64
            .iter()
            .find(|history| history.name == name)
    }

    /// Returns the number of leaderboard handles currently owned by this plugin.
    pub fn leaderboard_count(&self) -> usize {
        self.leaderboard_count
    }

    /// Returns the plugin-owned leaderboard ID most recently associated with a name.
    pub fn leaderboard_id(&self, name: &str) -> Option<SteamworksLeaderboardId> {
        self.leaderboard_ids
            .iter()
            .find_map(|(known_name, leaderboard)| (known_name == name).then_some(*leaderboard))
    }

    /// Returns leaderboard metadata snapshots read through this plugin.
    pub fn leaderboards(&self) -> &[SteamworksLeaderboardInfo] {
        &self.leaderboard_infos
    }

    /// Returns cached leaderboard metadata for a plugin-owned leaderboard ID.
    pub fn leaderboard_info(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<&SteamworksLeaderboardInfo> {
        self.leaderboard_infos
            .iter()
            .find(|info| info.leaderboard == leaderboard)
    }

    /// Returns cached leaderboard metadata for a Steamworks leaderboard name.
    pub fn leaderboard_info_by_name(&self, name: &str) -> Option<&SteamworksLeaderboardInfo> {
        self.leaderboard_infos.iter().find(|info| info.name == name)
    }

    /// Returns the most recent submitted leaderboard find request.
    pub fn last_leaderboard_find_request(&self) -> Option<&SteamworksLeaderboardFindRequest> {
        self.last_leaderboard_find_request.as_ref()
    }

    /// Returns the most recent completed leaderboard find result.
    pub fn last_leaderboard_find_result(&self) -> Option<&SteamworksLeaderboardFindResult> {
        self.last_leaderboard_find_result.as_ref()
    }

    /// Returns the most recent submitted leaderboard find-or-create request.
    pub fn last_leaderboard_find_or_create_request(
        &self,
    ) -> Option<&SteamworksLeaderboardFindOrCreateRequest> {
        self.last_leaderboard_find_or_create_request.as_ref()
    }

    /// Returns the most recent completed leaderboard find-or-create result.
    pub fn last_leaderboard_find_or_create_result(
        &self,
    ) -> Option<&SteamworksLeaderboardFindOrCreateResult> {
        self.last_leaderboard_find_or_create_result.as_ref()
    }

    /// Returns the most recent leaderboard info read through this plugin.
    pub fn last_leaderboard_info(&self) -> Option<&SteamworksLeaderboardInfo> {
        self.last_leaderboard_info.as_ref()
    }

    /// Returns the most recent submitted leaderboard score upload request.
    pub fn last_leaderboard_score_upload_request(
        &self,
    ) -> Option<&SteamworksLeaderboardScoreUploadRequest> {
        self.last_leaderboard_score_upload_request.as_ref()
    }

    /// Returns bounded submitted leaderboard score upload requests keyed by leaderboard.
    pub fn leaderboard_score_upload_requests(&self) -> &[SteamworksLeaderboardScoreUploadRequest] {
        &self.leaderboard_score_upload_requests
    }

    /// Returns the cached score upload request for one leaderboard.
    pub fn leaderboard_score_upload_request(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<&SteamworksLeaderboardScoreUploadRequest> {
        self.leaderboard_score_upload_requests
            .iter()
            .find(|request| request.leaderboard == leaderboard)
    }

    /// Returns the most recent completed leaderboard score upload result.
    pub fn last_leaderboard_score_upload_result(
        &self,
    ) -> Option<&SteamworksLeaderboardScoreUploadResult> {
        self.last_leaderboard_score_upload_result.as_ref()
    }

    /// Returns bounded completed leaderboard score upload results keyed by leaderboard.
    pub fn leaderboard_score_upload_results(&self) -> &[SteamworksLeaderboardScoreUploadResult] {
        &self.leaderboard_score_upload_results
    }

    /// Returns the cached score upload result for one leaderboard.
    pub fn leaderboard_score_upload_result(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<&SteamworksLeaderboardScoreUploadResult> {
        self.leaderboard_score_upload_results
            .iter()
            .find(|result| result.leaderboard == leaderboard)
    }

    /// Returns the most recent submitted leaderboard entries download request.
    pub fn last_leaderboard_entries_download_request(
        &self,
    ) -> Option<&SteamworksLeaderboardEntriesDownloadRequest> {
        self.last_leaderboard_entries_download_request.as_ref()
    }

    /// Returns bounded submitted leaderboard entries download requests keyed by leaderboard.
    pub fn leaderboard_entries_download_requests(
        &self,
    ) -> &[SteamworksLeaderboardEntriesDownloadRequest] {
        &self.leaderboard_entries_download_requests
    }

    /// Returns the cached entries download request for one leaderboard.
    pub fn leaderboard_entries_download_request(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<&SteamworksLeaderboardEntriesDownloadRequest> {
        self.leaderboard_entries_download_requests
            .iter()
            .find(|request| request.leaderboard == leaderboard)
    }

    /// Returns the most recent completed leaderboard entries download result.
    pub fn last_leaderboard_entries_download_result(
        &self,
    ) -> Option<&SteamworksLeaderboardEntriesDownloadResult> {
        self.last_leaderboard_entries_download_result.as_ref()
    }

    /// Returns bounded completed leaderboard entries download results keyed by leaderboard.
    pub fn leaderboard_entries_download_results(
        &self,
    ) -> &[SteamworksLeaderboardEntriesDownloadResult] {
        &self.leaderboard_entries_download_results
    }

    /// Returns the cached entries download result for one leaderboard.
    pub fn leaderboard_entries_download_result(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<&SteamworksLeaderboardEntriesDownloadResult> {
        self.leaderboard_entries_download_results
            .iter()
            .find(|result| result.leaderboard == leaderboard)
    }

    /// Returns the most recent downloaded leaderboard entries.
    pub fn last_leaderboard_entries(&self) -> &[SteamworksLeaderboardEntry] {
        &self.last_leaderboard_entries
    }

    /// Returns the cached downloaded entries for one leaderboard.
    pub fn leaderboard_entries(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<&[SteamworksLeaderboardEntry]> {
        self.leaderboard_entries_download_result(leaderboard)
            .map(|result| result.entries.as_slice())
    }

    /// Returns a cached downloaded leaderboard entry for one Steam user.
    pub fn leaderboard_entry_by_user(
        &self,
        leaderboard: SteamworksLeaderboardId,
        user: steamworks::SteamId,
    ) -> Option<&SteamworksLeaderboardEntry> {
        self.leaderboard_entries(leaderboard)
            .and_then(|entries| entries.iter().find(|entry| entry.user == user))
    }

    /// Returns a cached downloaded leaderboard entry for one global rank.
    pub fn leaderboard_entry_by_rank(
        &self,
        leaderboard: SteamworksLeaderboardId,
        global_rank: i32,
    ) -> Option<&SteamworksLeaderboardEntry> {
        self.leaderboard_entries(leaderboard).and_then(|entries| {
            entries
                .iter()
                .find(|entry| entry.global_rank == global_rank)
        })
    }

    /// Returns the cached score for one Steam user on a leaderboard.
    pub fn leaderboard_score_by_user(
        &self,
        leaderboard: SteamworksLeaderboardId,
        user: steamworks::SteamId,
    ) -> Option<i32> {
        self.leaderboard_entry_by_user(leaderboard, user)
            .map(|entry| entry.score)
    }

    /// Returns the cached global rank for one Steam user on a leaderboard.
    pub fn leaderboard_rank_by_user(
        &self,
        leaderboard: SteamworksLeaderboardId,
        user: steamworks::SteamId,
    ) -> Option<i32> {
        self.leaderboard_entry_by_user(leaderboard, user)
            .map(|entry| entry.global_rank)
    }

    /// Returns the cached detail integers for one Steam user on a leaderboard.
    pub fn leaderboard_entry_details(
        &self,
        leaderboard: SteamworksLeaderboardId,
        user: steamworks::SteamId,
    ) -> Option<&[i32]> {
        self.leaderboard_entry_by_user(leaderboard, user)
            .map(|entry| entry.details.as_slice())
    }

    /// Returns the most recent leaderboard ID forgotten by this plugin.
    pub fn last_forgotten_leaderboard(&self) -> Option<SteamworksLeaderboardId> {
        self.last_forgotten_leaderboard
    }
}
