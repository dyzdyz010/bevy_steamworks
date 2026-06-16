use super::{
    helpers::{
        merge_achievement_display_attributes, merge_achievement_info, update_achievement,
        upsert_achievement_display_attribute, upsert_global_achievement_percentage,
        upsert_named_value,
    },
    SteamworksAchievementInfo, SteamworksGlobalStatHistory, SteamworksGlobalStatValue,
    SteamworksLeaderboardEntriesDownloadRequest, SteamworksLeaderboardEntriesDownloadResult,
    SteamworksLeaderboardFindOrCreateRequest, SteamworksLeaderboardFindOrCreateResult,
    SteamworksLeaderboardFindRequest, SteamworksLeaderboardFindResult,
    SteamworksLeaderboardScoreUploadRequest, SteamworksLeaderboardScoreUploadResult,
    SteamworksStatsError, SteamworksStatsState,
};

use super::super::{leaderboards::SteamworksStatsLeaderboardHandles, SteamworksStatsOperation};

impl SteamworksStatsState {
    pub(in crate::user_stats) fn record_error(&mut self, error: SteamworksStatsError) {
        self.last_error = Some(error);
    }

    pub(in crate::user_stats) fn sync_leaderboard_count(
        &mut self,
        leaderboards: &SteamworksStatsLeaderboardHandles,
    ) {
        self.leaderboard_count = leaderboards.len();
    }

    pub(in crate::user_stats) fn record_operation(&mut self, operation: &SteamworksStatsOperation) {
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
