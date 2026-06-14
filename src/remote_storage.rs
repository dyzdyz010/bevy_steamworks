//! High-level Bevy ECS integration for Steam Remote Storage.
//!
//! This module builds on top of the upstream [`steamworks::RemoteStorage`] API.
//! It intentionally avoids the upstream blocking file reader/writer helpers in
//! Bevy systems; games can still access those through [`crate::SteamworksClient`]
//! when they can move file IO out of the frame-critical path.

use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::SteamworksSystem;

mod async_results;
mod commands;
mod messages;
mod state;
#[cfg(test)]
mod tests;
mod types;

use async_results::SteamworksRemoteStorageAsyncResults;
use commands::process_remote_storage_commands;
pub use messages::*;
pub use state::SteamworksRemoteStorageState;
pub use types::*;

/// Bevy plugin for high-level Steam Remote Storage commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksRemoteStorageCommand`] and [`SteamworksRemoteStorageResult`]
/// messages and runs its command processor in [`bevy_app::First`] after Steam
/// callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksRemoteStoragePlugin;

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
