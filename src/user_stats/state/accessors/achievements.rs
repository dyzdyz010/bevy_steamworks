use super::super::{
    SteamworksAchievementDisplayAttribute, SteamworksAchievementIcon, SteamworksAchievementInfo,
    SteamworksStatsState,
};

impl SteamworksStatsState {
    /// Returns the most recent achievement count read through this plugin.
    pub fn achievement_count(&self) -> Option<u32> {
        self.achievement_count
    }

    /// Returns the most recent achievement catalog snapshot.
    pub fn last_achievements(&self) -> &[SteamworksAchievementInfo] {
        &self.last_achievements
    }

    /// Returns all latest achievement snapshots cached by API name.
    pub fn achievements(&self) -> &[SteamworksAchievementInfo] {
        &self.achievements
    }

    /// Returns the number of latest achievement snapshots cached by API name.
    pub fn known_achievement_count(&self) -> usize {
        self.achievements.len()
    }

    /// Returns known achievement API names.
    pub fn achievement_names(&self) -> impl Iterator<Item = &str> + '_ {
        self.achievements
            .iter()
            .map(|achievement| achievement.api_name.as_str())
    }

    /// Returns the cached achievement snapshot for an API name.
    pub fn achievement(&self, api_name: &str) -> Option<&SteamworksAchievementInfo> {
        self.achievements
            .iter()
            .find(|achievement| achievement.api_name == api_name)
    }

    /// Returns whether any achievement snapshot is cached for an API name.
    pub fn has_achievement(&self, api_name: &str) -> bool {
        self.achievement(api_name).is_some()
    }

    /// Returns known achievements whose current-user unlock state is cached as unlocked.
    pub fn unlocked_achievements(&self) -> impl Iterator<Item = &SteamworksAchievementInfo> + '_ {
        self.achievements
            .iter()
            .filter(|achievement| achievement.achieved == Some(true))
    }

    /// Returns known achievements whose current-user unlock state is cached as locked.
    pub fn locked_achievements(&self) -> impl Iterator<Item = &SteamworksAchievementInfo> + '_ {
        self.achievements
            .iter()
            .filter(|achievement| achievement.achieved == Some(false))
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

    /// Returns a cached achievement display name, preserving a known achievement with no display name as `Some(None)`.
    pub fn achievement_display_name(&self, api_name: &str) -> Option<Option<&str>> {
        self.achievement(api_name)
            .map(|achievement| achievement.display_name.as_deref())
    }

    /// Returns a cached achievement description, preserving a known achievement with no description as `Some(None)`.
    pub fn achievement_description(&self, api_name: &str) -> Option<Option<&str>> {
        self.achievement(api_name)
            .map(|achievement| achievement.description.as_deref())
    }

    /// Returns a cached achievement hidden flag, preserving a known achievement with no hidden flag as `Some(None)`.
    pub fn achievement_hidden(&self, api_name: &str) -> Option<Option<bool>> {
        self.achievement(api_name)
            .map(|achievement| achievement.hidden)
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
}
