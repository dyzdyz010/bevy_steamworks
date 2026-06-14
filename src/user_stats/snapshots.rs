use super::{
    SteamworksLeaderboardEntry, SteamworksLeaderboardId, SteamworksLeaderboardInfo,
    SteamworksLeaderboardScoreUploaded,
};

pub(super) fn snapshot_leaderboard_info(
    stats: &steamworks::UserStats,
    leaderboard: SteamworksLeaderboardId,
    leaderboard_handle: &steamworks::Leaderboard,
) -> SteamworksLeaderboardInfo {
    SteamworksLeaderboardInfo {
        leaderboard,
        name: stats.get_leaderboard_name(leaderboard_handle),
        display_type: stats
            .get_leaderboard_display_type(leaderboard_handle)
            .map(Into::into),
        sort_method: stats
            .get_leaderboard_sort_method(leaderboard_handle)
            .map(Into::into),
        entry_count: stats.get_leaderboard_entry_count(leaderboard_handle),
    }
}

pub(super) fn snapshot_leaderboard_entry(
    entry: steamworks::LeaderboardEntry,
) -> SteamworksLeaderboardEntry {
    SteamworksLeaderboardEntry {
        user: entry.user,
        global_rank: entry.global_rank,
        score: entry.score,
        details: entry.details,
    }
}

pub(super) fn snapshot_leaderboard_score_uploaded(
    upload: steamworks::LeaderboardScoreUploaded,
) -> SteamworksLeaderboardScoreUploaded {
    SteamworksLeaderboardScoreUploaded {
        score: upload.score,
        was_changed: upload.was_changed,
        global_rank_new: upload.global_rank_new,
        global_rank_previous: upload.global_rank_previous,
    }
}
