//! High-level Bevy ECS integration for Steam screenshots.
//!
//! This module builds on top of the upstream [`steamworks::screenshots::Screenshots`] API.
//! It submits screenshot operations through Bevy messages while mirroring final
//! Steam callback confirmations from [`crate::SteamworksEvent`] into
//! [`SteamworksScreenshotsResult`].

mod callbacks;
mod commands;
mod messages;
mod plugin;
mod state;
#[cfg(test)]
mod tests;
mod types;

pub use messages::*;
pub use state::SteamworksScreenshotsState;
pub use types::*;

/// Bevy plugin for high-level Steam screenshot commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksScreenshotsCommand`] and [`SteamworksScreenshotsResult`] messages
/// and runs its command processor in [`bevy_app::First`] after Steam callbacks.
/// It also mirrors screenshot requested/ready callbacks into screenshot results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksScreenshotsPlugin;
