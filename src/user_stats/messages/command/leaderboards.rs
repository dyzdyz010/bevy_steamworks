use crate::user_stats::{
    SteamworksLeaderboardDataRequest, SteamworksLeaderboardDisplayType, SteamworksLeaderboardId,
    SteamworksLeaderboardSortMethod, SteamworksLeaderboardUploadScoreMethod,
};

use super::SteamworksStatsCommand;

impl SteamworksStatsCommand {
    /// Creates a [`SteamworksStatsCommand::FindLeaderboard`] command.
    pub fn find_leaderboard(name: impl Into<String>) -> Self {
        Self::FindLeaderboard { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::FindOrCreateLeaderboard`] command.
    pub fn find_or_create_leaderboard(
        name: impl Into<String>,
        sort_method: SteamworksLeaderboardSortMethod,
        display_type: SteamworksLeaderboardDisplayType,
    ) -> Self {
        Self::FindOrCreateLeaderboard {
            name: name.into(),
            sort_method,
            display_type,
        }
    }

    /// Creates a [`SteamworksStatsCommand::GetLeaderboardInfo`] command.
    pub fn get_leaderboard_info(leaderboard: SteamworksLeaderboardId) -> Self {
        Self::GetLeaderboardInfo { leaderboard }
    }

    /// Creates a [`SteamworksStatsCommand::UploadLeaderboardScore`] command.
    pub fn upload_leaderboard_score(
        leaderboard: SteamworksLeaderboardId,
        method: SteamworksLeaderboardUploadScoreMethod,
        score: i32,
        details: impl Into<Vec<i32>>,
    ) -> Self {
        Self::UploadLeaderboardScore {
            leaderboard,
            method,
            score,
            details: details.into(),
        }
    }

    /// Creates a [`SteamworksStatsCommand::DownloadLeaderboardEntries`] command.
    pub fn download_leaderboard_entries(
        leaderboard: SteamworksLeaderboardId,
        request: SteamworksLeaderboardDataRequest,
        max_details: usize,
    ) -> Self {
        Self::DownloadLeaderboardEntries {
            leaderboard,
            request,
            max_details,
        }
    }

    /// Creates a global [`SteamworksStatsCommand::DownloadLeaderboardEntries`] command.
    pub fn download_global_leaderboard_entries(
        leaderboard: SteamworksLeaderboardId,
        start: i32,
        end: i32,
        max_details: usize,
    ) -> Self {
        Self::download_leaderboard_entries(
            leaderboard,
            SteamworksLeaderboardDataRequest::Global { start, end },
            max_details,
        )
    }

    /// Creates a user-relative [`SteamworksStatsCommand::DownloadLeaderboardEntries`] command.
    pub fn download_leaderboard_entries_around_user(
        leaderboard: SteamworksLeaderboardId,
        start: i32,
        end: i32,
        max_details: usize,
    ) -> Self {
        Self::download_leaderboard_entries(
            leaderboard,
            SteamworksLeaderboardDataRequest::GlobalAroundUser { start, end },
            max_details,
        )
    }

    /// Creates a friends [`SteamworksStatsCommand::DownloadLeaderboardEntries`] command.
    pub fn download_friends_leaderboard_entries(
        leaderboard: SteamworksLeaderboardId,
        max_details: usize,
    ) -> Self {
        Self::download_leaderboard_entries(
            leaderboard,
            SteamworksLeaderboardDataRequest::Friends,
            max_details,
        )
    }

    /// Creates a [`SteamworksStatsCommand::ForgetLeaderboard`] command.
    pub fn forget_leaderboard(leaderboard: SteamworksLeaderboardId) -> Self {
        Self::ForgetLeaderboard { leaderboard }
    }
}
