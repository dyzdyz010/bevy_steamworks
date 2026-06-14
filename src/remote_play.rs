//! High-level Bevy ECS integration for Steam Remote Play.
//!
//! This module builds on top of the upstream [`steamworks::RemotePlay`] API.
//! Session connect/disconnect callbacks are mirrored from
//! [`crate::SteamworksEvent`] into [`SteamworksRemotePlayResult`].

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

use commands::process_remote_play_commands;
pub use messages::*;
pub use state::SteamworksRemotePlayState;
pub use types::*;

/// Bevy plugin for high-level Steam Remote Play commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksRemotePlayCommand`] and [`SteamworksRemotePlayResult`] messages
/// and runs its command processor in [`bevy_app::First`] after Steam callbacks.
/// It also mirrors Remote Play session connect/disconnect callbacks into Remote
/// Play results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksRemotePlayPlugin;

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
