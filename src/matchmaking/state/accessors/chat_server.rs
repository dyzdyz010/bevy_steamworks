use crate::matchmaking::*;

impl SteamworksMatchmakingState {
    /// Returns the most recent lobby chat message sent.
    pub fn last_lobby_chat_message_sent(&self) -> Option<&SteamworksLobbyChatMessageSent> {
        self.last_lobby_chat_message_sent.as_ref()
    }

    /// Returns bounded lobby chat send snapshots by lobby.
    pub fn lobby_chat_message_sends(&self) -> &[SteamworksLobbyChatMessageSent] {
        &self.lobby_chat_message_sends
    }

    /// Returns the latest chat send snapshot for a lobby.
    pub fn lobby_chat_message_sent(
        &self,
        lobby: steamworks::LobbyId,
    ) -> Option<&SteamworksLobbyChatMessageSent> {
        self.lobby_chat_message_sends
            .iter()
            .find(|sent| sent.lobby == lobby)
    }

    /// Returns the most recent lobby chat entry bytes read.
    pub fn last_lobby_chat_entry(&self) -> Option<&SteamworksLobbyChatEntry> {
        self.last_lobby_chat_entry.as_ref()
    }

    /// Returns bounded lobby chat entry snapshots by lobby and chat entry ID.
    pub fn lobby_chat_entries(&self) -> &[SteamworksLobbyChatEntry] {
        &self.lobby_chat_entries
    }

    /// Returns the latest chat entry snapshot for a lobby/chat entry pair.
    pub fn lobby_chat_entry(
        &self,
        lobby: steamworks::LobbyId,
        chat_id: i32,
    ) -> Option<&SteamworksLobbyChatEntry> {
        self.lobby_chat_entries
            .iter()
            .find(|entry| entry.lobby == lobby && entry.chat_id == chat_id)
    }

    /// Returns the latest owned chat entry bytes for a lobby/chat entry pair.
    pub fn lobby_chat_entry_data(&self, lobby: steamworks::LobbyId, chat_id: i32) -> Option<&[u8]> {
        self.lobby_chat_entry(lobby, chat_id)
            .map(|entry| entry.data.as_slice())
    }

    /// Returns the latest known byte length for a lobby/chat entry pair.
    pub fn lobby_chat_entry_len(&self, lobby: steamworks::LobbyId, chat_id: i32) -> Option<usize> {
        self.lobby_chat_entry(lobby, chat_id)
            .map(|entry| entry.data.len())
    }

    /// Returns bytes from the most recent lobby chat entry read.
    pub fn last_lobby_chat_entry_data(&self) -> Option<&[u8]> {
        self.last_lobby_chat_entry
            .as_ref()
            .map(|entry| entry.data.as_slice())
    }

    /// Returns the most recent lobby game-server data submitted.
    pub fn last_lobby_game_server_set(&self) -> Option<&SteamworksLobbyGameServerAssignment> {
        self.last_lobby_game_server_set.as_ref()
    }

    /// Returns bounded lobby game-server assignment snapshots by lobby.
    pub fn lobby_game_server_assignments(&self) -> &[SteamworksLobbyGameServerAssignment] {
        &self.lobby_game_server_assignments
    }

    /// Returns the latest game-server assignment snapshot for a lobby.
    pub fn lobby_game_server_assignment(
        &self,
        lobby: steamworks::LobbyId,
    ) -> Option<&SteamworksLobbyGameServerAssignment> {
        self.lobby_game_server_assignments
            .iter()
            .find(|assignment| assignment.lobby == lobby)
    }

    /// Returns the most recent lobby game-server data read.
    pub fn last_lobby_game_server(&self) -> Option<&SteamworksLobbyGameServerLookup> {
        self.last_lobby_game_server.as_ref()
    }

    /// Returns bounded lobby game-server lookup snapshots by lobby.
    pub fn lobby_game_server_lookups(&self) -> &[SteamworksLobbyGameServerLookup] {
        &self.lobby_game_server_lookups
    }

    /// Returns the latest game-server lookup snapshot for a lobby.
    pub fn lobby_game_server(
        &self,
        lobby: steamworks::LobbyId,
    ) -> Option<&SteamworksLobbyGameServerLookup> {
        self.lobby_game_server_lookups
            .iter()
            .find(|lookup| lookup.lobby == lobby)
    }

    /// Returns whether the latest game-server lookup for a lobby found a server.
    pub fn has_lobby_game_server(&self, lobby: steamworks::LobbyId) -> Option<bool> {
        self.lobby_game_server(lobby)
            .map(|lookup| lookup.server.is_some())
    }

    /// Returns the latest game-server address for a lobby, preserving an empty lookup as `Some(None)`.
    pub fn lobby_game_server_address(
        &self,
        lobby: steamworks::LobbyId,
    ) -> Option<Option<std::net::SocketAddrV4>> {
        self.lobby_game_server(lobby)
            .map(|lookup| lookup.server.as_ref().map(|server| server.address))
    }

    /// Returns the latest game-server Steam ID for a lobby, preserving absent IDs as `Some(None)`.
    pub fn lobby_game_server_steam_id(
        &self,
        lobby: steamworks::LobbyId,
    ) -> Option<Option<steamworks::SteamId>> {
        self.lobby_game_server(lobby)
            .map(|lookup| lookup.server.as_ref().and_then(|server| server.steam_id))
    }
}
