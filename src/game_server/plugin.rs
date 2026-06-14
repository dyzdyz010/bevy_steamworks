use std::sync::Mutex;

use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{SteamworksEvent, SteamworksFailurePolicy, SteamworksSystem};

use super::{
    callbacks::run_steam_server_callbacks, commands::process_server_commands,
    validation::validate_server_config, SteamworksServer, SteamworksServerCallbackRegistry,
    SteamworksServerCommand, SteamworksServerConfig, SteamworksServerInitMode,
    SteamworksServerPlugin, SteamworksServerResult, SteamworksServerState,
    SteamworksServerUnavailable,
};

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

    /// Returns how this plugin will create or locate the Steam Game Server.
    pub fn init_mode(&self) -> &SteamworksServerInitMode {
        &self.mode
    }

    /// Returns how this plugin reacts when Steam Game Server cannot be initialized.
    pub fn failure_policy_setting(&self) -> SteamworksFailurePolicy {
        self.failure_policy
    }

    /// Returns true when this plugin will automatically run Steam Game Server callbacks.
    pub fn runs_callbacks(&self) -> bool {
        self.run_callbacks
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
        app.init_resource::<SteamworksServerState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksServerCommand>()
            .add_message::<SteamworksServerResult>()
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

        app.configure_sets(
            First,
            SteamworksSystem::ProcessServerCommands
                .after(SteamworksSystem::RunCallbacks)
                .before(bevy_ecs::message::MessageUpdateSystems),
        )
        .add_systems(
            First,
            process_server_commands.in_set(SteamworksSystem::ProcessServerCommands),
        );

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
