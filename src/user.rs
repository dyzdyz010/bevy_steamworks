//! High-level Bevy ECS integration for Steam user identity and authentication.
//!
//! This module builds on top of the upstream [`steamworks::User`] API. It keeps
//! common authentication flows in Bevy messages while mirroring relevant
//! low-level callback confirmations from [`crate::SteamworksEvent`] into
//! [`SteamworksUserResult`].

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

use commands::process_user_commands;
pub use messages::*;
pub use state::SteamworksUserState;
pub use types::*;

/// Bevy plugin for high-level Steam user identity and authentication commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksUserCommand`] and [`SteamworksUserResult`] messages and runs its
/// command processor in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksUserPlugin;

impl SteamworksUserPlugin {
    /// Creates a user plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksUserPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksUserState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksUserCommand>()
            .add_message::<SteamworksUserResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessUserCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_user_commands.in_set(SteamworksSystem::ProcessUserCommands),
            );
    }
}
