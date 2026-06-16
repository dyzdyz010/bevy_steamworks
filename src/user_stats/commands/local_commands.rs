use crate::SteamworksClient;

use super::super::{
    SteamworksStatsCommand, SteamworksStatsError, SteamworksStatsOperation, SteamworksStatsState,
};

pub(super) fn handle_local_stats_command(
    client: &SteamworksClient,
    state: &mut SteamworksStatsState,
    command: SteamworksStatsCommand,
) -> Result<SteamworksStatsOperation, SteamworksStatsError> {
    match command {
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
        _ => unreachable!("non-local stats command routed to local stats handler"),
    }
}
