//! High-level Bevy ECS integration for Steam app, ownership, language, and
//! launch-parameter queries.
//!
//! This module builds on top of the upstream [`steamworks::Apps`] API. It keeps
//! common application-level Steam checks in Bevy messages while preserving raw
//! API access through [`crate::SteamworksClient`].

mod callbacks;
mod commands;
mod messages;
mod plugin;
mod state;
#[cfg(test)]
mod tests;
mod types;

pub use messages::*;
pub use state::SteamworksAppsState;
pub use types::*;

/// Bevy plugin for high-level Steam app and launch-parameter commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksAppsCommand`] and [`SteamworksAppsResult`] messages and runs its
/// command processor in [`bevy_app::First`] after Steam callbacks. It also
/// mirrors [`crate::SteamworksEvent::NewUrlLaunchParameters`] into apps results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksAppsPlugin;
