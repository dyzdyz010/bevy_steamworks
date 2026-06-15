//! High-level Bevy ECS integration for Steam Networking Sockets.
//!
//! This module builds on top of the upstream
//! [`steamworks::networking_sockets::NetworkingSockets`] API. It keeps listen
//! sockets and connections owned by a private Bevy resource, while exposing
//! stable integer IDs, owned snapshots, and command/result messages to game
//! systems. Most commands can run through either the Steam client or Steam Game
//! Server accessor; batch [`SteamworksNetworkingSocketsCommand::SendMessages`]
//! still requires a client because the upstream safe message allocator is
//! client-only.

/// Maximum number of socket/listen events processed by one poll command.
pub const STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND: usize = 256;

/// Maximum number of messages sent or received by one socket command.
pub const STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND: usize = 1024;

/// Maximum realtime lane statuses requested by one status command.
///
/// The Steamworks API accepts a signed lane count and returns one status per
/// requested lane. This cap prevents a single ECS command from allocating an
/// unbounded status vector.
pub const STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES: u32 = 64;

/// Maximum lanes configured by one lane configuration command.
pub const STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES: usize = 64;

/// Conservative maximum message payload size accepted by this command layer.
///
/// The upstream Steamworks API can reject oversize messages at send time. This
/// cap keeps one ECS command from allocating or attempting to submit unbounded
/// payloads in a frame.
pub const STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES: usize = 1_048_576;

/// Bevy plugin for high-level Steam Networking Sockets commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`] or
/// [`crate::SteamworksServerPlugin`]. It registers
/// [`SteamworksNetworkingSocketsCommand`] and
/// [`SteamworksNetworkingSocketsResult`] messages and processes commands in
/// [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksNetworkingSocketsPlugin;

mod commands;
mod handles;
mod messages;
mod plugin;
mod polling;
mod snapshots;
mod state;
mod types;
mod validation;

#[cfg(test)]
use handles::{
    SteamworksNetworkingSocketsConnectionMetadata, SteamworksNetworkingSocketsHandleStorage,
};
#[cfg(test)]
use validation::validate_command;

pub use messages::*;
pub use state::SteamworksNetworkingSocketsState;
pub use types::*;

#[cfg(test)]
mod tests;
