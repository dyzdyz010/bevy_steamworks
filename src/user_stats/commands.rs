use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
    system::SystemParam,
};

use crate::{SteamworksClient, SteamworksEvent};

use super::{
    achievements::{
        list_achievement_global_percentages, list_achievement_infos, list_achievement_names,
        read_achievement_icon,
    },
    async_results::SteamworksStatsAsyncResults,
    callbacks::process_stats_steam_events,
    leaderboards::SteamworksStatsLeaderboardHandles,
    lifecycle::{request_current_user_stats, should_submit_store},
    snapshots::{
        snapshot_leaderboard_entry, snapshot_leaderboard_info, snapshot_leaderboard_score_uploaded,
    },
    validation::{operation_requires_store, validate_stats_command},
    SteamworksStatsCommand, SteamworksStatsError, SteamworksStatsOperation, SteamworksStatsResult,
    SteamworksStatsSettings, SteamworksStatsState,
};

#[derive(SystemParam)]
pub(super) struct SteamworksStatsIo<'w, 's> {
    settings: Res<'w, SteamworksStatsSettings>,
    async_results: Res<'w, SteamworksStatsAsyncResults>,
    leaderboards: Res<'w, SteamworksStatsLeaderboardHandles>,
    steam_events: MessageReader<'w, 's, SteamworksEvent>,
    commands: ResMut<'w, Messages<SteamworksStatsCommand>>,
}

pub(super) fn process_stats_commands(
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
