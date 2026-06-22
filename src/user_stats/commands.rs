use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
    system::SystemParam,
};

use crate::{SteamworksClient, SteamworksEvent};

mod achievement_commands;
mod global_commands;
mod leaderboard_commands;
mod local_commands;

use achievement_commands::handle_achievement_command;
use global_commands::handle_global_stats_command;
use leaderboard_commands::handle_leaderboard_command;
use local_commands::handle_local_stats_command;

use super::{
    async_results::SteamworksStatsAsyncResults,
    callbacks::process_stats_steam_events,
    leaderboards::SteamworksStatsLeaderboardHandles,
    lifecycle::{request_current_user_stats, should_submit_store},
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
    let result = validate_stats_command(&command).and_then(|()| {
        route_stats_command(client, async_results, leaderboards, state, command.clone())
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

fn route_stats_command(
    client: &SteamworksClient,
    async_results: &SteamworksStatsAsyncResults,
    leaderboards: &SteamworksStatsLeaderboardHandles,
    state: &mut SteamworksStatsState,
    command: SteamworksStatsCommand,
) -> Result<SteamworksStatsOperation, SteamworksStatsError> {
    match command {
        SteamworksStatsCommand::RequestCurrentUserStats
        | SteamworksStatsCommand::RequestUserStats { .. }
        | SteamworksStatsCommand::GetStatI32 { .. }
        | SteamworksStatsCommand::SetStatI32 { .. }
        | SteamworksStatsCommand::GetStatF32 { .. }
        | SteamworksStatsCommand::SetStatF32 { .. }
        | SteamworksStatsCommand::StoreStats
        | SteamworksStatsCommand::ResetAllStats { .. } => {
            handle_local_stats_command(client, state, command)
        }
        SteamworksStatsCommand::GetAchievement { .. }
        | SteamworksStatsCommand::GetAchievementCount
        | SteamworksStatsCommand::ListAchievementNames { .. }
        | SteamworksStatsCommand::ListAchievements { .. }
        | SteamworksStatsCommand::GetAchievementIcon { .. }
        | SteamworksStatsCommand::UnlockAchievement { .. }
        | SteamworksStatsCommand::ClearAchievement { .. }
        | SteamworksStatsCommand::GetAchievementAndUnlockTime { .. }
        | SteamworksStatsCommand::GetAchievementDisplayAttribute { .. } => {
            handle_achievement_command(client, command)
        }
        SteamworksStatsCommand::RequestGlobalAchievementPercentages
        | SteamworksStatsCommand::GetAchievementAchievedPercent { .. }
        | SteamworksStatsCommand::ListAchievementGlobalPercentages { .. }
        | SteamworksStatsCommand::RequestGlobalStats { .. }
        | SteamworksStatsCommand::GetGlobalStatI64 { .. }
        | SteamworksStatsCommand::GetGlobalStatF64 { .. }
        | SteamworksStatsCommand::GetGlobalStatHistoryI64 { .. }
        | SteamworksStatsCommand::GetGlobalStatHistoryF64 { .. } => {
            handle_global_stats_command(client, async_results, command)
        }
        SteamworksStatsCommand::FindLeaderboard { .. }
        | SteamworksStatsCommand::FindOrCreateLeaderboard { .. }
        | SteamworksStatsCommand::GetLeaderboardInfo { .. }
        | SteamworksStatsCommand::UploadLeaderboardScore { .. }
        | SteamworksStatsCommand::DownloadLeaderboardEntries { .. }
        | SteamworksStatsCommand::ForgetLeaderboard { .. } => {
            handle_leaderboard_command(client, async_results, leaderboards, command)
        }
    }
}
