#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! Bevy integration for the Steamworks SDK via [`steamworks`].
//!
//! The plugin initializes a Steamworks client, stores it as a Bevy resource,
//! and pumps Steam callbacks every frame.

use bevy_ecs::schedule::SystemSet;

/// Re-export of the upstream Steamworks Rust bindings.
pub use steamworks;

/// Re-export of the upstream Steamworks API surface for ergonomic `use bevy_steamworks::*`.
pub use steamworks::*;
pub use steamworks::{
    networking_messages::{NetworkingMessagesSessionFailed, NetworkingMessagesSessionRequest},
    networking_types::NetConnectionStatusChanged,
    networking_utils::RelayNetworkStatusCallback,
    screenshots::{ScreenshotReady, ScreenshotRequested},
};

pub mod apps;
mod availability;
mod cache;
mod client;
mod events;
pub mod friends;
pub mod game_server;
pub mod input;
pub mod matchmaking;
pub mod matchmaking_servers;
pub mod networking;
pub mod networking_messages;
pub mod networking_sockets;
pub mod networking_utils;
mod plugin;
mod plugin_groups;
mod registry;
pub mod remote_play;
pub mod remote_storage;
mod result_ext;
pub mod screenshots;
pub mod timeline;
pub mod ugc;
pub mod user;
pub mod user_stats;
pub mod utils;
pub use apps::*;
pub use availability::{SteamworksFailurePolicy, SteamworksInitMode, SteamworksUnavailable};
pub use client::SteamworksClient;
pub use events::SteamworksEvent;
pub use friends::*;
pub use game_server::*;
pub use input::*;
pub use matchmaking::*;
pub use matchmaking_servers::*;
pub use networking::*;
pub use networking_messages::*;
pub use networking_sockets::*;
pub use networking_utils::*;
pub use plugin_groups::{
    SteamworksClientPlugins, SteamworksPlugins, SteamworksServerFeaturePlugins,
    SteamworksServerPlugins,
};
pub use registry::SteamworksCallbackRegistry;
pub use remote_play::*;
pub use remote_storage::*;
pub use result_ext::SteamworksCommandError;
pub use screenshots::*;
pub use timeline::*;
pub use ugc::*;
pub use user::*;
pub use user_stats::*;
pub use utils::*;

/// Common imports for Bevy apps using this crate.
pub mod prelude;

/// A Bevy plugin that integrates Steamworks into an app.
pub struct SteamworksPlugin {
    mode: SteamworksInitMode,
    failure_policy: SteamworksFailurePolicy,
    run_callbacks: bool,
    client: std::sync::Mutex<Option<steamworks::Client>>,
}

/// System sets used by [`SteamworksPlugin`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum SteamworksSystem {
    /// Runs Steamworks callbacks and writes [`SteamworksEvent`] messages.
    RunCallbacks,
    /// Processes high-level Steam user stats and achievements commands.
    ProcessStatsCommands,
    /// Processes high-level Steam friends, Rich Presence, overlay, and invite commands.
    ProcessFriendsCommands,
    /// Processes high-level Steam matchmaking and lobby commands.
    ProcessMatchmakingCommands,
    /// Processes high-level Steam Matchmaking Servers commands.
    ProcessMatchmakingServersCommands,
    /// Processes high-level Steam Game Server commands.
    ProcessServerCommands,
    /// Processes high-level Steam app and launch-parameter commands.
    ProcessAppsCommands,
    /// Processes high-level Steam user identity and authentication commands.
    ProcessUserCommands,
    /// Processes high-level Steam utility commands.
    ProcessUtilsCommands,
    /// Processes high-level Steam screenshots commands.
    ProcessScreenshotsCommands,
    /// Processes high-level Steam Remote Play commands.
    ProcessRemotePlayCommands,
    /// Processes high-level Steam Remote Storage commands.
    ProcessRemoteStorageCommands,
    /// Processes high-level Steam Timeline commands.
    ProcessTimelineCommands,
    /// Processes high-level Steam Input commands.
    ProcessInputCommands,
    /// Processes high-level legacy Steam P2P Networking commands.
    ProcessNetworkingCommands,
    /// Processes high-level Steam Networking Messages commands.
    ProcessNetworkingMessagesCommands,
    /// Processes high-level Steam Networking Sockets commands.
    ProcessNetworkingSocketsCommands,
    /// Processes high-level Steam Networking Utils commands.
    ProcessNetworkingUtilsCommands,
    /// Processes high-level Steam Workshop / UGC commands.
    ProcessUgcCommands,
}
