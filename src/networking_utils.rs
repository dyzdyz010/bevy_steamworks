//! High-level Bevy ECS integration for Steam Networking Utils.
//!
//! This module builds on top of the upstream
//! [`steamworks::networking_utils::NetworkingUtils`] API. It exposes Steam
//! Datagram Relay initialization and relay status diagnostics through Bevy
//! commands/results, and turns relay status callbacks into owned Bevy messages.

mod callbacks;
mod commands;
mod messages;
mod plugin;
mod state;
#[cfg(test)]
mod tests;
mod types;
mod validation;

pub use messages::*;
pub use state::SteamworksNetworkingUtilsState;
pub use types::*;

/// Bevy plugin for high-level Steam Networking Utils commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksNetworkingUtilsCommand`] and
/// [`SteamworksNetworkingUtilsResult`] messages and processes commands in
/// [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksNetworkingUtilsPlugin;
