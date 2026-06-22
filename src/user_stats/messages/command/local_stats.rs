use super::SteamworksStatsCommand;

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
}
