//! High-level Bevy ECS integration for Steam Input.
//!
//! This module builds on top of the upstream [`steamworks::Input`] API. It
//! exposes Bevy messages for common controller, action set, and action data
//! workflows while keeping raw Steamworks SDK binding types out of this crate's
//! public contract.

mod commands;
mod messages;
mod plugin;
mod state;
#[cfg(test)]
mod tests;
mod types;
mod validation;

pub use messages::*;
pub use state::SteamworksInputState;
pub use types::*;

/// Bevy plugin for high-level Steam Input commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksInputCommand`] and [`SteamworksInputResult`] messages and runs
/// its command processor in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksInputPlugin;
