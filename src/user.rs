//! High-level Bevy ECS integration for Steam user identity and authentication.
//!
//! This module builds on top of the upstream [`steamworks::User`] API. It keeps
//! common authentication flows in Bevy messages while mirroring relevant
//! low-level callback confirmations from [`crate::SteamworksEvent`] into
//! [`SteamworksUserResult`].

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
pub use state::SteamworksUserState;
pub use types::*;

/// Bevy plugin for high-level Steam user identity and authentication commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksUserCommand`] and [`SteamworksUserResult`] messages and runs its
/// command processor in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksUserPlugin;
