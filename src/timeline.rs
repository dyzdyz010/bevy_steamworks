//! High-level Bevy ECS integration for Steam Timeline.
//!
//! This module builds on top of the upstream [`steamworks::Timeline`] API. It
//! exposes Bevy messages for timeline state and event submissions, while
//! validating inputs that upstream converts into C strings.

mod commands;
mod messages;
mod plugin;
mod state;
#[cfg(test)]
mod tests;
mod types;
mod validation;

pub use messages::*;
pub use state::SteamworksTimelineState;
pub use types::*;

/// Bevy plugin for high-level Steam Timeline commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksTimelineCommand`] and [`SteamworksTimelineResult`] messages and
/// runs its command processor in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksTimelinePlugin;
