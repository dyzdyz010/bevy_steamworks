use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{SteamworksEvent, SteamworksSystem};

use super::{
    commands::process_remote_play_commands, SteamworksRemotePlayCommand,
    SteamworksRemotePlayPlugin, SteamworksRemotePlayResult, SteamworksRemotePlayState,
};

impl SteamworksRemotePlayPlugin {
    /// Creates a Remote Play plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksRemotePlayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksRemotePlayState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksRemotePlayCommand>()
            .add_message::<SteamworksRemotePlayResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessRemotePlayCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_remote_play_commands.in_set(SteamworksSystem::ProcessRemotePlayCommands),
            );
    }
}
