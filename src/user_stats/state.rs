use bevy_ecs::prelude::Resource;

use super::{
    SteamworksAchievementDisplayAttribute, SteamworksAchievementGlobalPercentage,
    SteamworksAchievementIcon, SteamworksAchievementInfo, SteamworksGlobalStatHistory,
    SteamworksGlobalStatValue, SteamworksLeaderboardEntriesDownloadRequest,
    SteamworksLeaderboardEntriesDownloadResult, SteamworksLeaderboardEntry,
    SteamworksLeaderboardFindOrCreateRequest, SteamworksLeaderboardFindOrCreateResult,
    SteamworksLeaderboardFindRequest, SteamworksLeaderboardFindResult, SteamworksLeaderboardId,
    SteamworksLeaderboardInfo, SteamworksLeaderboardScoreUploadRequest,
    SteamworksLeaderboardScoreUploadResult, SteamworksStatsError,
    SteamworksStatsLeaderboardHandles, SteamworksStatsOperation, SteamworksUserAchievementStored,
    SteamworksUserStatsReceived, SteamworksUserStatsStored,
};

/// Settings used by [`crate::SteamworksStatsPlugin`].
#[derive(Clone, Debug, Resource)]
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

    /// Returns the most recent aggregated global floating-point stat read through this plugin.
    pub fn last_global_stat_f64(&self) -> Option<&SteamworksGlobalStatValue<f64>> {
        self.last_global_stat_f64.as_ref()
    }

    /// Returns the most recent aggregated global integer stat history read through this plugin.
    pub fn last_global_stat_history_i64(&self) -> Option<&SteamworksGlobalStatHistory<i64>> {
        self.last_global_stat_history_i64.as_ref()
    }

    /// Returns the most recent aggregated global floating-point stat history read through this plugin.
    pub fn last_global_stat_history_f64(&self) -> Option<&SteamworksGlobalStatHistory<f64>> {
        self.last_global_stat_history_f64.as_ref()
    }

    /// Returns the number of leaderboard handles currently owned by this plugin.
    pub fn leaderboard_count(&self) -> usize {
        self.leaderboard_count
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

    /// Returns the most recent completed leaderboard score upload result.
    pub fn last_leaderboard_score_upload_result(
        &self,
    ) -> Option<&SteamworksLeaderboardScoreUploadResult> {
        self.last_leaderboard_score_upload_result.as_ref()
    }

    /// Returns the most recent submitted leaderboard entries download request.
    pub fn last_leaderboard_entries_download_request(
        &self,
    ) -> Option<&SteamworksLeaderboardEntriesDownloadRequest> {
        self.last_leaderboard_entries_download_request.as_ref()
    }

    /// Returns the most recent completed leaderboard entries download result.
    pub fn last_leaderboard_entries_download_result(
        &self,
    ) -> Option<&SteamworksLeaderboardEntriesDownloadResult> {
        self.last_leaderboard_entries_download_result.as_ref()
    }

    /// Returns the most recent downloaded leaderboard entries.
    pub fn last_leaderboard_entries(&self) -> &[SteamworksLeaderboardEntry] {
        &self.last_leaderboard_entries
    }

    /// Returns the most recent leaderboard ID forgotten by this plugin.
    pub fn last_forgotten_leaderboard(&self) -> Option<SteamworksLeaderboardId> {
        self.last_forgotten_leaderboard
    }

    pub(super) fn record_error(&mut self, error: SteamworksStatsError) {
        self.last_error = Some(error);
    }

    pub(super) fn sync_leaderboard_count(
        &mut self,
        leaderboards: &SteamworksStatsLeaderboardHandles,
    ) {
        self.leaderboard_count = leaderboards.len();
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksStatsOperation) {
        match operation {
            SteamworksStatsOperation::CurrentUserStatsRequested { .. } => {
                self.current_user_stats_requested = true;
            }
            SteamworksStatsOperation::UserStatsRequested { .. } => {}
            SteamworksStatsOperation::StatI32Read { name, value }
            | SteamworksStatsOperation::StatI32Set { name, value } => {
                upsert_named_value(&mut self.local_stat_i32, name.clone(), *value);
            }
            SteamworksStatsOperation::StatF32Read { name, value }
            | SteamworksStatsOperation::StatF32Set { name, value } => {
                upsert_named_value(&mut self.local_stat_f32, name.clone(), *value);
            }
            SteamworksStatsOperation::AchievementRead { name, achieved } => {
                update_achievement(&mut self.achievements, name, |achievement| {
                    achievement.achieved = Some(*achieved);
                });
            }
            SteamworksStatsOperation::AchievementNamesListed { names, .. } => {
                self.last_achievements = names
                    .iter()
                    .map(|api_name| SteamworksAchievementInfo {
                        api_name: api_name.clone(),
                        ..Default::default()
                    })
                    .collect();
                for achievement in &self.last_achievements {
                    merge_achievement_info(&mut self.achievements, achievement);
                }
            }
            SteamworksStatsOperation::AchievementsListed { achievements, .. } => {
                self.last_achievements.clone_from(achievements);
                for achievement in achievements {
                    merge_achievement_info(&mut self.achievements, achievement);
                    merge_achievement_display_attributes(
                        &mut self.achievement_display_attributes,
                        achievement,
                    );
                }
            }
            SteamworksStatsOperation::AchievementIconRead { icon, .. } => {
                if let Some(icon) = icon.as_icon() {
                    self.last_achievement_icon = Some(icon.clone());
                }
            }
            SteamworksStatsOperation::AchievementIconFetched {
                name,
                achieved,
                icon,
                ..
            } => {
                update_achievement(&mut self.achievements, name, |achievement| {
                    achievement.achieved = Some(*achieved);
                });
                if let Some(icon) = icon.as_icon() {
                    self.last_achievement_icon = Some(icon.clone());
                }
                self.achievement_icon_callback_count =
                    self.achievement_icon_callback_count.saturating_add(1);
            }
            SteamworksStatsOperation::UserStatsReceived { callback } => {
                self.last_user_stats_received = Some(callback.clone());
            }
            SteamworksStatsOperation::UserStatsStored { callback } => {
                self.last_user_stats_stored = Some(callback.clone());
            }
            SteamworksStatsOperation::UserAchievementStored { callback } => {
                self.last_user_achievement_stored = Some(callback.clone());
            }
            SteamworksStatsOperation::AchievementAndUnlockTimeRead {
                name,
                achieved,
                unlock_time,
            } => {
                update_achievement(&mut self.achievements, name, |achievement| {
                    achievement.achieved = Some(*achieved);
                    achievement.unlock_time = Some(*unlock_time);
                });
            }
            SteamworksStatsOperation::AchievementDisplayAttributeRead { name, key, value } => {
                upsert_achievement_display_attribute(
                    &mut self.achievement_display_attributes,
                    name.clone(),
                    key.clone(),
                    value.clone(),
                );
                update_achievement(&mut self.achievements, name, |achievement| {
                    if key == "name" {
                        achievement.display_name = Some(value.clone());
                    } else if key == "desc" {
                        achievement.description = Some(value.clone());
                    } else if key == "hidden" {
                        achievement.hidden = Some(value != "0");
                    }
                });
            }
            SteamworksStatsOperation::AchievementUnlocked { name } => {
                update_achievement(&mut self.achievements, name, |achievement| {
                    achievement.achieved = Some(true);
                    achievement.unlock_time = None;
                });
            }
            SteamworksStatsOperation::AchievementCleared { name } => {
                update_achievement(&mut self.achievements, name, |achievement| {
                    achievement.achieved = Some(false);
                    achievement.unlock_time = Some(0);
                });
            }
            SteamworksStatsOperation::GlobalAchievementPercentagesRequested => {}
            SteamworksStatsOperation::GlobalAchievementPercentagesReceived { .. } => {}
            SteamworksStatsOperation::AchievementAchievedPercentRead { name, percent } => {
                upsert_global_achievement_percentage(
                    &mut self.achievement_global_percentages,
                    name.clone(),
                    *percent,
                );
            }
            SteamworksStatsOperation::AchievementGlobalPercentagesListed {
                percentages, ..
            } => {
                self.last_global_achievement_percentages
                    .clone_from(percentages);
                for percentage in percentages {
                    upsert_global_achievement_percentage(
                        &mut self.achievement_global_percentages,
                        percentage.api_name.clone(),
                        percentage.percent,
                    );
                }
            }
            SteamworksStatsOperation::GlobalStatsReceived { game_id } => {
                self.last_global_stats_game_id = Some(*game_id);
            }
            SteamworksStatsOperation::GlobalStatsRequested { .. } => {
                self.last_global_stats_game_id = None;
                self.last_global_stat_i64 = None;
                self.last_global_stat_f64 = None;
                self.last_global_stat_history_i64 = None;
                self.last_global_stat_history_f64 = None;
            }
            SteamworksStatsOperation::GlobalStatI64Read { name, value } => {
                self.last_global_stat_i64 = Some(SteamworksGlobalStatValue {
                    name: name.clone(),
                    value: *value,
                });
            }
            SteamworksStatsOperation::GlobalStatF64Read { name, value } => {
                self.last_global_stat_f64 = Some(SteamworksGlobalStatValue {
                    name: name.clone(),
                    value: *value,
                });
            }
            SteamworksStatsOperation::GlobalStatHistoryI64Read { name, values } => {
                self.last_global_stat_history_i64 = Some(SteamworksGlobalStatHistory {
                    name: name.clone(),
                    values: values.clone(),
                });
            }
            SteamworksStatsOperation::GlobalStatHistoryF64Read { name, values } => {
                self.last_global_stat_history_f64 = Some(SteamworksGlobalStatHistory {
                    name: name.clone(),
                    values: values.clone(),
                });
            }
            SteamworksStatsOperation::StatsStoreSubmitted => {
                self.pending_store = false;
                self.force_store = false;
            }
            SteamworksStatsOperation::AllStatsReset { achievements_too } => {
                self.local_stat_i32.clear();
                self.local_stat_f32.clear();
                if *achievements_too {
                    for achievement in &mut self.achievements {
                        achievement.achieved = Some(false);
                        achievement.unlock_time = Some(0);
                    }
                    for achievement in &mut self.last_achievements {
                        achievement.achieved = Some(false);
                        achievement.unlock_time = Some(0);
                    }
                }
            }
            SteamworksStatsOperation::LeaderboardFindSubmitted { name } => {
                self.last_leaderboard_find_request =
                    Some(SteamworksLeaderboardFindRequest { name: name.clone() });
            }
            SteamworksStatsOperation::LeaderboardFindCompleted { name, leaderboard } => {
                self.last_leaderboard_find_result = Some(SteamworksLeaderboardFindResult {
                    name: name.clone(),
                    leaderboard: *leaderboard,
                });
            }
            SteamworksStatsOperation::LeaderboardFindOrCreateSubmitted {
                name,
                sort_method,
                display_type,
            } => {
                self.last_leaderboard_find_or_create_request =
                    Some(SteamworksLeaderboardFindOrCreateRequest {
                        name: name.clone(),
                        sort_method: *sort_method,
                        display_type: *display_type,
                    });
            }
            SteamworksStatsOperation::LeaderboardFindOrCreateCompleted { name, leaderboard } => {
                self.last_leaderboard_find_or_create_result =
                    Some(SteamworksLeaderboardFindOrCreateResult {
                        name: name.clone(),
                        leaderboard: *leaderboard,
                    });
            }
            SteamworksStatsOperation::LeaderboardInfoRead { info } => {
                self.last_leaderboard_info = Some(info.clone());
            }
            SteamworksStatsOperation::LeaderboardScoreUploadSubmitted {
                leaderboard,
                method,
                score,
                details,
            } => {
                self.last_leaderboard_score_upload_request =
                    Some(SteamworksLeaderboardScoreUploadRequest {
                        leaderboard: *leaderboard,
                        method: *method,
                        score: *score,
                        details: details.clone(),
                    });
            }
            SteamworksStatsOperation::LeaderboardScoreUploaded {
                leaderboard,
                upload,
            } => {
                self.last_leaderboard_score_upload_result =
                    Some(SteamworksLeaderboardScoreUploadResult {
                        leaderboard: *leaderboard,
                        upload: upload.clone(),
                    });
            }
            SteamworksStatsOperation::LeaderboardEntriesDownloadSubmitted {
                leaderboard,
                request,
                max_details,
            } => {
                self.last_leaderboard_entries_download_request =
                    Some(SteamworksLeaderboardEntriesDownloadRequest {
                        leaderboard: *leaderboard,
                        request: *request,
                        max_details: *max_details,
                    });
            }
            SteamworksStatsOperation::LeaderboardEntriesDownloaded {
                leaderboard,
                entries,
            } => {
                self.last_leaderboard_entries.clone_from(entries);
                self.last_leaderboard_entries_download_result =
                    Some(SteamworksLeaderboardEntriesDownloadResult {
                        leaderboard: *leaderboard,
                        entries: entries.clone(),
                    });
            }
            SteamworksStatsOperation::LeaderboardForgotten { leaderboard } => {
                self.last_forgotten_leaderboard = Some(*leaderboard);
            }
        }
    }
}

fn named_value<'a, T>(values: &'a [(String, T)], name: &str) -> Option<&'a T> {
    values
        .iter()
        .find_map(|(known_name, value)| (known_name == name).then_some(value))
}

fn upsert_named_value<T>(values: &mut Vec<(String, T)>, name: String, value: T) {
    if let Some((_, known_value)) = values
        .iter_mut()
        .find(|(known_name, _)| known_name == &name)
    {
        *known_value = value;
    } else {
        values.push((name, value));
    }
}

fn update_achievement<F>(
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

fn merge_achievement_info(
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

fn merge_achievement_display_attributes(
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

fn upsert_achievement_display_attribute(
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

fn upsert_global_achievement_percentage(
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
