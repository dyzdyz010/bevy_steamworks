//! High-level Bevy ECS integration for Steam utility queries and overlay helpers.
//!
//! This module builds on top of the upstream [`steamworks::Utils`] API. It keeps
//! common utility calls in Bevy messages while mirroring text-input dismissal
//! callbacks from [`crate::SteamworksEvent`] into [`SteamworksUtilsResult`].

mod callbacks;
mod commands;
mod messages;
mod plugin;
mod state;
#[cfg(test)]
mod tests;
mod types;

pub(crate) use callbacks::SteamworksUtilsCallbackQueue;
pub use messages::*;
pub use state::SteamworksUtilsState;
pub use types::*;

/// Bevy plugin for high-level Steam utility commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksUtilsCommand`] and [`SteamworksUtilsResult`] messages and runs
/// its command processor in [`bevy_app::First`] after Steam callbacks. It also
/// mirrors gamepad text input dismissal callbacks into utils results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksUtilsPlugin;
