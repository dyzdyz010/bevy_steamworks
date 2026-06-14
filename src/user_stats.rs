//! High-level Bevy ECS integration for Steam user stats and achievements.
//!
//! This module builds on top of the upstream [`steamworks::UserStats`] API.
//! Games can keep using the raw Steamworks API through [`crate::SteamworksClient`],
//! while this plugin provides a message-driven layer for common Bevy workflows.

use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{SteamworksEvent, SteamworksSystem};

/// Maximum leaderboard detail integers accepted by one command.
pub const STEAMWORKS_LEADERBOARD_MAX_DETAILS: usize = 64;

/// Maximum leaderboard entries requested by one download command.
pub const STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND: usize = 1000;

/// Default achievement catalog items read by one command.
pub const STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND: usize = 64;

/// Maximum achievement catalog items accepted by one command.
pub const STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND: usize = 256;

mod achievements;
mod async_results;
mod callbacks;
mod commands;
mod leaderboards;
mod lifecycle;
mod messages;
mod snapshots;
mod state;
#[cfg(test)]
mod tests;
mod types;
mod validation;

#[cfg(test)]
use achievements::achievement_icon_from_rgba;
use async_results::SteamworksStatsAsyncResults;
use commands::process_stats_commands;
use leaderboards::SteamworksStatsLeaderboardHandles;
#[cfg(test)]
use validation::{operation_requires_store, validate_stats_command};

pub use messages::*;
pub use state::{SteamworksStatsSettings, SteamworksStatsState};
pub use types::*;

/// Bevy plugin for high-level Steam user stats and achievements commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksStatsCommand`] and [`SteamworksStatsResult`] messages and runs
/// its command processor in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksStatsPlugin {
    settings: SteamworksStatsSettings,
}

impl SteamworksStatsPlugin {
    /// Creates a stats plugin with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a stats plugin with explicit settings.
    pub fn with_settings(settings: SteamworksStatsSettings) -> Self {
        Self { settings }
    }

    /// Sets whether current-user stats are requested automatically on startup.
    pub fn request_current_user_stats_on_startup(mut self, enabled: bool) -> Self {
        self.settings.request_current_user_stats_on_startup = enabled;
        self
    }

    /// Sets whether successful writes are automatically followed by one store call.
    pub fn auto_store(mut self, enabled: bool) -> Self {
        self.settings.auto_store = enabled;
        self
    }
}

impl Plugin for SteamworksStatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.settings.clone())
            .init_resource::<SteamworksStatsState>()
            .init_resource::<SteamworksStatsAsyncResults>()
            .init_resource::<SteamworksStatsLeaderboardHandles>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksStatsCommand>()
            .add_message::<SteamworksStatsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessStatsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_stats_commands.in_set(SteamworksSystem::ProcessStatsCommands),
            );
    }
}
