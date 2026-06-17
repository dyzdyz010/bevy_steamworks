use bevy_ecs::prelude::Resource;

use super::*;

mod accessors;
mod operations;

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
