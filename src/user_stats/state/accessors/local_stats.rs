use super::super::{helpers::named_value, SteamworksStatsState};

impl SteamworksStatsState {
    /// Returns the most recent current-user integer stat value read or set through this plugin.
    pub fn stat_i32(&self, name: &str) -> Option<i32> {
        named_value(&self.local_stat_i32, name).copied()
    }

    /// Returns current-user integer stat snapshots cached by this plugin.
    pub fn stat_i32_values(&self) -> impl Iterator<Item = (&str, i32)> + '_ {
        self.local_stat_i32
            .iter()
            .map(|(name, value)| (name.as_str(), *value))
    }

    /// Returns the number of current-user integer stat snapshots cached by this plugin.
    pub fn stat_i32_count(&self) -> usize {
        self.local_stat_i32.len()
    }

    /// Returns whether this plugin has cached a current-user integer stat.
    pub fn has_stat_i32(&self, name: &str) -> bool {
        self.stat_i32(name).is_some()
    }

    /// Returns the most recent current-user floating-point stat value read or set through this plugin.
    pub fn stat_f32(&self, name: &str) -> Option<f32> {
        named_value(&self.local_stat_f32, name).copied()
    }

    /// Returns current-user floating-point stat snapshots cached by this plugin.
    pub fn stat_f32_values(&self) -> impl Iterator<Item = (&str, f32)> + '_ {
        self.local_stat_f32
            .iter()
            .map(|(name, value)| (name.as_str(), *value))
    }

    /// Returns the number of current-user floating-point stat snapshots cached by this plugin.
    pub fn stat_f32_count(&self) -> usize {
        self.local_stat_f32.len()
    }

    /// Returns whether this plugin has cached a current-user floating-point stat.
    pub fn has_stat_f32(&self, name: &str) -> bool {
        self.stat_f32(name).is_some()
    }
}
