use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::SteamworksSystem;

use super::{
    commands::process_timeline_commands, SteamworksTimelineCommand, SteamworksTimelinePlugin,
    SteamworksTimelineResult, SteamworksTimelineState,
};

impl SteamworksTimelinePlugin {
    /// Creates a Timeline plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksTimelinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksTimelineState>()
            .add_message::<SteamworksTimelineCommand>()
            .add_message::<SteamworksTimelineResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessTimelineCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_timeline_commands.in_set(SteamworksSystem::ProcessTimelineCommands),
            );
    }
}
