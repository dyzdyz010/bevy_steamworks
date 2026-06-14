//! High-level Bevy ECS integration for Steam Matchmaking Servers.
//!
//! This module builds on top of the upstream
//! [`steamworks::MatchmakingServers`] API. It exposes Steam server-browser
//! list requests through Bevy commands/results while keeping the upstream
//! request handles owned by the plugin.

use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::SteamworksSystem;

mod callbacks;
mod commands;
mod messages;
mod requests;
mod state;
#[cfg(test)]
mod tests;
mod types;
mod validation;

use commands::process_matchmaking_servers_commands;
pub use messages::*;
use requests::*;
pub use state::SteamworksMatchmakingServersState;
pub use types::*;

/// Maximum byte length for one Steam server-list filter key or value.
pub const STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES: usize = 255;

/// Bevy plugin for high-level Steam Matchmaking Servers commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksMatchmakingServersCommand`] and
/// [`SteamworksMatchmakingServersResult`] messages and processes commands in
/// [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksMatchmakingServersPlugin;

impl SteamworksMatchmakingServersPlugin {
    /// Creates a Matchmaking Servers plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksMatchmakingServersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksMatchmakingServersState>()
            .init_resource::<SteamworksMatchmakingServersAsyncResults>()
            .init_resource::<SteamworksMatchmakingServerListRequests>()
            .add_message::<SteamworksMatchmakingServersCommand>()
            .add_message::<SteamworksMatchmakingServersResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessMatchmakingServersCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_matchmaking_servers_commands
                    .in_set(SteamworksSystem::ProcessMatchmakingServersCommands),
            );
    }
}
