//! High-level Bevy ECS integration for Steam Timeline.
//!
//! This module builds on top of the upstream [`steamworks::Timeline`] API. It
//! exposes Bevy messages for timeline state and event submissions, while
//! validating inputs that upstream converts into C strings.

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

use commands::process_timeline_commands;
pub use messages::*;
pub use state::SteamworksTimelineState;
pub use types::*;

/// Bevy plugin for high-level Steam Timeline commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksTimelineCommand`] and [`SteamworksTimelineResult`] messages and
/// runs its command processor in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksTimelinePlugin;

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
