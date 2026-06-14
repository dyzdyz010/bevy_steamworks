//! High-level Bevy ECS integration for Steam Remote Storage.
//!
//! This module builds on top of the upstream [`steamworks::RemoteStorage`] API.
//! It intentionally avoids the upstream blocking file reader/writer helpers in
//! Bevy systems; games can still access those through [`crate::SteamworksClient`]
//! when they can move file IO out of the frame-critical path.

mod async_results;
mod commands;
mod messages;
mod plugin;
mod state;
#[cfg(test)]
mod tests;
mod types;

pub use messages::*;
pub use state::SteamworksRemoteStorageState;
pub use types::*;

/// Bevy plugin for high-level Steam Remote Storage commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksRemoteStorageCommand`] and [`SteamworksRemoteStorageResult`]
/// messages and runs its command processor in [`bevy_app::First`] after Steam
/// callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksRemoteStoragePlugin;
