use super::{
    SteamworksLeaderboardDataRequest, SteamworksStatsCommand, SteamworksStatsError,
    SteamworksStatsOperation, STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND,
    STEAMWORKS_LEADERBOARD_MAX_DETAILS, STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND,
};

pub(super) fn operation_requires_store(operation: &SteamworksStatsOperation) -> bool {
    matches!(
        operation,
        SteamworksStatsOperation::StatI32Set { .. }
            | SteamworksStatsOperation::StatF32Set { .. }
            | SteamworksStatsOperation::AchievementUnlocked { .. }
            | SteamworksStatsOperation::AchievementCleared { .. }
    )
}

pub(super) fn validate_stats_command(
    command: &SteamworksStatsCommand,
) -> Result<(), SteamworksStatsError> {
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
