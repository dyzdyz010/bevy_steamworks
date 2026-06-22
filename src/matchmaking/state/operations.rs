use super::{
    upsert_by, SteamworksLobbyChatEntry, SteamworksLobbyChatMessageSent,
    SteamworksLobbyCreateRequest, SteamworksLobbyCreated, SteamworksLobbyDataCount,
    SteamworksLobbyDataEntries, SteamworksLobbyDataEntry, SteamworksLobbyDataMutation,
    SteamworksLobbyDataValue, SteamworksLobbyGameServerAssignment, SteamworksLobbyGameServerLookup,
    SteamworksLobbyJoinability, SteamworksLobbyJoined, SteamworksLobbyListRequest,
    SteamworksLobbyListResult, SteamworksLobbyMemberCount, SteamworksLobbyMemberDataValue,
    SteamworksLobbyMemberLimit, SteamworksLobbyMembers, SteamworksLobbyOwner,
    SteamworksMatchmakingError, SteamworksMatchmakingLobbyJoinRequest,
    SteamworksMatchmakingOperation, SteamworksMatchmakingState,
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
                let request = SteamworksLobbyListRequest {
                    request_id: *request_id,
                    filter: filter.clone(),
                };
                upsert_by(&mut self.lobby_list_requests, request.clone(), |existing| {
                    existing.request_id == *request_id
                });
                self.last_lobby_list_request = Some(request);
            }
            SteamworksMatchmakingOperation::LobbyListReceived {
                request_id,
                filter,
                lobbies,
            } => {
                upsert_by(
                    &mut self.lobby_list_requests,
                    SteamworksLobbyListRequest {
                        request_id: *request_id,
                        filter: filter.clone(),
                    },
                    |existing| existing.request_id == *request_id,
                );
                upsert_by(
                    &mut self.lobby_list_results,
                    SteamworksLobbyListResult {
                        request_id: *request_id,
                        filter: filter.clone(),
                        lobbies: lobbies.clone(),
                    },
                    |existing| existing.request_id == *request_id,
                );
                self.last_lobby_list.clone_from(lobbies);
            }
            SteamworksMatchmakingOperation::LobbyCreateRequested {
                request_id,
                lobby_type,
                max_members,
            } => {
                let request = SteamworksLobbyCreateRequest {
                    request_id: *request_id,
                    lobby_type: *lobby_type,
                    max_members: *max_members,
                };
                upsert_by(
                    &mut self.lobby_create_requests,
                    request.clone(),
                    |existing| existing.request_id == *request_id,
                );
                self.last_lobby_create_request = Some(request);
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
                upsert_by(
                    &mut self.lobby_create_requests,
                    SteamworksLobbyCreateRequest {
                        request_id: *request_id,
                        lobby_type: *lobby_type,
                        max_members: *max_members,
                    },
                    |existing| existing.request_id == *request_id,
                );
                let created = SteamworksLobbyCreated {
                    request_id: *request_id,
                    lobby_type: *lobby_type,
                    max_members: *max_members,
                    lobby: *lobby,
                };
                upsert_by(&mut self.created_lobbies, created.clone(), |existing| {
                    existing.request_id == *request_id
                });
                self.last_created_lobby = Some(created);
            }
            SteamworksMatchmakingOperation::LobbyJoinRequested { request_id, lobby } => {
                let request = SteamworksMatchmakingLobbyJoinRequest {
                    request_id: *request_id,
                    lobby: *lobby,
                };
                upsert_by(&mut self.lobby_join_requests, request.clone(), |existing| {
                    existing.request_id == *request_id
                });
                self.last_lobby_join_request = Some(request);
            }
            SteamworksMatchmakingOperation::LobbyJoined {
                request_id,
                requested_lobby,
                lobby,
            } => {
                if !self.joined_lobbies.contains(lobby) {
                    self.joined_lobbies.push(*lobby);
                }
                upsert_by(
                    &mut self.lobby_join_requests,
                    SteamworksMatchmakingLobbyJoinRequest {
                        request_id: *request_id,
                        lobby: *requested_lobby,
                    },
                    |existing| existing.request_id == *request_id,
                );
                let joined = SteamworksLobbyJoined {
                    request_id: *request_id,
                    requested_lobby: *requested_lobby,
                    lobby: *lobby,
                };
                upsert_by(&mut self.joined_lobby_results, joined.clone(), |existing| {
                    existing.request_id == *request_id
                });
                self.last_joined_lobby = Some(joined);
            }
            SteamworksMatchmakingOperation::LobbyLeft { lobby } => {
                self.joined_lobbies.retain(|known| known != lobby);
                self.last_left_lobby = Some(*lobby);
            }
            SteamworksMatchmakingOperation::LobbyDataCountRead { lobby, count } => {
                let snapshot = SteamworksLobbyDataCount {
                    lobby: *lobby,
                    count: *count,
                };
                upsert_by(&mut self.lobby_data_counts, snapshot.clone(), |existing| {
                    existing.lobby == *lobby
                });
                self.last_lobby_data_count = Some(snapshot);
            }
            SteamworksMatchmakingOperation::LobbyDataRead { lobby, key, value } => {
                let snapshot = SteamworksLobbyDataValue {
                    lobby: *lobby,
                    key: key.clone(),
                    value: value.clone(),
                };
                upsert_by(&mut self.lobby_data_values, snapshot.clone(), |existing| {
                    existing.lobby == *lobby && existing.key == *key
                });
                self.last_lobby_data = Some(snapshot);
            }
            SteamworksMatchmakingOperation::LobbyDataByIndexRead {
                lobby,
                index,
                entry,
            } => {
                let snapshot = SteamworksLobbyDataEntry {
                    lobby: *lobby,
                    index: *index,
                    entry: entry.clone(),
                };
                upsert_by(&mut self.lobby_data_entries, snapshot.clone(), |existing| {
                    existing.lobby == *lobby && existing.index == *index
                });
                self.last_lobby_data_entry = Some(snapshot);
            }
            SteamworksMatchmakingOperation::AllLobbyDataRead { lobby, entries } => {
                let snapshot = SteamworksLobbyDataEntries {
                    lobby: *lobby,
                    entries: entries.clone(),
                };
                upsert_by(&mut self.all_lobby_data, snapshot.clone(), |existing| {
                    existing.lobby == *lobby
                });
                self.last_all_lobby_data = Some(snapshot);
            }
            SteamworksMatchmakingOperation::LobbyDataSet { lobby, key } => {
                let snapshot = SteamworksLobbyDataMutation {
                    lobby: *lobby,
                    key: key.clone(),
                };
                upsert_by(&mut self.lobby_data_sets, snapshot.clone(), |existing| {
                    existing.lobby == *lobby && existing.key == *key
                });
                self.last_lobby_data_set = Some(snapshot);
            }
            SteamworksMatchmakingOperation::LobbyDataDeleted { lobby, key } => {
                let snapshot = SteamworksLobbyDataMutation {
                    lobby: *lobby,
                    key: key.clone(),
                };
                upsert_by(
                    &mut self.lobby_data_deletions,
                    snapshot.clone(),
                    |existing| existing.lobby == *lobby && existing.key == *key,
                );
                self.last_lobby_data_deleted = Some(snapshot);
            }
            SteamworksMatchmakingOperation::LobbyMemberDataSet { lobby, key } => {
                let snapshot = SteamworksLobbyDataMutation {
                    lobby: *lobby,
                    key: key.clone(),
                };
                upsert_by(
                    &mut self.lobby_member_data_sets,
                    snapshot.clone(),
                    |existing| existing.lobby == *lobby && existing.key == *key,
                );
                self.last_lobby_member_data_set = Some(snapshot);
            }
            SteamworksMatchmakingOperation::LobbyMemberDataRead {
                lobby,
                user,
                key,
                value,
            } => {
                let snapshot = SteamworksLobbyMemberDataValue {
                    lobby: *lobby,
                    user: *user,
                    key: key.clone(),
                    value: value.clone(),
                };
                upsert_by(
                    &mut self.lobby_member_data_values,
                    snapshot.clone(),
                    |existing| {
                        existing.lobby == *lobby && existing.user == *user && existing.key == *key
                    },
                );
                self.last_lobby_member_data = Some(snapshot);
            }
            SteamworksMatchmakingOperation::LobbyMemberLimitRead { lobby, limit } => {
                let snapshot = SteamworksLobbyMemberLimit {
                    lobby: *lobby,
                    limit: *limit,
                };
                upsert_by(
                    &mut self.lobby_member_limits,
                    snapshot.clone(),
                    |existing| existing.lobby == *lobby,
                );
                self.last_lobby_member_limit = Some(snapshot);
            }
            SteamworksMatchmakingOperation::LobbyOwnerRead { lobby, owner } => {
                let snapshot = SteamworksLobbyOwner {
                    lobby: *lobby,
                    owner: *owner,
                };
                upsert_by(&mut self.lobby_owners, snapshot.clone(), |existing| {
                    existing.lobby == *lobby
                });
                self.last_lobby_owner = Some(snapshot);
            }
            SteamworksMatchmakingOperation::LobbyMemberCountRead { lobby, count } => {
                let snapshot = SteamworksLobbyMemberCount {
                    lobby: *lobby,
                    count: *count,
                };
                upsert_by(
                    &mut self.lobby_member_counts,
                    snapshot.clone(),
                    |existing| existing.lobby == *lobby,
                );
                self.last_lobby_member_count = Some(snapshot);
            }
            SteamworksMatchmakingOperation::LobbyMembersListed { lobby, members } => {
                let snapshot = SteamworksLobbyMembers {
                    lobby: *lobby,
                    members: members.clone(),
                };
                upsert_by(&mut self.lobby_member_lists, snapshot.clone(), |existing| {
                    existing.lobby == *lobby
                });
                self.last_lobby_members = Some(snapshot);
            }
            SteamworksMatchmakingOperation::LobbyJoinableSet { lobby, joinable } => {
                let snapshot = SteamworksLobbyJoinability {
                    lobby: *lobby,
                    joinable: *joinable,
                };
                upsert_by(
                    &mut self.lobby_joinabilities,
                    snapshot.clone(),
                    |existing| existing.lobby == *lobby,
                );
                self.last_lobby_joinability = Some(snapshot);
            }
            SteamworksMatchmakingOperation::LobbyChatMessageSent { lobby, len } => {
                let snapshot = SteamworksLobbyChatMessageSent {
                    lobby: *lobby,
                    len: *len,
                };
                upsert_by(
                    &mut self.lobby_chat_message_sends,
                    snapshot.clone(),
                    |existing| existing.lobby == *lobby,
                );
                self.last_lobby_chat_message_sent = Some(snapshot);
            }
            SteamworksMatchmakingOperation::LobbyChatEntryRead {
                lobby,
                chat_id,
                data,
            } => {
                let snapshot = SteamworksLobbyChatEntry {
                    lobby: *lobby,
                    chat_id: *chat_id,
                    data: data.clone(),
                };
                upsert_by(&mut self.lobby_chat_entries, snapshot.clone(), |existing| {
                    existing.lobby == *lobby && existing.chat_id == *chat_id
                });
                self.last_lobby_chat_entry = Some(snapshot);
            }
            SteamworksMatchmakingOperation::LobbyGameServerSet { lobby, server } => {
                let snapshot = SteamworksLobbyGameServerAssignment {
                    lobby: *lobby,
                    server: server.clone(),
                };
                upsert_by(
                    &mut self.lobby_game_server_assignments,
                    snapshot.clone(),
                    |existing| existing.lobby == *lobby,
                );
                self.last_lobby_game_server_set = Some(snapshot);
            }
            SteamworksMatchmakingOperation::LobbyGameServerRead { lobby, server } => {
                let snapshot = SteamworksLobbyGameServerLookup {
                    lobby: *lobby,
                    server: server.clone(),
                };
                upsert_by(
                    &mut self.lobby_game_server_lookups,
                    snapshot.clone(),
                    |existing| existing.lobby == *lobby,
                );
                self.last_lobby_game_server = Some(snapshot);
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
