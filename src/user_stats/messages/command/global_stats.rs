use super::SteamworksStatsCommand;

impl SteamworksStatsCommand {
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
}
