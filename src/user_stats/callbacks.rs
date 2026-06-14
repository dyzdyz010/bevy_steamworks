use bevy_ecs::message::{MessageReader, MessageWriter};

use crate::{SteamworksClient, SteamworksEvent};

use super::{
    achievements::{achievement_icon_fetched_operation, read_achievement_icon},
    SteamworksAchievementIconStatus, SteamworksStatsOperation, SteamworksStatsResult,
    SteamworksStatsState, SteamworksUserAchievementStored, SteamworksUserStatsReceived,
    SteamworksUserStatsStored,
};

pub(super) fn process_stats_steam_events(
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
