//! High-level Bevy ECS integration for Steam utility queries and overlay helpers.
//!
//! This module builds on top of the upstream [`steamworks::Utils`] API. It keeps
//! common utility calls in Bevy messages while mirroring text-input dismissal
//! callbacks from [`crate::SteamworksEvent`] into [`SteamworksUtilsResult`].

use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{SteamworksEvent, SteamworksSystem};

mod callbacks;
mod commands;
mod messages;
mod state;
#[cfg(test)]
mod tests;
mod types;

use commands::process_utils_commands;
pub use messages::*;
pub use state::SteamworksUtilsState;
pub use types::*;

/// Bevy plugin for high-level Steam utility commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksUtilsCommand`] and [`SteamworksUtilsResult`] messages and runs
/// its command processor in [`bevy_app::First`] after Steam callbacks. It also
/// mirrors gamepad text input dismissal callbacks into utils results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksUtilsPlugin;

impl SteamworksUtilsPlugin {
    /// Creates a utils plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksUtilsState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksUtilsCommand>()
            .add_message::<SteamworksUtilsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessUtilsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_utils_commands.in_set(SteamworksSystem::ProcessUtilsCommands),
            );
    }
}
