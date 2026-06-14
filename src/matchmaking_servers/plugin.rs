use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::SteamworksSystem;

use super::{
    commands::process_matchmaking_servers_commands,
    requests::{SteamworksMatchmakingServerListRequests, SteamworksMatchmakingServersAsyncResults},
    SteamworksMatchmakingServersCommand, SteamworksMatchmakingServersPlugin,
    SteamworksMatchmakingServersResult, SteamworksMatchmakingServersState,
};

impl SteamworksMatchmakingServersPlugin {
    /// Creates a Matchmaking Servers plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksMatchmakingServersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksMatchmakingServersState>()
            .init_resource::<SteamworksMatchmakingServersAsyncResults>()
            .init_resource::<SteamworksMatchmakingServerListRequests>()
            .add_message::<SteamworksMatchmakingServersCommand>()
            .add_message::<SteamworksMatchmakingServersResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessMatchmakingServersCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_matchmaking_servers_commands
                    .in_set(SteamworksSystem::ProcessMatchmakingServersCommands),
            );
    }
}
