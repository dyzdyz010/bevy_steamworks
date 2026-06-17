use super::{
    SteamworksLobbyChatEntry, SteamworksLobbyChatMessage, SteamworksLobbyChatMessageSent,
    SteamworksLobbyChatUpdate, SteamworksLobbyCreateRequest, SteamworksLobbyCreated,
    SteamworksLobbyCreatedCallback, SteamworksLobbyDataCount, SteamworksLobbyDataEntries,
    SteamworksLobbyDataEntry, SteamworksLobbyDataMutation, SteamworksLobbyDataUpdate,
    SteamworksLobbyDataValue, SteamworksLobbyEnterCallback, SteamworksLobbyGameServerAssignment,
    SteamworksLobbyGameServerLookup, SteamworksLobbyJoinability, SteamworksLobbyJoined,
    SteamworksLobbyListRequest, SteamworksLobbyMemberCount, SteamworksLobbyMemberDataValue,
    SteamworksLobbyMemberLimit, SteamworksLobbyMembers, SteamworksLobbyOwner,
    SteamworksMatchmakingError, SteamworksMatchmakingLobbyJoinRequest, SteamworksMatchmakingState,
};

impl SteamworksMatchmakingState {
    /// Returns the most recent synchronous or async error observed by the plugin.
    pub fn last_error(&self) -> Option<&SteamworksMatchmakingError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent lobby-list request submitted through this plugin.
    pub fn last_lobby_list_request(&self) -> Option<&SteamworksLobbyListRequest> {
        self.last_lobby_list_request.as_ref()
    }

    /// Returns the most recent lobby list received from Steam.
    pub fn last_lobby_list(&self) -> &[steamworks::LobbyId] {
        &self.last_lobby_list
    }

    /// Returns the most recent lobby-create request submitted through this plugin.
    pub fn last_lobby_create_request(&self) -> Option<&SteamworksLobbyCreateRequest> {
        self.last_lobby_create_request.as_ref()
    }

    /// Returns the most recent lobby-join request submitted through this plugin.
    pub fn last_lobby_join_request(&self) -> Option<&SteamworksMatchmakingLobbyJoinRequest> {
        self.last_lobby_join_request.as_ref()
    }

    /// Returns the most recent lobby creation result observed through this plugin.
    pub fn last_created_lobby(&self) -> Option<&SteamworksLobbyCreated> {
        self.last_created_lobby.as_ref()
    }

    /// Returns the most recent lobby join result observed through this plugin.
    pub fn last_joined_lobby(&self) -> Option<&SteamworksLobbyJoined> {
        self.last_joined_lobby.as_ref()
    }

    /// Returns the most recent lobby left through this plugin.
    pub fn last_left_lobby(&self) -> Option<steamworks::LobbyId> {
        self.last_left_lobby
    }

    /// Returns lobbies this command layer has observed the local user joining.
    pub fn joined_lobbies(&self) -> &[steamworks::LobbyId] {
        &self.joined_lobbies
    }

    /// Returns the most recent lobby metadata count read.
    pub fn last_lobby_data_count(&self) -> Option<&SteamworksLobbyDataCount> {
        self.last_lobby_data_count.as_ref()
    }

    /// Returns the most recent lobby metadata value read.
    pub fn last_lobby_data(&self) -> Option<&SteamworksLobbyDataValue> {
        self.last_lobby_data.as_ref()
    }

    /// Returns the most recent lobby metadata entry read by index.
    pub fn last_lobby_data_entry(&self) -> Option<&SteamworksLobbyDataEntry> {
        self.last_lobby_data_entry.as_ref()
    }

    /// Returns the most recent full lobby metadata snapshot read.
    pub fn last_all_lobby_data(&self) -> Option<&SteamworksLobbyDataEntries> {
        self.last_all_lobby_data.as_ref()
    }

    /// Returns the most recent lobby metadata key set.
    pub fn last_lobby_data_set(&self) -> Option<&SteamworksLobbyDataMutation> {
        self.last_lobby_data_set.as_ref()
    }

    /// Returns the most recent lobby metadata key deleted.
    pub fn last_lobby_data_deleted(&self) -> Option<&SteamworksLobbyDataMutation> {
        self.last_lobby_data_deleted.as_ref()
    }

    /// Returns the most recent local-user lobby metadata key set.
    pub fn last_lobby_member_data_set(&self) -> Option<&SteamworksLobbyDataMutation> {
        self.last_lobby_member_data_set.as_ref()
    }

    /// Returns the most recent lobby member metadata value read.
    pub fn last_lobby_member_data(&self) -> Option<&SteamworksLobbyMemberDataValue> {
        self.last_lobby_member_data.as_ref()
    }

    /// Returns the most recent lobby member limit read.
    pub fn last_lobby_member_limit(&self) -> Option<&SteamworksLobbyMemberLimit> {
        self.last_lobby_member_limit.as_ref()
    }

    /// Returns the most recent lobby owner read.
    pub fn last_lobby_owner(&self) -> Option<&SteamworksLobbyOwner> {
        self.last_lobby_owner.as_ref()
    }

    /// Returns the most recent lobby member count read.
    pub fn last_lobby_member_count(&self) -> Option<&SteamworksLobbyMemberCount> {
        self.last_lobby_member_count.as_ref()
    }

    /// Returns the most recent lobby member list read.
    pub fn last_lobby_members(&self) -> Option<&SteamworksLobbyMembers> {
        self.last_lobby_members.as_ref()
    }

    /// Returns the most recent lobby joinability value set.
    pub fn last_lobby_joinability(&self) -> Option<&SteamworksLobbyJoinability> {
        self.last_lobby_joinability.as_ref()
    }

    /// Returns the most recent lobby chat message sent.
    pub fn last_lobby_chat_message_sent(&self) -> Option<&SteamworksLobbyChatMessageSent> {
        self.last_lobby_chat_message_sent.as_ref()
    }

    /// Returns the most recent lobby chat entry bytes read.
    pub fn last_lobby_chat_entry(&self) -> Option<&SteamworksLobbyChatEntry> {
        self.last_lobby_chat_entry.as_ref()
    }

    /// Returns the most recent lobby game-server data submitted.
    pub fn last_lobby_game_server_set(&self) -> Option<&SteamworksLobbyGameServerAssignment> {
        self.last_lobby_game_server_set.as_ref()
    }

    /// Returns the most recent lobby game-server data read.
    pub fn last_lobby_game_server(&self) -> Option<&SteamworksLobbyGameServerLookup> {
        self.last_lobby_game_server.as_ref()
    }

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
