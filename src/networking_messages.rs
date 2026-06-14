//! High-level Bevy ECS integration for Steam Networking Messages.
//!
//! This module builds on top of the upstream
//! [`steamworks::networking_messages::NetworkingMessages`] API. It exposes the
//! UDP-like Steam P2P message interface through Bevy commands/results while
//! copying received payloads into owned `Vec<u8>` values that are safe to keep
//! in ECS state.

mod callbacks;
mod commands;
mod messages;
mod plugin;
mod snapshots;
mod state;
#[cfg(test)]
mod tests;
mod types;
mod validation;

pub use messages::*;
pub use state::SteamworksNetworkingMessagesState;
pub use types::*;

/// Maximum number of messages one receive command will pull in a single frame.
///
/// Steam's upstream wrapper allocates a temporary pointer buffer with the
/// requested batch size before calling the C API. Keeping this bounded prevents
/// one malformed command from forcing a huge frame-loop allocation.
pub const STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE: usize = 1024;

/// Bevy plugin for high-level Steam Networking Messages commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksNetworkingMessagesCommand`] and
/// [`SteamworksNetworkingMessagesResult`] messages, installs the upstream
/// session callbacks once a [`crate::SteamworksClient`] exists, and processes commands
/// in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug)]
pub struct SteamworksNetworkingMessagesPlugin {
    auto_accept_session_requests: bool,
}
