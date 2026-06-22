use super::super::{
    SteamworksAchievementGlobalPercentage, SteamworksAchievementIconStatus,
    SteamworksAchievementInfo, SteamworksLeaderboardDataRequest, SteamworksLeaderboardDisplayType,
    SteamworksLeaderboardEntry, SteamworksLeaderboardId, SteamworksLeaderboardInfo,
    SteamworksLeaderboardScoreUploaded, SteamworksLeaderboardSortMethod,
    SteamworksLeaderboardUploadScoreMethod, SteamworksUserAchievementStored,
    SteamworksUserStatsReceived, SteamworksUserStatsStored,
};

/// A successfully submitted Steam stats operation.
#[derive(Clone, Debug, PartialEq)]
pub enum SteamworksStatsOperation {
    /// Current-user stats were requested.
    CurrentUserStatsRequested {
        /// Current Steam user id.
        steam_id: steamworks::SteamId,
    },
    /// Stats were requested for a specific Steam user.
    UserStatsRequested {
        /// Requested Steam user id.
        steam_id: steamworks::SteamId,
    },
    /// An integer stat was read.
    StatI32Read {
        /// Steamworks stat API name.
        name: String,
        /// Current value.
        value: i32,
    },
    /// An integer stat was set.
    StatI32Set {
        /// Steamworks stat API name.
        name: String,
        /// Submitted value.
        value: i32,
    },
    /// A floating-point stat was read.
    StatF32Read {
        /// Steamworks stat API name.
        name: String,
        /// Current value.
        value: f32,
    },
    /// A floating-point stat was set.
    StatF32Set {
        /// Steamworks stat API name.
        name: String,
        /// Submitted value.
        value: f32,
    },
    /// Achievement unlock state was read.
    AchievementRead {
        /// Steamworks achievement API name.
        name: String,
        /// Whether the achievement is unlocked.
        achieved: bool,
    },
    /// Achievement count was read.
    AchievementCountRead {
        /// Total achievements reported by Steam for the current app.
        count: u32,
    },
    /// Achievement API names were listed.
    AchievementNamesListed {
        /// Zero-based achievement index the page starts from.
        offset: usize,
        /// Total achievements reported by Steam for the current app.
        total: usize,
        /// Steamworks achievement API names.
        names: Vec<String>,
    },
    /// Achievement snapshots were listed.
    AchievementsListed {
        /// Zero-based achievement index the page starts from.
        offset: usize,
        /// Total achievements reported by Steam for the current app.
        total: usize,
        /// Achievement snapshots.
        achievements: Vec<SteamworksAchievementInfo>,
    },
    /// Achievement icon was read.
    AchievementIconRead {
        /// Steamworks achievement API name.
        name: String,
        /// Icon read status.
        icon: SteamworksAchievementIconStatus,
    },
    /// Achievement icon fetched callback was converted into a stats result.
    AchievementIconFetched {
        /// Steamworks achievement API name.
        name: String,
        /// Whether Steam reported the achievement as achieved in the callback.
        achieved: bool,
        /// Raw icon handle reported by Steam in the callback.
        icon_handle: i32,
        /// Icon read status.
        icon: SteamworksAchievementIconStatus,
    },
    /// User stats received callback was observed.
    UserStatsReceived {
        /// Callback snapshot.
        callback: SteamworksUserStatsReceived,
    },
    /// User stats stored callback was observed.
    UserStatsStored {
        /// Callback snapshot.
        callback: SteamworksUserStatsStored,
    },
    /// User achievement stored callback was observed.
    UserAchievementStored {
        /// Callback snapshot.
        callback: SteamworksUserAchievementStored,
    },
    /// Achievement unlock state and unlock time were read.
    AchievementAndUnlockTimeRead {
        /// Steamworks achievement API name.
        name: String,
        /// Whether the achievement is unlocked.
        achieved: bool,
        /// Unix timestamp in seconds, or zero when Steam has no unlock time.
        unlock_time: u32,
    },
    /// An achievement display attribute was read.
    AchievementDisplayAttributeRead {
        /// Steamworks achievement API name.
        name: String,
        /// Attribute key.
        key: String,
        /// Attribute value.
        value: String,
    },
    /// An achievement unlock was submitted.
    AchievementUnlocked {
        /// Steamworks achievement API name.
        name: String,
    },
    /// An achievement clear was submitted.
    AchievementCleared {
        /// Steamworks achievement API name.
        name: String,
    },
    /// Global achievement percentages were requested.
    GlobalAchievementPercentagesRequested,
    /// Global achievement percentages were received.
    GlobalAchievementPercentagesReceived {
        /// Game id returned by Steam.
        game_id: steamworks::GameId,
    },
    /// A global achievement unlock percentage was read.
    AchievementAchievedPercentRead {
        /// Steamworks achievement API name.
        name: String,
        /// Global unlock percentage.
        percent: f32,
    },
    /// Global achievement unlock percentage snapshots were listed.
    AchievementGlobalPercentagesListed {
        /// Zero-based achievement index the page starts from.
        offset: usize,
        /// Total achievements reported by Steam for the current app.
        total: usize,
        /// Global unlock percentages.
        percentages: Vec<SteamworksAchievementGlobalPercentage>,
    },
    /// Global stat data was requested.
    GlobalStatsRequested {
        /// Requested number of history days.
        history_days: i32,
    },
    /// Global stat data was received.
    GlobalStatsReceived {
        /// Game id returned by Steam.
        game_id: steamworks::GameId,
    },
    /// An aggregated global integer stat was read.
    GlobalStatI64Read {
        /// Steamworks stat API name.
        name: String,
        /// Global stat value.
        value: i64,
    },
    /// An aggregated global floating-point stat was read.
    GlobalStatF64Read {
        /// Steamworks stat API name.
        name: String,
        /// Global stat value.
        value: f64,
    },
    /// Daily history for an aggregated global integer stat was read.
    GlobalStatHistoryI64Read {
        /// Steamworks stat API name.
        name: String,
        /// Daily values from today backwards.
        values: Vec<i64>,
    },
    /// Daily history for an aggregated global floating-point stat was read.
    GlobalStatHistoryF64Read {
        /// Steamworks stat API name.
        name: String,
        /// Daily values from today backwards.
        values: Vec<f64>,
    },
    /// Changed stats and achievements were submitted for storage.
    ///
    /// Final server confirmation arrives later through both
    /// [`crate::SteamworksEvent::UserStatsStored`] and
    /// [`SteamworksStatsOperation::UserStatsStored`]. Stored achievements may
    /// also emit [`SteamworksStatsOperation::UserAchievementStored`].
    StatsStoreSubmitted,
    /// All stats were reset locally.
    AllStatsReset {
        /// Whether achievements were also reset.
        achievements_too: bool,
    },
    /// Leaderboard lookup was submitted to Steam.
    LeaderboardFindSubmitted {
        /// Steamworks leaderboard API name.
        name: String,
    },
    /// Leaderboard lookup completed.
    LeaderboardFindCompleted {
        /// Steamworks leaderboard API name.
        name: String,
        /// Plugin-owned leaderboard ID, or `None` if Steam did not find it.
        leaderboard: Option<SteamworksLeaderboardId>,
    },
    /// Leaderboard find-or-create was submitted to Steam.
    LeaderboardFindOrCreateSubmitted {
        /// Steamworks leaderboard API name.
        name: String,
        /// Requested sort method.
        sort_method: SteamworksLeaderboardSortMethod,
        /// Requested display type.
        display_type: SteamworksLeaderboardDisplayType,
    },
    /// Leaderboard find-or-create completed.
    LeaderboardFindOrCreateCompleted {
        /// Steamworks leaderboard API name.
        name: String,
        /// Plugin-owned leaderboard ID, or `None` if Steam did not return one.
        leaderboard: Option<SteamworksLeaderboardId>,
    },
    /// Leaderboard metadata was read.
    LeaderboardInfoRead {
        /// Leaderboard info snapshot.
        info: SteamworksLeaderboardInfo,
    },
    /// Leaderboard score upload was submitted to Steam.
    LeaderboardScoreUploadSubmitted {
        /// Plugin-owned leaderboard ID.
        leaderboard: SteamworksLeaderboardId,
        /// Upload behavior.
        method: SteamworksLeaderboardUploadScoreMethod,
        /// Submitted score.
        score: i32,
        /// Optional detail integers submitted with the score.
        details: Vec<i32>,
    },
    /// Leaderboard score upload completed.
    LeaderboardScoreUploaded {
        /// Plugin-owned leaderboard ID.
        leaderboard: SteamworksLeaderboardId,
        /// Upload result, or `None` if Steam reported no update.
        upload: Option<SteamworksLeaderboardScoreUploaded>,
    },
    /// Leaderboard entry download was submitted to Steam.
    LeaderboardEntriesDownloadSubmitted {
        /// Plugin-owned leaderboard ID.
        leaderboard: SteamworksLeaderboardId,
        /// Requested entry scope.
        request: SteamworksLeaderboardDataRequest,
        /// Maximum detail integers requested per entry.
        max_details: usize,
    },
    /// Leaderboard entries were downloaded.
    LeaderboardEntriesDownloaded {
        /// Plugin-owned leaderboard ID.
        leaderboard: SteamworksLeaderboardId,
        /// Downloaded entries.
        entries: Vec<SteamworksLeaderboardEntry>,
    },
    /// A leaderboard handle was forgotten by the plugin.
    LeaderboardForgotten {
        /// Plugin-owned leaderboard ID.
        leaderboard: SteamworksLeaderboardId,
    },
}
