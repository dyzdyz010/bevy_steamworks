//! High-level Bevy ECS integration for Steam's legacy P2P Networking API.
//!
//! This module builds on top of the upstream [`steamworks::Networking`] API. It
//! can run through either the Steam client or Steam Game Server networking
//! accessor, and exists for older Steam P2P workflows; new projects should
//! prefer [`crate::SteamworksNetworkingMessagesPlugin`].

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
pub use state::SteamworksNetworkingState;
pub use types::*;

/// Maximum unreliable legacy P2P packet size accepted by Steam.
pub const STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES: usize = 1_200;

/// Maximum reliable legacy P2P packet size accepted by Steam.
pub const STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES: usize = 1_048_576;

/// Maximum receive buffer this command layer will allocate in one frame.
pub const STEAMWORKS_P2P_MAX_READ_PACKET_BYTES: usize = STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES;

/// Bevy plugin for high-level legacy Steam P2P Networking commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`] or
/// [`crate::SteamworksServerPlugin`]. It registers
/// [`SteamworksNetworkingCommand`] and [`SteamworksNetworkingResult`] messages,
/// observes legacy P2P callbacks from [`crate::SteamworksEvent`], and processes
/// commands in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksNetworkingPlugin;
