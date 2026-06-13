//! Bevy ECS integration for Steam Game Server initialization and callbacks.
//!
//! This module builds on top of the upstream [`steamworks::Server`] API. It
//! inserts a Bevy resource for the initialized game server and pumps Steam Game
//! Server callbacks into the shared [`crate::SteamworksEvent`] stream.

use std::{net::Ipv4Addr, ops::Deref, sync::Mutex};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::MessageWriter,
    prelude::{Res, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksEvent, SteamworksFailurePolicy, SteamworksSystem};

/// Configuration used to initialize [`steamworks::Server`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksServerConfig {
    /// IPv4 address the Steam Game Server API should bind to.
    pub ip: Ipv4Addr,
    /// Game traffic port.
    pub game_port: u16,
    /// Server browser query port.
    ///
    /// Use [`steamworks::QUERY_PORT_SHARED`] when game and query traffic share
    /// the same socket.
    pub query_port: u16,
    /// Upstream Steam server mode.
    pub server_mode: steamworks::ServerMode,
    /// Version string reported to Steam.
    pub version: String,
}

impl SteamworksServerConfig {
    /// Creates a Steam Game Server initialization config.
    pub fn new(
        ip: Ipv4Addr,
        game_port: u16,
        query_port: u16,
        server_mode: steamworks::ServerMode,
        version: impl Into<String>,
    ) -> Self {
        Self {
            ip,
            game_port,
            query_port,
            server_mode,
            version: version.into(),
        }
    }
}

/// How [`SteamworksServerPlugin`] should create or locate the Steamworks server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksServerInitMode {
    /// Initialize a Steam Game Server from the supplied config.
    Config(SteamworksServerConfig),
    /// Do not initialize Steamworks.
    ///
    /// This is useful when another layer inserts [`SteamworksServer`] manually,
    /// or for tests that only need plugin schedules and messages.
    Manual,
}

/// Resource inserted when Steam Game Server initialization is explicitly allowed to fail.
#[derive(Clone, Debug, Error, PartialEq, Eq, Resource)]
pub enum SteamworksServerUnavailable {
    /// Manual mode was selected, but no [`SteamworksServer`] resource was present.
    #[error(
        "manual Steam Game Server initialization was selected, but no SteamworksServer resource exists"
    )]
    ManualServerMissing,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steam Game Server config field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// The upstream Steam Game Server initialization call returned an error.
    #[error("Steam Game Server initialization failed with {config:?}: {source}")]
    InitFailed {
        /// Initialization config used for the failed attempt.
        config: SteamworksServerConfig,
        /// Error returned by `steamworks`.
        source: steamworks::SteamAPIInitError,
    },
}

impl SteamworksServerUnavailable {
    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    fn init_failed(config: SteamworksServerConfig, source: steamworks::SteamAPIInitError) -> Self {
        Self::InitFailed { config, source }
    }
}

/// A Bevy resource wrapping [`steamworks::Server`].
#[derive(Clone, Resource)]
pub struct SteamworksServer(steamworks::Server);

impl SteamworksServer {
    /// Creates a Bevy resource from an initialized Steam Game Server.
    pub fn new(server: steamworks::Server) -> Self {
        Self(server)
    }

    /// Returns the underlying Steam Game Server handle.
    pub fn inner(&self) -> &steamworks::Server {
        &self.0
    }

    /// Clones the underlying Steam Game Server handle.
    pub fn clone_inner(&self) -> steamworks::Server {
        self.0.clone()
    }
}

impl From<steamworks::Server> for SteamworksServer {
    fn from(server: steamworks::Server) -> Self {
        Self::new(server)
    }
}

impl Deref for SteamworksServer {
    type Target = steamworks::Server;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

/// Stores Steam Game Server callback handles so callbacks stay registered.
#[derive(Default, Resource)]
pub struct SteamworksServerCallbackRegistry {
    handles: Vec<steamworks::CallbackHandle>,
}

impl SteamworksServerCallbackRegistry {
    /// Registers a typed Steam Game Server callback and stores its handle.
    pub fn register<C, F>(&mut self, server: &SteamworksServer, callback: F)
    where
        C: steamworks::Callback,
        F: FnMut(C) + 'static + Send,
    {
        self.handles.push(server.register_callback(callback));
    }

    /// Drops every registered callback handle.
    pub fn clear(&mut self) {
        self.handles.clear();
    }

    /// Number of callback handles currently held.
    pub fn len(&self) -> usize {
        self.handles.len()
    }

    /// Returns true when no callback handles are held.
    pub fn is_empty(&self) -> bool {
        self.handles.is_empty()
    }
}

/// A Bevy plugin that integrates Steam Game Server callbacks into an app.
pub struct SteamworksServerPlugin {
    mode: SteamworksServerInitMode,
    failure_policy: SteamworksFailurePolicy,
    run_callbacks: bool,
    server: Mutex<Option<steamworks::Server>>,
}

impl SteamworksServerPlugin {
    /// Creates a plugin that initializes Steam Game Server from a config.
    pub fn new(config: SteamworksServerConfig) -> Self {
        Self::with_mode(SteamworksServerInitMode::Config(config))
    }

    /// Creates a plugin that does not initialize Steam Game Server.
    ///
    /// Use this when you insert [`SteamworksServer`] yourself, or when tests only
    /// need the plugin's schedule and message setup.
    pub fn manual() -> Self {
        Self::with_mode(SteamworksServerInitMode::Manual)
    }

    /// Initializes Steam Game Server immediately and wraps it.
    pub fn init(config: SteamworksServerConfig) -> Result<Self, SteamworksServerUnavailable> {
        validate_server_config(&config)?;
        let (server, _server_client) = steamworks::Server::init(
            config.ip,
            config.game_port,
            config.query_port,
            config.server_mode,
            &config.version,
        )
        .map_err(|source| SteamworksServerUnavailable::init_failed(config, source))?;
        Ok(Self::from_server(server))
    }

    /// Creates a plugin from an already initialized Steam Game Server.
    pub fn from_server(server: steamworks::Server) -> Self {
        Self {
            mode: SteamworksServerInitMode::Manual,
            failure_policy: SteamworksFailurePolicy::Panic,
            run_callbacks: true,
            server: Mutex::new(Some(server)),
        }
    }

    /// Sets the initialization failure policy.
    pub fn failure_policy(mut self, policy: SteamworksFailurePolicy) -> Self {
        self.failure_policy = policy;
        self
    }

    /// Keeps the Bevy app running when Steam Game Server cannot be initialized.
    ///
    /// The plugin will insert [`SteamworksServerUnavailable`] and emit a
    /// structured `tracing` error.
    pub fn log_and_continue(self) -> Self {
        self.failure_policy(SteamworksFailurePolicy::LogAndContinue)
    }

    /// Sets whether the plugin should automatically run Steam Game Server callbacks.
    pub fn run_callbacks(mut self, run_callbacks: bool) -> Self {
        self.run_callbacks = run_callbacks;
        self
    }

    fn with_mode(mode: SteamworksServerInitMode) -> Self {
        Self {
            mode,
            failure_policy: SteamworksFailurePolicy::Panic,
            run_callbacks: true,
            server: Mutex::new(None),
        }
    }

    fn initialize_server(&self) -> Result<steamworks::Server, SteamworksServerUnavailable> {
        let injected = self
            .server
            .lock()
            .expect("SteamworksServerPlugin server mutex was poisoned")
            .take();

        if let Some(server) = injected {
            return Ok(server);
        }

        match &self.mode {
            SteamworksServerInitMode::Config(config) => {
                validate_server_config(config)?;
                let (server, _server_client) = steamworks::Server::init(
                    config.ip,
                    config.game_port,
                    config.query_port,
                    config.server_mode,
                    &config.version,
                )
                .map_err(|source| {
                    SteamworksServerUnavailable::init_failed(config.clone(), source)
                })?;
                Ok(server)
            }
            SteamworksServerInitMode::Manual => {
                Err(SteamworksServerUnavailable::ManualServerMissing)
            }
        }
    }

    fn handle_unavailable(&self, app: &mut App, error: SteamworksServerUnavailable) {
        match self.failure_policy {
            SteamworksFailurePolicy::Panic => panic!("{error}"),
            SteamworksFailurePolicy::LogAndContinue => {
                tracing::error!(
                    target: "bevy_steamworks",
                    init_mode = ?self.mode,
                    error = %error,
                    "Steam Game Server unavailable"
                );
                app.insert_resource(error);
            }
        }
    }
}

impl From<steamworks::Server> for SteamworksServerPlugin {
    fn from(server: steamworks::Server) -> Self {
        Self::from_server(server)
    }
}

impl Plugin for SteamworksServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SteamworksEvent>()
            .init_resource::<SteamworksServerCallbackRegistry>();

        if self.run_callbacks {
            app.configure_sets(First, SteamworksSystem::RunCallbacks)
                .add_systems(
                    First,
                    run_steam_server_callbacks
                        .in_set(SteamworksSystem::RunCallbacks)
                        .before(bevy_ecs::message::MessageUpdateSystems),
                );
        }

        if app.world().contains_resource::<SteamworksServer>() {
            tracing::debug!(
                target: "bevy_steamworks",
                init_mode = ?self.mode,
                "using existing SteamworksServer resource"
            );
            return;
        }

        match self.initialize_server() {
            Ok(server) => {
                tracing::info!(
                    target: "bevy_steamworks",
                    init_mode = ?self.mode,
                    "Steam Game Server initialized"
                );
                app.insert_resource(SteamworksServer::new(server));
            }
            Err(error) => self.handle_unavailable(app, error),
        }
    }
}

fn run_steam_server_callbacks(
    server: Option<Res<SteamworksServer>>,
    mut output: MessageWriter<SteamworksEvent>,
) {
    let Some(server) = server else {
        return;
    };

    server.process_callbacks(|callback| {
        output.write(SteamworksEvent::from(callback));
    });
}

fn validate_server_config(
    config: &SteamworksServerConfig,
) -> Result<(), SteamworksServerUnavailable> {
    if config.version.as_bytes().contains(&0) {
        Err(SteamworksServerUnavailable::invalid_string("version"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    #[test]
    fn manual_mode_can_continue_without_server() {
        let mut app = App::new();

        app.add_plugins(SteamworksServerPlugin::manual().log_and_continue());

        assert!(app
            .world()
            .contains_resource::<SteamworksServerUnavailable>());
        assert!(app
            .world()
            .contains_resource::<SteamworksServerCallbackRegistry>());
        assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
        assert!(!app.world().contains_resource::<SteamworksServer>());

        app.update();
    }

    #[test]
    #[should_panic(expected = "manual Steam Game Server initialization was selected")]
    fn manual_mode_panics_by_default() {
        let mut app = App::new();

        app.add_plugins(SteamworksServerPlugin::manual());
    }

    #[test]
    fn invalid_version_can_continue_without_server() {
        let mut app = App::new();

        app.add_plugins(
            SteamworksServerPlugin::new(SteamworksServerConfig::new(
                Ipv4Addr::LOCALHOST,
                27015,
                27016,
                steamworks::ServerMode::Authentication,
                "bad\0version",
            ))
            .log_and_continue(),
        );

        assert_eq!(
            app.world().resource::<SteamworksServerUnavailable>(),
            &SteamworksServerUnavailable::InvalidString { field: "version" }
        );
        assert!(!app.world().contains_resource::<SteamworksServer>());
    }

    #[test]
    fn server_callback_registry_tracks_handles() {
        let registry = SteamworksServerCallbackRegistry::default();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }
}
