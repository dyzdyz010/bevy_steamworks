//! High-level Bevy ECS integration for Steam user stats and achievements.
//!
//! This module builds on top of the upstream [`steamworks::UserStats`] API.
//! Games can keep using the raw Steamworks API through [`SteamworksClient`],
//! while this plugin provides a message-driven layer for common Bevy workflows.

use std::sync::{Arc, Mutex};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksSystem};

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

    /// Returns the most recent synchronous error observed by the stats plugin.
    pub fn last_error(&self) -> Option<&SteamworksStatsError> {
        self.last_error.as_ref()
    }

    fn record_error(&mut self, error: SteamworksStatsError) {
        self.last_error = Some(error);
    }
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
    /// Final server confirmation arrives later through
    /// [`crate::SteamworksEvent::UserStatsStored`].
    StatsStoreSubmitted,
    /// All stats were reset locally.
    AllStatsReset {
        /// Whether achievements were also reset.
        achievements_too: bool,
    },
}

/// Result message emitted by [`SteamworksStatsPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksStatsResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksStatsOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksStatsCommand,
        /// Failure reason.
        error: SteamworksStatsError,
    },
}

/// Synchronous errors from [`SteamworksStatsPlugin`].
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
}

impl SteamworksStatsError {
    fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }

    fn steam_error(operation: &'static str, source: steamworks::SteamError) -> Self {
        Self::SteamError { operation, source }
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

fn process_stats_commands(
    client: Option<Res<SteamworksClient>>,
    settings: Res<SteamworksStatsSettings>,
    mut state: ResMut<SteamworksStatsState>,
    async_results: Res<SteamworksStatsAsyncResults>,
    mut commands: ResMut<Messages<SteamworksStatsCommand>>,
    mut results: MessageWriter<SteamworksStatsResult>,
) {
    for result in async_results.drain() {
        if let SteamworksStatsResult::Err { error, .. } = &result {
            state.record_error(error.clone());
        }
        results.write(result);
    }

    let Some(client) = client else {
        let error = SteamworksStatsError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
            results.write(SteamworksStatsResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    if settings.request_current_user_stats_on_startup && !state.current_user_stats_requested {
        request_current_user_stats(&client, &mut state, &mut results);
    }

    for command in commands.drain() {
        handle_stats_command(
            &client,
            command,
            &settings,
            &async_results,
            &mut state,
            &mut results,
        );
    }

    if settings.auto_store && state.pending_store {
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

fn handle_stats_command(
    client: &SteamworksClient,
    command: SteamworksStatsCommand,
    settings: &SteamworksStatsSettings,
    async_results: &SteamworksStatsAsyncResults,
    state: &mut SteamworksStatsState,
    results: &mut MessageWriter<SteamworksStatsResult>,
) {
    let result = match command.clone() {
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
    };

    match result {
        Ok(operation) => {
            if settings.auto_store && operation_requires_store(&operation) {
                state.pending_store = true;
            }
            tracing::debug!(
                target: "bevy_steamworks",
                operation = ?operation,
                "processed Steamworks stats command"
            );
            results.write(SteamworksStatsResult::Ok(operation));
        }
        Err(error) => {
            state.record_error(error.clone());
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
}
