use bevy_ecs::prelude::Resource;

use crate::cache::trim_oldest;

use super::*;

mod accessors;
mod operations;

pub(in crate::matchmaking) const STEAMWORKS_MATCHMAKING_STATE_CACHE_LIMIT: usize = 1_024;

/// Runtime state for [`crate::SteamworksMatchmakingPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksMatchmakingState {
    last_error: Option<SteamworksMatchmakingError>,
    last_lobby_list_request: Option<SteamworksLobbyListRequest>,
    lobby_list_requests: Vec<SteamworksLobbyListRequest>,
    last_lobby_list: Vec<steamworks::LobbyId>,
    lobby_list_results: Vec<SteamworksLobbyListResult>,
    last_lobby_create_request: Option<SteamworksLobbyCreateRequest>,
    lobby_create_requests: Vec<SteamworksLobbyCreateRequest>,
    last_lobby_join_request: Option<SteamworksMatchmakingLobbyJoinRequest>,
    lobby_join_requests: Vec<SteamworksMatchmakingLobbyJoinRequest>,
    last_created_lobby: Option<SteamworksLobbyCreated>,
    created_lobbies: Vec<SteamworksLobbyCreated>,
    last_joined_lobby: Option<SteamworksLobbyJoined>,
    joined_lobby_results: Vec<SteamworksLobbyJoined>,
    last_left_lobby: Option<steamworks::LobbyId>,
    joined_lobbies: Vec<steamworks::LobbyId>,
    last_lobby_data_count: Option<SteamworksLobbyDataCount>,
    lobby_data_counts: Vec<SteamworksLobbyDataCount>,
    last_lobby_data: Option<SteamworksLobbyDataValue>,
    lobby_data_values: Vec<SteamworksLobbyDataValue>,
    last_lobby_data_entry: Option<SteamworksLobbyDataEntry>,
    lobby_data_entries: Vec<SteamworksLobbyDataEntry>,
    last_all_lobby_data: Option<SteamworksLobbyDataEntries>,
    all_lobby_data: Vec<SteamworksLobbyDataEntries>,
    last_lobby_data_set: Option<SteamworksLobbyDataMutation>,
    lobby_data_sets: Vec<SteamworksLobbyDataMutation>,
    last_lobby_data_deleted: Option<SteamworksLobbyDataMutation>,
    lobby_data_deletions: Vec<SteamworksLobbyDataMutation>,
    last_lobby_member_data_set: Option<SteamworksLobbyDataMutation>,
    lobby_member_data_sets: Vec<SteamworksLobbyDataMutation>,
    last_lobby_member_data: Option<SteamworksLobbyMemberDataValue>,
    lobby_member_data_values: Vec<SteamworksLobbyMemberDataValue>,
    last_lobby_member_limit: Option<SteamworksLobbyMemberLimit>,
    lobby_member_limits: Vec<SteamworksLobbyMemberLimit>,
    last_lobby_owner: Option<SteamworksLobbyOwner>,
    lobby_owners: Vec<SteamworksLobbyOwner>,
    last_lobby_member_count: Option<SteamworksLobbyMemberCount>,
    lobby_member_counts: Vec<SteamworksLobbyMemberCount>,
    last_lobby_members: Option<SteamworksLobbyMembers>,
    lobby_member_lists: Vec<SteamworksLobbyMembers>,
    last_lobby_joinability: Option<SteamworksLobbyJoinability>,
    lobby_joinabilities: Vec<SteamworksLobbyJoinability>,
    last_lobby_chat_message_sent: Option<SteamworksLobbyChatMessageSent>,
    lobby_chat_message_sends: Vec<SteamworksLobbyChatMessageSent>,
    last_lobby_chat_entry: Option<SteamworksLobbyChatEntry>,
    lobby_chat_entries: Vec<SteamworksLobbyChatEntry>,
    last_lobby_game_server_set: Option<SteamworksLobbyGameServerAssignment>,
    lobby_game_server_assignments: Vec<SteamworksLobbyGameServerAssignment>,
    last_lobby_game_server: Option<SteamworksLobbyGameServerLookup>,
    lobby_game_server_lookups: Vec<SteamworksLobbyGameServerLookup>,
    last_lobby_created_callback: Option<SteamworksLobbyCreatedCallback>,
    last_lobby_enter_callback: Option<SteamworksLobbyEnterCallback>,
    last_lobby_chat_message: Option<SteamworksLobbyChatMessage>,
    last_lobby_chat_update: Option<SteamworksLobbyChatUpdate>,
    last_lobby_data_update: Option<SteamworksLobbyDataUpdate>,
    next_request_id: u64,
}

pub(super) fn upsert_by<T>(
    items: &mut Vec<T>,
    item: T,
    mut matches_existing: impl FnMut(&T) -> bool,
) {
    if let Some(existing) = items.iter_mut().find(|existing| matches_existing(existing)) {
        *existing = item;
    } else {
        items.push(item);
        trim_oldest(items, STEAMWORKS_MATCHMAKING_STATE_CACHE_LIMIT);
    }
}
