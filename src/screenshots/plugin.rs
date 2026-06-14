use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{SteamworksEvent, SteamworksSystem};

use super::{
    commands::process_screenshots_commands, SteamworksScreenshotsCommand,
    SteamworksScreenshotsPlugin, SteamworksScreenshotsResult, SteamworksScreenshotsState,
};

impl SteamworksScreenshotsPlugin {
    /// Creates a screenshots plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksScreenshotsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksScreenshotsState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksScreenshotsCommand>()
            .add_message::<SteamworksScreenshotsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessScreenshotsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_screenshots_commands.in_set(SteamworksSystem::ProcessScreenshotsCommands),
            );
    }
}
