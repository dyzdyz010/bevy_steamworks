//! High-level Bevy ECS integration for Steam's legacy P2P Networking API.
//!
//! This module builds on top of the upstream [`steamworks::Networking`] API. It
//! exists for older Steam P2P workflows; new projects should prefer
//! [`crate::SteamworksNetworkingMessagesPlugin`].

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

use commands::process_networking_commands;
pub use messages::*;
pub use state::SteamworksNetworkingState;
pub use types::*;

/// Maximum unreliable legacy P2P packet size accepted by Steam.
pub const STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES: usize = 1_200;

/// Maximum reliable legacy P2P packet size accepted by Steam.
pub const STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES: usize = 1_048_576;

/// Maximum receive buffer this command layer will allocate in one frame.
pub const STEAMWORKS_P2P_MAX_READ_PACKET_BYTES: usize = STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES;

/// Bevy plugin for high-level legacy Steam P2P Networking commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksNetworkingCommand`] and [`SteamworksNetworkingResult`] messages,
/// observes legacy P2P callbacks from [`crate::SteamworksEvent`], and processes
/// commands in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksNetworkingPlugin;

impl SteamworksNetworkingPlugin {
    /// Creates a legacy P2P Networking plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksNetworkingState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksNetworkingCommand>()
            .add_message::<SteamworksNetworkingResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessNetworkingCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_networking_commands.in_set(SteamworksSystem::ProcessNetworkingCommands),
            );
    }
}
