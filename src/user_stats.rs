//! High-level Bevy ECS integration for Steam user stats and achievements.
//!
//! This module builds on top of the upstream [`steamworks::UserStats`] API.
//! Games can keep using the raw Steamworks API through [`SteamworksClient`],
//! while this plugin provides a message-driven layer for common Bevy workflows.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
    system::SystemParam,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

/// Maximum leaderboard detail integers accepted by one command.
pub const STEAMWORKS_LEADERBOARD_MAX_DETAILS: usize = 64;

/// Maximum leaderboard entries requested by one download command.
pub const STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND: usize = 1000;

/// Default achievement catalog items read by one command.
pub const STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND: usize = 64;

/// Maximum achievement catalog items accepted by one command.
pub const STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND: usize = 256;

/// Settings used by [`SteamworksStatsPlugin`].
#[derive(Clone, Debug, Resource)]
pub struct SteamworksStatsSettings {
    /// Request stats for the current Steam user when the plugin starts.
    ///
    /// Steam stats and achievement reads/writes require user stats to be loaded.
    /// The upstream `steamworks` crate exposes this through
    /// [`steamworks::UserStats::request_user_stats`], so this plugin requests
    /// stats for [`steamworks::User::steam_id`] by default.
    pub request_current_user_stats_on_startup: bool,
    /// Call [`steamworks::UserStats::store_stats`] once after a frame with
    /// successful stat or achievement writes.
    pub auto_store: bool,
}

impl Default for SteamworksStatsSettings {
    fn default() -> Self {
        Self {
            request_current_user_stats_on_startup: true,
            auto_store: true,
        }
    }
}

/// Runtime state for [`SteamworksStatsPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksStatsState {
    current_user_stats_requested: bool,
    pending_store: bool,
    last_error: Option<SteamworksStatsError>,
    last_achievements: Vec<SteamworksAchievementInfo>,
    last_achievement_icon: Option<SteamworksAchievementIcon>,
    achievement_icon_callback_count: u64,
    last_user_stats_received: Option<SteamworksUserStatsReceived>,
    last_user_stats_stored: Option<SteamworksUserStatsStored>,
    last_user_achievement_stored: Option<SteamworksUserAchievementStored>,
    last_global_achievement_percentages: Vec<SteamworksAchievementGlobalPercentage>,
    last_global_stats_game_id: Option<steamworks::GameId>,
    last_global_stat_i64: Option<SteamworksGlobalStatValue<i64>>,
    last_global_stat_f64: Option<SteamworksGlobalStatValue<f64>>,
    last_global_stat_history_i64: Option<SteamworksGlobalStatHistory<i64>>,
    last_global_stat_history_f64: Option<SteamworksGlobalStatHistory<f64>>,
    leaderboard_count: usize,
    last_leaderboard_info: Option<SteamworksLeaderboardInfo>,
    last_leaderboard_entries: Vec<SteamworksLeaderboardEntry>,
}

impl SteamworksStatsState {
    /// Returns whether this plugin has requested stats for the current user.
    pub fn current_user_stats_requested(&self) -> bool {
        self.current_user_stats_requested
    }

    /// Returns whether successful writes are waiting for `store_stats`.
    pub fn pending_store(&self) -> bool {
        self.pending_store
    }

    /// Returns the most recent command or asynchronous callback error observed by the stats plugin.
    pub fn last_error(&self) -> Option<&SteamworksStatsError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent achievement catalog snapshot.
    pub fn last_achievements(&self) -> &[SteamworksAchievementInfo] {
        &self.last_achievements
    }

    /// Returns the most recent achievement icon snapshot read through this plugin.
    pub fn last_achievement_icon(&self) -> Option<&SteamworksAchievementIcon> {
        self.last_achievement_icon.as_ref()
    }

    /// Returns how many achievement icon fetched callbacks this plugin observed.
    pub fn achievement_icon_callback_count(&self) -> u64 {
        self.achievement_icon_callback_count
    }

    /// Returns the most recent user stats received callback snapshot.
    pub fn last_user_stats_received(&self) -> Option<&SteamworksUserStatsReceived> {
        self.last_user_stats_received.as_ref()
    }

    /// Returns the most recent user stats stored callback snapshot.
    pub fn last_user_stats_stored(&self) -> Option<&SteamworksUserStatsStored> {
        self.last_user_stats_stored.as_ref()
    }

    /// Returns the most recent achievement stored callback snapshot.
    pub fn last_user_achievement_stored(&self) -> Option<&SteamworksUserAchievementStored> {
        self.last_user_achievement_stored.as_ref()
    }

    /// Returns the most recent global achievement percentage page.
    pub fn last_global_achievement_percentages(&self) -> &[SteamworksAchievementGlobalPercentage] {
        &self.last_global_achievement_percentages
    }

    /// Returns the most recent global stats received callback game ID.
    pub fn last_global_stats_game_id(&self) -> Option<steamworks::GameId> {
        self.last_global_stats_game_id
    }

    /// Returns the most recent aggregated global integer stat read through this plugin.
    pub fn last_global_stat_i64(&self) -> Option<&SteamworksGlobalStatValue<i64>> {
        self.last_global_stat_i64.as_ref()
    }

    /// Returns the most recent aggregated global floating-point stat read through this plugin.
    pub fn last_global_stat_f64(&self) -> Option<&SteamworksGlobalStatValue<f64>> {
        self.last_global_stat_f64.as_ref()
    }

    /// Returns the most recent aggregated global integer stat history read through this plugin.
    pub fn last_global_stat_history_i64(&self) -> Option<&SteamworksGlobalStatHistory<i64>> {
        self.last_global_stat_history_i64.as_ref()
    }

    /// Returns the most recent aggregated global floating-point stat history read through this plugin.
    pub fn last_global_stat_history_f64(&self) -> Option<&SteamworksGlobalStatHistory<f64>> {
        self.last_global_stat_history_f64.as_ref()
    }

    /// Returns the number of leaderboard handles currently owned by this plugin.
    pub fn leaderboard_count(&self) -> usize {
        self.leaderboard_count
    }

    /// Returns the most recent leaderboard info read through this plugin.
    pub fn last_leaderboard_info(&self) -> Option<&SteamworksLeaderboardInfo> {
        self.last_leaderboard_info.as_ref()
    }

    /// Returns the most recent downloaded leaderboard entries.
    pub fn last_leaderboard_entries(&self) -> &[SteamworksLeaderboardEntry] {
        &self.last_leaderboard_entries
    }

    fn record_error(&mut self, error: SteamworksStatsError) {
        self.last_error = Some(error);
    }

    fn sync_leaderboard_count(&mut self, leaderboards: &SteamworksStatsLeaderboardHandles) {
        self.leaderboard_count = leaderboards.len();
    }

    fn record_operation(&mut self, operation: &SteamworksStatsOperation) {
        match operation {
            SteamworksStatsOperation::AchievementNamesListed { names, .. } => {
                self.last_achievements = names
                    .iter()
                    .map(|api_name| SteamworksAchievementInfo {
                        api_name: api_name.clone(),
                        ..Default::default()
                    })
                    .collect();
            }
            SteamworksStatsOperation::AchievementsListed { achievements, .. } => {
                self.last_achievements.clone_from(achievements);
            }
            SteamworksStatsOperation::AchievementIconRead { icon, .. } => {
                if let Some(icon) = icon.as_icon() {
                    self.last_achievement_icon = Some(icon.clone());
                }
            }
            SteamworksStatsOperation::AchievementIconFetched { icon, .. } => {
                if let Some(icon) = icon.as_icon() {
                    self.last_achievement_icon = Some(icon.clone());
                }
                self.achievement_icon_callback_count =
                    self.achievement_icon_callback_count.saturating_add(1);
            }
            SteamworksStatsOperation::UserStatsReceived { callback } => {
                self.last_user_stats_received = Some(callback.clone());
            }
            SteamworksStatsOperation::UserStatsStored { callback } => {
                self.last_user_stats_stored = Some(callback.clone());
            }
            SteamworksStatsOperation::UserAchievementStored { callback } => {
                self.last_user_achievement_stored = Some(callback.clone());
            }
            SteamworksStatsOperation::AchievementGlobalPercentagesListed {
                percentages, ..
            } => {
                self.last_global_achievement_percentages
                    .clone_from(percentages);
            }
            SteamworksStatsOperation::GlobalStatsReceived { game_id } => {
                self.last_global_stats_game_id = Some(*game_id);
            }
            SteamworksStatsOperation::GlobalStatsRequested { .. } => {
                self.last_global_stats_game_id = None;
                self.last_global_stat_i64 = None;
                self.last_global_stat_f64 = None;
                self.last_global_stat_history_i64 = None;
                self.last_global_stat_history_f64 = None;
            }
            SteamworksStatsOperation::GlobalStatI64Read { name, value } => {
                self.last_global_stat_i64 = Some(SteamworksGlobalStatValue {
                    name: name.clone(),
                    value: *value,
                });
            }
            SteamworksStatsOperation::GlobalStatF64Read { name, value } => {
                self.last_global_stat_f64 = Some(SteamworksGlobalStatValue {
                    name: name.clone(),
                    value: *value,
                });
            }
            SteamworksStatsOperation::GlobalStatHistoryI64Read { name, values } => {
                self.last_global_stat_history_i64 = Some(SteamworksGlobalStatHistory {
                    name: name.clone(),
                    values: values.clone(),
                });
            }
            SteamworksStatsOperation::GlobalStatHistoryF64Read { name, values } => {
                self.last_global_stat_history_f64 = Some(SteamworksGlobalStatHistory {
                    name: name.clone(),
                    values: values.clone(),
                });
            }
            SteamworksStatsOperation::LeaderboardInfoRead { info } => {
                self.last_leaderboard_info = Some(info.clone());
            }
            SteamworksStatsOperation::LeaderboardEntriesDownloaded { entries, .. } => {
                self.last_leaderboard_entries.clone_from(entries);
            }
            _ => {}
        }
    }
}

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

/// Opaque ID for a leaderboard handle owned by [`SteamworksStatsPlugin`].
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
    fn upstream_range(self) -> (usize, usize) {
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

#[derive(Clone, Debug, Default, Resource)]
struct SteamworksStatsAsyncResults {
    queue: Arc<Mutex<Vec<SteamworksStatsResult>>>,
}

impl SteamworksStatsAsyncResults {
    fn push(&self, result: SteamworksStatsResult) {
        self.queue
            .lock()
            .expect("Steamworks stats async result mutex was poisoned")
            .push(result);
    }

    fn drain(&self) -> Vec<SteamworksStatsResult> {
        self.queue
            .lock()
            .expect("Steamworks stats async result mutex was poisoned")
            .drain(..)
            .collect()
    }
}

#[derive(Clone, Debug, Default, Resource)]
struct SteamworksStatsLeaderboardHandles {
    storage: Arc<Mutex<SteamworksStatsLeaderboardHandleStorage>>,
}

impl SteamworksStatsLeaderboardHandles {
    fn insert(&self, leaderboard: steamworks::Leaderboard) -> SteamworksLeaderboardId {
        self.storage
            .lock()
            .expect("Steamworks leaderboard handle storage mutex was poisoned")
            .insert(leaderboard)
    }

    fn get(&self, id: SteamworksLeaderboardId) -> Option<steamworks::Leaderboard> {
        self.storage
            .lock()
            .expect("Steamworks leaderboard handle storage mutex was poisoned")
            .get(id)
    }

    fn remove(&self, id: SteamworksLeaderboardId) -> Option<steamworks::Leaderboard> {
        self.storage
            .lock()
            .expect("Steamworks leaderboard handle storage mutex was poisoned")
            .remove(id)
    }

    fn len(&self) -> usize {
        self.storage
            .lock()
            .expect("Steamworks leaderboard handle storage mutex was poisoned")
            .len()
    }
}

#[derive(Debug)]
struct SteamworksStatsLeaderboardHandleStorage {
    next_id: u64,
    handles: HashMap<SteamworksLeaderboardId, steamworks::Leaderboard>,
}

impl Default for SteamworksStatsLeaderboardHandleStorage {
    fn default() -> Self {
        Self {
            next_id: 1,
            handles: HashMap::default(),
        }
    }
}

impl SteamworksStatsLeaderboardHandleStorage {
    fn insert(&mut self, leaderboard: steamworks::Leaderboard) -> SteamworksLeaderboardId {
        if let Some((id, _)) = self
            .handles
            .iter()
            .find(|(_, known)| known.raw() == leaderboard.raw())
        {
            return *id;
        }

        let id = SteamworksLeaderboardId(self.next_id);
        self.next_id = self.next_id.saturating_add(1).max(1);
        self.handles.insert(id, leaderboard);
        id
    }

    fn get(&self, id: SteamworksLeaderboardId) -> Option<steamworks::Leaderboard> {
        self.handles.get(&id).cloned()
    }

    fn remove(&mut self, id: SteamworksLeaderboardId) -> Option<steamworks::Leaderboard> {
        self.handles.remove(&id)
    }

    fn len(&self) -> usize {
        self.handles.len()
    }
}

/// A high-level command for Steam user stats and achievements.
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksStatsCommand {
    /// Request stats for the current Steam user.
    RequestCurrentUserStats,
    /// Request stats for a specific Steam user.
    RequestUserStats {
        /// Steam user to request stats for.
        steam_id: steamworks::SteamId,
    },
    /// Read the current user's integer stat.
    GetStatI32 {
        /// Steamworks stat API name.
        name: String,
    },
    /// Set the current user's integer stat.
    SetStatI32 {
        /// Steamworks stat API name.
        name: String,
        /// New stat value.
        value: i32,
    },
    /// Read the current user's floating-point stat.
    GetStatF32 {
        /// Steamworks stat API name.
        name: String,
    },
    /// Set the current user's floating-point stat.
    SetStatF32 {
        /// Steamworks stat API name.
        name: String,
        /// New stat value.
        value: f32,
    },
    /// Read whether an achievement is unlocked for the current user.
    GetAchievement {
        /// Steamworks achievement API name.
        name: String,
    },
    /// List achievement API names for the current app.
    ///
    /// The upstream safe wrapper enumerates the catalog names internally. Keep
    /// this as a startup or tooling command, and use pages instead of doing
    /// repeated full catalog work every frame.
    ListAchievementNames {
        /// Zero-based achievement index to start from.
        offset: usize,
        /// Maximum names returned by this command.
        limit: usize,
    },
    /// List achievement snapshots for the current app.
    ListAchievements {
        /// Include localized display name, description, and hidden flag.
        include_display_attributes: bool,
        /// Include current-user unlock state and unlock time.
        include_unlock_state: bool,
        /// Zero-based achievement index to start from.
        offset: usize,
        /// Maximum achievement snapshots returned by this command.
        limit: usize,
    },
    /// Read a 64x64 RGBA icon for an achievement.
    GetAchievementIcon {
        /// Steamworks achievement API name.
        name: String,
    },
    /// Unlock an achievement for the current user.
    UnlockAchievement {
        /// Steamworks achievement API name.
        name: String,
    },
    /// Clear an achievement for the current user.
    ClearAchievement {
        /// Steamworks achievement API name.
        name: String,
    },
    /// Read achievement unlock state and unlock time.
    GetAchievementAndUnlockTime {
        /// Steamworks achievement API name.
        name: String,
    },
    /// Read a localized achievement display attribute.
    GetAchievementDisplayAttribute {
        /// Steamworks achievement API name.
        name: String,
        /// Attribute key such as `"name"`, `"desc"`, or `"hidden"`.
        key: String,
    },
    /// Request global achievement percentages.
    RequestGlobalAchievementPercentages,
    /// Read the global unlock percentage for one achievement.
    GetAchievementAchievedPercent {
        /// Steamworks achievement API name.
        name: String,
    },
    /// List global achievement unlock percentages for the current app.
    ///
    /// Call [`SteamworksStatsCommand::RequestGlobalAchievementPercentages`] and
    /// wait for [`SteamworksStatsOperation::GlobalAchievementPercentagesReceived`]
    /// before reading percentages.
    ListAchievementGlobalPercentages {
        /// Zero-based achievement index to start from.
        offset: usize,
        /// Maximum percentages returned by this command.
        limit: usize,
    },
    /// Request global stat data for stats marked as aggregated.
    RequestGlobalStats {
        /// Number of history days to request, up to Steam's supported limit.
        history_days: i32,
    },
    /// Read an aggregated global stat as an integer.
    GetGlobalStatI64 {
        /// Steamworks stat API name.
        name: String,
    },
    /// Read an aggregated global stat as a floating-point value.
    GetGlobalStatF64 {
        /// Steamworks stat API name.
        name: String,
    },
    /// Read daily history for an aggregated global integer stat.
    GetGlobalStatHistoryI64 {
        /// Steamworks stat API name.
        name: String,
        /// Maximum number of days to read.
        max_days: usize,
    },
    /// Read daily history for an aggregated global floating-point stat.
    GetGlobalStatHistoryF64 {
        /// Steamworks stat API name.
        name: String,
        /// Maximum number of days to read.
        max_days: usize,
    },
    /// Store any changed stats and achievements on Steam.
    StoreStats,
    /// Reset the current user's stats.
    ResetAllStats {
        /// Whether achievements should also be reset.
        achievements_too: bool,
        /// Whether to store the reset immediately if it succeeds.
        store_after_reset: bool,
    },
    /// Find an existing leaderboard by API name.
    FindLeaderboard {
        /// Steamworks leaderboard API name.
        name: String,
    },
    /// Find or create a leaderboard by API name.
    FindOrCreateLeaderboard {
        /// Steamworks leaderboard API name.
        name: String,
        /// Sort method used if Steam creates the leaderboard.
        sort_method: SteamworksLeaderboardSortMethod,
        /// Display type used if Steam creates the leaderboard.
        display_type: SteamworksLeaderboardDisplayType,
    },
    /// Read metadata for a known leaderboard.
    GetLeaderboardInfo {
        /// Plugin-owned leaderboard ID.
        leaderboard: SteamworksLeaderboardId,
    },
    /// Upload a score to a known leaderboard.
    UploadLeaderboardScore {
        /// Plugin-owned leaderboard ID.
        leaderboard: SteamworksLeaderboardId,
        /// Upload behavior.
        method: SteamworksLeaderboardUploadScoreMethod,
        /// Score to submit.
        score: i32,
        /// Optional detail integers.
        details: Vec<i32>,
    },
    /// Download entries from a known leaderboard.
    DownloadLeaderboardEntries {
        /// Plugin-owned leaderboard ID.
        leaderboard: SteamworksLeaderboardId,
        /// Entry scope to download.
        request: SteamworksLeaderboardDataRequest,
        /// Maximum detail integers to read per entry.
        max_details: usize,
    },
    /// Forget a leaderboard handle owned by the plugin.
    ForgetLeaderboard {
        /// Plugin-owned leaderboard ID.
        leaderboard: SteamworksLeaderboardId,
    },
}

impl SteamworksStatsCommand {
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

    /// Creates a [`SteamworksStatsCommand::GetAchievement`] command.
    pub fn get_achievement(name: impl Into<String>) -> Self {
        Self::GetAchievement { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::ListAchievementNames`] command.
    pub fn list_achievement_names() -> Self {
        Self::list_achievement_names_page(0, STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND)
    }

    /// Creates a paged [`SteamworksStatsCommand::ListAchievementNames`] command.
    pub fn list_achievement_names_page(offset: usize, limit: usize) -> Self {
        Self::ListAchievementNames { offset, limit }
    }

    /// Creates a [`SteamworksStatsCommand::ListAchievements`] command.
    pub fn list_achievements(include_display_attributes: bool, include_unlock_state: bool) -> Self {
        Self::list_achievements_page(
            include_display_attributes,
            include_unlock_state,
            0,
            STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND,
        )
    }

    /// Creates a paged [`SteamworksStatsCommand::ListAchievements`] command.
    pub fn list_achievements_page(
        include_display_attributes: bool,
        include_unlock_state: bool,
        offset: usize,
        limit: usize,
    ) -> Self {
        Self::ListAchievements {
            include_display_attributes,
            include_unlock_state,
            offset,
            limit,
        }
    }

    /// Creates a [`SteamworksStatsCommand::GetAchievementIcon`] command.
    pub fn get_achievement_icon(name: impl Into<String>) -> Self {
        Self::GetAchievementIcon { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::UnlockAchievement`] command.
    pub fn unlock_achievement(name: impl Into<String>) -> Self {
        Self::UnlockAchievement { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::ClearAchievement`] command.
    pub fn clear_achievement(name: impl Into<String>) -> Self {
        Self::ClearAchievement { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::GetAchievementAndUnlockTime`] command.
    pub fn get_achievement_and_unlock_time(name: impl Into<String>) -> Self {
        Self::GetAchievementAndUnlockTime { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::GetAchievementDisplayAttribute`] command.
    pub fn get_achievement_display_attribute(
        name: impl Into<String>,
        key: impl Into<String>,
    ) -> Self {
        Self::GetAchievementDisplayAttribute {
            name: name.into(),
            key: key.into(),
        }
    }

    /// Creates a [`SteamworksStatsCommand::GetAchievementAchievedPercent`] command.
    pub fn get_achievement_achieved_percent(name: impl Into<String>) -> Self {
        Self::GetAchievementAchievedPercent { name: name.into() }
    }

    /// Creates a [`SteamworksStatsCommand::ListAchievementGlobalPercentages`] command.
    pub fn list_achievement_global_percentages() -> Self {
        Self::list_achievement_global_percentages_page(
            0,
            STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND,
        )
    }

    /// Creates a paged [`SteamworksStatsCommand::ListAchievementGlobalPercentages`] command.
    pub fn list_achievement_global_percentages_page(offset: usize, limit: usize) -> Self {
        Self::ListAchievementGlobalPercentages { offset, limit }
    }

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

    /// Creates a [`SteamworksStatsCommand::ResetAllStats`] command.
    pub fn reset_all_stats(achievements_too: bool, store_after_reset: bool) -> Self {
        Self::ResetAllStats {
            achievements_too,
            store_after_reset,
        }
    }

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
        /// Submitted score.
        score: i32,
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

/// Result message emitted by [`SteamworksStatsPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksStatsResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksStatsOperation),
    /// The command failed synchronously or an asynchronous callback failed.
    Err {
        /// Command that failed.
        command: SteamworksStatsCommand,
        /// Failure reason.
        error: SteamworksStatsError,
    },
}

/// Command and asynchronous callback errors from [`SteamworksStatsPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksStatsError {
    /// No [`SteamworksClient`] resource exists.
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
    fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }

    fn steam_error(operation: &'static str, source: steamworks::SteamError) -> Self {
        Self::SteamError { operation, source }
    }

    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }
}

/// Bevy plugin for high-level Steam user stats and achievements commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksStatsCommand`] and [`SteamworksStatsResult`] messages and runs
/// its command processor in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksStatsPlugin {
    settings: SteamworksStatsSettings,
}

impl SteamworksStatsPlugin {
    /// Creates a stats plugin with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a stats plugin with explicit settings.
    pub fn with_settings(settings: SteamworksStatsSettings) -> Self {
        Self { settings }
    }

    /// Sets whether current-user stats are requested automatically on startup.
    pub fn request_current_user_stats_on_startup(mut self, enabled: bool) -> Self {
        self.settings.request_current_user_stats_on_startup = enabled;
        self
    }

    /// Sets whether successful writes are automatically followed by one store call.
    pub fn auto_store(mut self, enabled: bool) -> Self {
        self.settings.auto_store = enabled;
        self
    }
}

impl Plugin for SteamworksStatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.settings.clone())
            .init_resource::<SteamworksStatsState>()
            .init_resource::<SteamworksStatsAsyncResults>()
            .init_resource::<SteamworksStatsLeaderboardHandles>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksStatsCommand>()
            .add_message::<SteamworksStatsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessStatsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_stats_commands.in_set(SteamworksSystem::ProcessStatsCommands),
            );
    }
}

#[derive(SystemParam)]
struct SteamworksStatsIo<'w, 's> {
    settings: Res<'w, SteamworksStatsSettings>,
    async_results: Res<'w, SteamworksStatsAsyncResults>,
    leaderboards: Res<'w, SteamworksStatsLeaderboardHandles>,
    steam_events: MessageReader<'w, 's, SteamworksEvent>,
    commands: ResMut<'w, Messages<SteamworksStatsCommand>>,
}

fn process_stats_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksStatsState>,
    mut io: SteamworksStatsIo,
    mut results: MessageWriter<SteamworksStatsResult>,
) {
    for result in io.async_results.drain() {
        match &result {
            SteamworksStatsResult::Ok(operation) => {
                state.record_operation(operation);
                state.sync_leaderboard_count(&io.leaderboards);
            }
            SteamworksStatsResult::Err { error, .. } => {
                state.record_error(error.clone());
                state.sync_leaderboard_count(&io.leaderboards);
            }
        }
        results.write(result);
    }

    process_stats_steam_events(
        client.as_deref(),
        &mut state,
        &mut io.steam_events,
        &mut results,
    );

    let Some(client) = client else {
        let error = SteamworksStatsError::ClientUnavailable;
        for command in io.commands.drain() {
            state.record_error(error.clone());
            results.write(SteamworksStatsResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    if io.settings.request_current_user_stats_on_startup && !state.current_user_stats_requested {
        request_current_user_stats(&client, &mut state, &mut results);
    }

    for command in io.commands.drain() {
        handle_stats_command(
            &client,
            command,
            &io.settings,
            &io.async_results,
            &io.leaderboards,
            &mut state,
            &mut results,
        );
    }

    if io.settings.auto_store && state.pending_store {
        match client.user_stats().store_stats() {
            Ok(()) => {
                state.pending_store = false;
                results.write(SteamworksStatsResult::Ok(
                    SteamworksStatsOperation::StatsStoreSubmitted,
                ));
                tracing::debug!(
                    target: "bevy_steamworks",
                    "submitted Steamworks stats store"
                );
            }
            Err(()) => {
                let error = SteamworksStatsError::operation_failed("store_stats");
                state.record_error(error.clone());
                results.write(SteamworksStatsResult::Err {
                    command: SteamworksStatsCommand::StoreStats,
                    error,
                });
            }
        }
    }
}

fn request_current_user_stats(
    client: &SteamworksClient,
    state: &mut SteamworksStatsState,
    results: &mut MessageWriter<SteamworksStatsResult>,
) {
    let steam_id = client.user().steam_id();
    client.user_stats().request_user_stats(steam_id.raw());
    state.current_user_stats_requested = true;
    results.write(SteamworksStatsResult::Ok(
        SteamworksStatsOperation::CurrentUserStatsRequested { steam_id },
    ));
    tracing::debug!(
        target: "bevy_steamworks",
        steam_id = steam_id.raw(),
        "requested Steamworks stats for current user"
    );
}

fn process_stats_steam_events(
    client: Option<&SteamworksClient>,
    state: &mut SteamworksStatsState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksStatsResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::UserStatsReceived(event) => {
                SteamworksStatsOperation::UserStatsReceived {
                    callback: SteamworksUserStatsReceived {
                        steam_id: event.steam_id,
                        game_id: event.game_id,
                        result: event.result,
                    },
                }
            }
            SteamworksEvent::UserStatsStored(event) => SteamworksStatsOperation::UserStatsStored {
                callback: SteamworksUserStatsStored {
                    game_id: event.game_id,
                    result: event.result,
                },
            },
            SteamworksEvent::UserAchievementStored(event) => {
                SteamworksStatsOperation::UserAchievementStored {
                    callback: SteamworksUserAchievementStored {
                        game_id: event.game_id,
                        achievement_name: event.achievement_name.clone(),
                        current_progress: event.current_progress,
                        max_progress: event.max_progress,
                    },
                }
            }
            SteamworksEvent::UserAchievementIconFetched(event) => {
                let icon = client
                    .map(|client| {
                        read_achievement_icon(&client.user_stats(), &event.achievement_name)
                    })
                    .unwrap_or(SteamworksAchievementIconStatus::PendingOrUnavailable);
                achievement_icon_fetched_operation(event, icon)
            }
            _ => continue,
        };

        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks stats callback"
        );
        results.write(SteamworksStatsResult::Ok(operation));
    }
}

fn handle_stats_command(
    client: &SteamworksClient,
    command: SteamworksStatsCommand,
    settings: &SteamworksStatsSettings,
    async_results: &SteamworksStatsAsyncResults,
    leaderboards: &SteamworksStatsLeaderboardHandles,
    state: &mut SteamworksStatsState,
    results: &mut MessageWriter<SteamworksStatsResult>,
) {
    let result = validate_stats_command(&command).and_then(|()| match command.clone() {
        SteamworksStatsCommand::RequestCurrentUserStats => {
            let steam_id = client.user().steam_id();
            client.user_stats().request_user_stats(steam_id.raw());
            state.current_user_stats_requested = true;
            Ok(SteamworksStatsOperation::CurrentUserStatsRequested { steam_id })
        }
        SteamworksStatsCommand::RequestUserStats { steam_id } => {
            client.user_stats().request_user_stats(steam_id.raw());
            Ok(SteamworksStatsOperation::UserStatsRequested { steam_id })
        }
        SteamworksStatsCommand::GetStatI32 { name } => client
            .user_stats()
            .get_stat_i32(&name)
            .map(|value| SteamworksStatsOperation::StatI32Read { name, value })
            .map_err(|()| SteamworksStatsError::operation_failed("get_stat_i32")),
        SteamworksStatsCommand::SetStatI32 { name, value } => client
            .user_stats()
            .set_stat_i32(&name, value)
            .map(|()| SteamworksStatsOperation::StatI32Set { name, value })
            .map_err(|()| SteamworksStatsError::operation_failed("set_stat_i32")),
        SteamworksStatsCommand::GetStatF32 { name } => client
            .user_stats()
            .get_stat_f32(&name)
            .map(|value| SteamworksStatsOperation::StatF32Read { name, value })
            .map_err(|()| SteamworksStatsError::operation_failed("get_stat_f32")),
        SteamworksStatsCommand::SetStatF32 { name, value } => client
            .user_stats()
            .set_stat_f32(&name, value)
            .map(|()| SteamworksStatsOperation::StatF32Set { name, value })
            .map_err(|()| SteamworksStatsError::operation_failed("set_stat_f32")),
        SteamworksStatsCommand::GetAchievement { name } => client
            .user_stats()
            .achievement(&name)
            .get()
            .map(|achieved| SteamworksStatsOperation::AchievementRead { name, achieved })
            .map_err(|()| SteamworksStatsError::operation_failed("achievement.get")),
        SteamworksStatsCommand::ListAchievementNames { offset, limit } => {
            list_achievement_names(&client.user_stats(), offset, limit).map(|(total, names)| {
                SteamworksStatsOperation::AchievementNamesListed {
                    offset,
                    total,
                    names,
                }
            })
        }
        SteamworksStatsCommand::ListAchievements {
            include_display_attributes,
            include_unlock_state,
            offset,
            limit,
        } => list_achievement_infos(
            &client.user_stats(),
            include_display_attributes,
            include_unlock_state,
            offset,
            limit,
        )
        .map(
            |(total, achievements)| SteamworksStatsOperation::AchievementsListed {
                offset,
                total,
                achievements,
            },
        ),
        SteamworksStatsCommand::GetAchievementIcon { name } => {
            Ok(SteamworksStatsOperation::AchievementIconRead {
                icon: read_achievement_icon(&client.user_stats(), &name),
                name,
            })
        }
        SteamworksStatsCommand::UnlockAchievement { name } => client
            .user_stats()
            .achievement(&name)
            .set()
            .map(|()| SteamworksStatsOperation::AchievementUnlocked { name })
            .map_err(|()| SteamworksStatsError::operation_failed("achievement.set")),
        SteamworksStatsCommand::ClearAchievement { name } => client
            .user_stats()
            .achievement(&name)
            .clear()
            .map(|()| SteamworksStatsOperation::AchievementCleared { name })
            .map_err(|()| SteamworksStatsError::operation_failed("achievement.clear")),
        SteamworksStatsCommand::GetAchievementAndUnlockTime { name } => client
            .user_stats()
            .achievement(&name)
            .get_achievement_and_unlock_time()
            .map(
                |(achieved, unlock_time)| SteamworksStatsOperation::AchievementAndUnlockTimeRead {
                    name,
                    achieved,
                    unlock_time,
                },
            )
            .map_err(|()| {
                SteamworksStatsError::operation_failed(
                    "achievement.get_achievement_and_unlock_time",
                )
            }),
        SteamworksStatsCommand::GetAchievementDisplayAttribute { name, key } => client
            .user_stats()
            .achievement(&name)
            .get_achievement_display_attribute(&key)
            .map(
                |value| SteamworksStatsOperation::AchievementDisplayAttributeRead {
                    name,
                    key,
                    value: value.to_owned(),
                },
            )
            .map_err(|()| {
                SteamworksStatsError::operation_failed(
                    "achievement.get_achievement_display_attribute",
                )
            }),
        SteamworksStatsCommand::RequestGlobalAchievementPercentages => {
            let async_results = async_results.clone();
            client
                .user_stats()
                .request_global_achievement_percentages(move |result| {
                    let result = match result {
                        Ok(game_id) => SteamworksStatsResult::Ok(
                            SteamworksStatsOperation::GlobalAchievementPercentagesReceived {
                                game_id,
                            },
                        ),
                        Err(source) => SteamworksStatsResult::Err {
                            command: SteamworksStatsCommand::RequestGlobalAchievementPercentages,
                            error: SteamworksStatsError::steam_error(
                                "request_global_achievement_percentages",
                                source,
                            ),
                        },
                    };
                    async_results.push(result);
                });
            Ok(SteamworksStatsOperation::GlobalAchievementPercentagesRequested)
        }
        SteamworksStatsCommand::GetAchievementAchievedPercent { name } => client
            .user_stats()
            .achievement(&name)
            .get_achievement_achieved_percent()
            .map(
                |percent| SteamworksStatsOperation::AchievementAchievedPercentRead {
                    name,
                    percent,
                },
            )
            .map_err(|()| {
                SteamworksStatsError::operation_failed(
                    "achievement.get_achievement_achieved_percent",
                )
            }),
        SteamworksStatsCommand::ListAchievementGlobalPercentages { offset, limit } => {
            list_achievement_global_percentages(&client.user_stats(), offset, limit).map(
                |(total, percentages)| {
                    SteamworksStatsOperation::AchievementGlobalPercentagesListed {
                        offset,
                        total,
                        percentages,
                    }
                },
            )
        }
        SteamworksStatsCommand::RequestGlobalStats { history_days } => {
            let async_results = async_results.clone();
            client
                .user_stats()
                .request_global_stats(history_days, move |result| {
                    let result = match result {
                        Ok(game_id) => SteamworksStatsResult::Ok(
                            SteamworksStatsOperation::GlobalStatsReceived { game_id },
                        ),
                        Err(source) => SteamworksStatsResult::Err {
                            command: SteamworksStatsCommand::RequestGlobalStats { history_days },
                            error: SteamworksStatsError::steam_error(
                                "request_global_stats",
                                source,
                            ),
                        },
                    };
                    async_results.push(result);
                });
            Ok(SteamworksStatsOperation::GlobalStatsRequested { history_days })
        }
        SteamworksStatsCommand::GetGlobalStatI64 { name } => client
            .user_stats()
            .get_global_stat_i64(&name)
            .map(|value| SteamworksStatsOperation::GlobalStatI64Read { name, value })
            .map_err(|()| SteamworksStatsError::operation_failed("get_global_stat_i64")),
        SteamworksStatsCommand::GetGlobalStatF64 { name } => client
            .user_stats()
            .get_global_stat_f64(&name)
            .map(|value| SteamworksStatsOperation::GlobalStatF64Read { name, value })
            .map_err(|()| SteamworksStatsError::operation_failed("get_global_stat_f64")),
        SteamworksStatsCommand::GetGlobalStatHistoryI64 { name, max_days } => client
            .user_stats()
            .get_global_stat_history_i64(&name, max_days)
            .map(|values| SteamworksStatsOperation::GlobalStatHistoryI64Read { name, values })
            .map_err(|()| SteamworksStatsError::operation_failed("get_global_stat_history_i64")),
        SteamworksStatsCommand::GetGlobalStatHistoryF64 { name, max_days } => client
            .user_stats()
            .get_global_stat_history_f64(&name, max_days)
            .map(|values| SteamworksStatsOperation::GlobalStatHistoryF64Read { name, values })
            .map_err(|()| SteamworksStatsError::operation_failed("get_global_stat_history_f64")),
        SteamworksStatsCommand::StoreStats => client
            .user_stats()
            .store_stats()
            .map(|()| {
                state.pending_store = false;
                SteamworksStatsOperation::StatsStoreSubmitted
            })
            .map_err(|()| SteamworksStatsError::operation_failed("store_stats")),
        SteamworksStatsCommand::ResetAllStats {
            achievements_too,
            store_after_reset,
        } => client
            .user_stats()
            .reset_all_stats(achievements_too)
            .map(|()| {
                if store_after_reset {
                    state.pending_store = true;
                }
                SteamworksStatsOperation::AllStatsReset { achievements_too }
            })
            .map_err(|()| SteamworksStatsError::operation_failed("reset_all_stats")),
        SteamworksStatsCommand::FindLeaderboard { name } => {
            let async_results = async_results.clone();
            let leaderboards = leaderboards.clone();
            let command_name = name.clone();
            client.user_stats().find_leaderboard(&name, move |result| {
                let result = match result {
                    Ok(leaderboard) => {
                        let leaderboard =
                            leaderboard.map(|leaderboard| leaderboards.insert(leaderboard));
                        SteamworksStatsResult::Ok(
                            SteamworksStatsOperation::LeaderboardFindCompleted {
                                name: command_name,
                                leaderboard,
                            },
                        )
                    }
                    Err(source) => SteamworksStatsResult::Err {
                        command: SteamworksStatsCommand::FindLeaderboard { name: command_name },
                        error: SteamworksStatsError::steam_error("find_leaderboard", source),
                    },
                };
                async_results.push(result);
            });
            Ok(SteamworksStatsOperation::LeaderboardFindSubmitted { name })
        }
        SteamworksStatsCommand::FindOrCreateLeaderboard {
            name,
            sort_method,
            display_type,
        } => {
            let async_results = async_results.clone();
            let leaderboards = leaderboards.clone();
            let command_name = name.clone();
            client.user_stats().find_or_create_leaderboard(
                &name,
                sort_method.into(),
                display_type.into(),
                move |result| {
                    let result = match result {
                        Ok(leaderboard) => {
                            let leaderboard =
                                leaderboard.map(|leaderboard| leaderboards.insert(leaderboard));
                            SteamworksStatsResult::Ok(
                                SteamworksStatsOperation::LeaderboardFindOrCreateCompleted {
                                    name: command_name,
                                    leaderboard,
                                },
                            )
                        }
                        Err(source) => SteamworksStatsResult::Err {
                            command: SteamworksStatsCommand::FindOrCreateLeaderboard {
                                name: command_name,
                                sort_method,
                                display_type,
                            },
                            error: SteamworksStatsError::steam_error(
                                "find_or_create_leaderboard",
                                source,
                            ),
                        },
                    };
                    async_results.push(result);
                },
            );
            Ok(SteamworksStatsOperation::LeaderboardFindOrCreateSubmitted {
                name,
                sort_method,
                display_type,
            })
        }
        SteamworksStatsCommand::GetLeaderboardInfo { leaderboard } => {
            let leaderboard_handle = leaderboards
                .get(leaderboard)
                .ok_or(SteamworksStatsError::LeaderboardNotFound { id: leaderboard })?;
            Ok(SteamworksStatsOperation::LeaderboardInfoRead {
                info: snapshot_leaderboard_info(
                    &client.user_stats(),
                    leaderboard,
                    &leaderboard_handle,
                ),
            })
        }
        SteamworksStatsCommand::UploadLeaderboardScore {
            leaderboard,
            method,
            score,
            details,
        } => {
            let leaderboard_handle = leaderboards
                .get(leaderboard)
                .ok_or(SteamworksStatsError::LeaderboardNotFound { id: leaderboard })?;
            let async_results = async_results.clone();
            let command_details = details.clone();
            client.user_stats().upload_leaderboard_score(
                &leaderboard_handle,
                method.into(),
                score,
                &details,
                move |result| {
                    let result = match result {
                        Ok(upload) => SteamworksStatsResult::Ok(
                            SteamworksStatsOperation::LeaderboardScoreUploaded {
                                leaderboard,
                                upload: upload.map(snapshot_leaderboard_score_uploaded),
                            },
                        ),
                        Err(source) => SteamworksStatsResult::Err {
                            command: SteamworksStatsCommand::UploadLeaderboardScore {
                                leaderboard,
                                method,
                                score,
                                details: command_details,
                            },
                            error: SteamworksStatsError::steam_error(
                                "upload_leaderboard_score",
                                source,
                            ),
                        },
                    };
                    async_results.push(result);
                },
            );
            Ok(SteamworksStatsOperation::LeaderboardScoreUploadSubmitted { leaderboard, score })
        }
        SteamworksStatsCommand::DownloadLeaderboardEntries {
            leaderboard,
            request,
            max_details,
        } => {
            let leaderboard_handle = leaderboards
                .get(leaderboard)
                .ok_or(SteamworksStatsError::LeaderboardNotFound { id: leaderboard })?;
            let async_results = async_results.clone();
            let (start, end) = request.upstream_range();
            client.user_stats().download_leaderboard_entries(
                &leaderboard_handle,
                request.into(),
                start,
                end,
                max_details,
                move |result| {
                    let result = match result {
                        Ok(entries) => SteamworksStatsResult::Ok(
                            SteamworksStatsOperation::LeaderboardEntriesDownloaded {
                                leaderboard,
                                entries: entries
                                    .into_iter()
                                    .map(snapshot_leaderboard_entry)
                                    .collect(),
                            },
                        ),
                        Err(source) => SteamworksStatsResult::Err {
                            command: SteamworksStatsCommand::DownloadLeaderboardEntries {
                                leaderboard,
                                request,
                                max_details,
                            },
                            error: SteamworksStatsError::steam_error(
                                "download_leaderboard_entries",
                                source,
                            ),
                        },
                    };
                    async_results.push(result);
                },
            );
            Ok(
                SteamworksStatsOperation::LeaderboardEntriesDownloadSubmitted {
                    leaderboard,
                    request,
                    max_details,
                },
            )
        }
        SteamworksStatsCommand::ForgetLeaderboard { leaderboard } => leaderboards
            .remove(leaderboard)
            .map(|_| SteamworksStatsOperation::LeaderboardForgotten { leaderboard })
            .ok_or(SteamworksStatsError::LeaderboardNotFound { id: leaderboard }),
    });

    match result {
        Ok(operation) => {
            if settings.auto_store && operation_requires_store(&operation) {
                state.pending_store = true;
            }
            state.record_operation(&operation);
            state.sync_leaderboard_count(leaderboards);
            tracing::debug!(
                target: "bevy_steamworks",
                operation = ?operation,
                "processed Steamworks stats command"
            );
            results.write(SteamworksStatsResult::Ok(operation));
        }
        Err(error) => {
            state.record_error(error.clone());
            state.sync_leaderboard_count(leaderboards);
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks stats command failed"
            );
            results.write(SteamworksStatsResult::Err { command, error });
        }
    }
}

fn operation_requires_store(operation: &SteamworksStatsOperation) -> bool {
    matches!(
        operation,
        SteamworksStatsOperation::StatI32Set { .. }
            | SteamworksStatsOperation::StatF32Set { .. }
            | SteamworksStatsOperation::AchievementUnlocked { .. }
            | SteamworksStatsOperation::AchievementCleared { .. }
            | SteamworksStatsOperation::AllStatsReset { .. }
    )
}

fn list_achievement_names(
    stats: &steamworks::UserStats,
    offset: usize,
    limit: usize,
) -> Result<(usize, Vec<String>), SteamworksStatsError> {
    let names = read_all_achievement_names(stats)?;
    let total = names.len();
    let page = names.into_iter().skip(offset).take(limit).collect();

    Ok((total, page))
}

fn read_all_achievement_names(
    stats: &steamworks::UserStats,
) -> Result<Vec<String>, SteamworksStatsError> {
    stats
        .get_num_achievements()
        .map_err(|()| SteamworksStatsError::operation_failed("user_stats.get_num_achievements"))?;

    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        stats.get_achievement_names()
    }))
    .map_err(|_| SteamworksStatsError::operation_failed("user_stats.get_achievement_names"))?
    .ok_or_else(|| SteamworksStatsError::operation_failed("user_stats.get_achievement_names"))
}

fn list_achievement_infos(
    stats: &steamworks::UserStats,
    include_display_attributes: bool,
    include_unlock_state: bool,
    offset: usize,
    limit: usize,
) -> Result<(usize, Vec<SteamworksAchievementInfo>), SteamworksStatsError> {
    let (total, names) = list_achievement_names(stats, offset, limit)?;
    let achievements = names
        .into_iter()
        .map(|api_name| {
            let mut info = SteamworksAchievementInfo {
                api_name,
                ..Default::default()
            };

            if include_display_attributes {
                info.display_name = achievement_display_attribute(stats, &info.api_name, "name")?;
                info.description = achievement_display_attribute(stats, &info.api_name, "desc")?;
                info.hidden = achievement_display_attribute(stats, &info.api_name, "hidden")?
                    .map(|value| value != "0");
            }
            if include_unlock_state {
                let (achieved, unlock_time) = stats
                    .achievement(&info.api_name)
                    .get_achievement_and_unlock_time()
                    .map_err(|()| {
                        SteamworksStatsError::operation_failed(
                            "achievement.get_achievement_and_unlock_time",
                        )
                    })?;
                info.achieved = Some(achieved);
                info.unlock_time = Some(unlock_time);
            }

            Ok(info)
        })
        .collect::<Result<Vec<_>, SteamworksStatsError>>()?;

    Ok((total, achievements))
}

fn list_achievement_global_percentages(
    stats: &steamworks::UserStats,
    offset: usize,
    limit: usize,
) -> Result<(usize, Vec<SteamworksAchievementGlobalPercentage>), SteamworksStatsError> {
    let (total, names) = list_achievement_names(stats, offset, limit)?;
    let percentages = names
        .into_iter()
        .map(|api_name| {
            let percent = stats
                .achievement(&api_name)
                .get_achievement_achieved_percent()
                .map_err(|()| {
                    SteamworksStatsError::operation_failed(
                        "achievement.get_achievement_achieved_percent",
                    )
                })?;

            Ok(SteamworksAchievementGlobalPercentage { api_name, percent })
        })
        .collect::<Result<Vec<_>, SteamworksStatsError>>()?;

    Ok((total, percentages))
}

fn achievement_display_attribute(
    stats: &steamworks::UserStats,
    achievement: &str,
    key: &str,
) -> Result<Option<String>, SteamworksStatsError> {
    stats
        .achievement(achievement)
        .get_achievement_display_attribute(key)
        .map(|value| (!value.is_empty()).then(|| value.to_owned()))
        .map_err(|()| {
            SteamworksStatsError::operation_failed("achievement.get_achievement_display_attribute")
        })
}

fn read_achievement_icon(
    stats: &steamworks::UserStats,
    achievement: &str,
) -> SteamworksAchievementIconStatus {
    stats
        .achievement(achievement)
        .get_achievement_icon()
        .map(|rgba| {
            SteamworksAchievementIconStatus::Available(achievement_icon_from_rgba(
                achievement,
                rgba,
            ))
        })
        .unwrap_or(SteamworksAchievementIconStatus::PendingOrUnavailable)
}

fn achievement_icon_from_rgba(achievement: &str, rgba: Vec<u8>) -> SteamworksAchievementIcon {
    SteamworksAchievementIcon {
        api_name: achievement.to_owned(),
        width: 64,
        height: 64,
        rgba,
    }
}

fn achievement_icon_fetched_operation(
    event: &steamworks::UserAchievementIconFetched,
    icon: SteamworksAchievementIconStatus,
) -> SteamworksStatsOperation {
    SteamworksStatsOperation::AchievementIconFetched {
        name: event.achievement_name.clone(),
        achieved: event.achieved,
        icon_handle: event.icon_handle,
        icon,
    }
}

fn snapshot_leaderboard_info(
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

fn snapshot_leaderboard_entry(entry: steamworks::LeaderboardEntry) -> SteamworksLeaderboardEntry {
    SteamworksLeaderboardEntry {
        user: entry.user,
        global_rank: entry.global_rank,
        score: entry.score,
        details: entry.details,
    }
}

fn snapshot_leaderboard_score_uploaded(
    upload: steamworks::LeaderboardScoreUploaded,
) -> SteamworksLeaderboardScoreUploaded {
    SteamworksLeaderboardScoreUploaded {
        score: upload.score,
        was_changed: upload.was_changed,
        global_rank_new: upload.global_rank_new,
        global_rank_previous: upload.global_rank_previous,
    }
}

fn validate_stats_command(command: &SteamworksStatsCommand) -> Result<(), SteamworksStatsError> {
    match command {
        SteamworksStatsCommand::GetStatI32 { name }
        | SteamworksStatsCommand::SetStatI32 { name, .. }
        | SteamworksStatsCommand::GetStatF32 { name }
        | SteamworksStatsCommand::SetStatF32 { name, .. }
        | SteamworksStatsCommand::GetAchievement { name }
        | SteamworksStatsCommand::GetAchievementIcon { name }
        | SteamworksStatsCommand::UnlockAchievement { name }
        | SteamworksStatsCommand::ClearAchievement { name }
        | SteamworksStatsCommand::GetAchievementAndUnlockTime { name }
        | SteamworksStatsCommand::GetAchievementAchievedPercent { name }
        | SteamworksStatsCommand::GetGlobalStatI64 { name }
        | SteamworksStatsCommand::GetGlobalStatF64 { name }
        | SteamworksStatsCommand::GetGlobalStatHistoryI64 { name, .. }
        | SteamworksStatsCommand::GetGlobalStatHistoryF64 { name, .. }
        | SteamworksStatsCommand::FindLeaderboard { name }
        | SteamworksStatsCommand::FindOrCreateLeaderboard { name, .. } => {
            validate_no_nul("name", name)
        }
        SteamworksStatsCommand::GetAchievementDisplayAttribute { name, key } => {
            validate_no_nul("name", name)?;
            validate_no_nul("key", key)
        }
        SteamworksStatsCommand::ListAchievementNames { limit, .. }
        | SteamworksStatsCommand::ListAchievements { limit, .. }
        | SteamworksStatsCommand::ListAchievementGlobalPercentages { limit, .. } => {
            validate_achievement_page_limit(*limit)
        }
        SteamworksStatsCommand::UploadLeaderboardScore { details, .. } => {
            validate_leaderboard_details_len(details.len())
        }
        SteamworksStatsCommand::DownloadLeaderboardEntries {
            request,
            max_details,
            ..
        } => {
            validate_leaderboard_details_len(*max_details)?;
            validate_leaderboard_data_request(*request)
        }
        _ => Ok(()),
    }
}

fn validate_no_nul(field: &'static str, value: &str) -> Result<(), SteamworksStatsError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksStatsError::invalid_string(field))
    } else {
        Ok(())
    }
}

fn validate_achievement_page_limit(limit: usize) -> Result<(), SteamworksStatsError> {
    if limit > STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND {
        Err(SteamworksStatsError::TooManyAchievementEntries {
            requested: limit,
            max_supported: STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND,
        })
    } else {
        Ok(())
    }
}

fn validate_leaderboard_details_len(len: usize) -> Result<(), SteamworksStatsError> {
    if len > STEAMWORKS_LEADERBOARD_MAX_DETAILS {
        Err(SteamworksStatsError::TooManyLeaderboardDetails {
            requested: len,
            max_supported: STEAMWORKS_LEADERBOARD_MAX_DETAILS,
        })
    } else {
        Ok(())
    }
}

fn validate_leaderboard_data_request(
    request: SteamworksLeaderboardDataRequest,
) -> Result<(), SteamworksStatsError> {
    match request {
        SteamworksLeaderboardDataRequest::Global { start, end } => {
            if start < 1 || start > end {
                return Err(SteamworksStatsError::InvalidLeaderboardRange { start, end });
            }
            validate_leaderboard_requested_entries(start, end)
        }
        SteamworksLeaderboardDataRequest::GlobalAroundUser { start, end } => {
            if start > end {
                return Err(SteamworksStatsError::InvalidLeaderboardRange { start, end });
            }
            validate_leaderboard_requested_entries(start, end)
        }
        SteamworksLeaderboardDataRequest::Friends => Ok(()),
    }
}

fn validate_leaderboard_requested_entries(
    start: i32,
    end: i32,
) -> Result<(), SteamworksStatsError> {
    let requested = i64::from(end) - i64::from(start) + 1;
    let requested = usize::try_from(requested)
        .map_err(|_| SteamworksStatsError::InvalidLeaderboardRange { start, end })?;
    if requested > STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND {
        return Err(SteamworksStatsError::TooManyLeaderboardEntries {
            requested,
            max_supported: STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND,
        });
    }

    Ok(())
}

fn leaderboard_bound_to_upstream_usize(bound: i32) -> usize {
    // The upstream safe wrapper accepts `usize`, but Steam's
    // DownloadLeaderboardEntries API interprets these bounds as signed i32.
    // Passing the i32 bit pattern through usize lets the wrapper cast it back
    // to Steam's signed type without leaving the safe `steamworks` API.
    bound as usize
}

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    #[test]
    fn stats_plugin_registers_resources_and_messages() {
        let mut app = App::new();

        app.add_plugins(SteamworksStatsPlugin::new());

        assert!(app.world().contains_resource::<SteamworksStatsSettings>());
        assert!(app.world().contains_resource::<SteamworksStatsState>());
        assert!(app
            .world()
            .contains_resource::<SteamworksStatsLeaderboardHandles>());
        assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksStatsCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksStatsResult>>());
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksStatsPlugin::new().request_current_user_stats_on_startup(false));
        app.world_mut()
            .resource_mut::<Messages<SteamworksStatsCommand>>()
            .write(SteamworksStatsCommand::get_stat_i32("total_kills"));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksStatsResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksStatsResult::Err {
                command: SteamworksStatsCommand::get_stat_i32("total_kills"),
                error: SteamworksStatsError::ClientUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksStatsState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksStatsError::ClientUnavailable)
        );
    }

    #[test]
    fn stats_callbacks_are_bridged_without_client() {
        let mut app = App::new();
        let steam_id = steamworks::SteamId::from_raw(42);
        let game_id = steamworks::GameId::from_raw(480);

        app.add_plugins(SteamworksStatsPlugin::new().request_current_user_stats_on_startup(false));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::UserStatsReceived(
                steamworks::UserStatsReceived {
                    steam_id,
                    game_id,
                    result: Ok(()),
                },
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::UserStatsStored(
                steamworks::UserStatsStored {
                    game_id,
                    result: Err(steamworks::SteamError::PersistFailed),
                },
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::UserAchievementStored(
                steamworks::UserAchievementStored {
                    game_id,
                    achievement_name: "ACH_WIN".to_owned(),
                    current_progress: 5,
                    max_progress: 10,
                },
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::UserAchievementIconFetched(
                steamworks::UserAchievementIconFetched {
                    game_id,
                    achievement_name: "ACH_WIN".to_owned(),
                    achieved: true,
                    icon_handle: 99,
                },
            ));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksStatsResult>>();
        let drained = results.drain().collect::<Vec<_>>();
        let expected_received = SteamworksUserStatsReceived {
            steam_id,
            game_id,
            result: Ok(()),
        };
        let expected_stored = SteamworksUserStatsStored {
            game_id,
            result: Err(steamworks::SteamError::PersistFailed),
        };
        let expected_achievement = SteamworksUserAchievementStored {
            game_id,
            achievement_name: "ACH_WIN".to_owned(),
            current_progress: 5,
            max_progress: 10,
        };

        assert_eq!(
            drained,
            vec![
                SteamworksStatsResult::Ok(SteamworksStatsOperation::UserStatsReceived {
                    callback: expected_received.clone(),
                }),
                SteamworksStatsResult::Ok(SteamworksStatsOperation::UserStatsStored {
                    callback: expected_stored.clone(),
                }),
                SteamworksStatsResult::Ok(SteamworksStatsOperation::UserAchievementStored {
                    callback: expected_achievement.clone(),
                }),
                SteamworksStatsResult::Ok(SteamworksStatsOperation::AchievementIconFetched {
                    name: "ACH_WIN".to_owned(),
                    achieved: true,
                    icon_handle: 99,
                    icon: SteamworksAchievementIconStatus::PendingOrUnavailable,
                }),
            ]
        );

        let state = app.world().resource::<SteamworksStatsState>();
        assert_eq!(state.last_user_stats_received(), Some(&expected_received));
        assert_eq!(state.last_user_stats_stored(), Some(&expected_stored));
        assert_eq!(
            state.last_user_achievement_stored(),
            Some(&expected_achievement)
        );
        assert_eq!(state.achievement_icon_callback_count(), 1);
        assert_eq!(state.last_error(), None);
    }

    #[test]
    fn operation_requires_store_for_writes_only() {
        assert!(operation_requires_store(
            &SteamworksStatsOperation::AchievementUnlocked {
                name: "ACH_WIN".to_owned(),
            }
        ));
        assert!(operation_requires_store(
            &SteamworksStatsOperation::StatI32Set {
                name: "kills".to_owned(),
                value: 1,
            }
        ));
        assert!(!operation_requires_store(
            &SteamworksStatsOperation::StatI32Read {
                name: "kills".to_owned(),
                value: 1,
            }
        ));
        assert!(!operation_requires_store(
            &SteamworksStatsOperation::StatsStoreSubmitted
        ));
    }

    #[test]
    fn achievement_commands_preserve_inputs() {
        assert_eq!(
            SteamworksStatsCommand::list_achievement_names(),
            SteamworksStatsCommand::ListAchievementNames {
                offset: 0,
                limit: STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND,
            }
        );
        assert_eq!(
            SteamworksStatsCommand::list_achievement_names_page(8, 4),
            SteamworksStatsCommand::ListAchievementNames {
                offset: 8,
                limit: 4,
            }
        );
        assert_eq!(
            SteamworksStatsCommand::list_achievements(true, false),
            SteamworksStatsCommand::ListAchievements {
                include_display_attributes: true,
                include_unlock_state: false,
                offset: 0,
                limit: STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND,
            }
        );
        assert_eq!(
            SteamworksStatsCommand::list_achievements_page(false, true, 4, 12),
            SteamworksStatsCommand::ListAchievements {
                include_display_attributes: false,
                include_unlock_state: true,
                offset: 4,
                limit: 12,
            }
        );
        assert_eq!(
            SteamworksStatsCommand::get_achievement_icon("ACH_WIN"),
            SteamworksStatsCommand::GetAchievementIcon {
                name: "ACH_WIN".to_owned(),
            }
        );
        assert_eq!(
            SteamworksStatsCommand::list_achievement_global_percentages(),
            SteamworksStatsCommand::ListAchievementGlobalPercentages {
                offset: 0,
                limit: STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND,
            }
        );
        assert_eq!(
            SteamworksStatsCommand::list_achievement_global_percentages_page(6, 9),
            SteamworksStatsCommand::ListAchievementGlobalPercentages {
                offset: 6,
                limit: 9,
            }
        );
    }

    #[test]
    fn leaderboard_commands_preserve_inputs() {
        let leaderboard = SteamworksLeaderboardId::from_raw(7);
        assert_eq!(SteamworksLeaderboardId::from_raw(7).raw(), 7);

        assert_eq!(
            SteamworksStatsCommand::find_leaderboard("daily_score"),
            SteamworksStatsCommand::FindLeaderboard {
                name: "daily_score".to_owned(),
            }
        );
        assert_eq!(
            SteamworksStatsCommand::find_or_create_leaderboard(
                "daily_score",
                SteamworksLeaderboardSortMethod::Descending,
                SteamworksLeaderboardDisplayType::Numeric,
            ),
            SteamworksStatsCommand::FindOrCreateLeaderboard {
                name: "daily_score".to_owned(),
                sort_method: SteamworksLeaderboardSortMethod::Descending,
                display_type: SteamworksLeaderboardDisplayType::Numeric,
            }
        );
        assert_eq!(
            SteamworksStatsCommand::get_leaderboard_info(leaderboard),
            SteamworksStatsCommand::GetLeaderboardInfo { leaderboard }
        );
        assert_eq!(
            SteamworksStatsCommand::upload_leaderboard_score(
                leaderboard,
                SteamworksLeaderboardUploadScoreMethod::KeepBest,
                10,
                vec![1, 2],
            ),
            SteamworksStatsCommand::UploadLeaderboardScore {
                leaderboard,
                method: SteamworksLeaderboardUploadScoreMethod::KeepBest,
                score: 10,
                details: vec![1, 2],
            }
        );
        assert_eq!(
            SteamworksStatsCommand::download_leaderboard_entries(
                leaderboard,
                SteamworksLeaderboardDataRequest::Global { start: 1, end: 10 },
                4,
            ),
            SteamworksStatsCommand::DownloadLeaderboardEntries {
                leaderboard,
                request: SteamworksLeaderboardDataRequest::Global { start: 1, end: 10 },
                max_details: 4,
            }
        );
        assert_eq!(
            SteamworksStatsCommand::download_leaderboard_entries_around_user(leaderboard, -2, 2, 0,),
            SteamworksStatsCommand::DownloadLeaderboardEntries {
                leaderboard,
                request: SteamworksLeaderboardDataRequest::GlobalAroundUser { start: -2, end: 2 },
                max_details: 0,
            }
        );
        assert_eq!(
            SteamworksStatsCommand::download_friends_leaderboard_entries(leaderboard, 0),
            SteamworksStatsCommand::DownloadLeaderboardEntries {
                leaderboard,
                request: SteamworksLeaderboardDataRequest::Friends,
                max_details: 0,
            }
        );
        assert_eq!(
            SteamworksStatsCommand::forget_leaderboard(leaderboard),
            SteamworksStatsCommand::ForgetLeaderboard { leaderboard }
        );
    }

    #[test]
    fn leaderboard_validation_rejects_invalid_inputs() {
        assert_eq!(
            validate_stats_command(&SteamworksStatsCommand::find_leaderboard("bad\0name")),
            Err(SteamworksStatsError::InvalidString { field: "name" })
        );
        assert_eq!(
            validate_stats_command(&SteamworksStatsCommand::get_achievement_icon("bad\0name",)),
            Err(SteamworksStatsError::InvalidString { field: "name" })
        );
        assert_eq!(
            validate_stats_command(&SteamworksStatsCommand::list_achievements_page(
                false,
                false,
                0,
                STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND + 1,
            )),
            Err(SteamworksStatsError::TooManyAchievementEntries {
                requested: STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND + 1,
                max_supported: STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND,
            })
        );
        assert_eq!(
            validate_stats_command(
                &SteamworksStatsCommand::list_achievement_global_percentages_page(
                    0,
                    STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND + 1,
                )
            ),
            Err(SteamworksStatsError::TooManyAchievementEntries {
                requested: STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND + 1,
                max_supported: STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND,
            })
        );
        assert_eq!(
            validate_stats_command(&SteamworksStatsCommand::get_achievement_display_attribute(
                "ACH_WIN", "bad\0key",
            )),
            Err(SteamworksStatsError::InvalidString { field: "key" })
        );
        assert_eq!(
            validate_stats_command(&SteamworksStatsCommand::upload_leaderboard_score(
                SteamworksLeaderboardId::from_raw(1),
                SteamworksLeaderboardUploadScoreMethod::ForceUpdate,
                5,
                vec![0; STEAMWORKS_LEADERBOARD_MAX_DETAILS + 1],
            )),
            Err(SteamworksStatsError::TooManyLeaderboardDetails {
                requested: STEAMWORKS_LEADERBOARD_MAX_DETAILS + 1,
                max_supported: STEAMWORKS_LEADERBOARD_MAX_DETAILS,
            })
        );
        assert_eq!(
            validate_stats_command(&SteamworksStatsCommand::download_leaderboard_entries(
                SteamworksLeaderboardId::from_raw(1),
                SteamworksLeaderboardDataRequest::Global { start: 10, end: 5 },
                0,
            )),
            Err(SteamworksStatsError::InvalidLeaderboardRange { start: 10, end: 5 })
        );
        assert_eq!(
            validate_stats_command(&SteamworksStatsCommand::download_leaderboard_entries(
                SteamworksLeaderboardId::from_raw(1),
                SteamworksLeaderboardDataRequest::GlobalAroundUser { start: -2, end: 2 },
                0,
            )),
            Ok(())
        );
        assert_eq!(
            validate_stats_command(&SteamworksStatsCommand::download_leaderboard_entries(
                SteamworksLeaderboardId::from_raw(1),
                SteamworksLeaderboardDataRequest::Global {
                    start: 0,
                    end: STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND as i32,
                },
                0,
            )),
            Err(SteamworksStatsError::InvalidLeaderboardRange {
                start: 0,
                end: STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND as i32,
            })
        );
        assert_eq!(
            validate_stats_command(&SteamworksStatsCommand::download_leaderboard_entries(
                SteamworksLeaderboardId::from_raw(1),
                SteamworksLeaderboardDataRequest::GlobalAroundUser {
                    start: -(STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND as i32),
                    end: 0,
                },
                0,
            )),
            Err(SteamworksStatsError::TooManyLeaderboardEntries {
                requested: STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND + 1,
                max_supported: STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND,
            })
        );
    }

    #[test]
    fn leaderboard_state_records_latest_info_and_entries() {
        let mut state = SteamworksStatsState::default();
        let leaderboard = SteamworksLeaderboardId::from_raw(3);
        let info = SteamworksLeaderboardInfo {
            leaderboard,
            name: "daily_score".to_owned(),
            display_type: Some(SteamworksLeaderboardDisplayType::Numeric),
            sort_method: Some(SteamworksLeaderboardSortMethod::Descending),
            entry_count: 42,
        };
        let entry = SteamworksLeaderboardEntry {
            user: steamworks::SteamId::from_raw(123),
            global_rank: 1,
            score: 9000,
            details: vec![7, 8],
        };

        state.record_operation(&SteamworksStatsOperation::LeaderboardInfoRead {
            info: info.clone(),
        });
        state.record_operation(&SteamworksStatsOperation::LeaderboardEntriesDownloaded {
            leaderboard,
            entries: vec![entry.clone()],
        });

        assert_eq!(state.last_leaderboard_info(), Some(&info));
        assert_eq!(state.last_leaderboard_entries(), &[entry]);
    }

    #[test]
    fn achievement_state_records_catalog_and_icons() {
        let mut state = SteamworksStatsState::default();
        let achievement = SteamworksAchievementInfo {
            api_name: "ACH_WIN".to_owned(),
            display_name: Some("Winner".to_owned()),
            description: Some("Win once".to_owned()),
            hidden: Some(false),
            achieved: Some(true),
            unlock_time: Some(12),
        };
        let icon = SteamworksAchievementIcon {
            api_name: "ACH_WIN".to_owned(),
            width: 64,
            height: 64,
            rgba: vec![255; 64 * 64 * 4],
        };

        state.record_operation(&SteamworksStatsOperation::AchievementNamesListed {
            offset: 0,
            total: 1,
            names: vec!["ACH_ONE".to_owned()],
        });
        assert_eq!(
            state.last_achievements(),
            &[SteamworksAchievementInfo {
                api_name: "ACH_ONE".to_owned(),
                ..Default::default()
            }]
        );

        state.record_operation(&SteamworksStatsOperation::AchievementsListed {
            offset: 0,
            total: 1,
            achievements: vec![achievement.clone()],
        });
        state.record_operation(&SteamworksStatsOperation::AchievementIconRead {
            name: "ACH_WIN".to_owned(),
            icon: SteamworksAchievementIconStatus::Available(icon.clone()),
        });
        state.record_operation(&SteamworksStatsOperation::AchievementIconFetched {
            name: "ACH_WIN".to_owned(),
            achieved: true,
            icon_handle: 99,
            icon: SteamworksAchievementIconStatus::PendingOrUnavailable,
        });
        state.record_operation(
            &SteamworksStatsOperation::AchievementGlobalPercentagesListed {
                offset: 0,
                total: 1,
                percentages: vec![SteamworksAchievementGlobalPercentage {
                    api_name: "ACH_WIN".to_owned(),
                    percent: 12.5,
                }],
            },
        );

        assert_eq!(state.last_achievements(), &[achievement]);
        assert_eq!(state.last_achievement_icon(), Some(&icon));
        assert_eq!(state.achievement_icon_callback_count(), 1);
        assert_eq!(
            state.last_global_achievement_percentages(),
            &[SteamworksAchievementGlobalPercentage {
                api_name: "ACH_WIN".to_owned(),
                percent: 12.5,
            }]
        );
    }

    #[test]
    fn global_stats_state_records_latest_values() {
        let mut state = SteamworksStatsState::default();
        let game_id = steamworks::GameId::from_raw(480);

        state.record_operation(&SteamworksStatsOperation::GlobalStatsReceived { game_id });
        state.record_operation(&SteamworksStatsOperation::GlobalStatI64Read {
            name: "total_kills".to_owned(),
            value: 123,
        });
        state.record_operation(&SteamworksStatsOperation::GlobalStatF64Read {
            name: "average_accuracy".to_owned(),
            value: 0.75,
        });
        state.record_operation(&SteamworksStatsOperation::GlobalStatHistoryI64Read {
            name: "daily_kills".to_owned(),
            values: vec![3, 2, 1],
        });
        state.record_operation(&SteamworksStatsOperation::GlobalStatHistoryF64Read {
            name: "daily_accuracy".to_owned(),
            values: vec![0.5, 0.6],
        });

        assert_eq!(state.last_global_stats_game_id(), Some(game_id));
        assert_eq!(
            state.last_global_stat_i64(),
            Some(&SteamworksGlobalStatValue {
                name: "total_kills".to_owned(),
                value: 123,
            })
        );
        assert_eq!(
            state.last_global_stat_f64(),
            Some(&SteamworksGlobalStatValue {
                name: "average_accuracy".to_owned(),
                value: 0.75,
            })
        );
        assert_eq!(
            state.last_global_stat_history_i64(),
            Some(&SteamworksGlobalStatHistory {
                name: "daily_kills".to_owned(),
                values: vec![3, 2, 1],
            })
        );
        assert_eq!(
            state.last_global_stat_history_f64(),
            Some(&SteamworksGlobalStatHistory {
                name: "daily_accuracy".to_owned(),
                values: vec![0.5, 0.6],
            })
        );

        state.record_operation(&SteamworksStatsOperation::GlobalStatsRequested { history_days: 7 });

        assert_eq!(state.last_global_stats_game_id(), None);
        assert_eq!(state.last_global_stat_i64(), None);
        assert_eq!(state.last_global_stat_f64(), None);
        assert_eq!(state.last_global_stat_history_i64(), None);
        assert_eq!(state.last_global_stat_history_f64(), None);
    }

    #[test]
    fn achievement_icon_callback_operation_preserves_context() {
        let icon = SteamworksAchievementIconStatus::Available(achievement_icon_from_rgba(
            "ACH_WIN",
            vec![255; 64 * 64 * 4],
        ));
        let event = steamworks::UserAchievementIconFetched {
            game_id: steamworks::GameId::from_raw(480),
            achievement_name: "ACH_WIN".to_owned(),
            achieved: true,
            icon_handle: 42,
        };

        assert_eq!(
            achievement_icon_fetched_operation(&event, icon.clone()),
            SteamworksStatsOperation::AchievementIconFetched {
                name: "ACH_WIN".to_owned(),
                achieved: true,
                icon_handle: 42,
                icon,
            }
        );
        assert_eq!(
            SteamworksAchievementIconStatus::PendingOrUnavailable.as_icon(),
            None
        );
    }
}
