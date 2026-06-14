//! High-level Bevy ECS integration for Steam friends, Rich Presence, overlays,
//! and invites.
//!
//! This module builds on top of the upstream [`steamworks::Friends`] API. Games
//! can keep using the raw Steamworks API through [`crate::SteamworksClient`],
//! while this plugin provides a message-driven layer for common Bevy workflows.

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
pub use state::SteamworksFriendsState;
pub use types::*;

/// Bevy plugin for high-level Steam friends, Rich Presence, overlay, and invite commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksFriendsCommand`] and [`SteamworksFriendsResult`] messages and
/// runs its command processor in [`bevy_app::First`] after Steam callbacks. It
/// also mirrors common friends, overlay, and invite callbacks into friends results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksFriendsPlugin;
