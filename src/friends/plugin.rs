use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{SteamworksEvent, SteamworksSystem};

use super::{
    commands::process_friends_commands, SteamworksFriendsCommand, SteamworksFriendsPlugin,
    SteamworksFriendsResult, SteamworksFriendsState,
};

impl SteamworksFriendsPlugin {
    /// Creates a friends plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksFriendsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksFriendsState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksFriendsCommand>()
            .add_message::<SteamworksFriendsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessFriendsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_friends_commands.in_set(SteamworksSystem::ProcessFriendsCommands),
            );
    }
}
