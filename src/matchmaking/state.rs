use bevy_ecs::prelude::Resource;

use super::*;

/// Runtime state for [`crate::SteamworksMatchmakingPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksMatchmakingState {
    last_error: Option<SteamworksMatchmakingError>,
    last_lobby_list_request: Option<SteamworksLobbyListRequest>,
    last_lobby_list: Vec<steamworks::LobbyId>,
    last_lobby_create_request: Option<SteamworksLobbyCreateRequest>,
    last_lobby_join_request: Option<SteamworksMatchmakingLobbyJoinRequest>,
    last_created_lobby: Option<SteamworksLobbyCreated>,
    last_joined_lobby: Option<SteamworksLobbyJoined>,
    last_left_lobby: Option<steamworks::LobbyId>,
    joined_lobbies: Vec<steamworks::LobbyId>,
    last_lobby_data_count: Option<SteamworksLobbyDataCount>,
    last_lobby_data: Option<SteamworksLobbyDataValue>,
    last_lobby_data_entry: Option<SteamworksLobbyDataEntry>,
    last_all_lobby_data: Option<SteamworksLobbyDataEntries>,
    last_lobby_data_set: Option<SteamworksLobbyDataMutation>,
    last_lobby_data_deleted: Option<SteamworksLobbyDataMutation>,
    last_lobby_member_data_set: Option<SteamworksLobbyDataMutation>,
    last_lobby_member_data: Option<SteamworksLobbyMemberDataValue>,
    last_lobby_member_limit: Option<SteamworksLobbyMemberLimit>,
    last_lobby_owner: Option<SteamworksLobbyOwner>,
    last_lobby_member_count: Option<SteamworksLobbyMemberCount>,
    last_lobby_members: Option<SteamworksLobbyMembers>,
    last_lobby_joinability: Option<SteamworksLobbyJoinability>,
    last_lobby_chat_message_sent: Option<SteamworksLobbyChatMessageSent>,
    last_lobby_chat_entry: Option<SteamworksLobbyChatEntry>,
    last_lobby_game_server_set: Option<SteamworksLobbyGameServerAssignment>,
    last_lobby_game_server: Option<SteamworksLobbyGameServerLookup>,
    last_lobby_created_callback: Option<SteamworksLobbyCreatedCallback>,
    last_lobby_enter_callback: Option<SteamworksLobbyEnterCallback>,
    last_lobby_chat_message: Option<SteamworksLobbyChatMessage>,
    last_lobby_chat_update: Option<SteamworksLobbyChatUpdate>,
    last_lobby_data_update: Option<SteamworksLobbyDataUpdate>,
    next_request_id: u64,
}

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

    pub(super) fn record_error(&mut self, error: SteamworksMatchmakingError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksMatchmakingOperation) {
        match operation {
            SteamworksMatchmakingOperation::LobbyListRequested { request_id, filter } => {
                self.last_lobby_list_request = Some(SteamworksLobbyListRequest {
                    request_id: *request_id,
                    filter: filter.clone(),
                });
            }
            SteamworksMatchmakingOperation::LobbyListReceived { lobbies, .. } => {
                self.last_lobby_list.clone_from(lobbies);
            }
            SteamworksMatchmakingOperation::LobbyCreateRequested {
                request_id,
                lobby_type,
                max_members,
            } => {
                self.last_lobby_create_request = Some(SteamworksLobbyCreateRequest {
                    request_id: *request_id,
                    lobby_type: *lobby_type,
                    max_members: *max_members,
                });
            }
            SteamworksMatchmakingOperation::LobbyCreated {
                request_id,
                lobby_type,
                max_members,
                lobby,
            } => {
                if !self.joined_lobbies.contains(lobby) {
                    self.joined_lobbies.push(*lobby);
                }
                self.last_created_lobby = Some(SteamworksLobbyCreated {
                    request_id: *request_id,
                    lobby_type: *lobby_type,
                    max_members: *max_members,
                    lobby: *lobby,
                });
            }
            SteamworksMatchmakingOperation::LobbyJoinRequested { request_id, lobby } => {
                self.last_lobby_join_request = Some(SteamworksMatchmakingLobbyJoinRequest {
                    request_id: *request_id,
                    lobby: *lobby,
                });
            }
            SteamworksMatchmakingOperation::LobbyJoined {
                request_id,
                requested_lobby,
                lobby,
            } => {
                if !self.joined_lobbies.contains(lobby) {
                    self.joined_lobbies.push(*lobby);
                }
                self.last_joined_lobby = Some(SteamworksLobbyJoined {
                    request_id: *request_id,
                    requested_lobby: *requested_lobby,
                    lobby: *lobby,
                });
            }
            SteamworksMatchmakingOperation::LobbyLeft { lobby } => {
                self.joined_lobbies.retain(|known| known != lobby);
                self.last_left_lobby = Some(*lobby);
            }
            SteamworksMatchmakingOperation::LobbyDataCountRead { lobby, count } => {
                self.last_lobby_data_count = Some(SteamworksLobbyDataCount {
                    lobby: *lobby,
                    count: *count,
                });
            }
            SteamworksMatchmakingOperation::LobbyDataRead { lobby, key, value } => {
                self.last_lobby_data = Some(SteamworksLobbyDataValue {
                    lobby: *lobby,
                    key: key.clone(),
                    value: value.clone(),
                });
            }
            SteamworksMatchmakingOperation::LobbyDataByIndexRead {
                lobby,
                index,
                entry,
            } => {
                self.last_lobby_data_entry = Some(SteamworksLobbyDataEntry {
                    lobby: *lobby,
                    index: *index,
                    entry: entry.clone(),
                });
            }
            SteamworksMatchmakingOperation::AllLobbyDataRead { lobby, entries } => {
                self.last_all_lobby_data = Some(SteamworksLobbyDataEntries {
                    lobby: *lobby,
                    entries: entries.clone(),
                });
            }
            SteamworksMatchmakingOperation::LobbyDataSet { lobby, key } => {
                self.last_lobby_data_set = Some(SteamworksLobbyDataMutation {
                    lobby: *lobby,
                    key: key.clone(),
                });
            }
            SteamworksMatchmakingOperation::LobbyDataDeleted { lobby, key } => {
                self.last_lobby_data_deleted = Some(SteamworksLobbyDataMutation {
                    lobby: *lobby,
                    key: key.clone(),
                });
            }
            SteamworksMatchmakingOperation::LobbyMemberDataSet { lobby, key } => {
                self.last_lobby_member_data_set = Some(SteamworksLobbyDataMutation {
                    lobby: *lobby,
                    key: key.clone(),
                });
            }
            SteamworksMatchmakingOperation::LobbyMemberDataRead {
                lobby,
                user,
                key,
                value,
            } => {
                self.last_lobby_member_data = Some(SteamworksLobbyMemberDataValue {
                    lobby: *lobby,
                    user: *user,
                    key: key.clone(),
                    value: value.clone(),
                });
            }
            SteamworksMatchmakingOperation::LobbyMemberLimitRead { lobby, limit } => {
                self.last_lobby_member_limit = Some(SteamworksLobbyMemberLimit {
                    lobby: *lobby,
                    limit: *limit,
                });
            }
            SteamworksMatchmakingOperation::LobbyOwnerRead { lobby, owner } => {
                self.last_lobby_owner = Some(SteamworksLobbyOwner {
                    lobby: *lobby,
                    owner: *owner,
                });
            }
            SteamworksMatchmakingOperation::LobbyMemberCountRead { lobby, count } => {
                self.last_lobby_member_count = Some(SteamworksLobbyMemberCount {
                    lobby: *lobby,
                    count: *count,
                });
            }
            SteamworksMatchmakingOperation::LobbyMembersListed { lobby, members } => {
                self.last_lobby_members = Some(SteamworksLobbyMembers {
                    lobby: *lobby,
                    members: members.clone(),
                });
            }
            SteamworksMatchmakingOperation::LobbyJoinableSet { lobby, joinable } => {
                self.last_lobby_joinability = Some(SteamworksLobbyJoinability {
                    lobby: *lobby,
                    joinable: *joinable,
                });
            }
            SteamworksMatchmakingOperation::LobbyChatMessageSent { lobby, len } => {
                self.last_lobby_chat_message_sent = Some(SteamworksLobbyChatMessageSent {
                    lobby: *lobby,
                    len: *len,
                });
            }
            SteamworksMatchmakingOperation::LobbyChatEntryRead {
                lobby,
                chat_id,
                data,
            } => {
                self.last_lobby_chat_entry = Some(SteamworksLobbyChatEntry {
                    lobby: *lobby,
                    chat_id: *chat_id,
                    data: data.clone(),
                });
            }
            SteamworksMatchmakingOperation::LobbyGameServerSet { lobby, server } => {
                self.last_lobby_game_server_set = Some(SteamworksLobbyGameServerAssignment {
                    lobby: *lobby,
                    server: server.clone(),
                });
            }
            SteamworksMatchmakingOperation::LobbyGameServerRead { lobby, server } => {
                self.last_lobby_game_server = Some(SteamworksLobbyGameServerLookup {
                    lobby: *lobby,
                    server: server.clone(),
                });
            }
            SteamworksMatchmakingOperation::LobbyCreateCallbackReceived { callback } => {
                self.last_lobby_created_callback = Some(callback.clone());
            }
            SteamworksMatchmakingOperation::LobbyEnterCallbackReceived { callback } => {
                if callback.chat_room_enter_response == steamworks::ChatRoomEnterResponse::Success
                    && !self.joined_lobbies.contains(&callback.lobby)
                {
                    self.joined_lobbies.push(callback.lobby);
                }
                self.last_lobby_enter_callback = Some(callback.clone());
            }
            SteamworksMatchmakingOperation::LobbyChatMessageReceived { message } => {
                self.last_lobby_chat_message = Some(message.clone());
            }
            SteamworksMatchmakingOperation::LobbyChatUpdateReceived { update } => {
                self.last_lobby_chat_update = Some(update.clone());
            }
            SteamworksMatchmakingOperation::LobbyDataUpdateReceived { update } => {
                self.last_lobby_data_update = Some(update.clone());
            }
        }
    }

    pub(super) fn next_request_id(&mut self) -> u64 {
        let request_id = self.next_request_id;
        self.next_request_id = self.next_request_id.wrapping_add(1);
        request_id
    }
}
