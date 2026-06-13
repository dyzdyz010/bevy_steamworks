//! High-level Bevy ECS integration for Steam matchmaking and lobbies.
//!
//! This module builds on top of the upstream [`steamworks::Matchmaking`] API.
//! It keeps async Steam call results flowing through Bevy messages, while
//! avoiding blocking work in the frame loop.

use std::{
    net::SocketAddrV4,
    sync::{Arc, Mutex},
};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksSystem};

const MAX_LOBBY_MEMBERS: u32 = 250;
const MAX_LOBBY_CHAT_MESSAGE_BYTES: usize = 4096;

/// Bevy plugin for high-level Steam matchmaking and lobby commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksMatchmakingCommand`] and [`SteamworksMatchmakingResult`]
/// messages and runs its command processor in [`bevy_app::First`] after Steam
/// callbacks.
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
    last_lobby_list: Vec<steamworks::LobbyId>,
    joined_lobbies: Vec<steamworks::LobbyId>,
}

impl SteamworksMatchmakingState {
    /// Returns the most recent synchronous or async error observed by the plugin.
    pub fn last_error(&self) -> Option<&SteamworksMatchmakingError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent lobby list received from Steam.
    pub fn last_lobby_list(&self) -> &[steamworks::LobbyId] {
        &self.last_lobby_list
    }

    /// Returns lobbies this command layer has observed the local user joining.
    pub fn joined_lobbies(&self) -> &[steamworks::LobbyId] {
        &self.joined_lobbies
    }

    fn record_error(&mut self, error: SteamworksMatchmakingError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksMatchmakingOperation) {
        match operation {
            SteamworksMatchmakingOperation::LobbyListReceived { lobbies } => {
                self.last_lobby_list.clone_from(lobbies);
            }
            SteamworksMatchmakingOperation::LobbyCreated { lobby, .. }
            | SteamworksMatchmakingOperation::LobbyJoined { lobby }
                if !self.joined_lobbies.contains(lobby) =>
            {
                self.joined_lobbies.push(*lobby);
            }
            SteamworksMatchmakingOperation::LobbyLeft { lobby } => {
                self.joined_lobbies.retain(|known| known != lobby);
            }
            _ => {}
        }
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

/// A high-level command for Steam matchmaking and lobbies.
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksMatchmakingCommand {
    /// Request a lobby list from Steam.
    RequestLobbyList {
        /// Owned filters to apply before requesting the lobby list.
        filter: SteamworksLobbyListFilter,
    },
    /// Create a lobby.
    CreateLobby {
        /// Lobby visibility.
        lobby_type: steamworks::LobbyType,
        /// Maximum lobby members. Steam supports at most 250.
        max_members: u32,
    },
    /// Join a lobby.
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
    /// Read the bytes for a lobby chat entry after a [`crate::SteamworksEvent::LobbyChatMsg`].
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
#[derive(Clone, Debug, PartialEq)]
pub enum SteamworksMatchmakingOperation {
    /// Lobby list request was submitted.
    LobbyListRequested {
        /// Filters applied to the request.
        filter: SteamworksLobbyListFilter,
    },
    /// Lobby list request completed.
    LobbyListReceived {
        /// Matching lobby IDs.
        lobbies: Vec<steamworks::LobbyId>,
    },
    /// Lobby creation was submitted.
    LobbyCreateRequested {
        /// Lobby visibility.
        lobby_type: steamworks::LobbyType,
        /// Maximum members requested.
        max_members: u32,
    },
    /// Lobby creation completed.
    LobbyCreated {
        /// Lobby visibility requested.
        lobby_type: steamworks::LobbyType,
        /// Created lobby.
        lobby: steamworks::LobbyId,
    },
    /// Lobby join was submitted.
    LobbyJoinRequested {
        /// Lobby requested.
        lobby: steamworks::LobbyId,
    },
    /// Lobby join completed.
    LobbyJoined {
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
}

/// Result message emitted by [`SteamworksMatchmakingPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksMatchmakingResult {
    /// The command was submitted to Steamworks or a value was read.
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
    mut results: MessageWriter<SteamworksMatchmakingResult>,
) {
    for result in async_results.drain() {
        record_matchmaking_result(&mut state, &result);
        results.write(result);
    }

    let Some(client) = client else {
        let error = SteamworksMatchmakingError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
            results.write(SteamworksMatchmakingResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        match handle_matchmaking_command(&client, &async_results, command.clone()) {
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

fn record_matchmaking_result(
    state: &mut SteamworksMatchmakingState,
    result: &SteamworksMatchmakingResult,
) {
    match result {
        SteamworksMatchmakingResult::Ok(operation) => state.record_operation(operation),
        SteamworksMatchmakingResult::Err { error, .. } => state.record_error(error.clone()),
    }
}

fn handle_matchmaking_command(
    client: &SteamworksClient,
    async_results: &SteamworksMatchmakingAsyncResults,
    command: SteamworksMatchmakingCommand,
) -> Result<SteamworksMatchmakingOperation, SteamworksMatchmakingError> {
    validate_command(&command)?;

    let matchmaking = client.matchmaking();
    match command {
        SteamworksMatchmakingCommand::RequestLobbyList { filter } => {
            apply_lobby_list_filter(&matchmaking, &filter)?;
            let async_results = async_results.clone();
            let command = SteamworksMatchmakingCommand::RequestLobbyList {
                filter: filter.clone(),
            };
            matchmaking.request_lobby_list(move |result| {
                async_results.push(match result {
                    Ok(lobbies) => SteamworksMatchmakingResult::Ok(
                        SteamworksMatchmakingOperation::LobbyListReceived { lobbies },
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
            Ok(SteamworksMatchmakingOperation::LobbyListRequested { filter })
        }
        SteamworksMatchmakingCommand::CreateLobby {
            lobby_type,
            max_members,
        } => {
            let async_results = async_results.clone();
            let command = SteamworksMatchmakingCommand::CreateLobby {
                lobby_type,
                max_members,
            };
            matchmaking.create_lobby(lobby_type, max_members, move |result| {
                async_results.push(match result {
                    Ok(lobby) => SteamworksMatchmakingResult::Ok(
                        SteamworksMatchmakingOperation::LobbyCreated { lobby_type, lobby },
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
                lobby_type,
                max_members,
            })
        }
        SteamworksMatchmakingCommand::JoinLobby { lobby } => {
            let async_results = async_results.clone();
            let command = SteamworksMatchmakingCommand::JoinLobby { lobby };
            matchmaking.join_lobby(lobby, move |result| {
                async_results.push(match result {
                    Ok(lobby) => SteamworksMatchmakingResult::Ok(
                        SteamworksMatchmakingOperation::LobbyJoined { lobby },
                    ),
                    Err(()) => SteamworksMatchmakingResult::Err {
                        command,
                        error: SteamworksMatchmakingError::operation_failed(
                            "matchmaking.join_lobby",
                        ),
                    },
                });
            });
            Ok(SteamworksMatchmakingOperation::LobbyJoinRequested { lobby })
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
    }
}
