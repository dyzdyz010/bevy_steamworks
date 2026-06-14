//! High-level Bevy ECS integration for Steam user stats and achievements.
//!
//! This module builds on top of the upstream [`steamworks::UserStats`] API.
//! Games can keep using the raw Steamworks API through [`crate::SteamworksClient`],
//! while this plugin provides a message-driven layer for common Bevy workflows.

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
mod plugin;
mod snapshots;
mod state;
#[cfg(test)]
mod tests;
mod types;
mod validation;

#[cfg(test)]
use achievements::achievement_icon_from_rgba;
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
