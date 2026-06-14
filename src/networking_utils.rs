//! High-level Bevy ECS integration for Steam Networking Utils.
//!
//! This module builds on top of the upstream
//! [`steamworks::networking_utils::NetworkingUtils`] API. It exposes Steam
//! Datagram Relay initialization and relay status diagnostics through Bevy
//! commands/results, and turns relay status callbacks into owned Bevy messages.

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
mod validation;

use commands::process_networking_utils_commands;
pub use messages::*;
pub use state::SteamworksNetworkingUtilsState;
pub use types::*;

/// Bevy plugin for high-level Steam Networking Utils commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksNetworkingUtilsCommand`] and
/// [`SteamworksNetworkingUtilsResult`] messages and processes commands in
/// [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksNetworkingUtilsPlugin;

impl SteamworksNetworkingUtilsPlugin {
    /// Creates a Networking Utils plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksNetworkingUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksNetworkingUtilsState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksNetworkingUtilsCommand>()
            .add_message::<SteamworksNetworkingUtilsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessNetworkingUtilsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_networking_utils_commands
                    .in_set(SteamworksSystem::ProcessNetworkingUtilsCommands),
            );
    }
}
