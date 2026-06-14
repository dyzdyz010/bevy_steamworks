#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! Bevy integration for the Steamworks SDK via [`steamworks`].
//!
//! The plugin initializes a Steamworks client, stores it as a Bevy resource,
//! and pumps Steam callbacks every frame.

use std::sync::Mutex;

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::MessageWriter,
    schedule::{IntoScheduleConfigs, SystemSet},
    system::Res,
};

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
mod plugin_groups;
mod registry;
pub mod remote_play;
pub mod remote_storage;
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
pub use plugin_groups::{SteamworksClientPlugins, SteamworksPlugins};
pub use registry::SteamworksCallbackRegistry;
pub use remote_play::*;
pub use remote_storage::*;
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
    client: Mutex<Option<steamworks::Client>>,
}

impl Default for SteamworksPlugin {
    fn default() -> Self {
        Self::automatic()
    }
}

impl SteamworksPlugin {
    /// Creates a plugin that initializes Steamworks from the environment.
    ///
    /// This uses [`steamworks::Client::init`].
    pub fn automatic() -> Self {
        Self::with_mode(SteamworksInitMode::Automatic)
    }

    /// Creates a plugin that initializes Steamworks with a specific app id.
    ///
    /// This uses [`steamworks::Client::init_app`] when the plugin is built.
    pub fn app_id(app_id: impl Into<AppId>) -> Self {
        Self::with_mode(SteamworksInitMode::AppId(app_id.into()))
    }

    /// Creates a plugin that does not initialize Steamworks.
    ///
    /// Use this when you insert [`SteamworksClient`] yourself, or when tests only
    /// need the plugin's schedule and message setup.
    pub fn manual() -> Self {
        Self::with_mode(SteamworksInitMode::Manual)
    }

    /// Initializes Steamworks immediately from the environment and wraps it.
    pub fn init() -> Result<Self, SteamAPIInitError> {
        steamworks::Client::init().map(Self::from_client)
    }

    /// Initializes Steamworks immediately with a specific app id and wraps it.
    pub fn init_app(app_id: impl Into<AppId>) -> Result<Self, SteamAPIInitError> {
        steamworks::Client::init_app(app_id.into()).map(Self::from_client)
    }

    /// Creates a plugin from an already initialized Steamworks client.
    pub fn from_client(client: steamworks::Client) -> Self {
        Self {
            mode: SteamworksInitMode::Manual,
            failure_policy: SteamworksFailurePolicy::Panic,
            run_callbacks: true,
            client: Mutex::new(Some(client)),
        }
    }

    /// Sets the initialization failure policy.
    pub fn failure_policy(mut self, policy: SteamworksFailurePolicy) -> Self {
        self.failure_policy = policy;
        self
    }

    /// Keeps the Bevy app running when Steamworks cannot be initialized.
    ///
    /// The plugin will insert [`SteamworksUnavailable`] and emit a structured
    /// `tracing` error.
    pub fn log_and_continue(self) -> Self {
        self.failure_policy(SteamworksFailurePolicy::LogAndContinue)
    }

    /// Sets whether the plugin should automatically run Steam callbacks.
    pub fn run_callbacks(mut self, run_callbacks: bool) -> Self {
        self.run_callbacks = run_callbacks;
        self
    }

    fn with_mode(mode: SteamworksInitMode) -> Self {
        Self {
            mode,
            failure_policy: SteamworksFailurePolicy::Panic,
            run_callbacks: true,
            client: Mutex::new(None),
        }
    }

    fn initialize_client(&self) -> Result<steamworks::Client, SteamworksUnavailable> {
        let injected = self
            .client
            .lock()
            .expect("SteamworksPlugin client mutex was poisoned")
            .take();

        if let Some(client) = injected {
            return Ok(client);
        }

        match self.mode {
            SteamworksInitMode::Automatic => steamworks::Client::init().map_err(|source| {
                SteamworksUnavailable::init_failed(SteamworksInitMode::Automatic, source)
            }),
            SteamworksInitMode::AppId(app_id) => {
                steamworks::Client::init_app(app_id).map_err(|source| {
                    SteamworksUnavailable::init_failed(SteamworksInitMode::AppId(app_id), source)
                })
            }
            SteamworksInitMode::Manual => Err(SteamworksUnavailable::ManualClientMissing),
        }
    }

    fn handle_unavailable(&self, app: &mut App, error: SteamworksUnavailable) {
        match self.failure_policy {
            SteamworksFailurePolicy::Panic => panic!("{error}"),
            SteamworksFailurePolicy::LogAndContinue => {
                tracing::error!(
                    target: "bevy_steamworks",
                    init_mode = ?self.mode,
                    app_id = ?self.mode.raw_app_id(),
                    error = %error,
                    "Steamworks unavailable"
                );
                app.insert_resource(error);
            }
        }
    }
}

impl From<steamworks::Client> for SteamworksPlugin {
    fn from(client: steamworks::Client) -> Self {
        Self::from_client(client)
    }
}

impl Plugin for SteamworksPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SteamworksEvent>()
            .init_resource::<SteamworksCallbackRegistry>();

        if self.run_callbacks {
            app.configure_sets(First, SteamworksSystem::RunCallbacks)
                .add_systems(
                    First,
                    run_steam_callbacks
                        .in_set(SteamworksSystem::RunCallbacks)
                        .before(bevy_ecs::message::MessageUpdateSystems),
                );
        }

        if app.world().contains_resource::<SteamworksClient>() {
            tracing::debug!(
                target: "bevy_steamworks",
                init_mode = ?self.mode,
                "using existing SteamworksClient resource"
            );
            return;
        }

        match self.initialize_client() {
            Ok(client) => {
                tracing::info!(
                    target: "bevy_steamworks",
                    init_mode = ?self.mode,
                    app_id = ?self.mode.raw_app_id(),
                    "Steamworks initialized"
                );
                app.insert_resource(SteamworksClient::new(client));
            }
            Err(error) => self.handle_unavailable(app, error),
        }
    }
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

fn run_steam_callbacks(
    client: Option<Res<SteamworksClient>>,
    mut output: MessageWriter<SteamworksEvent>,
) {
    let Some(client) = client else {
        return;
    };

    client.process_callbacks(|callback| {
        output.write(SteamworksEvent::from(callback));
    });
}

#[cfg(test)]
mod tests {
    use bevy_app::App;

    use super::*;

    #[test]
    fn manual_mode_can_continue_without_client() {
        let mut app = App::new();

        app.add_plugins(SteamworksPlugin::manual().log_and_continue());

        assert!(app.world().contains_resource::<SteamworksUnavailable>());
        let unavailable = app.world().resource::<SteamworksUnavailable>();
        assert!(unavailable.is_manual_client_missing());
        assert!(!unavailable.is_init_failed());
        assert_eq!(unavailable.init_mode(), None);
        assert_eq!(unavailable.app_id(), None);
        assert_eq!(unavailable.raw_app_id(), None);
        assert_eq!(unavailable.init_error(), None);
        assert!(app
            .world()
            .contains_resource::<SteamworksCallbackRegistry>());
        assert!(!app.world().contains_resource::<SteamworksClient>());

        app.update();
    }

    #[test]
    #[should_panic(expected = "manual Steamworks initialization was selected")]
    fn manual_mode_panics_by_default() {
        let mut app = App::new();

        app.add_plugins(SteamworksPlugin::manual());
    }
}
