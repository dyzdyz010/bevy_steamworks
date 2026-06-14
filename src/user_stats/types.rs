/// Owned snapshot of one Steam achievement.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SteamworksAchievementInfo {
    /// Steam achievement API name.
    pub api_name: String,
    /// Localized display name, if requested and available.
    pub display_name: Option<String>,
    /// Localized description, if requested and available.
    pub description: Option<String>,
    /// Whether the achievement is hidden, if requested and available.
    pub hidden: Option<bool>,
    /// Whether the current user has achieved it, if requested and available.
    pub achieved: Option<bool>,
    /// Unix epoch seconds when the current user unlocked it, if requested and available.
    pub unlock_time: Option<u32>,
}

/// Global unlock percentage snapshot for one Steam achievement.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksAchievementGlobalPercentage {
    /// Steam achievement API name.
    pub api_name: String,
    /// Percentage of players who have unlocked this achievement.
    pub percent: f32,
}

/// Localized display attribute snapshot for one Steam achievement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksAchievementDisplayAttribute {
    /// Steam achievement API name.
    pub api_name: String,
    /// Display attribute key, such as `"name"`, `"desc"`, or `"hidden"`.
    pub key: String,
    /// Attribute value returned by Steam.
    pub value: String,
}

/// Aggregated global stat value snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksGlobalStatValue<T> {
    /// Steamworks stat API name.
    pub name: String,
    /// Aggregated global value.
    pub value: T,
}

/// Aggregated global stat history snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksGlobalStatHistory<T> {
    /// Steamworks stat API name.
    pub name: String,
    /// Daily values from today backwards.
    pub values: Vec<T>,
}

/// RGBA icon snapshot for one Steam achievement.
///
/// The upstream `steamworks` wrapper exposes achievement icons as 64x64 RGBA
/// bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksAchievementIcon {
    /// Steam achievement API name.
    pub api_name: String,
    /// Icon width in pixels.
    pub width: u32,
    /// Icon height in pixels.
    pub height: u32,
    /// RGBA bytes, four bytes per pixel.
    pub rgba: Vec<u8>,
}

/// User stats received callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUserStatsReceived {
    /// User whose stats were received.
    pub steam_id: steamworks::SteamId,
    /// Game ID reported by Steam.
    pub game_id: steamworks::GameId,
    /// Steam result for the stats request.
    pub result: Result<(), steamworks::SteamError>,
}

/// User stats stored callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUserStatsStored {
    /// Game ID reported by Steam.
    pub game_id: steamworks::GameId,
    /// Steam result for the store request.
    pub result: Result<(), steamworks::SteamError>,
}

/// User achievement stored callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUserAchievementStored {
    /// Game ID reported by Steam.
    pub game_id: steamworks::GameId,
    /// Steamworks achievement API name.
    pub achievement_name: String,
    /// Current progress toward the achievement.
    pub current_progress: u32,
    /// Required progress to unlock the achievement.
    ///
    /// Steam reports both progress fields as zero when the achievement was
    /// fully unlocked.
    pub max_progress: u32,
}

/// Result of reading a Steam achievement icon through the upstream safe wrapper.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksAchievementIconStatus {
    /// The icon RGBA bytes were available.
    Available(SteamworksAchievementIcon),
    /// The icon bytes are not currently available.
    ///
    /// The upstream safe wrapper exposes this as a single `None`, which can mean
    /// Steam is still fetching the icon, the icon is missing, the image is not
    /// the 64x64 size returned by `get_achievement_icon`, or Steam image reads
    /// failed. A later [`crate::SteamworksEvent::UserAchievementIconFetched`]
    /// may make the icon available.
    PendingOrUnavailable,
}

impl SteamworksAchievementIconStatus {
    /// Returns the available icon snapshot, if one was returned.
    pub fn as_icon(&self) -> Option<&SteamworksAchievementIcon> {
        match self {
            Self::Available(icon) => Some(icon),
            Self::PendingOrUnavailable => None,
        }
    }
}

/// Opaque ID for a leaderboard handle owned by [`crate::SteamworksStatsPlugin`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SteamworksLeaderboardId(u64);

impl SteamworksLeaderboardId {
    /// Creates an ID from a raw integer.
    pub fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw integer backing this plugin-owned ID.
    pub fn raw(self) -> u64 {
        self.0
    }
}

/// Snapshot of a Steam leaderboard handle known to this plugin.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLeaderboardInfo {
    /// Plugin-owned leaderboard ID.
    pub leaderboard: SteamworksLeaderboardId,
    /// Name reported by Steam.
    pub name: String,
    /// Display type reported by Steam, if the handle is valid.
    pub display_type: Option<SteamworksLeaderboardDisplayType>,
    /// Sort method reported by Steam, if the handle is valid.
    pub sort_method: Option<SteamworksLeaderboardSortMethod>,
    /// Total number of entries reported by Steam.
    pub entry_count: i32,
}

/// Snapshot of a submitted leaderboard find request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLeaderboardFindRequest {
    /// Steamworks leaderboard API name.
    pub name: String,
}

/// Snapshot of a completed leaderboard find operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLeaderboardFindResult {
    /// Steamworks leaderboard API name.
    pub name: String,
    /// Plugin-owned leaderboard ID, or `None` if Steam did not find it.
    pub leaderboard: Option<SteamworksLeaderboardId>,
}

/// Snapshot of a submitted leaderboard find-or-create request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLeaderboardFindOrCreateRequest {
    /// Steamworks leaderboard API name.
    pub name: String,
    /// Requested sort method.
    pub sort_method: SteamworksLeaderboardSortMethod,
    /// Requested display type.
    pub display_type: SteamworksLeaderboardDisplayType,
}

/// Snapshot of a completed leaderboard find-or-create operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLeaderboardFindOrCreateResult {
    /// Steamworks leaderboard API name.
    pub name: String,
    /// Plugin-owned leaderboard ID, or `None` if Steam did not return one.
    pub leaderboard: Option<SteamworksLeaderboardId>,
}

/// Leaderboard data request scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksLeaderboardDataRequest {
    /// Global leaderboard entries by absolute rank range.
    Global {
        /// Inclusive start rank. Steam ranks are one-based for global requests.
        start: i32,
        /// Inclusive end rank.
        end: i32,
    },
    /// Entries around the current user, using signed offsets from the current user's rank.
    GlobalAroundUser {
        /// Inclusive start offset, such as `-2`.
        start: i32,
        /// Inclusive end offset, such as `2`.
        end: i32,
    },
    /// Entries for the current user's friends.
    Friends,
}

impl From<SteamworksLeaderboardDataRequest> for steamworks::LeaderboardDataRequest {
    fn from(value: SteamworksLeaderboardDataRequest) -> Self {
        match value {
            SteamworksLeaderboardDataRequest::Global { .. } => Self::Global,
            SteamworksLeaderboardDataRequest::GlobalAroundUser { .. } => Self::GlobalAroundUser,
            SteamworksLeaderboardDataRequest::Friends => Self::Friends,
        }
    }
}

impl SteamworksLeaderboardDataRequest {
    pub(super) fn upstream_range(self) -> (usize, usize) {
        match self {
            SteamworksLeaderboardDataRequest::Global { start, end }
            | SteamworksLeaderboardDataRequest::GlobalAroundUser { start, end } => (
                leaderboard_bound_to_upstream_usize(start),
                leaderboard_bound_to_upstream_usize(end),
            ),
            SteamworksLeaderboardDataRequest::Friends => (0, 0),
        }
    }
}

/// Leaderboard score upload behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksLeaderboardUploadScoreMethod {
    /// Keep the user's best existing score.
    KeepBest,
    /// Force Steam to update the score.
    ForceUpdate,
}

impl From<SteamworksLeaderboardUploadScoreMethod> for steamworks::UploadScoreMethod {
    fn from(value: SteamworksLeaderboardUploadScoreMethod) -> Self {
        match value {
            SteamworksLeaderboardUploadScoreMethod::KeepBest => Self::KeepBest,
            SteamworksLeaderboardUploadScoreMethod::ForceUpdate => Self::ForceUpdate,
        }
    }
}

/// Leaderboard sort direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksLeaderboardSortMethod {
    /// Lower scores rank first.
    Ascending,
    /// Higher scores rank first.
    Descending,
}

impl From<SteamworksLeaderboardSortMethod> for steamworks::LeaderboardSortMethod {
    fn from(value: SteamworksLeaderboardSortMethod) -> Self {
        match value {
            SteamworksLeaderboardSortMethod::Ascending => Self::Ascending,
            SteamworksLeaderboardSortMethod::Descending => Self::Descending,
        }
    }
}

impl From<steamworks::LeaderboardSortMethod> for SteamworksLeaderboardSortMethod {
    fn from(value: steamworks::LeaderboardSortMethod) -> Self {
        match value {
            steamworks::LeaderboardSortMethod::Ascending => Self::Ascending,
            steamworks::LeaderboardSortMethod::Descending => Self::Descending,
        }
    }
}

/// Leaderboard display format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksLeaderboardDisplayType {
    /// Display scores as plain numbers.
    Numeric,
    /// Display scores as seconds.
    TimeSeconds,
    /// Display scores as milliseconds.
    TimeMilliSeconds,
}

impl From<SteamworksLeaderboardDisplayType> for steamworks::LeaderboardDisplayType {
    fn from(value: SteamworksLeaderboardDisplayType) -> Self {
        match value {
            SteamworksLeaderboardDisplayType::Numeric => Self::Numeric,
            SteamworksLeaderboardDisplayType::TimeSeconds => Self::TimeSeconds,
            SteamworksLeaderboardDisplayType::TimeMilliSeconds => Self::TimeMilliSeconds,
        }
    }
}

impl From<steamworks::LeaderboardDisplayType> for SteamworksLeaderboardDisplayType {
    fn from(value: steamworks::LeaderboardDisplayType) -> Self {
        match value {
            steamworks::LeaderboardDisplayType::Numeric => Self::Numeric,
            steamworks::LeaderboardDisplayType::TimeSeconds => Self::TimeSeconds,
            steamworks::LeaderboardDisplayType::TimeMilliSeconds => Self::TimeMilliSeconds,
        }
    }
}

/// Snapshot of one downloaded leaderboard entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLeaderboardEntry {
    /// Steam user that owns this entry.
    pub user: steamworks::SteamId,
    /// Global rank returned by Steam.
    pub global_rank: i32,
    /// Score returned by Steam.
    pub score: i32,
    /// Optional details returned by Steam.
    pub details: Vec<i32>,
}

/// Snapshot of a leaderboard score upload result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLeaderboardScoreUploaded {
    /// Score submitted or retained by Steam.
    pub score: i32,
    /// Whether Steam changed the score.
    pub was_changed: bool,
    /// New global rank.
    pub global_rank_new: i32,
    /// Previous global rank.
    pub global_rank_previous: i32,
}

/// Snapshot of a submitted leaderboard score upload request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLeaderboardScoreUploadRequest {
    /// Plugin-owned leaderboard ID.
    pub leaderboard: SteamworksLeaderboardId,
    /// Upload behavior.
    pub method: SteamworksLeaderboardUploadScoreMethod,
    /// Submitted score.
    pub score: i32,
    /// Optional detail integers submitted with the score.
    pub details: Vec<i32>,
}

/// Snapshot of a completed leaderboard score upload operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLeaderboardScoreUploadResult {
    /// Plugin-owned leaderboard ID.
    pub leaderboard: SteamworksLeaderboardId,
    /// Upload result, or `None` if Steam reported no update.
    pub upload: Option<SteamworksLeaderboardScoreUploaded>,
}

/// Snapshot of a submitted leaderboard entries download request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SteamworksLeaderboardEntriesDownloadRequest {
    /// Plugin-owned leaderboard ID.
    pub leaderboard: SteamworksLeaderboardId,
    /// Entry scope requested from Steam.
    pub request: SteamworksLeaderboardDataRequest,
    /// Maximum detail integers requested per entry.
    pub max_details: usize,
}

/// Snapshot of a completed leaderboard entries download operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLeaderboardEntriesDownloadResult {
    /// Plugin-owned leaderboard ID.
    pub leaderboard: SteamworksLeaderboardId,
    /// Downloaded entries.
    pub entries: Vec<SteamworksLeaderboardEntry>,
}

fn leaderboard_bound_to_upstream_usize(bound: i32) -> usize {
    // The upstream safe wrapper accepts `usize`, but Steam's
    // DownloadLeaderboardEntries API interprets these bounds as signed i32.
    // Passing the i32 bit pattern through usize lets the wrapper cast it back
    // to Steam's signed type without leaving the safe `steamworks` API.
    bound as usize
}
