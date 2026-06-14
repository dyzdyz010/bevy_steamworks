use thiserror::Error;

use super::super::SteamworksLeaderboardId;

/// Command and asynchronous callback errors from [`crate::SteamworksStatsPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksStatsError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// The upstream Steamworks API rejected the operation.
    #[error("Steamworks user stats operation failed: {operation}")]
    OperationFailed {
        /// Operation that failed.
        operation: &'static str,
    },
    /// The upstream Steamworks API returned an explicit Steam error.
    #[error("Steamworks user stats operation {operation} failed: {source}")]
    SteamError {
        /// Operation that failed.
        operation: &'static str,
        /// Steamworks error.
        source: steamworks::SteamError,
    },
    /// A leaderboard ID is not owned by this plugin.
    #[error("Steamworks leaderboard {id:?} was not found")]
    LeaderboardNotFound {
        /// Missing leaderboard ID.
        id: SteamworksLeaderboardId,
    },
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks stats command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// Leaderboard details exceed the per-command cap.
    #[error("Steamworks leaderboard details length {requested} exceeds max {max_supported}")]
    TooManyLeaderboardDetails {
        /// Requested detail count.
        requested: usize,
        /// Maximum accepted detail count.
        max_supported: usize,
    },
    /// Leaderboard download range is invalid.
    #[error("Steamworks leaderboard download range {start}..={end} is invalid")]
    InvalidLeaderboardRange {
        /// Inclusive start index.
        start: i32,
        /// Inclusive end index.
        end: i32,
    },
    /// Leaderboard download range exceeds the per-command cap.
    #[error("Steamworks leaderboard download entry count {requested} exceeds max {max_supported}")]
    TooManyLeaderboardEntries {
        /// Requested entry count.
        requested: usize,
        /// Maximum accepted entry count.
        max_supported: usize,
    },
    /// Achievement catalog page limit exceeds the per-command cap.
    #[error("Steamworks achievement catalog page limit {requested} exceeds max {max_supported}")]
    TooManyAchievementEntries {
        /// Requested achievement count.
        requested: usize,
        /// Maximum accepted achievement count.
        max_supported: usize,
    },
}

impl SteamworksStatsError {
    pub(in crate::user_stats) fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }

    pub(in crate::user_stats) fn steam_error(
        operation: &'static str,
        source: steamworks::SteamError,
    ) -> Self {
        Self::SteamError { operation, source }
    }

    pub(in crate::user_stats) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }
}
