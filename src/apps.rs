//! High-level Bevy ECS integration for Steam app, ownership, language, and
//! launch-parameter queries.
//!
//! This module builds on top of the upstream [`steamworks::Apps`] API. It keeps
//! common application-level Steam checks in Bevy messages while preserving raw
//! API access through [`crate::SteamworksClient`].

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

use commands::process_apps_commands;
pub use messages::*;
pub use state::SteamworksAppsState;
pub use types::*;

/// Bevy plugin for high-level Steam app and launch-parameter commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksAppsCommand`] and [`SteamworksAppsResult`] messages and runs its
/// command processor in [`bevy_app::First`] after Steam callbacks. It also
/// mirrors [`crate::SteamworksEvent::NewUrlLaunchParameters`] into apps results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksAppsPlugin;

impl SteamworksAppsPlugin {
    /// Creates an apps plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksAppsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksAppsState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksAppsCommand>()
            .add_message::<SteamworksAppsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessAppsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_apps_commands.in_set(SteamworksSystem::ProcessAppsCommands),
            );
    }
}
