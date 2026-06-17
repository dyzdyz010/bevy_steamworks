use super::{
    SteamworksLobbyChatEntry, SteamworksLobbyChatMessageSent, SteamworksLobbyCreateRequest,
    SteamworksLobbyCreated, SteamworksLobbyDataCount, SteamworksLobbyDataEntries,
    SteamworksLobbyDataEntry, SteamworksLobbyDataMutation, SteamworksLobbyDataValue,
    SteamworksLobbyGameServerAssignment, SteamworksLobbyGameServerLookup,
    SteamworksLobbyJoinability, SteamworksLobbyJoined, SteamworksLobbyListRequest,
    SteamworksLobbyMemberCount, SteamworksLobbyMemberDataValue, SteamworksLobbyMemberLimit,
    SteamworksLobbyMembers, SteamworksLobbyOwner, SteamworksMatchmakingError,
    SteamworksMatchmakingLobbyJoinRequest, SteamworksMatchmakingOperation,
    SteamworksMatchmakingState,
};

impl SteamworksMatchmakingState {
    pub(in crate::matchmaking) fn record_error(&mut self, error: SteamworksMatchmakingError) {
        self.last_error = Some(error);
    }

    pub(in crate::matchmaking) fn record_operation(
        &mut self,
        operation: &SteamworksMatchmakingOperation,
    ) {
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

    pub(in crate::matchmaking) fn next_request_id(&mut self) -> u64 {
        let request_id = self.next_request_id;
        self.next_request_id = self.next_request_id.wrapping_add(1);
        request_id
    }
}
