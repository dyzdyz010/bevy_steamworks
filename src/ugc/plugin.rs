use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{SteamworksEvent, SteamworksSystem};

use super::{
    async_results::SteamworksUgcAsyncResults, commands::process_ugc_commands,
    update_watches::SteamworksUgcUpdateWatches, SteamworksUgcCommand, SteamworksUgcPlugin,
    SteamworksUgcResult, SteamworksUgcState,
};

impl SteamworksUgcPlugin {
    /// Creates a UGC plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksUgcPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksUgcState>()
            .init_resource::<SteamworksUgcAsyncResults>()
            .init_resource::<SteamworksUgcUpdateWatches>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksUgcCommand>()
            .add_message::<SteamworksUgcResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessUgcCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_ugc_commands.in_set(SteamworksSystem::ProcessUgcCommands),
            );
    }
}
