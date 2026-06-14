use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{SteamworksEvent, SteamworksSystem};

use super::{
    async_results::SteamworksStatsAsyncResults, commands::process_stats_commands,
    leaderboards::SteamworksStatsLeaderboardHandles, SteamworksStatsCommand, SteamworksStatsPlugin,
    SteamworksStatsResult, SteamworksStatsSettings, SteamworksStatsState,
};

impl SteamworksStatsPlugin {
    /// Creates a stats plugin with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a stats plugin with explicit settings.
    pub fn with_settings(settings: SteamworksStatsSettings) -> Self {
        Self { settings }
    }

    /// Sets whether current-user stats are requested automatically on startup.
    pub fn request_current_user_stats_on_startup(mut self, enabled: bool) -> Self {
        self.settings.request_current_user_stats_on_startup = enabled;
        self
    }

    /// Sets whether successful writes are automatically followed by one store call.
    pub fn auto_store(mut self, enabled: bool) -> Self {
        self.settings.auto_store = enabled;
        self
    }

    /// Returns the settings this plugin will insert when built.
    pub fn settings(&self) -> &SteamworksStatsSettings {
        &self.settings
    }

    /// Returns true when this plugin requests current-user stats on startup.
    pub fn requests_current_user_stats_on_startup(&self) -> bool {
        self.settings.request_current_user_stats_on_startup
    }

    /// Returns true when successful writes are automatically followed by `store_stats()`.
    pub fn auto_store_enabled(&self) -> bool {
        self.settings.auto_store
    }
}

impl Plugin for SteamworksStatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.settings.clone())
            .init_resource::<SteamworksStatsState>()
            .init_resource::<SteamworksStatsAsyncResults>()
            .init_resource::<SteamworksStatsLeaderboardHandles>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksStatsCommand>()
            .add_message::<SteamworksStatsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessStatsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_stats_commands.in_set(SteamworksSystem::ProcessStatsCommands),
            );
    }
}
