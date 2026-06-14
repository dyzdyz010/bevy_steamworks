use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{SteamworksEvent, SteamworksSystem};

use super::{
    commands::process_networking_utils_commands, SteamworksNetworkingUtilsCommand,
    SteamworksNetworkingUtilsPlugin, SteamworksNetworkingUtilsResult,
    SteamworksNetworkingUtilsState,
};

impl SteamworksNetworkingUtilsPlugin {
    /// Creates a Networking Utils plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksNetworkingUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksNetworkingUtilsState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksNetworkingUtilsCommand>()
            .add_message::<SteamworksNetworkingUtilsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessNetworkingUtilsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_networking_utils_commands
                    .in_set(SteamworksSystem::ProcessNetworkingUtilsCommands),
            );
    }
}
