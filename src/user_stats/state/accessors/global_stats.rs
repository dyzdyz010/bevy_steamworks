use super::super::{
    SteamworksAchievementGlobalPercentage, SteamworksGlobalStatHistory, SteamworksGlobalStatValue,
    SteamworksStatsState,
};

impl SteamworksStatsState {
    /// Returns the most recent global achievement percentage page.
    pub fn last_global_achievement_percentages(&self) -> &[SteamworksAchievementGlobalPercentage] {
        &self.last_global_achievement_percentages
    }

    /// Returns all global achievement percentage snapshots cached by API name.
    pub fn global_achievement_percentages(&self) -> &[SteamworksAchievementGlobalPercentage] {
        &self.achievement_global_percentages
    }

    /// Returns the number of global achievement percentage snapshots cached by API name.
    pub fn global_achievement_percentage_count(&self) -> usize {
        self.achievement_global_percentages.len()
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

    /// Returns the cached aggregated global integer stat history values for one stat name.
    pub fn global_stat_history_i64_series(&self, name: &str) -> Option<&[i64]> {
        self.global_stat_history_i64(name)
            .map(|history| history.values.as_slice())
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

    /// Returns the cached aggregated global floating-point stat history values for one stat name.
    pub fn global_stat_history_f64_series(&self, name: &str) -> Option<&[f64]> {
        self.global_stat_history_f64(name)
            .map(|history| history.values.as_slice())
    }
}
