use super::{
    SteamworksLobbyChatEntry, SteamworksLobbyChatMessage, SteamworksLobbyChatMessageSent,
    SteamworksLobbyChatUpdate, SteamworksLobbyCreateRequest, SteamworksLobbyCreated,
    SteamworksLobbyCreatedCallback, SteamworksLobbyDataCount, SteamworksLobbyDataEntries,
    SteamworksLobbyDataEntry, SteamworksLobbyDataMutation, SteamworksLobbyDataUpdate,
    SteamworksLobbyDataValue, SteamworksLobbyEnterCallback, SteamworksLobbyGameServerAssignment,
    SteamworksLobbyGameServerLookup, SteamworksLobbyJoinability, SteamworksLobbyJoined,
    SteamworksLobbyListRequest, SteamworksLobbyListResult, SteamworksLobbyMemberCount,
    SteamworksLobbyMemberDataValue, SteamworksLobbyMemberLimit, SteamworksLobbyMembers,
    SteamworksLobbyOwner, SteamworksMatchmakingError, SteamworksMatchmakingLobbyJoinRequest,
    SteamworksMatchmakingState,
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

    /// Returns whether this command layer currently considers the lobby joined.
    pub fn is_lobby_joined(&self, lobby: steamworks::LobbyId) -> bool {
        self.joined_lobbies.contains(&lobby)
    }

    /// Returns the most recent lobby metadata count read.
    pub fn last_lobby_data_count(&self) -> Option<&SteamworksLobbyDataCount> {
        self.last_lobby_data_count.as_ref()
    }

    /// Returns bounded lobby metadata count snapshots by lobby.
    pub fn lobby_data_counts(&self) -> &[SteamworksLobbyDataCount] {
        &self.lobby_data_counts
    }

    /// Returns the latest metadata count snapshot for a lobby.
    pub fn lobby_data_count(
        &self,
        lobby: steamworks::LobbyId,
    ) -> Option<&SteamworksLobbyDataCount> {
        self.lobby_data_counts
            .iter()
            .find(|count| count.lobby == lobby)
    }

    /// Returns the most recent lobby metadata value read.
    pub fn last_lobby_data(&self) -> Option<&SteamworksLobbyDataValue> {
        self.last_lobby_data.as_ref()
    }

    /// Returns bounded lobby metadata value snapshots by lobby and key.
    pub fn lobby_data_values(&self) -> &[SteamworksLobbyDataValue] {
        &self.lobby_data_values
    }

    /// Returns the latest metadata value snapshot for a lobby/key pair.
    pub fn lobby_data_value(
        &self,
        lobby: steamworks::LobbyId,
        key: impl AsRef<str>,
    ) -> Option<&SteamworksLobbyDataValue> {
        let key = key.as_ref();
        self.lobby_data_values
            .iter()
            .find(|value| value.lobby == lobby && value.key.as_str() == key)
    }

    /// Returns the latest known lobby metadata value for a key.
    ///
    /// The outer `Option` distinguishes no cached data from cached data. The
    /// inner `Option` is `None` when a direct read reported no value or a full
    /// metadata snapshot did not include the key.
    pub fn lobby_data(
        &self,
        lobby: steamworks::LobbyId,
        key: impl AsRef<str>,
    ) -> Option<Option<&str>> {
        let key = key.as_ref();
        self.lobby_data_value(lobby, key)
            .map(|value| value.value.as_deref())
            .or_else(|| {
                self.all_lobby_data(lobby).map(|entries| {
                    entries
                        .entries
                        .iter()
                        .find(|(entry_key, _)| entry_key == key)
                        .map(|(_, value)| value.as_str())
                })
            })
    }

    /// Returns the most recent lobby metadata entry read by index.
    pub fn last_lobby_data_entry(&self) -> Option<&SteamworksLobbyDataEntry> {
        self.last_lobby_data_entry.as_ref()
    }

    /// Returns bounded lobby metadata entry snapshots by lobby and index.
    pub fn lobby_data_entries(&self) -> &[SteamworksLobbyDataEntry] {
        &self.lobby_data_entries
    }

    /// Returns the latest metadata entry snapshot for a lobby/index pair.
    pub fn lobby_data_entry(
        &self,
        lobby: steamworks::LobbyId,
        index: u32,
    ) -> Option<&SteamworksLobbyDataEntry> {
        self.lobby_data_entries
            .iter()
            .find(|entry| entry.lobby == lobby && entry.index == index)
    }

    /// Returns the most recent full lobby metadata snapshot read.
    pub fn last_all_lobby_data(&self) -> Option<&SteamworksLobbyDataEntries> {
        self.last_all_lobby_data.as_ref()
    }

    /// Returns bounded full metadata snapshots by lobby.
    pub fn all_lobby_data_entries(&self) -> &[SteamworksLobbyDataEntries] {
        &self.all_lobby_data
    }

    /// Returns the latest full metadata snapshot for a lobby.
    pub fn all_lobby_data(
        &self,
        lobby: steamworks::LobbyId,
    ) -> Option<&SteamworksLobbyDataEntries> {
        self.all_lobby_data
            .iter()
            .find(|entries| entries.lobby == lobby)
    }

    /// Returns one value from the latest full lobby metadata snapshot.
    pub fn all_lobby_data_value(
        &self,
        lobby: steamworks::LobbyId,
        key: impl AsRef<str>,
    ) -> Option<&str> {
        let key = key.as_ref();
        self.all_lobby_data(lobby).and_then(|entries| {
            entries
                .entries
                .iter()
                .find(|(entry_key, _)| entry_key == key)
                .map(|(_, value)| value.as_str())
        })
    }

    /// Returns the most recent lobby metadata key set.
    pub fn last_lobby_data_set(&self) -> Option<&SteamworksLobbyDataMutation> {
        self.last_lobby_data_set.as_ref()
    }

    /// Returns bounded lobby metadata set snapshots by lobby and key.
    pub fn lobby_data_sets(&self) -> &[SteamworksLobbyDataMutation] {
        &self.lobby_data_sets
    }

    /// Returns the latest metadata set snapshot for a lobby/key pair.
    pub fn lobby_data_set(
        &self,
        lobby: steamworks::LobbyId,
        key: impl AsRef<str>,
    ) -> Option<&SteamworksLobbyDataMutation> {
        let key = key.as_ref();
        self.lobby_data_sets
            .iter()
            .find(|set| set.lobby == lobby && set.key.as_str() == key)
    }

    /// Returns the most recent lobby metadata key deleted.
    pub fn last_lobby_data_deleted(&self) -> Option<&SteamworksLobbyDataMutation> {
        self.last_lobby_data_deleted.as_ref()
    }

    /// Returns bounded lobby metadata deletion snapshots by lobby and key.
    pub fn lobby_data_deletions(&self) -> &[SteamworksLobbyDataMutation] {
        &self.lobby_data_deletions
    }

    /// Returns the latest metadata deletion snapshot for a lobby/key pair.
    pub fn lobby_data_deletion(
        &self,
        lobby: steamworks::LobbyId,
        key: impl AsRef<str>,
    ) -> Option<&SteamworksLobbyDataMutation> {
        let key = key.as_ref();
        self.lobby_data_deletions
            .iter()
            .find(|deleted| deleted.lobby == lobby && deleted.key.as_str() == key)
    }

    /// Returns the most recent local-user lobby metadata key set.
    pub fn last_lobby_member_data_set(&self) -> Option<&SteamworksLobbyDataMutation> {
        self.last_lobby_member_data_set.as_ref()
    }

    /// Returns bounded local-user lobby metadata set snapshots by lobby and key.
    pub fn lobby_member_data_sets(&self) -> &[SteamworksLobbyDataMutation] {
        &self.lobby_member_data_sets
    }

    /// Returns the latest local-user metadata set snapshot for a lobby/key pair.
    pub fn lobby_member_data_set(
        &self,
        lobby: steamworks::LobbyId,
        key: impl AsRef<str>,
    ) -> Option<&SteamworksLobbyDataMutation> {
        let key = key.as_ref();
        self.lobby_member_data_sets
            .iter()
            .find(|set| set.lobby == lobby && set.key.as_str() == key)
    }

    /// Returns the most recent lobby member metadata value read.
    pub fn last_lobby_member_data(&self) -> Option<&SteamworksLobbyMemberDataValue> {
        self.last_lobby_member_data.as_ref()
    }

    /// Returns bounded lobby member metadata value snapshots by lobby, user, and key.
    pub fn lobby_member_data_values(&self) -> &[SteamworksLobbyMemberDataValue] {
        &self.lobby_member_data_values
    }

    /// Returns the latest member metadata value snapshot for a lobby/user/key triple.
    pub fn lobby_member_data_value(
        &self,
        lobby: steamworks::LobbyId,
        user: steamworks::SteamId,
        key: impl AsRef<str>,
    ) -> Option<&SteamworksLobbyMemberDataValue> {
        let key = key.as_ref();
        self.lobby_member_data_values
            .iter()
            .find(|value| value.lobby == lobby && value.user == user && value.key.as_str() == key)
    }

    /// Returns the latest known member metadata value for a lobby/user/key triple.
    ///
    /// The outer `Option` distinguishes no cached read from a completed read.
    /// The inner `Option` is `None` when Steam reported no value for the key.
    pub fn lobby_member_data(
        &self,
        lobby: steamworks::LobbyId,
        user: steamworks::SteamId,
        key: impl AsRef<str>,
    ) -> Option<Option<&str>> {
        self.lobby_member_data_value(lobby, user, key)
            .map(|value| value.value.as_deref())
    }

    /// Returns the most recent lobby member limit read.
    pub fn last_lobby_member_limit(&self) -> Option<&SteamworksLobbyMemberLimit> {
        self.last_lobby_member_limit.as_ref()
    }

    /// Returns bounded lobby member limit snapshots by lobby.
    pub fn lobby_member_limits(&self) -> &[SteamworksLobbyMemberLimit] {
        &self.lobby_member_limits
    }

    /// Returns the latest member limit snapshot for a lobby.
    pub fn lobby_member_limit(
        &self,
        lobby: steamworks::LobbyId,
    ) -> Option<&SteamworksLobbyMemberLimit> {
        self.lobby_member_limits
            .iter()
            .find(|limit| limit.lobby == lobby)
    }

    /// Returns the latest known member limit for a lobby, preserving an unknown limit as `Some(None)`.
    pub fn lobby_member_limit_value(&self, lobby: steamworks::LobbyId) -> Option<Option<usize>> {
        self.lobby_member_limit(lobby).map(|limit| limit.limit)
    }

    /// Returns the most recent lobby owner read.
    pub fn last_lobby_owner(&self) -> Option<&SteamworksLobbyOwner> {
        self.last_lobby_owner.as_ref()
    }

    /// Returns bounded lobby owner snapshots by lobby.
    pub fn lobby_owners(&self) -> &[SteamworksLobbyOwner] {
        &self.lobby_owners
    }

    /// Returns the latest owner snapshot for a lobby.
    pub fn lobby_owner(&self, lobby: steamworks::LobbyId) -> Option<&SteamworksLobbyOwner> {
        self.lobby_owners.iter().find(|owner| owner.lobby == lobby)
    }

    /// Returns the latest known lobby owner Steam ID.
    pub fn lobby_owner_id(&self, lobby: steamworks::LobbyId) -> Option<steamworks::SteamId> {
        self.lobby_owner(lobby).map(|owner| owner.owner)
    }

    /// Returns the most recent lobby member count read.
    pub fn last_lobby_member_count(&self) -> Option<&SteamworksLobbyMemberCount> {
        self.last_lobby_member_count.as_ref()
    }

    /// Returns bounded lobby member count snapshots by lobby.
    pub fn lobby_member_counts(&self) -> &[SteamworksLobbyMemberCount] {
        &self.lobby_member_counts
    }

    /// Returns the latest member count snapshot for a lobby.
    pub fn lobby_member_count(
        &self,
        lobby: steamworks::LobbyId,
    ) -> Option<&SteamworksLobbyMemberCount> {
        self.lobby_member_counts
            .iter()
            .find(|count| count.lobby == lobby)
    }

    /// Returns the latest known lobby member count.
    pub fn lobby_member_count_value(&self, lobby: steamworks::LobbyId) -> Option<usize> {
        self.lobby_member_count(lobby).map(|count| count.count)
    }

    /// Returns the most recent lobby member list read.
    pub fn last_lobby_members(&self) -> Option<&SteamworksLobbyMembers> {
        self.last_lobby_members.as_ref()
    }

    /// Returns bounded lobby member list snapshots by lobby.
    pub fn lobby_member_lists(&self) -> &[SteamworksLobbyMembers] {
        &self.lobby_member_lists
    }

    /// Returns the latest member list snapshot for a lobby.
    pub fn lobby_members(&self, lobby: steamworks::LobbyId) -> Option<&SteamworksLobbyMembers> {
        self.lobby_member_lists
            .iter()
            .find(|members| members.lobby == lobby)
    }

    /// Returns the latest known member IDs for a lobby.
    pub fn lobby_member_ids(&self, lobby: steamworks::LobbyId) -> Option<&[steamworks::SteamId]> {
        self.lobby_members(lobby)
            .map(|members| members.members.as_slice())
    }

    /// Returns whether the latest known member list for a lobby contains a user.
    pub fn has_lobby_member(
        &self,
        lobby: steamworks::LobbyId,
        user: steamworks::SteamId,
    ) -> Option<bool> {
        self.lobby_member_ids(lobby)
            .map(|members| members.contains(&user))
    }

    /// Returns the most recent lobby joinability value set.
    pub fn last_lobby_joinability(&self) -> Option<&SteamworksLobbyJoinability> {
        self.last_lobby_joinability.as_ref()
    }

    /// Returns bounded lobby joinability snapshots by lobby.
    pub fn lobby_joinabilities(&self) -> &[SteamworksLobbyJoinability] {
        &self.lobby_joinabilities
    }

    /// Returns the latest joinability snapshot for a lobby.
    pub fn lobby_joinability(
        &self,
        lobby: steamworks::LobbyId,
    ) -> Option<&SteamworksLobbyJoinability> {
        self.lobby_joinabilities
            .iter()
            .find(|joinability| joinability.lobby == lobby)
    }

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
