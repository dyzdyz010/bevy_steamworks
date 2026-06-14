use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{SteamworksEvent, SteamworksSystem};

use super::{
    async_results::SteamworksMatchmakingAsyncResults, commands::process_matchmaking_commands,
    SteamworksMatchmakingCommand, SteamworksMatchmakingPlugin, SteamworksMatchmakingResult,
    SteamworksMatchmakingState,
};

impl SteamworksMatchmakingPlugin {
    /// Creates a matchmaking plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksMatchmakingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksMatchmakingState>()
            .init_resource::<SteamworksMatchmakingAsyncResults>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksMatchmakingCommand>()
            .add_message::<SteamworksMatchmakingResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessMatchmakingCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_matchmaking_commands.in_set(SteamworksSystem::ProcessMatchmakingCommands),
            );
    }
}
