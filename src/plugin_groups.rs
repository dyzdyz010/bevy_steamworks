//! Bevy plugin groups for ergonomic Steamworks setup.

use bevy_app::{PluginGroup, PluginGroupBuilder};

use crate::{
    steamworks, AppId, SteamAPIInitError, SteamworksAppsPlugin, SteamworksFailurePolicy,
    SteamworksFriendsPlugin, SteamworksInputPlugin, SteamworksMatchmakingPlugin,
    SteamworksMatchmakingServersPlugin, SteamworksNetworkingMessagesPlugin,
    SteamworksNetworkingPlugin, SteamworksNetworkingSocketsPlugin, SteamworksNetworkingUtilsPlugin,
    SteamworksPlugin, SteamworksRemotePlayPlugin, SteamworksRemoteStoragePlugin,
    SteamworksScreenshotsPlugin, SteamworksStatsPlugin, SteamworksTimelinePlugin,
    SteamworksUgcPlugin, SteamworksUserPlugin, SteamworksUtilsPlugin,
};

#[cfg(test)]
mod tests;

/// Installs every default client-side high-level Steamworks feature plugin.
///
/// This is a convenience plugin group for games that want the Bevy
/// message/resource wrappers for apps, friends, input, lobbies, networking,
/// screenshots, Steam Cloud, stats, timeline, UGC, user, and utility APIs.
///
/// It does not initialize Steamworks and it does not install the dedicated game
/// server plugin. Add [`SteamworksPlugin`] separately for the client lifecycle,
/// and add [`crate::SteamworksServerPlugin`] separately for dedicated server
/// builds.
#[derive(Clone, Copy, Debug, Default)]
pub struct SteamworksClientPlugins;

impl SteamworksClientPlugins {
    /// Creates the default client-side feature plugin collection.
    pub fn new() -> Self {
        Self
    }
}

impl PluginGroup for SteamworksClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SteamworksAppsPlugin::new())
            .add(SteamworksFriendsPlugin::new())
            .add(SteamworksInputPlugin::new())
            .add(SteamworksMatchmakingPlugin::new())
            .add(SteamworksMatchmakingServersPlugin::new())
            .add(SteamworksNetworkingPlugin::new())
            .add(SteamworksNetworkingMessagesPlugin::new())
            .add(SteamworksNetworkingSocketsPlugin::new())
            .add(SteamworksNetworkingUtilsPlugin::new())
            .add(SteamworksRemotePlayPlugin::new())
            .add(SteamworksRemoteStoragePlugin::new())
            .add(SteamworksScreenshotsPlugin::new())
            .add(SteamworksStatsPlugin::new())
            .add(SteamworksTimelinePlugin::new())
            .add(SteamworksUgcPlugin::new())
            .add(SteamworksUserPlugin::new())
            .add(SteamworksUtilsPlugin::new())
    }
}

/// A Bevy plugin group that installs Steamworks client initialization and every
/// default client-side high-level feature plugin.
///
/// This is the shortest path for games that want a full Bevy-native Steamworks
/// client integration:
///
/// ```rust,no_run
/// # use bevy_app::prelude::*;
/// # use bevy_steamworks::prelude::*;
/// App::new().add_plugins(SteamworksPlugins::app_id(480));
/// ```
///
/// Use Bevy's [`PluginGroup`] customization methods such as
/// [`set`](PluginGroup::set) and [`PluginGroupBuilder::disable`] to replace or
/// disable individual feature plugins. Use [`SteamworksPlugin`] plus selected
/// feature plugins directly when you only want the raw
/// [`crate::SteamworksClient`] resource.
pub struct SteamworksPlugins {
    core: SteamworksPlugin,
    client_plugins: SteamworksClientPlugins,
}

impl Default for SteamworksPlugins {
    fn default() -> Self {
        Self::automatic()
    }
}

impl SteamworksPlugins {
    /// Creates a plugin group that initializes Steamworks from the environment.
    ///
    /// This uses [`steamworks::Client::init`] through [`SteamworksPlugin`].
    pub fn automatic() -> Self {
        Self::from_plugin(SteamworksPlugin::automatic())
    }

    /// Creates a plugin group that initializes Steamworks with a specific app id.
    ///
    /// This uses [`steamworks::Client::init_app`] through [`SteamworksPlugin`].
    pub fn app_id(app_id: impl Into<AppId>) -> Self {
        Self::from_plugin(SteamworksPlugin::app_id(app_id))
    }

    /// Creates a plugin group that does not initialize Steamworks.
    ///
    /// Use this when another layer inserts [`crate::SteamworksClient`] before
    /// the app runs, or for tests that only need message/resource setup.
    pub fn manual() -> Self {
        Self::from_plugin(SteamworksPlugin::manual())
    }

    /// Initializes Steamworks immediately from the environment and wraps it in
    /// the full default client feature plugin group.
    pub fn init() -> Result<Self, SteamAPIInitError> {
        SteamworksPlugin::init().map(Self::from_plugin)
    }

    /// Initializes Steamworks immediately with a specific app id and wraps it in
    /// the full default client feature plugin group.
    pub fn init_app(app_id: impl Into<AppId>) -> Result<Self, SteamAPIInitError> {
        SteamworksPlugin::init_app(app_id).map(Self::from_plugin)
    }

    /// Creates a plugin group from an already initialized Steamworks client.
    pub fn from_client(client: steamworks::Client) -> Self {
        Self::from_plugin(SteamworksPlugin::from_client(client))
    }

    /// Creates a plugin group from an already configured [`SteamworksPlugin`].
    pub fn from_plugin(plugin: SteamworksPlugin) -> Self {
        Self {
            core: plugin,
            client_plugins: SteamworksClientPlugins::new(),
        }
    }

    /// Replaces the client-side feature plugin collection.
    pub fn client_plugins(mut self, client_plugins: SteamworksClientPlugins) -> Self {
        self.client_plugins = client_plugins;
        self
    }

    /// Sets the initialization failure policy.
    pub fn failure_policy(mut self, policy: SteamworksFailurePolicy) -> Self {
        self.core = self.core.failure_policy(policy);
        self
    }

    /// Keeps the Bevy app running when Steamworks cannot be initialized.
    ///
    /// The underlying [`SteamworksPlugin`] will insert
    /// [`crate::SteamworksUnavailable`] and emit a structured `tracing` error.
    pub fn log_and_continue(self) -> Self {
        self.failure_policy(SteamworksFailurePolicy::LogAndContinue)
    }

    /// Sets whether the plugin group should automatically run Steam callbacks.
    pub fn run_callbacks(mut self, run_callbacks: bool) -> Self {
        self.core = self.core.run_callbacks(run_callbacks);
        self
    }

    /// Returns the configured core Steamworks lifecycle plugin.
    pub fn core_plugin(&self) -> &SteamworksPlugin {
        &self.core
    }

    /// Returns how the core plugin will create or locate the Steamworks client.
    pub fn init_mode(&self) -> crate::SteamworksInitMode {
        self.core.init_mode()
    }

    /// Returns how the core plugin reacts when Steamworks cannot be initialized.
    pub fn failure_policy_setting(&self) -> SteamworksFailurePolicy {
        self.core.failure_policy_setting()
    }

    /// Returns true when the core plugin will automatically run Steam callbacks.
    pub fn runs_callbacks(&self) -> bool {
        self.core.runs_callbacks()
    }
}

impl From<SteamworksPlugin> for SteamworksPlugins {
    fn from(plugin: SteamworksPlugin) -> Self {
        Self::from_plugin(plugin)
    }
}

impl From<steamworks::Client> for SteamworksPlugins {
    fn from(client: steamworks::Client) -> Self {
        Self::from_client(client)
    }
}

impl PluginGroup for SteamworksPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(self.core)
            .add_group(self.client_plugins)
    }
}
