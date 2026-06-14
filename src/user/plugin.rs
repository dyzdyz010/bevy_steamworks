use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{SteamworksEvent, SteamworksSystem};

use super::{
    commands::process_user_commands, SteamworksUserCommand, SteamworksUserPlugin,
    SteamworksUserResult, SteamworksUserState,
};

impl SteamworksUserPlugin {
    /// Creates a user plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksUserPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksUserState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksUserCommand>()
            .add_message::<SteamworksUserResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessUserCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_user_commands.in_set(SteamworksSystem::ProcessUserCommands),
            );
    }
}
