use std::sync::Mutex;

use bevy_app::{App, First, Plugin};
use bevy_ecs::{message::MessageWriter, schedule::IntoScheduleConfigs, system::Res};
use steamworks::{AppId, SteamAPIInitError};

use crate::{
    SteamworksCallbackRegistry, SteamworksClient, SteamworksEvent, SteamworksFailurePolicy,
    SteamworksInitMode, SteamworksPlugin, SteamworksSystem, SteamworksUnavailable,
};

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

    /// Returns how this plugin will create or locate the Steamworks client.
    pub fn init_mode(&self) -> SteamworksInitMode {
        self.mode
    }

    /// Returns how this plugin reacts when Steamworks cannot be initialized.
    pub fn failure_policy_setting(&self) -> SteamworksFailurePolicy {
        self.failure_policy
    }

    /// Returns true when this plugin will automatically run Steam callbacks.
    pub fn runs_callbacks(&self) -> bool {
        self.run_callbacks
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
    use bevy_app::{App, Plugin};

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
    fn plugin_name_matches_root_type_path_for_bevy_tracking() {
        let plugin = SteamworksPlugin::manual().log_and_continue();

        assert_eq!(plugin.name(), std::any::type_name::<SteamworksPlugin>());
        assert_eq!(plugin.name(), "bevy_steamworks::SteamworksPlugin");

        let mut app = App::new();
        app.add_plugins(plugin);

        assert!(app.is_plugin_added::<SteamworksPlugin>());
    }

    #[test]
    fn configuration_accessors_expose_builder_settings() {
        let plugin = SteamworksPlugin::app_id(AppId(480))
            .failure_policy(SteamworksFailurePolicy::LogAndContinue)
            .run_callbacks(false);

        assert_eq!(plugin.init_mode(), SteamworksInitMode::AppId(AppId(480)));
        assert_eq!(
            plugin.failure_policy_setting(),
            SteamworksFailurePolicy::LogAndContinue
        );
        assert!(!plugin.runs_callbacks());
    }

    #[test]
    #[should_panic(expected = "manual Steamworks initialization was selected")]
    fn manual_mode_panics_by_default() {
        let mut app = App::new();

        app.add_plugins(SteamworksPlugin::manual());
    }
}
