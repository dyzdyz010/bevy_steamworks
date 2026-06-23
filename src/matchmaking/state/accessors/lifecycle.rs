use crate::matchmaking::*;

impl SteamworksMatchmakingState {
    /// Returns the most recent synchronous or async error observed by the plugin.
    pub fn last_error(&self) -> Option<&SteamworksMatchmakingError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent lobby-list request submitted through this plugin.
    pub fn last_lobby_list_request(&self) -> Option<&SteamworksLobbyListRequest> {
        self.last_lobby_list_request.as_ref()
    }

    /// Returns bounded submitted lobby-list request snapshots by request ID.
    pub fn lobby_list_requests(&self) -> &[SteamworksLobbyListRequest] {
        &self.lobby_list_requests
    }

    /// Returns the submitted lobby-list request snapshot for a request ID.
    pub fn lobby_list_request(&self, request_id: u64) -> Option<&SteamworksLobbyListRequest> {
        self.lobby_list_requests
            .iter()
            .find(|request| request.request_id == request_id)
    }

    /// Returns the most recent lobby list received from Steam.
    pub fn last_lobby_list(&self) -> &[steamworks::LobbyId] {
        &self.last_lobby_list
    }

    /// Returns the number of lobbies in the most recent lobby-list result.
    pub fn last_lobby_list_count(&self) -> usize {
        self.last_lobby_list.len()
    }

    /// Returns bounded completed lobby-list snapshots by request ID.
    pub fn lobby_list_results(&self) -> &[SteamworksLobbyListResult] {
        &self.lobby_list_results
    }

    /// Returns the completed lobby-list snapshot for a request ID.
    pub fn lobby_list_result(&self, request_id: u64) -> Option<&SteamworksLobbyListResult> {
        self.lobby_list_results
            .iter()
            .find(|result| result.request_id == request_id)
    }

    /// Returns the most recent lobby-create request submitted through this plugin.
    pub fn last_lobby_create_request(&self) -> Option<&SteamworksLobbyCreateRequest> {
        self.last_lobby_create_request.as_ref()
    }

    /// Returns bounded submitted lobby-create request snapshots by request ID.
    pub fn lobby_create_requests(&self) -> &[SteamworksLobbyCreateRequest] {
        &self.lobby_create_requests
    }

    /// Returns the submitted lobby-create request snapshot for a request ID.
    pub fn lobby_create_request(&self, request_id: u64) -> Option<&SteamworksLobbyCreateRequest> {
        self.lobby_create_requests
            .iter()
            .find(|request| request.request_id == request_id)
    }

    /// Returns the most recent lobby-join request submitted through this plugin.
    pub fn last_lobby_join_request(&self) -> Option<&SteamworksMatchmakingLobbyJoinRequest> {
        self.last_lobby_join_request.as_ref()
    }

    /// Returns bounded submitted lobby-join request snapshots by request ID.
    pub fn lobby_join_requests(&self) -> &[SteamworksMatchmakingLobbyJoinRequest] {
        &self.lobby_join_requests
    }

    /// Returns the submitted lobby-join request snapshot for a request ID.
    pub fn lobby_join_request(
        &self,
        request_id: u64,
    ) -> Option<&SteamworksMatchmakingLobbyJoinRequest> {
        self.lobby_join_requests
            .iter()
            .find(|request| request.request_id == request_id)
    }

    /// Returns the most recent lobby creation result observed through this plugin.
    pub fn last_created_lobby(&self) -> Option<&SteamworksLobbyCreated> {
        self.last_created_lobby.as_ref()
    }

    /// Returns bounded completed lobby creation snapshots by request ID.
    pub fn created_lobbies(&self) -> &[SteamworksLobbyCreated] {
        &self.created_lobbies
    }

    /// Returns the completed lobby creation snapshot for a request ID.
    pub fn created_lobby(&self, request_id: u64) -> Option<&SteamworksLobbyCreated> {
        self.created_lobbies
            .iter()
            .find(|created| created.request_id == request_id)
    }

    /// Returns the most recent lobby join result observed through this plugin.
    pub fn last_joined_lobby(&self) -> Option<&SteamworksLobbyJoined> {
        self.last_joined_lobby.as_ref()
    }

    /// Returns bounded completed lobby join snapshots by request ID.
    pub fn joined_lobby_results(&self) -> &[SteamworksLobbyJoined] {
        &self.joined_lobby_results
    }

    /// Returns the completed lobby join snapshot for a request ID.
    pub fn joined_lobby_result(&self, request_id: u64) -> Option<&SteamworksLobbyJoined> {
        self.joined_lobby_results
            .iter()
            .find(|joined| joined.request_id == request_id)
    }

    /// Returns the most recent lobby left through this plugin.
    pub fn last_left_lobby(&self) -> Option<steamworks::LobbyId> {
        self.last_left_lobby
    }

    /// Returns lobbies this command layer has observed the local user joining.
    pub fn joined_lobbies(&self) -> &[steamworks::LobbyId] {
        &self.joined_lobbies
    }

    /// Returns the number of lobbies this command layer currently considers joined.
    pub fn joined_lobby_count(&self) -> usize {
        self.joined_lobbies.len()
    }

    /// Returns the most recent lobby ID this command layer observed as joined.
    pub fn last_joined_lobby_id(&self) -> Option<steamworks::LobbyId> {
        self.last_joined_lobby.as_ref().map(|joined| joined.lobby)
    }

    /// Returns whether this command layer currently considers the lobby joined.
    pub fn is_lobby_joined(&self, lobby: steamworks::LobbyId) -> bool {
        self.joined_lobbies.contains(&lobby)
    }
}
