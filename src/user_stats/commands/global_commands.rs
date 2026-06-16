use crate::SteamworksClient;

use super::super::{
    achievements::list_achievement_global_percentages, async_results::SteamworksStatsAsyncResults,
    SteamworksStatsCommand, SteamworksStatsError, SteamworksStatsOperation, SteamworksStatsResult,
};

pub(super) fn handle_global_stats_command(
    client: &SteamworksClient,
    async_results: &SteamworksStatsAsyncResults,
    command: SteamworksStatsCommand,
) -> Result<SteamworksStatsOperation, SteamworksStatsError> {
    match command {
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
        _ => unreachable!("non-global stats command routed to global stats handler"),
    }
}
