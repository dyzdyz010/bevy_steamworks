use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::SteamworksSystem;

use super::{
    commands::process_input_commands, SteamworksInputCommand, SteamworksInputPlugin,
    SteamworksInputResult, SteamworksInputState,
};

impl SteamworksInputPlugin {
    /// Creates a Steam Input plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksInputState>()
            .add_message::<SteamworksInputCommand>()
            .add_message::<SteamworksInputResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessInputCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_input_commands.in_set(SteamworksSystem::ProcessInputCommands),
            );
    }
}
