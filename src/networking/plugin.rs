use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{SteamworksEvent, SteamworksSystem};

use super::{
    commands::process_networking_commands, SteamworksNetworkingCommand, SteamworksNetworkingPlugin,
    SteamworksNetworkingResult, SteamworksNetworkingState,
};

impl SteamworksNetworkingPlugin {
    /// Creates a legacy P2P Networking plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksNetworkingState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksNetworkingCommand>()
            .add_message::<SteamworksNetworkingResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessNetworkingCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_networking_commands.in_set(SteamworksSystem::ProcessNetworkingCommands),
            );
    }
}
