//! High-level Bevy ECS integration for Steam Remote Storage.
//!
//! This module builds on top of the upstream [`steamworks::RemoteStorage`] API.
//! Payload reads and writes are submitted from Bevy systems and completed on
//! background workers so upstream file IO does not block the frame loop.

mod async_results;
mod commands;
mod file_io;
mod messages;
mod plugin;
mod snapshots;
mod state;
#[cfg(test)]
mod tests;
mod types;
mod validation;

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
