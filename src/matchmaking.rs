//! High-level Bevy ECS integration for Steam matchmaking and lobbies.
//!
//! This module builds on top of the upstream [`steamworks::Matchmaking`] API.
//! It keeps async Steam call results and lobby callbacks flowing through Bevy
//! messages, while avoiding blocking work in the frame loop.

mod async_results;
mod callbacks;
mod commands;
mod filters;
mod messages;
mod plugin;
mod state;
#[cfg(test)]
mod tests;
mod types;
mod validation;

pub use messages::*;
pub use state::SteamworksMatchmakingState;
pub use types::*;

const MAX_LOBBY_MEMBERS: u32 = 250;
const MAX_LOBBY_CHAT_MESSAGE_BYTES: usize = 4096;
const MAX_LOBBY_LIST_RESULTS: u64 = i32::MAX as u64;

/// Bevy plugin for high-level Steam matchmaking and lobby commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksMatchmakingCommand`] and [`SteamworksMatchmakingResult`]
/// messages and runs its command processor in [`bevy_app::First`] after Steam
/// callbacks. It also mirrors lobby callbacks into matchmaking results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksMatchmakingPlugin;
