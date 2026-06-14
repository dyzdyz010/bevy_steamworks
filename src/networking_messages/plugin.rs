use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::SteamworksSystem;

use super::{
    callbacks::{apply_networking_messages_policy_commands, ensure_networking_messages_callbacks},
    commands::process_networking_messages_commands,
    SteamworksNetworkingMessagesCommand, SteamworksNetworkingMessagesPlugin,
    SteamworksNetworkingMessagesResult, SteamworksNetworkingMessagesState,
};

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

    /// Returns true when incoming session requests are accepted in the Steam callback.
    pub fn auto_accepts_session_requests(&self) -> bool {
        self.auto_accept_session_requests
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
