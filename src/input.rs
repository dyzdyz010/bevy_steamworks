//! High-level Bevy ECS integration for Steam Input.
//!
//! This module builds on top of the upstream [`steamworks::Input`] API. It
//! exposes Bevy messages for common controller, action set, and action data
//! workflows while keeping raw Steamworks SDK binding types out of this crate's
//! public contract.

use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::SteamworksSystem;

mod commands;
mod messages;
mod state;
#[cfg(test)]
mod tests;
mod types;
mod validation;

use commands::process_input_commands;
pub use messages::*;
pub use state::SteamworksInputState;
pub use types::*;

/// Bevy plugin for high-level Steam Input commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksInputCommand`] and [`SteamworksInputResult`] messages and runs
/// its command processor in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksInputPlugin;

impl SteamworksInputPlugin {
    /// Creates a Steam Input plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksInputState>()
            .add_message::<SteamworksInputCommand>()
            .add_message::<SteamworksInputResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessInputCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_input_commands.in_set(SteamworksSystem::ProcessInputCommands),
            );
    }
}
