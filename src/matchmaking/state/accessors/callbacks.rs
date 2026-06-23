use crate::matchmaking::*;

impl SteamworksMatchmakingState {
    /// Returns the most recent lobby created callback snapshot.
    pub fn last_lobby_created_callback(&self) -> Option<&SteamworksLobbyCreatedCallback> {
        self.last_lobby_created_callback.as_ref()
    }

    /// Returns the most recent lobby enter callback snapshot.
    pub fn last_lobby_enter_callback(&self) -> Option<&SteamworksLobbyEnterCallback> {
        self.last_lobby_enter_callback.as_ref()
    }

    /// Returns the most recent lobby chat message callback snapshot.
    pub fn last_lobby_chat_message(&self) -> Option<&SteamworksLobbyChatMessage> {
        self.last_lobby_chat_message.as_ref()
    }

    /// Returns the most recent lobby membership change callback snapshot.
    pub fn last_lobby_chat_update(&self) -> Option<&SteamworksLobbyChatUpdate> {
        self.last_lobby_chat_update.as_ref()
    }

    /// Returns the most recent lobby metadata update callback snapshot.
    pub fn last_lobby_data_update(&self) -> Option<&SteamworksLobbyDataUpdate> {
        self.last_lobby_data_update.as_ref()
    }
}
