//! High-level Bevy ECS integration for Steam screenshots.
//!
//! This module builds on top of the upstream [`steamworks::screenshots::Screenshots`] API.
//! It submits screenshot operations through Bevy messages while mirroring final
//! Steam callback confirmations from [`crate::SteamworksEvent`] into
//! [`SteamworksScreenshotsResult`].

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

use commands::process_screenshots_commands;
pub use messages::*;
pub use state::SteamworksScreenshotsState;
pub use types::*;

/// Bevy plugin for high-level Steam screenshot commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksScreenshotsCommand`] and [`SteamworksScreenshotsResult`] messages
/// and runs its command processor in [`bevy_app::First`] after Steam callbacks.
/// It also mirrors screenshot requested/ready callbacks into screenshot results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksScreenshotsPlugin;

impl SteamworksScreenshotsPlugin {
    /// Creates a screenshots plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksScreenshotsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksScreenshotsState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksScreenshotsCommand>()
            .add_message::<SteamworksScreenshotsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessScreenshotsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_screenshots_commands.in_set(SteamworksSystem::ProcessScreenshotsCommands),
            );
    }
}
