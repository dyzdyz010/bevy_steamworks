use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::SteamworksSystem;

use super::{
    async_results::SteamworksRemoteStorageAsyncResults, commands::process_remote_storage_commands,
    SteamworksRemoteStorageCommand, SteamworksRemoteStoragePlugin, SteamworksRemoteStorageResult,
    SteamworksRemoteStorageState,
};

impl SteamworksRemoteStoragePlugin {
    /// Creates a Remote Storage plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksRemoteStoragePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksRemoteStorageState>()
            .init_resource::<SteamworksRemoteStorageAsyncResults>()
            .add_message::<SteamworksRemoteStorageCommand>()
            .add_message::<SteamworksRemoteStorageResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessRemoteStorageCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_remote_storage_commands
                    .in_set(SteamworksSystem::ProcessRemoteStorageCommands),
            );
    }
}
