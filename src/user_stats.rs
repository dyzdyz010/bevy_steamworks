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
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
    system::SystemParam,
};

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

/// Maximum leaderboard detail integers accepted by one command.
pub const STEAMWORKS_LEADERBOARD_MAX_DETAILS: usize = 64;

/// Maximum leaderboard entries requested by one download command.
pub const STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND: usize = 1000;

/// Default achievement catalog items read by one command.
pub const STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND: usize = 64;

/// Maximum achievement catalog items accepted by one command.
pub const STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND: usize = 256;

mod messages;
mod state;
mod types;

pub use messages::*;
pub use state::{SteamworksStatsSettings, SteamworksStatsState};
pub use types::*;

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

        let id = SteamworksLeaderboardId::from_raw(self.next_id);
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

    if should_submit_store(&io.settings, &state) {
        match client.user_stats().store_stats() {
            Ok(()) => {
                state.pending_store = false;
                state.force_store = false;
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

fn should_submit_store(settings: &SteamworksStatsSettings, state: &SteamworksStatsState) -> bool {
    state.pending_store && (settings.auto_store || state.force_store)
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
                    state.force_store = true;
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
            Ok(SteamworksStatsOperation::LeaderboardScoreUploadSubmitted {
                leaderboard,
                method,
                score,
                details,
            })
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

#[cfg(test)]
mod tests;
