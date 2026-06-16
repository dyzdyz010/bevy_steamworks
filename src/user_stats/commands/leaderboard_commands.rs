use crate::SteamworksClient;

use super::super::{
    async_results::SteamworksStatsAsyncResults,
    leaderboards::SteamworksStatsLeaderboardHandles,
    snapshots::{
        snapshot_leaderboard_entry, snapshot_leaderboard_info, snapshot_leaderboard_score_uploaded,
    },
    SteamworksStatsCommand, SteamworksStatsError, SteamworksStatsOperation, SteamworksStatsResult,
};

pub(super) fn handle_leaderboard_command(
    client: &SteamworksClient,
    async_results: &SteamworksStatsAsyncResults,
    leaderboards: &SteamworksStatsLeaderboardHandles,
    command: SteamworksStatsCommand,
) -> Result<SteamworksStatsOperation, SteamworksStatsError> {
    match command {
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
        _ => unreachable!("non-leaderboard command routed to leaderboard handler"),
    }
}
