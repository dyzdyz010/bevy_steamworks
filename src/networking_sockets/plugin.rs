use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::SteamworksSystem;

use super::{
    commands::process_networking_sockets_commands, handles::SteamworksNetworkingSocketsHandles,
    SteamworksNetworkingSocketsCommand, SteamworksNetworkingSocketsPlugin,
    SteamworksNetworkingSocketsResult, SteamworksNetworkingSocketsState,
};

impl SteamworksNetworkingSocketsPlugin {
    /// Creates a Networking Sockets plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksNetworkingSocketsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksNetworkingSocketsState>()
            .init_resource::<SteamworksNetworkingSocketsHandles>()
            .add_message::<SteamworksNetworkingSocketsCommand>()
            .add_message::<SteamworksNetworkingSocketsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessNetworkingSocketsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_networking_sockets_commands
                    .in_set(SteamworksSystem::ProcessNetworkingSocketsCommands),
            );
    }
}
