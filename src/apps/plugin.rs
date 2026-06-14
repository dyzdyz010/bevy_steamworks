use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{SteamworksEvent, SteamworksSystem};

use super::{
    commands::process_apps_commands, SteamworksAppsCommand, SteamworksAppsPlugin,
    SteamworksAppsResult, SteamworksAppsState,
};

impl SteamworksAppsPlugin {
    /// Creates an apps plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksAppsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksAppsState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksAppsCommand>()
            .add_message::<SteamworksAppsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessAppsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_apps_commands.in_set(SteamworksSystem::ProcessAppsCommands),
            );
    }
}
