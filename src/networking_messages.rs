//! High-level Bevy ECS integration for Steam Networking Messages.
//!
//! This module builds on top of the upstream
//! [`steamworks::networking_messages::NetworkingMessages`] API. It exposes the
//! UDP-like Steam P2P message interface through Bevy commands/results while
//! copying received payloads into owned `Vec<u8>` values that are safe to keep
//! in ECS state.

use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::SteamworksSystem;

mod callbacks;
mod commands;
mod messages;
mod snapshots;
mod state;
#[cfg(test)]
mod tests;
mod types;
mod validation;

use callbacks::{apply_networking_messages_policy_commands, ensure_networking_messages_callbacks};
use commands::process_networking_messages_commands;

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

impl Default for SteamworksNetworkingMessagesPlugin {
    fn default() -> Self {
        Self {
            auto_accept_session_requests: true,
        }
    }
}

impl SteamworksNetworkingMessagesPlugin {
    /// Creates a Networking Messages plugin with default behavior.
    ///
    /// Incoming session requests are accepted by default. Use
    /// [`Self::auto_accept_session_requests`] to opt out.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether incoming session requests are accepted in the Steam callback.
    ///
    /// The upstream safe API only allows accepting a session while handling the
    /// callback; it cannot defer the accept/reject decision to a later ECS frame.
    pub fn auto_accept_session_requests(mut self, enabled: bool) -> Self {
        self.auto_accept_session_requests = enabled;
        self
    }
}

impl Plugin for SteamworksNetworkingMessagesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SteamworksNetworkingMessagesState::new(
            self.auto_accept_session_requests,
        ))
        .add_message::<SteamworksNetworkingMessagesCommand>()
        .add_message::<SteamworksNetworkingMessagesResult>()
        .configure_sets(
            First,
            SteamworksSystem::ProcessNetworkingMessagesCommands
                .after(SteamworksSystem::RunCallbacks)
                .before(bevy_ecs::message::MessageUpdateSystems),
        )
        .add_systems(
            First,
            (
                ensure_networking_messages_callbacks,
                apply_networking_messages_policy_commands,
            )
                .chain()
                .before(SteamworksSystem::RunCallbacks),
        )
        .add_systems(
            First,
            process_networking_messages_commands
                .in_set(SteamworksSystem::ProcessNetworkingMessagesCommands),
        );
    }
}
