use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{SteamworksEvent, SteamworksSystem};

use super::{
    commands::process_utils_commands, SteamworksUtilsCommand, SteamworksUtilsPlugin,
    SteamworksUtilsResult, SteamworksUtilsState,
};

impl SteamworksUtilsPlugin {
    /// Creates a utils plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksUtilsState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksUtilsCommand>()
            .add_message::<SteamworksUtilsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessUtilsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_utils_commands.in_set(SteamworksSystem::ProcessUtilsCommands),
            );
    }
}
