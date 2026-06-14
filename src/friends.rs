//! High-level Bevy ECS integration for Steam friends, Rich Presence, overlays,
//! and invites.
//!
//! This module builds on top of the upstream [`steamworks::Friends`] API. Games
//! can keep using the raw Steamworks API through [`crate::SteamworksClient`],
//! while this plugin provides a message-driven layer for common Bevy workflows.

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

use commands::process_friends_commands;
pub use messages::*;
pub use state::SteamworksFriendsState;
pub use types::*;

/// Bevy plugin for high-level Steam friends, Rich Presence, overlay, and invite commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksFriendsCommand`] and [`SteamworksFriendsResult`] messages and
/// runs its command processor in [`bevy_app::First`] after Steam callbacks. It
/// also mirrors common friends, overlay, and invite callbacks into friends results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksFriendsPlugin;

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
