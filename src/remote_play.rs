//! High-level Bevy ECS integration for Steam Remote Play.
//!
//! This module builds on top of the upstream [`steamworks::RemotePlay`] API.
//! Session connect/disconnect callbacks are mirrored from
//! [`crate::SteamworksEvent`] into [`SteamworksRemotePlayResult`].

mod callbacks;
mod commands;
mod messages;
mod plugin;
mod state;
#[cfg(test)]
mod tests;
mod types;

pub use messages::*;
pub use state::SteamworksRemotePlayState;
pub use types::*;

/// Bevy plugin for high-level Steam Remote Play commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksRemotePlayCommand`] and [`SteamworksRemotePlayResult`] messages
/// and runs its command processor in [`bevy_app::First`] after Steam callbacks.
/// It also mirrors Remote Play session connect/disconnect callbacks into Remote
/// Play results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksRemotePlayPlugin;
