//! High-level Bevy ECS integration for Steam matchmaking and lobbies.
//!
//! This module builds on top of the upstream [`steamworks::Matchmaking`] API.
//! It keeps async Steam call results and lobby callbacks flowing through Bevy
//! messages, while avoiding blocking work in the frame loop.

use std::{
    net::SocketAddrV4,
    sync::{Arc, Mutex},
};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

const MAX_LOBBY_MEMBERS: u32 = 250;
const MAX_LOBBY_CHAT_MESSAGE_BYTES: usize = 4096;
const MAX_LOBBY_LIST_RESULTS: u64 = i32::MAX as u64;

/// Bevy plugin for high-level Steam matchmaking and lobby commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksMatchmakingCommand`] and [`SteamworksMatchmakingResult`]
/// messages and runs its command processor in [`bevy_app::First`] after Steam
/// callbacks. It also mirrors lobby callbacks into matchmaking results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksMatchmakingPlugin;

impl SteamworksMatchmakingPlugin {
    /// Creates a matchmaking plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksMatchmakingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksMatchmakingState>()
            .init_resource::<SteamworksMatchmakingAsyncResults>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksMatchmakingCommand>()
            .add_message::<SteamworksMatchmakingResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessMatchmakingCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_matchmaking_commands.in_set(SteamworksSystem::ProcessMatchmakingCommands),
            );
    }
}

/// Runtime state for [`SteamworksMatchmakingPlugin`].
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

    fn record_error(&mut self, error: SteamworksMatchmakingError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksMatchmakingOperation) {
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

    fn next_request_id(&mut self) -> u64 {
        let request_id = self.next_request_id;
        self.next_request_id = self.next_request_id.wrapping_add(1);
        request_id
    }
}

#[derive(Clone, Debug, Default, Resource)]
struct SteamworksMatchmakingAsyncResults {
    queue: Arc<Mutex<Vec<SteamworksMatchmakingResult>>>,
}

impl SteamworksMatchmakingAsyncResults {
    fn push(&self, result: SteamworksMatchmakingResult) {
        self.queue
            .lock()
            .expect("Steamworks matchmaking async result mutex was poisoned")
            .push(result);
    }

    fn drain(&self) -> Vec<SteamworksMatchmakingResult> {
        self.queue
            .lock()
            .expect("Steamworks matchmaking async result mutex was poisoned")
            .drain(..)
            .collect()
    }
}

/// Owned lobby-list filters for [`SteamworksMatchmakingCommand::RequestLobbyList`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SteamworksLobbyListFilter {
    /// String lobby metadata filters.
    pub string: Vec<SteamworksLobbyStringFilter>,
    /// Numeric lobby metadata filters.
    pub number: Vec<SteamworksLobbyNumberFilter>,
    /// Near-value sort filters.
    pub near_value: Vec<SteamworksLobbyNearFilter>,
    /// Minimum available open slots.
    pub open_slots: Option<u8>,
    /// Distance bucket used by Steam's lobby search.
    pub distance: Option<steamworks::DistanceFilter>,
    /// Maximum number of lobby results to return.
    pub max_results: Option<u64>,
}

impl SteamworksLobbyListFilter {
    /// Creates an empty lobby-list filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a string metadata filter.
    pub fn with_string(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
        comparison: steamworks::StringFilterKind,
    ) -> Self {
        self.string.push(SteamworksLobbyStringFilter {
            key: key.into(),
            value: value.into(),
            comparison,
        });
        self
    }

    /// Adds a numeric metadata filter.
    pub fn with_number(
        mut self,
        key: impl Into<String>,
        value: i32,
        comparison: steamworks::ComparisonFilter,
    ) -> Self {
        self.number.push(SteamworksLobbyNumberFilter {
            key: key.into(),
            value,
            comparison,
        });
        self
    }

    /// Adds a near-value sort filter.
    pub fn with_near_value(mut self, key: impl Into<String>, value: i32) -> Self {
        self.near_value.push(SteamworksLobbyNearFilter {
            key: key.into(),
            value,
        });
        self
    }

    /// Sets the minimum available open slots.
    pub fn with_open_slots(mut self, open_slots: u8) -> Self {
        self.open_slots = Some(open_slots);
        self
    }

    /// Sets the Steam lobby search distance.
    pub fn with_distance(mut self, distance: steamworks::DistanceFilter) -> Self {
        self.distance = Some(distance);
        self
    }

    /// Sets the maximum number of lobby results.
    pub fn with_max_results(mut self, max_results: u64) -> Self {
        self.max_results = Some(max_results);
        self
    }
}

/// A string lobby metadata filter.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyStringFilter {
    /// Lobby metadata key.
    pub key: String,
    /// Lobby metadata value.
    pub value: String,
    /// String comparison mode.
    pub comparison: steamworks::StringFilterKind,
}

/// A numeric lobby metadata filter.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyNumberFilter {
    /// Lobby metadata key.
    pub key: String,
    /// Numeric comparison value.
    pub value: i32,
    /// Numeric comparison mode.
    pub comparison: steamworks::ComparisonFilter,
}

/// A near-value lobby sort filter.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyNearFilter {
    /// Lobby metadata key.
    pub key: String,
    /// Value used for proximity sorting.
    pub value: i32,
}

/// Game-server data associated with a Steam lobby.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyGameServer {
    /// Server IPv4 address and port.
    pub address: SocketAddrV4,
    /// Optional Steam ID for the game server.
    pub steam_id: Option<steamworks::SteamId>,
}

/// Lobby created callback snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyCreatedCallback {
    /// Raw Steam result code reported by the upstream callback.
    pub result: u32,
    /// Lobby created by Steam, or zero when creation failed.
    pub lobby: steamworks::LobbyId,
}

/// Lobby enter callback snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyEnterCallback {
    /// Lobby entered by the local user.
    pub lobby: steamworks::LobbyId,
    /// Raw chat permissions reported by Steam.
    pub chat_permissions: u32,
    /// Whether Steam reported the lobby as locked.
    pub blocked: bool,
    /// Steam lobby enter response.
    pub chat_room_enter_response: steamworks::ChatRoomEnterResponse,
}

/// Lobby chat message callback snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyChatMessage {
    /// Lobby that received the chat entry.
    pub lobby: steamworks::LobbyId,
    /// User who sent the message.
    pub user: steamworks::SteamId,
    /// Chat entry kind reported by Steam.
    pub chat_entry_type: steamworks::ChatEntryType,
    /// Chat entry index reported by Steam.
    pub chat_id: i32,
}

/// Lobby member state change callback snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyChatUpdate {
    /// Lobby where the membership change occurred.
    pub lobby: steamworks::LobbyId,
    /// User whose lobby state changed.
    pub user_changed: steamworks::SteamId,
    /// User who caused the change.
    pub making_change: steamworks::SteamId,
    /// Member state change reported by Steam.
    pub member_state_change: steamworks::ChatMemberStateChange,
}

/// Lobby metadata update callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyDataUpdate {
    /// Lobby whose metadata changed.
    pub lobby: steamworks::LobbyId,
    /// Lobby member whose data changed, or the lobby ID when room data changed.
    pub member: steamworks::SteamId,
    /// Whether Steam reported the metadata update as successful.
    pub success: bool,
}

/// Submitted lobby-list request context.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyListRequest {
    /// Unique request ID assigned by the plugin.
    pub request_id: u64,
    /// Filters applied to the request.
    pub filter: SteamworksLobbyListFilter,
}

/// Submitted lobby-create request context.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyCreateRequest {
    /// Unique request ID assigned by the plugin.
    pub request_id: u64,
    /// Lobby visibility requested.
    pub lobby_type: steamworks::LobbyType,
    /// Maximum members requested.
    pub max_members: u32,
}

/// Submitted lobby-join request context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksMatchmakingLobbyJoinRequest {
    /// Unique request ID assigned by the plugin.
    pub request_id: u64,
    /// Lobby requested by the command.
    pub lobby: steamworks::LobbyId,
}

/// Completed lobby creation context.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksLobbyCreated {
    /// Unique request ID assigned by the plugin.
    pub request_id: u64,
    /// Lobby visibility requested.
    pub lobby_type: steamworks::LobbyType,
    /// Maximum members requested.
    pub max_members: u32,
    /// Created lobby.
    pub lobby: steamworks::LobbyId,
}

/// Completed lobby join context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyJoined {
    /// Unique request ID assigned by the plugin.
    pub request_id: u64,
    /// Lobby requested by the command.
    pub requested_lobby: steamworks::LobbyId,
    /// Lobby joined according to Steam.
    pub lobby: steamworks::LobbyId,
}

/// Lobby metadata count snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyDataCount {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Metadata entry count.
    pub count: u32,
}

/// Lobby metadata value snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyDataValue {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Metadata key.
    pub key: String,
    /// Metadata value, if Steam had one.
    pub value: Option<String>,
}

/// Lobby metadata entry snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyDataEntry {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Metadata entry index.
    pub index: u32,
    /// Metadata key/value pair, if Steam had one.
    pub entry: Option<(String, String)>,
}

/// Full lobby metadata snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyDataEntries {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Metadata key/value pairs.
    pub entries: Vec<(String, String)>,
}

/// Lobby metadata mutation snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyDataMutation {
    /// Lobby mutated.
    pub lobby: steamworks::LobbyId,
    /// Metadata key.
    pub key: String,
}

/// Lobby member metadata value snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyMemberDataValue {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Member inspected.
    pub user: steamworks::SteamId,
    /// Metadata key.
    pub key: String,
    /// Metadata value, if Steam had one.
    pub value: Option<String>,
}

/// Lobby member limit snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyMemberLimit {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Member limit, if known.
    pub limit: Option<usize>,
}

/// Lobby owner snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyOwner {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Owner Steam ID.
    pub owner: steamworks::SteamId,
}

/// Lobby member count snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyMemberCount {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Member count.
    pub count: usize,
}

/// Lobby member list snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyMembers {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Member Steam IDs.
    pub members: Vec<steamworks::SteamId>,
}

/// Lobby joinability mutation snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyJoinability {
    /// Lobby mutated.
    pub lobby: steamworks::LobbyId,
    /// Joinable value submitted.
    pub joinable: bool,
}

/// Lobby chat send snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyChatMessageSent {
    /// Lobby sent into.
    pub lobby: steamworks::LobbyId,
    /// Message length in bytes.
    pub len: usize,
}

/// Lobby chat entry bytes snapshot.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksLobbyChatEntry {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Chat entry index.
    pub chat_id: i32,
    /// Message bytes read from Steam.
    pub data: Vec<u8>,
}

impl std::fmt::Debug for SteamworksLobbyChatEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SteamworksLobbyChatEntry")
            .field("lobby", &self.lobby)
            .field("chat_id", &self.chat_id)
            .field("data_len", &self.data.len())
            .finish()
    }
}

/// Lobby game-server assignment snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyGameServerAssignment {
    /// Lobby mutated.
    pub lobby: steamworks::LobbyId,
    /// Game-server data submitted.
    pub server: SteamworksLobbyGameServer,
}

/// Lobby game-server lookup snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyGameServerLookup {
    /// Lobby inspected.
    pub lobby: steamworks::LobbyId,
    /// Game-server data, if Steam had one.
    pub server: Option<SteamworksLobbyGameServer>,
}

/// A high-level command for Steam matchmaking and lobbies.
#[derive(Clone, Message, PartialEq)]
pub enum SteamworksMatchmakingCommand {
    /// Request a lobby list from Steam.
    RequestLobbyList {
        /// Owned filters to apply before requesting the lobby list.
        filter: SteamworksLobbyListFilter,
    },
    /// Create a lobby.
    ///
    /// The async command result emits
    /// [`SteamworksMatchmakingOperation::LobbyCreated`] with a request ID. Steam
    /// may also emit [`SteamworksMatchmakingOperation::LobbyCreateCallbackReceived`]
    /// and [`SteamworksMatchmakingOperation::LobbyEnterCallbackReceived`] as
    /// callback observations.
    CreateLobby {
        /// Lobby visibility.
        lobby_type: steamworks::LobbyType,
        /// Maximum lobby members. Steam supports at most 250.
        max_members: u32,
    },
    /// Join a lobby.
    ///
    /// The async command result emits [`SteamworksMatchmakingOperation::LobbyJoined`]
    /// with a request ID. Steam may also emit
    /// [`SteamworksMatchmakingOperation::LobbyEnterCallbackReceived`] as a
    /// callback observation.
    JoinLobby {
        /// Lobby to join.
        lobby: steamworks::LobbyId,
    },
    /// Leave a lobby.
    LeaveLobby {
        /// Lobby to leave.
        lobby: steamworks::LobbyId,
    },
    /// Read the number of lobby metadata entries.
    GetLobbyDataCount {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
    },
    /// Read one lobby metadata value.
    GetLobbyData {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
        /// Metadata key.
        key: String,
    },
    /// Read one lobby metadata entry by index.
    GetLobbyDataByIndex {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
        /// Metadata entry index.
        index: u32,
    },
    /// Read all lobby metadata entries currently cached by Steam.
    GetAllLobbyData {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
    },
    /// Set one lobby metadata value.
    SetLobbyData {
        /// Lobby to mutate.
        lobby: steamworks::LobbyId,
        /// Metadata key.
        key: String,
        /// Metadata value.
        value: String,
    },
    /// Delete one lobby metadata value.
    DeleteLobbyData {
        /// Lobby to mutate.
        lobby: steamworks::LobbyId,
        /// Metadata key.
        key: String,
    },
    /// Set local-user metadata inside a lobby.
    SetLobbyMemberData {
        /// Lobby to mutate.
        lobby: steamworks::LobbyId,
        /// Metadata key.
        key: String,
        /// Metadata value.
        value: String,
    },
    /// Read one member metadata value.
    GetLobbyMemberData {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
        /// Member to inspect.
        user: steamworks::SteamId,
        /// Metadata key.
        key: String,
    },
    /// Read a lobby's member limit.
    GetLobbyMemberLimit {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
    },
    /// Read a lobby's owner.
    GetLobbyOwner {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
    },
    /// Read a lobby's member count.
    GetLobbyMemberCount {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
    },
    /// Read all currently known lobby members.
    ListLobbyMembers {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
    },
    /// Set whether a lobby is joinable.
    SetLobbyJoinable {
        /// Lobby to mutate.
        lobby: steamworks::LobbyId,
        /// Whether the lobby should be joinable.
        joinable: bool,
    },
    /// Send a lobby chat message.
    SendLobbyChatMessage {
        /// Lobby to send into.
        lobby: steamworks::LobbyId,
        /// Message bytes. Steam supports up to 4096 bytes.
        data: Vec<u8>,
    },
    /// Read the bytes for a lobby chat entry.
    ///
    /// Steam treats chat entry IDs as callback-scope values. Prefer a lower-level
    /// callback registered through [`crate::SteamworksCallbackRegistry`] when
    /// bytes must be copied immediately and reliably. This command is retained
    /// for callers that know the entry is still available through Steam's lobby
    /// cache.
    GetLobbyChatEntry {
        /// Lobby that received the chat entry.
        lobby: steamworks::LobbyId,
        /// Chat entry index from the Steam callback.
        chat_id: i32,
        /// Maximum bytes to read, up to 4096.
        max_bytes: usize,
    },
    /// Set game-server information for a lobby.
    SetLobbyGameServer {
        /// Lobby to mutate.
        lobby: steamworks::LobbyId,
        /// Server IPv4 address and port.
        address: SocketAddrV4,
        /// Optional Steam ID for the game server.
        steam_id: Option<steamworks::SteamId>,
    },
    /// Read game-server information for a lobby.
    GetLobbyGameServer {
        /// Lobby to inspect.
        lobby: steamworks::LobbyId,
    },
}

impl std::fmt::Debug for SteamworksMatchmakingCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestLobbyList { filter } => f
                .debug_struct("RequestLobbyList")
                .field("filter", filter)
                .finish(),
            Self::CreateLobby {
                lobby_type,
                max_members,
            } => f
                .debug_struct("CreateLobby")
                .field("lobby_type", lobby_type)
                .field("max_members", max_members)
                .finish(),
            Self::JoinLobby { lobby } => f.debug_struct("JoinLobby").field("lobby", lobby).finish(),
            Self::LeaveLobby { lobby } => {
                f.debug_struct("LeaveLobby").field("lobby", lobby).finish()
            }
            Self::GetLobbyDataCount { lobby } => f
                .debug_struct("GetLobbyDataCount")
                .field("lobby", lobby)
                .finish(),
            Self::GetLobbyData { lobby, key } => f
                .debug_struct("GetLobbyData")
                .field("lobby", lobby)
                .field("key", key)
                .finish(),
            Self::GetLobbyDataByIndex { lobby, index } => f
                .debug_struct("GetLobbyDataByIndex")
                .field("lobby", lobby)
                .field("index", index)
                .finish(),
            Self::GetAllLobbyData { lobby } => f
                .debug_struct("GetAllLobbyData")
                .field("lobby", lobby)
                .finish(),
            Self::SetLobbyData { lobby, key, value } => f
                .debug_struct("SetLobbyData")
                .field("lobby", lobby)
                .field("key", key)
                .field("value", value)
                .finish(),
            Self::DeleteLobbyData { lobby, key } => f
                .debug_struct("DeleteLobbyData")
                .field("lobby", lobby)
                .field("key", key)
                .finish(),
            Self::SetLobbyMemberData { lobby, key, value } => f
                .debug_struct("SetLobbyMemberData")
                .field("lobby", lobby)
                .field("key", key)
                .field("value", value)
                .finish(),
            Self::GetLobbyMemberData { lobby, user, key } => f
                .debug_struct("GetLobbyMemberData")
                .field("lobby", lobby)
                .field("user", user)
                .field("key", key)
                .finish(),
            Self::GetLobbyMemberLimit { lobby } => f
                .debug_struct("GetLobbyMemberLimit")
                .field("lobby", lobby)
                .finish(),
            Self::GetLobbyOwner { lobby } => f
                .debug_struct("GetLobbyOwner")
                .field("lobby", lobby)
                .finish(),
            Self::GetLobbyMemberCount { lobby } => f
                .debug_struct("GetLobbyMemberCount")
                .field("lobby", lobby)
                .finish(),
            Self::ListLobbyMembers { lobby } => f
                .debug_struct("ListLobbyMembers")
                .field("lobby", lobby)
                .finish(),
            Self::SetLobbyJoinable { lobby, joinable } => f
                .debug_struct("SetLobbyJoinable")
                .field("lobby", lobby)
                .field("joinable", joinable)
                .finish(),
            Self::SendLobbyChatMessage { lobby, data } => f
                .debug_struct("SendLobbyChatMessage")
                .field("lobby", lobby)
                .field("data_len", &data.len())
                .finish(),
            Self::GetLobbyChatEntry {
                lobby,
                chat_id,
                max_bytes,
            } => f
                .debug_struct("GetLobbyChatEntry")
                .field("lobby", lobby)
                .field("chat_id", chat_id)
                .field("max_bytes", max_bytes)
                .finish(),
            Self::SetLobbyGameServer {
                lobby,
                address,
                steam_id,
            } => f
                .debug_struct("SetLobbyGameServer")
                .field("lobby", lobby)
                .field("address", address)
                .field("steam_id", steam_id)
                .finish(),
            Self::GetLobbyGameServer { lobby } => f
                .debug_struct("GetLobbyGameServer")
                .field("lobby", lobby)
                .finish(),
        }
    }
}

impl SteamworksMatchmakingCommand {
    /// Creates a [`SteamworksMatchmakingCommand::RequestLobbyList`] command.
    pub fn request_lobby_list(filter: SteamworksLobbyListFilter) -> Self {
        Self::RequestLobbyList { filter }
    }

    /// Creates a [`SteamworksMatchmakingCommand::CreateLobby`] command.
    pub fn create_lobby(lobby_type: steamworks::LobbyType, max_members: u32) -> Self {
        Self::CreateLobby {
            lobby_type,
            max_members,
        }
    }

    /// Creates a [`SteamworksMatchmakingCommand::JoinLobby`] command.
    pub fn join_lobby(lobby: steamworks::LobbyId) -> Self {
        Self::JoinLobby { lobby }
    }

    /// Creates a [`SteamworksMatchmakingCommand::LeaveLobby`] command.
    pub fn leave_lobby(lobby: steamworks::LobbyId) -> Self {
        Self::LeaveLobby { lobby }
    }

    /// Creates a [`SteamworksMatchmakingCommand::GetLobbyData`] command.
    pub fn get_lobby_data(lobby: steamworks::LobbyId, key: impl Into<String>) -> Self {
        Self::GetLobbyData {
            lobby,
            key: key.into(),
        }
    }

    /// Creates a [`SteamworksMatchmakingCommand::SetLobbyData`] command.
    pub fn set_lobby_data(
        lobby: steamworks::LobbyId,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self::SetLobbyData {
            lobby,
            key: key.into(),
            value: value.into(),
        }
    }

    /// Creates a [`SteamworksMatchmakingCommand::DeleteLobbyData`] command.
    pub fn delete_lobby_data(lobby: steamworks::LobbyId, key: impl Into<String>) -> Self {
        Self::DeleteLobbyData {
            lobby,
            key: key.into(),
        }
    }

    /// Creates a [`SteamworksMatchmakingCommand::SendLobbyChatMessage`] command.
    pub fn send_lobby_chat_message(lobby: steamworks::LobbyId, data: impl Into<Vec<u8>>) -> Self {
        Self::SendLobbyChatMessage {
            lobby,
            data: data.into(),
        }
    }
}

/// A successfully submitted Steam matchmaking operation or synchronous read.
#[derive(Clone, PartialEq)]
pub enum SteamworksMatchmakingOperation {
    /// Lobby list request was submitted.
    LobbyListRequested {
        /// Unique request ID assigned by the plugin.
        request_id: u64,
        /// Filters applied to the request.
        filter: SteamworksLobbyListFilter,
    },
    /// Lobby list request completed.
    LobbyListReceived {
        /// Unique request ID assigned by the plugin.
        request_id: u64,
        /// Filters applied to the request.
        filter: SteamworksLobbyListFilter,
        /// Matching lobby IDs.
        lobbies: Vec<steamworks::LobbyId>,
    },
    /// Lobby creation was submitted.
    LobbyCreateRequested {
        /// Unique request ID assigned by the plugin.
        request_id: u64,
        /// Lobby visibility.
        lobby_type: steamworks::LobbyType,
        /// Maximum members requested.
        max_members: u32,
    },
    /// Lobby creation completed.
    LobbyCreated {
        /// Unique request ID assigned by the plugin.
        request_id: u64,
        /// Lobby visibility requested.
        lobby_type: steamworks::LobbyType,
        /// Maximum members requested.
        max_members: u32,
        /// Created lobby.
        lobby: steamworks::LobbyId,
    },
    /// Lobby join was submitted.
    LobbyJoinRequested {
        /// Unique request ID assigned by the plugin.
        request_id: u64,
        /// Lobby requested.
        lobby: steamworks::LobbyId,
    },
    /// Lobby join completed.
    LobbyJoined {
        /// Unique request ID assigned by the plugin.
        request_id: u64,
        /// Lobby requested by the command.
        requested_lobby: steamworks::LobbyId,
        /// Joined lobby.
        lobby: steamworks::LobbyId,
    },
    /// Lobby was left.
    LobbyLeft {
        /// Left lobby.
        lobby: steamworks::LobbyId,
    },
    /// Lobby metadata count was read.
    LobbyDataCountRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Metadata entry count.
        count: u32,
    },
    /// Lobby metadata value was read.
    LobbyDataRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Metadata key.
        key: String,
        /// Metadata value, if Steam had one.
        value: Option<String>,
    },
    /// Lobby metadata entry was read by index.
    LobbyDataByIndexRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Metadata entry index.
        index: u32,
        /// Metadata key/value pair, if Steam had one.
        entry: Option<(String, String)>,
    },
    /// All currently cached lobby metadata was read.
    AllLobbyDataRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Metadata key/value pairs.
        entries: Vec<(String, String)>,
    },
    /// Lobby metadata was set.
    LobbyDataSet {
        /// Lobby mutated.
        lobby: steamworks::LobbyId,
        /// Metadata key.
        key: String,
    },
    /// Lobby metadata was deleted.
    LobbyDataDeleted {
        /// Lobby mutated.
        lobby: steamworks::LobbyId,
        /// Metadata key.
        key: String,
    },
    /// Local-user lobby metadata was set.
    LobbyMemberDataSet {
        /// Lobby mutated.
        lobby: steamworks::LobbyId,
        /// Metadata key.
        key: String,
    },
    /// Member metadata was read.
    LobbyMemberDataRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Member inspected.
        user: steamworks::SteamId,
        /// Metadata key.
        key: String,
        /// Metadata value, if Steam had one.
        value: Option<String>,
    },
    /// Lobby member limit was read.
    LobbyMemberLimitRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Member limit, if known.
        limit: Option<usize>,
    },
    /// Lobby owner was read.
    LobbyOwnerRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Owner Steam ID.
        owner: steamworks::SteamId,
    },
    /// Lobby member count was read.
    LobbyMemberCountRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Member count.
        count: usize,
    },
    /// Lobby members were read.
    LobbyMembersListed {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Member Steam IDs.
        members: Vec<steamworks::SteamId>,
    },
    /// Lobby joinability was set.
    LobbyJoinableSet {
        /// Lobby mutated.
        lobby: steamworks::LobbyId,
        /// Joinable value submitted.
        joinable: bool,
    },
    /// Lobby chat message was sent.
    LobbyChatMessageSent {
        /// Lobby sent into.
        lobby: steamworks::LobbyId,
        /// Message length in bytes.
        len: usize,
    },
    /// Lobby chat entry bytes were read.
    LobbyChatEntryRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Chat entry index.
        chat_id: i32,
        /// Message bytes read from Steam.
        data: Vec<u8>,
    },
    /// Lobby game-server data was set.
    LobbyGameServerSet {
        /// Lobby mutated.
        lobby: steamworks::LobbyId,
        /// Game-server data submitted.
        server: SteamworksLobbyGameServer,
    },
    /// Lobby game-server data was read.
    LobbyGameServerRead {
        /// Lobby inspected.
        lobby: steamworks::LobbyId,
        /// Game-server data, if Steam had one.
        server: Option<SteamworksLobbyGameServer>,
    },
    /// A lobby created callback was observed.
    LobbyCreateCallbackReceived {
        /// Callback snapshot.
        callback: SteamworksLobbyCreatedCallback,
    },
    /// A lobby enter callback was observed.
    LobbyEnterCallbackReceived {
        /// Callback snapshot.
        callback: SteamworksLobbyEnterCallback,
    },
    /// A lobby chat message callback was observed.
    LobbyChatMessageReceived {
        /// Callback snapshot.
        message: SteamworksLobbyChatMessage,
    },
    /// A lobby membership change callback was observed.
    LobbyChatUpdateReceived {
        /// Callback snapshot.
        update: SteamworksLobbyChatUpdate,
    },
    /// A lobby metadata update callback was observed.
    LobbyDataUpdateReceived {
        /// Callback snapshot.
        update: SteamworksLobbyDataUpdate,
    },
}

impl std::fmt::Debug for SteamworksMatchmakingOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LobbyListRequested { request_id, filter } => f
                .debug_struct("LobbyListRequested")
                .field("request_id", request_id)
                .field("filter", filter)
                .finish(),
            Self::LobbyListReceived {
                request_id,
                filter,
                lobbies,
            } => f
                .debug_struct("LobbyListReceived")
                .field("request_id", request_id)
                .field("filter", filter)
                .field("lobbies", lobbies)
                .finish(),
            Self::LobbyCreateRequested {
                request_id,
                lobby_type,
                max_members,
            } => f
                .debug_struct("LobbyCreateRequested")
                .field("request_id", request_id)
                .field("lobby_type", lobby_type)
                .field("max_members", max_members)
                .finish(),
            Self::LobbyCreated {
                request_id,
                lobby_type,
                max_members,
                lobby,
            } => f
                .debug_struct("LobbyCreated")
                .field("request_id", request_id)
                .field("lobby_type", lobby_type)
                .field("max_members", max_members)
                .field("lobby", lobby)
                .finish(),
            Self::LobbyJoinRequested { request_id, lobby } => f
                .debug_struct("LobbyJoinRequested")
                .field("request_id", request_id)
                .field("lobby", lobby)
                .finish(),
            Self::LobbyJoined {
                request_id,
                requested_lobby,
                lobby,
            } => f
                .debug_struct("LobbyJoined")
                .field("request_id", request_id)
                .field("requested_lobby", requested_lobby)
                .field("lobby", lobby)
                .finish(),
            Self::LobbyLeft { lobby } => f.debug_struct("LobbyLeft").field("lobby", lobby).finish(),
            Self::LobbyDataCountRead { lobby, count } => f
                .debug_struct("LobbyDataCountRead")
                .field("lobby", lobby)
                .field("count", count)
                .finish(),
            Self::LobbyDataRead { lobby, key, value } => f
                .debug_struct("LobbyDataRead")
                .field("lobby", lobby)
                .field("key", key)
                .field("value", value)
                .finish(),
            Self::LobbyDataByIndexRead {
                lobby,
                index,
                entry,
            } => f
                .debug_struct("LobbyDataByIndexRead")
                .field("lobby", lobby)
                .field("index", index)
                .field("entry", entry)
                .finish(),
            Self::AllLobbyDataRead { lobby, entries } => f
                .debug_struct("AllLobbyDataRead")
                .field("lobby", lobby)
                .field("entries", entries)
                .finish(),
            Self::LobbyDataSet { lobby, key } => f
                .debug_struct("LobbyDataSet")
                .field("lobby", lobby)
                .field("key", key)
                .finish(),
            Self::LobbyDataDeleted { lobby, key } => f
                .debug_struct("LobbyDataDeleted")
                .field("lobby", lobby)
                .field("key", key)
                .finish(),
            Self::LobbyMemberDataSet { lobby, key } => f
                .debug_struct("LobbyMemberDataSet")
                .field("lobby", lobby)
                .field("key", key)
                .finish(),
            Self::LobbyMemberDataRead {
                lobby,
                user,
                key,
                value,
            } => f
                .debug_struct("LobbyMemberDataRead")
                .field("lobby", lobby)
                .field("user", user)
                .field("key", key)
                .field("value", value)
                .finish(),
            Self::LobbyMemberLimitRead { lobby, limit } => f
                .debug_struct("LobbyMemberLimitRead")
                .field("lobby", lobby)
                .field("limit", limit)
                .finish(),
            Self::LobbyOwnerRead { lobby, owner } => f
                .debug_struct("LobbyOwnerRead")
                .field("lobby", lobby)
                .field("owner", owner)
                .finish(),
            Self::LobbyMemberCountRead { lobby, count } => f
                .debug_struct("LobbyMemberCountRead")
                .field("lobby", lobby)
                .field("count", count)
                .finish(),
            Self::LobbyMembersListed { lobby, members } => f
                .debug_struct("LobbyMembersListed")
                .field("lobby", lobby)
                .field("members", members)
                .finish(),
            Self::LobbyJoinableSet { lobby, joinable } => f
                .debug_struct("LobbyJoinableSet")
                .field("lobby", lobby)
                .field("joinable", joinable)
                .finish(),
            Self::LobbyChatMessageSent { lobby, len } => f
                .debug_struct("LobbyChatMessageSent")
                .field("lobby", lobby)
                .field("len", len)
                .finish(),
            Self::LobbyChatEntryRead {
                lobby,
                chat_id,
                data,
            } => f
                .debug_struct("LobbyChatEntryRead")
                .field("lobby", lobby)
                .field("chat_id", chat_id)
                .field("data_len", &data.len())
                .finish(),
            Self::LobbyGameServerSet { lobby, server } => f
                .debug_struct("LobbyGameServerSet")
                .field("lobby", lobby)
                .field("server", server)
                .finish(),
            Self::LobbyGameServerRead { lobby, server } => f
                .debug_struct("LobbyGameServerRead")
                .field("lobby", lobby)
                .field("server", server)
                .finish(),
            Self::LobbyCreateCallbackReceived { callback } => f
                .debug_struct("LobbyCreateCallbackReceived")
                .field("callback", callback)
                .finish(),
            Self::LobbyEnterCallbackReceived { callback } => f
                .debug_struct("LobbyEnterCallbackReceived")
                .field("callback", callback)
                .finish(),
            Self::LobbyChatMessageReceived { message } => f
                .debug_struct("LobbyChatMessageReceived")
                .field("message", message)
                .finish(),
            Self::LobbyChatUpdateReceived { update } => f
                .debug_struct("LobbyChatUpdateReceived")
                .field("update", update)
                .finish(),
            Self::LobbyDataUpdateReceived { update } => f
                .debug_struct("LobbyDataUpdateReceived")
                .field("update", update)
                .finish(),
        }
    }
}

/// Result message emitted by [`SteamworksMatchmakingPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksMatchmakingResult {
    /// The command, async call result, or observed callback was processed successfully.
    Ok(SteamworksMatchmakingOperation),
    /// The command failed synchronously or through a Steam async call result.
    Err {
        /// Command that failed.
        command: SteamworksMatchmakingCommand,
        /// Failure reason.
        error: SteamworksMatchmakingError,
    },
}

/// Synchronous and async errors from [`SteamworksMatchmakingPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksMatchmakingError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks matchmaking command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A lobby metadata key is longer than Steam supports.
    #[error("Steamworks lobby key is too long: {key}")]
    LobbyKeyTooLong {
        /// Key rejected by the upstream Steamworks API wrapper.
        key: String,
    },
    /// A lobby creation request exceeded Steam's member limit.
    #[error("Steamworks lobbies support at most {max_supported} members, got {requested}")]
    MaxLobbyMembersExceeded {
        /// Requested member count.
        requested: u32,
        /// Maximum supported member count.
        max_supported: u32,
    },
    /// A lobby list result count exceeded the upstream Steam API wrapper's safe range.
    #[error("Steamworks lobby list result count must be <= {max_supported}, got {requested}")]
    MaxLobbyListResultsExceeded {
        /// Requested result count.
        requested: u64,
        /// Maximum supported result count before upstream integer truncation.
        max_supported: u64,
    },
    /// A lobby chat message length is outside Steam's supported range.
    #[error("Steamworks lobby chat messages must be 1..={max_supported} bytes, got {requested}")]
    InvalidChatMessageLength {
        /// Requested message length.
        requested: usize,
        /// Maximum supported message length.
        max_supported: usize,
    },
    /// The upstream Steamworks API rejected the operation.
    #[error("Steamworks matchmaking operation failed: {operation}")]
    OperationFailed {
        /// Operation that failed.
        operation: &'static str,
    },
    /// The upstream Steamworks API returned an explicit Steam error.
    #[error("Steamworks matchmaking operation {operation} failed: {source}")]
    SteamError {
        /// Operation that failed.
        operation: &'static str,
        /// Steamworks error.
        source: steamworks::SteamError,
    },
}

impl SteamworksMatchmakingError {
    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    fn lobby_key_too_long(key: impl Into<String>) -> Self {
        Self::LobbyKeyTooLong { key: key.into() }
    }

    fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }

    fn steam_error(operation: &'static str, source: steamworks::SteamError) -> Self {
        Self::SteamError { operation, source }
    }
}

fn process_matchmaking_commands(
    client: Option<Res<SteamworksClient>>,
    async_results: Res<SteamworksMatchmakingAsyncResults>,
    mut state: ResMut<SteamworksMatchmakingState>,
    mut commands: ResMut<Messages<SteamworksMatchmakingCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksMatchmakingResult>,
) {
    for result in async_results.drain() {
        record_matchmaking_result(&mut state, &result);
        results.write(result);
    }

    process_matchmaking_steam_events(&mut state, &mut steam_events, &mut results);

    let Some(client) = client else {
        let error = SteamworksMatchmakingError::ClientUnavailable;
        for command in commands.drain() {
            state.record_error(error.clone());
            results.write(SteamworksMatchmakingResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        let request_id = async_command_request_id(&command, &mut state);
        match handle_matchmaking_command(&client, &async_results, command.clone(), request_id) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks matchmaking command"
                );
                results.write(SteamworksMatchmakingResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks matchmaking command failed"
                );
                results.write(SteamworksMatchmakingResult::Err { command, error });
            }
        }
    }
}

fn process_matchmaking_steam_events(
    state: &mut SteamworksMatchmakingState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksMatchmakingResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::LobbyCreated(event) => {
                SteamworksMatchmakingOperation::LobbyCreateCallbackReceived {
                    callback: SteamworksLobbyCreatedCallback {
                        result: event.result,
                        lobby: event.lobby,
                    },
                }
            }
            SteamworksEvent::LobbyEnter(event) => {
                SteamworksMatchmakingOperation::LobbyEnterCallbackReceived {
                    callback: SteamworksLobbyEnterCallback {
                        lobby: event.lobby,
                        chat_permissions: event.chat_permissions,
                        blocked: event.blocked,
                        chat_room_enter_response: event.chat_room_enter_response,
                    },
                }
            }
            SteamworksEvent::LobbyChatMsg(event) => {
                SteamworksMatchmakingOperation::LobbyChatMessageReceived {
                    message: snapshot_lobby_chat_message(event),
                }
            }
            SteamworksEvent::LobbyChatUpdate(event) => {
                SteamworksMatchmakingOperation::LobbyChatUpdateReceived {
                    update: SteamworksLobbyChatUpdate {
                        lobby: event.lobby,
                        user_changed: event.user_changed,
                        making_change: event.making_change,
                        member_state_change: event.member_state_change.clone(),
                    },
                }
            }
            SteamworksEvent::LobbyDataUpdate(event) => {
                SteamworksMatchmakingOperation::LobbyDataUpdateReceived {
                    update: SteamworksLobbyDataUpdate {
                        lobby: event.lobby,
                        member: event.member,
                        success: event.success,
                    },
                }
            }
            _ => continue,
        };

        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks matchmaking callback"
        );
        results.write(SteamworksMatchmakingResult::Ok(operation));
    }
}

fn snapshot_lobby_chat_message(event: &steamworks::LobbyChatMsg) -> SteamworksLobbyChatMessage {
    SteamworksLobbyChatMessage {
        lobby: event.lobby,
        user: event.user,
        chat_entry_type: event.chat_entry_type,
        chat_id: event.chat_id,
    }
}

fn record_matchmaking_result(
    state: &mut SteamworksMatchmakingState,
    result: &SteamworksMatchmakingResult,
) {
    match result {
        SteamworksMatchmakingResult::Ok(operation) => state.record_operation(operation),
        SteamworksMatchmakingResult::Err { error, .. } => state.record_error(error.clone()),
    }
}

fn async_command_request_id(
    command: &SteamworksMatchmakingCommand,
    state: &mut SteamworksMatchmakingState,
) -> Option<u64> {
    matches!(
        command,
        SteamworksMatchmakingCommand::RequestLobbyList { .. }
            | SteamworksMatchmakingCommand::CreateLobby { .. }
            | SteamworksMatchmakingCommand::JoinLobby { .. }
    )
    .then(|| state.next_request_id())
}

fn handle_matchmaking_command(
    client: &SteamworksClient,
    async_results: &SteamworksMatchmakingAsyncResults,
    command: SteamworksMatchmakingCommand,
    request_id: Option<u64>,
) -> Result<SteamworksMatchmakingOperation, SteamworksMatchmakingError> {
    validate_command(&command)?;

    let matchmaking = client.matchmaking();
    match command {
        SteamworksMatchmakingCommand::RequestLobbyList { filter } => {
            let request_id = request_id.expect("async matchmaking command missing request id");
            apply_lobby_list_filter(&matchmaking, &filter)?;
            let async_results = async_results.clone();
            let command = SteamworksMatchmakingCommand::RequestLobbyList {
                filter: filter.clone(),
            };
            let request_filter = filter.clone();
            matchmaking.request_lobby_list(move |result| {
                async_results.push(match result {
                    Ok(lobbies) => SteamworksMatchmakingResult::Ok(
                        SteamworksMatchmakingOperation::LobbyListReceived {
                            request_id,
                            filter: request_filter.clone(),
                            lobbies,
                        },
                    ),
                    Err(source) => SteamworksMatchmakingResult::Err {
                        command,
                        error: SteamworksMatchmakingError::steam_error(
                            "matchmaking.request_lobby_list",
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksMatchmakingOperation::LobbyListRequested { request_id, filter })
        }
        SteamworksMatchmakingCommand::CreateLobby {
            lobby_type,
            max_members,
        } => {
            let request_id = request_id.expect("async matchmaking command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksMatchmakingCommand::CreateLobby {
                lobby_type,
                max_members,
            };
            matchmaking.create_lobby(lobby_type, max_members, move |result| {
                async_results.push(match result {
                    Ok(lobby) => SteamworksMatchmakingResult::Ok(
                        SteamworksMatchmakingOperation::LobbyCreated {
                            request_id,
                            lobby_type,
                            max_members,
                            lobby,
                        },
                    ),
                    Err(source) => SteamworksMatchmakingResult::Err {
                        command,
                        error: SteamworksMatchmakingError::steam_error(
                            "matchmaking.create_lobby",
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksMatchmakingOperation::LobbyCreateRequested {
                request_id,
                lobby_type,
                max_members,
            })
        }
        SteamworksMatchmakingCommand::JoinLobby { lobby } => {
            let request_id = request_id.expect("async matchmaking command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksMatchmakingCommand::JoinLobby { lobby };
            let requested_lobby = lobby;
            matchmaking.join_lobby(lobby, move |result| {
                async_results.push(match result {
                    Ok(lobby) => SteamworksMatchmakingResult::Ok(
                        SteamworksMatchmakingOperation::LobbyJoined {
                            request_id,
                            requested_lobby,
                            lobby,
                        },
                    ),
                    Err(()) => SteamworksMatchmakingResult::Err {
                        command,
                        error: SteamworksMatchmakingError::operation_failed(
                            "matchmaking.join_lobby",
                        ),
                    },
                });
            });
            Ok(SteamworksMatchmakingOperation::LobbyJoinRequested { request_id, lobby })
        }
        SteamworksMatchmakingCommand::LeaveLobby { lobby } => {
            matchmaking.leave_lobby(lobby);
            Ok(SteamworksMatchmakingOperation::LobbyLeft { lobby })
        }
        SteamworksMatchmakingCommand::GetLobbyDataCount { lobby } => {
            Ok(SteamworksMatchmakingOperation::LobbyDataCountRead {
                lobby,
                count: matchmaking.lobby_data_count(lobby),
            })
        }
        SteamworksMatchmakingCommand::GetLobbyData { lobby, key } => {
            Ok(SteamworksMatchmakingOperation::LobbyDataRead {
                lobby,
                value: matchmaking.lobby_data(lobby, &key),
                key,
            })
        }
        SteamworksMatchmakingCommand::GetLobbyDataByIndex { lobby, index } => {
            Ok(SteamworksMatchmakingOperation::LobbyDataByIndexRead {
                lobby,
                index,
                entry: matchmaking.lobby_data_by_index(lobby, index),
            })
        }
        SteamworksMatchmakingCommand::GetAllLobbyData { lobby } => {
            let entries = (0..matchmaking.lobby_data_count(lobby))
                .filter_map(|index| matchmaking.lobby_data_by_index(lobby, index))
                .collect();
            Ok(SteamworksMatchmakingOperation::AllLobbyDataRead { lobby, entries })
        }
        SteamworksMatchmakingCommand::SetLobbyData { lobby, key, value } => {
            if matchmaking.set_lobby_data(lobby, &key, &value) {
                Ok(SteamworksMatchmakingOperation::LobbyDataSet { lobby, key })
            } else {
                Err(SteamworksMatchmakingError::operation_failed(
                    "matchmaking.set_lobby_data",
                ))
            }
        }
        SteamworksMatchmakingCommand::DeleteLobbyData { lobby, key } => {
            if matchmaking.delete_lobby_data(lobby, &key) {
                Ok(SteamworksMatchmakingOperation::LobbyDataDeleted { lobby, key })
            } else {
                Err(SteamworksMatchmakingError::operation_failed(
                    "matchmaking.delete_lobby_data",
                ))
            }
        }
        SteamworksMatchmakingCommand::SetLobbyMemberData { lobby, key, value } => {
            matchmaking.set_lobby_member_data(lobby, &key, &value);
            Ok(SteamworksMatchmakingOperation::LobbyMemberDataSet { lobby, key })
        }
        SteamworksMatchmakingCommand::GetLobbyMemberData { lobby, user, key } => {
            Ok(SteamworksMatchmakingOperation::LobbyMemberDataRead {
                lobby,
                user,
                value: matchmaking.get_lobby_member_data(lobby, user, &key),
                key,
            })
        }
        SteamworksMatchmakingCommand::GetLobbyMemberLimit { lobby } => {
            Ok(SteamworksMatchmakingOperation::LobbyMemberLimitRead {
                lobby,
                limit: matchmaking.lobby_member_limit(lobby),
            })
        }
        SteamworksMatchmakingCommand::GetLobbyOwner { lobby } => {
            Ok(SteamworksMatchmakingOperation::LobbyOwnerRead {
                lobby,
                owner: matchmaking.lobby_owner(lobby),
            })
        }
        SteamworksMatchmakingCommand::GetLobbyMemberCount { lobby } => {
            Ok(SteamworksMatchmakingOperation::LobbyMemberCountRead {
                lobby,
                count: matchmaking.lobby_member_count(lobby),
            })
        }
        SteamworksMatchmakingCommand::ListLobbyMembers { lobby } => {
            Ok(SteamworksMatchmakingOperation::LobbyMembersListed {
                lobby,
                members: matchmaking.lobby_members(lobby),
            })
        }
        SteamworksMatchmakingCommand::SetLobbyJoinable { lobby, joinable } => {
            if matchmaking.set_lobby_joinable(lobby, joinable) {
                Ok(SteamworksMatchmakingOperation::LobbyJoinableSet { lobby, joinable })
            } else {
                Err(SteamworksMatchmakingError::operation_failed(
                    "matchmaking.set_lobby_joinable",
                ))
            }
        }
        SteamworksMatchmakingCommand::SendLobbyChatMessage { lobby, data } => matchmaking
            .send_lobby_chat_message(lobby, &data)
            .map(|()| SteamworksMatchmakingOperation::LobbyChatMessageSent {
                lobby,
                len: data.len(),
            })
            .map_err(|source| {
                SteamworksMatchmakingError::steam_error(
                    "matchmaking.send_lobby_chat_message",
                    source,
                )
            }),
        SteamworksMatchmakingCommand::GetLobbyChatEntry {
            lobby,
            chat_id,
            max_bytes,
        } => {
            let mut buffer = vec![0; max_bytes];
            let data = matchmaking
                .get_lobby_chat_entry(lobby, chat_id, &mut buffer)
                .to_vec();
            Ok(SteamworksMatchmakingOperation::LobbyChatEntryRead {
                lobby,
                chat_id,
                data,
            })
        }
        SteamworksMatchmakingCommand::SetLobbyGameServer {
            lobby,
            address,
            steam_id,
        } => {
            matchmaking.set_lobby_game_server(lobby, address, steam_id);
            Ok(SteamworksMatchmakingOperation::LobbyGameServerSet {
                lobby,
                server: SteamworksLobbyGameServer { address, steam_id },
            })
        }
        SteamworksMatchmakingCommand::GetLobbyGameServer { lobby } => {
            let server = matchmaking
                .get_lobby_game_server(lobby)
                .map(|(address, steam_id)| SteamworksLobbyGameServer { address, steam_id });
            Ok(SteamworksMatchmakingOperation::LobbyGameServerRead { lobby, server })
        }
    }
}

fn apply_lobby_list_filter(
    matchmaking: &steamworks::Matchmaking,
    filter: &SteamworksLobbyListFilter,
) -> Result<(), SteamworksMatchmakingError> {
    for item in &filter.string {
        let key = lobby_key(&item.key)?;
        matchmaking.add_request_lobby_list_string_filter(steamworks::StringFilter(
            key,
            &item.value,
            item.comparison,
        ));
    }

    for item in &filter.number {
        let key = lobby_key(&item.key)?;
        matchmaking.add_request_lobby_list_numerical_filter(steamworks::NumberFilter(
            key,
            item.value,
            item.comparison,
        ));
    }

    for item in &filter.near_value {
        let key = lobby_key(&item.key)?;
        matchmaking
            .add_request_lobby_list_near_value_filter(steamworks::NearFilter(key, item.value));
    }

    if let Some(open_slots) = filter.open_slots {
        matchmaking.set_request_lobby_list_slots_available_filter(open_slots);
    }
    if let Some(distance) = filter.distance {
        matchmaking.set_request_lobby_list_distance_filter(distance);
    }
    if let Some(max_results) = filter.max_results {
        matchmaking.set_request_lobby_list_result_count_filter(max_results);
    }

    Ok(())
}

fn lobby_key(key: &str) -> Result<steamworks::LobbyKey<'_>, SteamworksMatchmakingError> {
    steamworks::LobbyKey::try_new(key)
        .map_err(|_| SteamworksMatchmakingError::lobby_key_too_long(key))
}

fn validate_command(
    command: &SteamworksMatchmakingCommand,
) -> Result<(), SteamworksMatchmakingError> {
    match command {
        SteamworksMatchmakingCommand::RequestLobbyList { filter } => validate_filter(filter),
        SteamworksMatchmakingCommand::CreateLobby { max_members, .. } => {
            if *max_members > MAX_LOBBY_MEMBERS {
                Err(SteamworksMatchmakingError::MaxLobbyMembersExceeded {
                    requested: *max_members,
                    max_supported: MAX_LOBBY_MEMBERS,
                })
            } else {
                Ok(())
            }
        }
        SteamworksMatchmakingCommand::GetLobbyData { key, .. }
        | SteamworksMatchmakingCommand::DeleteLobbyData { key, .. } => validate_lobby_key(key),
        SteamworksMatchmakingCommand::SetLobbyData { key, value, .. }
        | SteamworksMatchmakingCommand::SetLobbyMemberData { key, value, .. } => {
            validate_lobby_key(key)?;
            validate_steam_string("value", value)
        }
        SteamworksMatchmakingCommand::GetLobbyMemberData { key, .. } => validate_lobby_key(key),
        SteamworksMatchmakingCommand::SendLobbyChatMessage { data, .. } => {
            validate_lobby_chat_message(data.len())
        }
        SteamworksMatchmakingCommand::GetLobbyChatEntry { max_bytes, .. } => {
            validate_lobby_chat_message(*max_bytes)
        }
        _ => Ok(()),
    }
}

fn validate_filter(filter: &SteamworksLobbyListFilter) -> Result<(), SteamworksMatchmakingError> {
    for item in &filter.string {
        validate_lobby_key(&item.key)?;
        validate_steam_string("value", &item.value)?;
    }
    for item in &filter.number {
        validate_lobby_key(&item.key)?;
    }
    for item in &filter.near_value {
        validate_lobby_key(&item.key)?;
    }
    if let Some(max_results) = filter.max_results {
        validate_lobby_list_result_count(max_results)?;
    }
    Ok(())
}

fn validate_lobby_key(key: &str) -> Result<(), SteamworksMatchmakingError> {
    validate_steam_string("key", key)?;
    lobby_key(key).map(|_| ())
}

fn validate_steam_string(
    field: &'static str,
    value: &str,
) -> Result<(), SteamworksMatchmakingError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksMatchmakingError::invalid_string(field))
    } else {
        Ok(())
    }
}

fn validate_lobby_chat_message(len: usize) -> Result<(), SteamworksMatchmakingError> {
    if len == 0 || len > MAX_LOBBY_CHAT_MESSAGE_BYTES {
        Err(SteamworksMatchmakingError::InvalidChatMessageLength {
            requested: len,
            max_supported: MAX_LOBBY_CHAT_MESSAGE_BYTES,
        })
    } else {
        Ok(())
    }
}

fn validate_lobby_list_result_count(count: u64) -> Result<(), SteamworksMatchmakingError> {
    if count > MAX_LOBBY_LIST_RESULTS {
        Err(SteamworksMatchmakingError::MaxLobbyListResultsExceeded {
            requested: count,
            max_supported: MAX_LOBBY_LIST_RESULTS,
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    #[test]
    fn matchmaking_plugin_registers_resources_and_messages() {
        let mut app = App::new();

        app.add_plugins(SteamworksMatchmakingPlugin::new());

        assert!(app
            .world()
            .contains_resource::<SteamworksMatchmakingState>());
        assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksMatchmakingCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksMatchmakingResult>>());
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksMatchmakingPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksMatchmakingCommand>>()
            .write(SteamworksMatchmakingCommand::request_lobby_list(
                SteamworksLobbyListFilter::new(),
            ));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksMatchmakingResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksMatchmakingResult::Err {
                command: SteamworksMatchmakingCommand::request_lobby_list(
                    SteamworksLobbyListFilter::new()
                ),
                error: SteamworksMatchmakingError::ClientUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksMatchmakingState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksMatchmakingError::ClientUnavailable)
        );
    }

    #[test]
    fn validation_rejects_interior_nul() {
        let command = SteamworksMatchmakingCommand::set_lobby_data(
            steamworks::LobbyId::from_raw(1),
            "mode\0bad",
            "dm",
        );

        assert_eq!(
            validate_command(&command),
            Err(SteamworksMatchmakingError::InvalidString { field: "key" })
        );

        let filter = SteamworksLobbyListFilter::new().with_string(
            "mode",
            "dm\0bad",
            steamworks::StringFilterKind::Equal,
        );

        assert_eq!(
            validate_filter(&filter),
            Err(SteamworksMatchmakingError::InvalidString { field: "value" })
        );
    }

    #[test]
    fn validation_rejects_steam_assert_inputs() {
        assert_eq!(
            validate_command(&SteamworksMatchmakingCommand::create_lobby(
                steamworks::LobbyType::Private,
                251
            )),
            Err(SteamworksMatchmakingError::MaxLobbyMembersExceeded {
                requested: 251,
                max_supported: MAX_LOBBY_MEMBERS,
            })
        );

        assert_eq!(
            validate_command(&SteamworksMatchmakingCommand::send_lobby_chat_message(
                steamworks::LobbyId::from_raw(1),
                Vec::new()
            )),
            Err(SteamworksMatchmakingError::InvalidChatMessageLength {
                requested: 0,
                max_supported: MAX_LOBBY_CHAT_MESSAGE_BYTES,
            })
        );

        let filter = SteamworksLobbyListFilter::new().with_max_results(MAX_LOBBY_LIST_RESULTS + 1);
        assert_eq!(
            validate_filter(&filter),
            Err(SteamworksMatchmakingError::MaxLobbyListResultsExceeded {
                requested: MAX_LOBBY_LIST_RESULTS + 1,
                max_supported: MAX_LOBBY_LIST_RESULTS,
            })
        );
    }

    #[test]
    fn debug_redacts_lobby_chat_payload_bytes() {
        let lobby = steamworks::LobbyId::from_raw(1);
        let command = SteamworksMatchmakingCommand::send_lobby_chat_message(lobby, vec![1, 2, 3]);
        let operation = SteamworksMatchmakingOperation::LobbyChatEntryRead {
            lobby,
            chat_id: 7,
            data: vec![4, 5, 6],
        };
        let entry = SteamworksLobbyChatEntry {
            lobby,
            chat_id: 7,
            data: vec![7, 8, 9],
        };

        let command_debug = format!("{command:?}");
        let operation_debug = format!("{operation:?}");
        let entry_debug = format!("{entry:?}");

        assert!(command_debug.contains("data_len: 3"));
        assert!(!command_debug.contains("[1, 2, 3]"));
        assert!(operation_debug.contains("data_len: 3"));
        assert!(!operation_debug.contains("[4, 5, 6]"));
        assert!(entry_debug.contains("data_len: 3"));
        assert!(!entry_debug.contains("[7, 8, 9]"));
    }

    #[test]
    fn async_success_operations_preserve_request_context() {
        let filter = SteamworksLobbyListFilter::new().with_max_results(2);
        let lobbies = vec![steamworks::LobbyId::from_raw(11)];

        assert_eq!(
            SteamworksMatchmakingOperation::LobbyListReceived {
                request_id: 7,
                filter: filter.clone(),
                lobbies: lobbies.clone(),
            },
            SteamworksMatchmakingOperation::LobbyListReceived {
                request_id: 7,
                filter,
                lobbies,
            }
        );

        let requested_lobby = steamworks::LobbyId::from_raw(22);
        let joined_lobby = steamworks::LobbyId::from_raw(33);
        assert_eq!(
            SteamworksMatchmakingOperation::LobbyJoined {
                request_id: 8,
                requested_lobby,
                lobby: joined_lobby,
            },
            SteamworksMatchmakingOperation::LobbyJoined {
                request_id: 8,
                requested_lobby,
                lobby: joined_lobby,
            }
        );
    }

    #[test]
    fn async_commands_get_unique_request_ids() {
        let mut state = SteamworksMatchmakingState::default();
        let command =
            SteamworksMatchmakingCommand::request_lobby_list(SteamworksLobbyListFilter::new());

        assert_eq!(async_command_request_id(&command, &mut state), Some(0));
        assert_eq!(async_command_request_id(&command, &mut state), Some(1));
        assert_eq!(
            async_command_request_id(
                &SteamworksMatchmakingCommand::GetLobbyDataCount {
                    lobby: steamworks::LobbyId::from_raw(1),
                },
                &mut state,
            ),
            None
        );
    }

    #[test]
    fn state_records_matchmaking_operations_without_unbounded_history() {
        let mut state = SteamworksMatchmakingState::default();
        let filter = SteamworksLobbyListFilter::new().with_max_results(2);
        let first_lobby = steamworks::LobbyId::from_raw(11);
        let second_lobby = steamworks::LobbyId::from_raw(22);
        let user = steamworks::SteamId::from_raw(33);
        let owner = steamworks::SteamId::from_raw(44);
        let server = SteamworksLobbyGameServer {
            address: "127.0.0.1:27015".parse().expect("valid socket address"),
            steam_id: Some(owner),
        };

        state.record_operation(&SteamworksMatchmakingOperation::LobbyListRequested {
            request_id: 1,
            filter: filter.clone(),
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyListReceived {
            request_id: 1,
            filter: filter.clone(),
            lobbies: vec![first_lobby, second_lobby],
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyCreateRequested {
            request_id: 2,
            lobby_type: steamworks::LobbyType::FriendsOnly,
            max_members: 4,
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyCreated {
            request_id: 2,
            lobby_type: steamworks::LobbyType::FriendsOnly,
            max_members: 4,
            lobby: first_lobby,
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyJoinRequested {
            request_id: 3,
            lobby: second_lobby,
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyJoined {
            request_id: 3,
            requested_lobby: second_lobby,
            lobby: second_lobby,
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyJoined {
            request_id: 3,
            requested_lobby: second_lobby,
            lobby: second_lobby,
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyLeft { lobby: first_lobby });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyDataCountRead {
            lobby: second_lobby,
            count: 2,
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyDataRead {
            lobby: second_lobby,
            key: "mode".to_owned(),
            value: Some("dm".to_owned()),
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyDataByIndexRead {
            lobby: second_lobby,
            index: 1,
            entry: Some(("map".to_owned(), "arena".to_owned())),
        });
        state.record_operation(&SteamworksMatchmakingOperation::AllLobbyDataRead {
            lobby: second_lobby,
            entries: vec![
                ("mode".to_owned(), "dm".to_owned()),
                ("map".to_owned(), "arena".to_owned()),
            ],
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyDataSet {
            lobby: second_lobby,
            key: "mode".to_owned(),
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyDataDeleted {
            lobby: second_lobby,
            key: "old".to_owned(),
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyMemberDataSet {
            lobby: second_lobby,
            key: "loadout".to_owned(),
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyMemberDataRead {
            lobby: second_lobby,
            user,
            key: "rank".to_owned(),
            value: Some("gold".to_owned()),
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyMemberLimitRead {
            lobby: second_lobby,
            limit: Some(8),
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyOwnerRead {
            lobby: second_lobby,
            owner,
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyMemberCountRead {
            lobby: second_lobby,
            count: 3,
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyMembersListed {
            lobby: second_lobby,
            members: vec![user, owner],
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyJoinableSet {
            lobby: second_lobby,
            joinable: false,
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyChatMessageSent {
            lobby: second_lobby,
            len: 5,
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyChatEntryRead {
            lobby: second_lobby,
            chat_id: 7,
            data: vec![1, 2, 3],
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyGameServerSet {
            lobby: second_lobby,
            server: server.clone(),
        });
        state.record_operation(&SteamworksMatchmakingOperation::LobbyGameServerRead {
            lobby: second_lobby,
            server: Some(server.clone()),
        });

        assert_eq!(
            state.last_lobby_list_request(),
            Some(&SteamworksLobbyListRequest {
                request_id: 1,
                filter: filter.clone(),
            })
        );
        assert_eq!(state.last_lobby_list(), &[first_lobby, second_lobby]);
        assert_eq!(
            state.last_lobby_create_request(),
            Some(&SteamworksLobbyCreateRequest {
                request_id: 2,
                lobby_type: steamworks::LobbyType::FriendsOnly,
                max_members: 4,
            })
        );
        assert_eq!(
            state.last_lobby_join_request(),
            Some(&SteamworksMatchmakingLobbyJoinRequest {
                request_id: 3,
                lobby: second_lobby,
            })
        );
        assert_eq!(
            state.last_created_lobby(),
            Some(&SteamworksLobbyCreated {
                request_id: 2,
                lobby_type: steamworks::LobbyType::FriendsOnly,
                max_members: 4,
                lobby: first_lobby,
            })
        );
        assert_eq!(
            state.last_joined_lobby(),
            Some(&SteamworksLobbyJoined {
                request_id: 3,
                requested_lobby: second_lobby,
                lobby: second_lobby,
            })
        );
        assert_eq!(state.last_left_lobby(), Some(first_lobby));
        assert_eq!(state.joined_lobbies(), &[second_lobby]);
        assert_eq!(
            state.last_lobby_data_count(),
            Some(&SteamworksLobbyDataCount {
                lobby: second_lobby,
                count: 2,
            })
        );
        assert_eq!(
            state.last_lobby_data(),
            Some(&SteamworksLobbyDataValue {
                lobby: second_lobby,
                key: "mode".to_owned(),
                value: Some("dm".to_owned()),
            })
        );
        assert_eq!(
            state.last_lobby_data_entry(),
            Some(&SteamworksLobbyDataEntry {
                lobby: second_lobby,
                index: 1,
                entry: Some(("map".to_owned(), "arena".to_owned())),
            })
        );
        assert_eq!(
            state.last_all_lobby_data(),
            Some(&SteamworksLobbyDataEntries {
                lobby: second_lobby,
                entries: vec![
                    ("mode".to_owned(), "dm".to_owned()),
                    ("map".to_owned(), "arena".to_owned()),
                ],
            })
        );
        assert_eq!(
            state.last_lobby_data_set(),
            Some(&SteamworksLobbyDataMutation {
                lobby: second_lobby,
                key: "mode".to_owned(),
            })
        );
        assert_eq!(
            state.last_lobby_data_deleted(),
            Some(&SteamworksLobbyDataMutation {
                lobby: second_lobby,
                key: "old".to_owned(),
            })
        );
        assert_eq!(
            state.last_lobby_member_data_set(),
            Some(&SteamworksLobbyDataMutation {
                lobby: second_lobby,
                key: "loadout".to_owned(),
            })
        );
        assert_eq!(
            state.last_lobby_member_data(),
            Some(&SteamworksLobbyMemberDataValue {
                lobby: second_lobby,
                user,
                key: "rank".to_owned(),
                value: Some("gold".to_owned()),
            })
        );
        assert_eq!(
            state.last_lobby_member_limit(),
            Some(&SteamworksLobbyMemberLimit {
                lobby: second_lobby,
                limit: Some(8),
            })
        );
        assert_eq!(
            state.last_lobby_owner(),
            Some(&SteamworksLobbyOwner {
                lobby: second_lobby,
                owner,
            })
        );
        assert_eq!(
            state.last_lobby_member_count(),
            Some(&SteamworksLobbyMemberCount {
                lobby: second_lobby,
                count: 3,
            })
        );
        assert_eq!(
            state.last_lobby_members(),
            Some(&SteamworksLobbyMembers {
                lobby: second_lobby,
                members: vec![user, owner],
            })
        );
        assert_eq!(
            state.last_lobby_joinability(),
            Some(&SteamworksLobbyJoinability {
                lobby: second_lobby,
                joinable: false,
            })
        );
        assert_eq!(
            state.last_lobby_chat_message_sent(),
            Some(&SteamworksLobbyChatMessageSent {
                lobby: second_lobby,
                len: 5,
            })
        );
        assert_eq!(
            state.last_lobby_chat_entry(),
            Some(&SteamworksLobbyChatEntry {
                lobby: second_lobby,
                chat_id: 7,
                data: vec![1, 2, 3],
            })
        );
        assert_eq!(
            state.last_lobby_game_server_set(),
            Some(&SteamworksLobbyGameServerAssignment {
                lobby: second_lobby,
                server: server.clone(),
            })
        );
        assert_eq!(
            state.last_lobby_game_server(),
            Some(&SteamworksLobbyGameServerLookup {
                lobby: second_lobby,
                server: Some(server),
            })
        );
    }

    #[test]
    fn lobby_callbacks_are_bridged_without_client() {
        let mut app = App::new();
        let lobby = steamworks::LobbyId::from_raw(11);
        let user = steamworks::SteamId::from_raw(22);
        let maker = steamworks::SteamId::from_raw(33);

        app.add_plugins(SteamworksMatchmakingPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::LobbyCreated(steamworks::LobbyCreated {
                result: 1,
                lobby,
            }));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::LobbyEnter(steamworks::LobbyEnter {
                lobby,
                chat_permissions: 0,
                blocked: false,
                chat_room_enter_response: steamworks::ChatRoomEnterResponse::Success,
            }));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::LobbyChatMsg(steamworks::LobbyChatMsg {
                lobby,
                user,
                chat_entry_type: steamworks::ChatEntryType::ChatMsg,
                chat_id: 5,
            }));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::LobbyChatUpdate(
                steamworks::LobbyChatUpdate {
                    lobby,
                    user_changed: user,
                    making_change: maker,
                    member_state_change: steamworks::ChatMemberStateChange::Entered,
                },
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::LobbyDataUpdate(
                steamworks::LobbyDataUpdate {
                    lobby,
                    member: user,
                    success: true,
                },
            ));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksMatchmakingResult>>();
        let drained = results.drain().collect::<Vec<_>>();
        let expected_created = SteamworksLobbyCreatedCallback { result: 1, lobby };
        let expected_enter = SteamworksLobbyEnterCallback {
            lobby,
            chat_permissions: 0,
            blocked: false,
            chat_room_enter_response: steamworks::ChatRoomEnterResponse::Success,
        };
        let expected_message = SteamworksLobbyChatMessage {
            lobby,
            user,
            chat_entry_type: steamworks::ChatEntryType::ChatMsg,
            chat_id: 5,
        };
        let expected_chat_update = SteamworksLobbyChatUpdate {
            lobby,
            user_changed: user,
            making_change: maker,
            member_state_change: steamworks::ChatMemberStateChange::Entered,
        };
        let expected_data_update = SteamworksLobbyDataUpdate {
            lobby,
            member: user,
            success: true,
        };

        assert_eq!(
            drained,
            vec![
                SteamworksMatchmakingResult::Ok(
                    SteamworksMatchmakingOperation::LobbyCreateCallbackReceived {
                        callback: expected_created.clone(),
                    },
                ),
                SteamworksMatchmakingResult::Ok(
                    SteamworksMatchmakingOperation::LobbyEnterCallbackReceived {
                        callback: expected_enter.clone(),
                    },
                ),
                SteamworksMatchmakingResult::Ok(
                    SteamworksMatchmakingOperation::LobbyChatMessageReceived {
                        message: expected_message.clone(),
                    },
                ),
                SteamworksMatchmakingResult::Ok(
                    SteamworksMatchmakingOperation::LobbyChatUpdateReceived {
                        update: expected_chat_update.clone(),
                    },
                ),
                SteamworksMatchmakingResult::Ok(
                    SteamworksMatchmakingOperation::LobbyDataUpdateReceived {
                        update: expected_data_update.clone(),
                    },
                ),
            ]
        );

        let state = app.world().resource::<SteamworksMatchmakingState>();
        assert_eq!(state.joined_lobbies(), &[lobby]);
        assert_eq!(state.last_lobby_created_callback(), Some(&expected_created));
        assert_eq!(state.last_lobby_enter_callback(), Some(&expected_enter));
        assert_eq!(state.last_lobby_chat_message(), Some(&expected_message));
        assert_eq!(state.last_lobby_chat_update(), Some(&expected_chat_update));
        assert_eq!(state.last_lobby_data_update(), Some(&expected_data_update));
        assert_eq!(state.last_error(), None);
    }
}
