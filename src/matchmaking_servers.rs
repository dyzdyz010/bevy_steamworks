//! High-level Bevy ECS integration for Steam Matchmaking Servers.
//!
//! This module builds on top of the upstream
//! [`steamworks::MatchmakingServers`] API. It exposes Steam server-browser
//! list requests through Bevy commands/results while keeping the upstream
//! request handles owned by the plugin.

mod callbacks;
mod commands;
mod messages;
mod plugin;
mod requests;
mod state;
#[cfg(test)]
mod tests;
mod types;
mod validation;

pub use messages::*;
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
