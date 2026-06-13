//! Bevy ECS integration for Steam Game Server initialization and callbacks.
//!
//! This module builds on top of the upstream [`steamworks::Server`] API. It
//! inserts a Bevy resource for the initialized game server, pumps Steam Game
//! Server callbacks into the shared [`crate::SteamworksEvent`] stream, and
//! mirrors relevant callback confirmations into [`SteamworksServerResult`].

use std::{
    net::{Ipv4Addr, SocketAddrV4},
    ops::Deref,
    sync::Mutex,
};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{
    user::{
        SteamworksAuthSessionError, SteamworksAuthSessionTicketResponse,
        SteamworksAuthTicketValidation, SteamworksSteamServerConnectionEvent,
    },
    SteamworksEvent, SteamworksFailurePolicy, SteamworksSystem,
};

/// Configuration used to initialize [`steamworks::Server`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksServerConfig {
    /// IPv4 address the Steam Game Server API should bind to.
    pub ip: Ipv4Addr,
    /// Game traffic port.
    pub game_port: u16,
    /// Server browser query port.
    ///
    /// Use [`steamworks::QUERY_PORT_SHARED`] when game and query traffic share
    /// the same socket.
    pub query_port: u16,
    /// Upstream Steam server mode.
    pub server_mode: steamworks::ServerMode,
    /// Version string reported to Steam.
    pub version: String,
}

impl SteamworksServerConfig {
    /// Creates a Steam Game Server initialization config.
    pub fn new(
        ip: Ipv4Addr,
        game_port: u16,
        query_port: u16,
        server_mode: steamworks::ServerMode,
        version: impl Into<String>,
    ) -> Self {
        Self {
            ip,
            game_port,
            query_port,
            server_mode,
            version: version.into(),
        }
    }
}

/// How [`SteamworksServerPlugin`] should create or locate the Steamworks server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksServerInitMode {
    /// Initialize a Steam Game Server from the supplied config.
    Config(SteamworksServerConfig),
    /// Do not initialize Steamworks.
    ///
    /// This is useful when another layer inserts [`SteamworksServer`] manually,
    /// or for tests that only need plugin schedules and messages.
    Manual,
}

/// Resource inserted when Steam Game Server initialization is explicitly allowed to fail.
#[derive(Clone, Debug, Error, PartialEq, Eq, Resource)]
pub enum SteamworksServerUnavailable {
    /// Manual mode was selected, but no [`SteamworksServer`] resource was present.
    #[error(
        "manual Steam Game Server initialization was selected, but no SteamworksServer resource exists"
    )]
    ManualServerMissing,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steam Game Server config field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// The upstream Steam Game Server initialization call returned an error.
    #[error("Steam Game Server initialization failed with {config:?}: {source}")]
    InitFailed {
        /// Initialization config used for the failed attempt.
        config: SteamworksServerConfig,
        /// Error returned by `steamworks`.
        source: steamworks::SteamAPIInitError,
    },
}

impl SteamworksServerUnavailable {
    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    fn init_failed(config: SteamworksServerConfig, source: steamworks::SteamAPIInitError) -> Self {
        Self::InitFailed { config, source }
    }
}

/// A Bevy resource wrapping [`steamworks::Server`].
#[derive(Clone, Resource)]
pub struct SteamworksServer(steamworks::Server);

impl SteamworksServer {
    /// Creates a Bevy resource from an initialized Steam Game Server.
    pub fn new(server: steamworks::Server) -> Self {
        Self(server)
    }

    /// Returns the underlying Steam Game Server handle.
    pub fn inner(&self) -> &steamworks::Server {
        &self.0
    }

    /// Clones the underlying Steam Game Server handle.
    pub fn clone_inner(&self) -> steamworks::Server {
        self.0.clone()
    }
}

impl From<steamworks::Server> for SteamworksServer {
    fn from(server: steamworks::Server) -> Self {
        Self::new(server)
    }
}

impl Deref for SteamworksServer {
    type Target = steamworks::Server;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

/// Runtime state for [`SteamworksServerPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksServerState {
    last_error: Option<SteamworksServerError>,
    steam_id: Option<steamworks::SteamId>,
    steam_server_connected: Option<bool>,
    active_auth_tickets: Vec<steamworks::AuthTicket>,
    authenticated_users: Vec<steamworks::SteamId>,
    last_steam_server_connection_event: Option<SteamworksSteamServerConnectionEvent>,
    last_auth_ticket_response: Option<SteamworksAuthSessionTicketResponse>,
    last_auth_ticket_validation: Option<SteamworksAuthTicketValidation>,
    last_client_approval: Option<SteamworksServerClientApproval>,
    last_client_denial: Option<SteamworksServerClientDenial>,
    last_client_kick: Option<SteamworksServerClientKick>,
    last_client_group_status: Option<SteamworksServerClientGroupStatus>,
    product: Option<String>,
    game_description: Option<String>,
    game_data: Option<String>,
    dedicated: Option<bool>,
    anonymous_logon_submitted: bool,
    advertise_server_active: Option<bool>,
    mod_dir: Option<String>,
    map_name: Option<String>,
    server_name: Option<String>,
    max_players: Option<i32>,
    game_tags: Option<String>,
    key_values: Vec<(String, String)>,
    password_protected: Option<bool>,
    bot_player_count: Option<i32>,
}

impl SteamworksServerState {
    /// Returns the most recent synchronous error observed by the server plugin.
    pub fn last_error(&self) -> Option<&SteamworksServerError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent Steam ID read for this game server.
    pub fn steam_id(&self) -> Option<steamworks::SteamId> {
        self.steam_id
    }

    /// Returns the latest known Steam server connection state.
    ///
    /// This is updated by Steam server connection callbacks.
    pub fn steam_server_connected(&self) -> Option<bool> {
        self.steam_server_connected
    }

    /// Returns authentication ticket handles issued through this command layer.
    ///
    /// Handles are removed after [`SteamworksServerCommand::CancelAuthenticationTicket`]
    /// is processed for the same ticket or after Steam reports ticket creation failure
    /// for the same ticket.
    pub fn active_auth_tickets(&self) -> &[steamworks::AuthTicket] {
        &self.active_auth_tickets
    }

    /// Returns users currently considered authenticated or approved by this layer.
    ///
    /// Entries are removed after [`SteamworksServerCommand::EndAuthenticationSession`]
    /// is processed for the same user or after Steam reports validation failure,
    /// denial, or kick for the same user.
    pub fn authenticated_users(&self) -> &[steamworks::SteamId] {
        &self.authenticated_users
    }

    /// Returns the most recent Steam server connection callback snapshot.
    pub fn last_steam_server_connection_event(
        &self,
    ) -> Option<&SteamworksSteamServerConnectionEvent> {
        self.last_steam_server_connection_event.as_ref()
    }

    /// Returns the most recent auth session ticket response callback snapshot.
    pub fn last_auth_ticket_response(&self) -> Option<&SteamworksAuthSessionTicketResponse> {
        self.last_auth_ticket_response.as_ref()
    }

    /// Returns the most recent auth ticket validation callback snapshot.
    pub fn last_auth_ticket_validation(&self) -> Option<&SteamworksAuthTicketValidation> {
        self.last_auth_ticket_validation.as_ref()
    }

    /// Returns the most recent game-server client approval callback snapshot.
    pub fn last_client_approval(&self) -> Option<&SteamworksServerClientApproval> {
        self.last_client_approval.as_ref()
    }

    /// Returns the most recent game-server client denial callback snapshot.
    pub fn last_client_denial(&self) -> Option<&SteamworksServerClientDenial> {
        self.last_client_denial.as_ref()
    }

    /// Returns the most recent game-server client kick callback snapshot.
    pub fn last_client_kick(&self) -> Option<&SteamworksServerClientKick> {
        self.last_client_kick.as_ref()
    }

    /// Returns the most recent game-server group status callback snapshot.
    pub fn last_client_group_status(&self) -> Option<&SteamworksServerClientGroupStatus> {
        self.last_client_group_status.as_ref()
    }

    /// Returns the most recent product string submitted through this command layer.
    pub fn product(&self) -> Option<&str> {
        self.product.as_deref()
    }

    /// Returns the most recent game description submitted through this command layer.
    pub fn game_description(&self) -> Option<&str> {
        self.game_description.as_deref()
    }

    /// Returns the most recent game data string submitted through this command layer.
    pub fn game_data(&self) -> Option<&str> {
        self.game_data.as_deref()
    }

    /// Returns the most recent dedicated/listen server flag submitted through this command layer.
    pub fn dedicated(&self) -> Option<bool> {
        self.dedicated
    }

    /// Returns whether anonymous logon was submitted through this command layer.
    pub fn anonymous_logon_submitted(&self) -> bool {
        self.anonymous_logon_submitted
    }

    /// Returns the most recent advertise-server-active flag submitted through this command layer.
    pub fn advertise_server_active(&self) -> Option<bool> {
        self.advertise_server_active
    }

    /// Returns the most recent mod dir submitted through this command layer.
    pub fn mod_dir(&self) -> Option<&str> {
        self.mod_dir.as_deref()
    }

    /// Returns the most recent map name submitted through this command layer.
    pub fn map_name(&self) -> Option<&str> {
        self.map_name.as_deref()
    }

    /// Returns the most recent server name submitted through this command layer.
    pub fn server_name(&self) -> Option<&str> {
        self.server_name.as_deref()
    }

    /// Returns the most recent maximum player count submitted through this command layer.
    pub fn max_players(&self) -> Option<i32> {
        self.max_players
    }

    /// Returns the most recent game tags string submitted through this command layer.
    pub fn game_tags(&self) -> Option<&str> {
        self.game_tags.as_deref()
    }

    /// Returns key/value rules submitted through this command layer.
    pub fn key_values(&self) -> &[(String, String)] {
        &self.key_values
    }

    /// Returns the most recent password-protected flag submitted through this command layer.
    pub fn password_protected(&self) -> Option<bool> {
        self.password_protected
    }

    /// Returns the most recent bot player count submitted through this command layer.
    pub fn bot_player_count(&self) -> Option<i32> {
        self.bot_player_count
    }

    fn record_error(&mut self, error: SteamworksServerError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksServerOperation) {
        match operation {
            SteamworksServerOperation::SteamIdRead { steam_id } => {
                self.steam_id = Some(*steam_id);
            }
            SteamworksServerOperation::AuthenticationSessionTicketIssued { ticket, .. }
                if !self.active_auth_tickets.contains(ticket) =>
            {
                self.active_auth_tickets.push(*ticket);
            }
            SteamworksServerOperation::AuthenticationTicketCancelled { ticket } => {
                self.active_auth_tickets.retain(|known| known != ticket);
            }
            SteamworksServerOperation::AuthenticationSessionStarted { user }
                if !self.authenticated_users.contains(user) =>
            {
                self.authenticated_users.push(*user);
            }
            SteamworksServerOperation::AuthenticationSessionEnded { user } => {
                self.authenticated_users.retain(|known| known != user);
            }
            SteamworksServerOperation::AuthenticationSessionTicketResponse { response } => {
                if response.result.is_err() {
                    self.active_auth_tickets
                        .retain(|known| *known != response.ticket);
                }
                self.last_auth_ticket_response = Some(response.clone());
            }
            SteamworksServerOperation::AuthenticationTicketValidationReceived { validation } => {
                if validation.response.is_err() {
                    self.authenticated_users
                        .retain(|known| *known != validation.steam_id);
                }
                self.last_auth_ticket_validation = Some(validation.clone());
            }
            SteamworksServerOperation::SteamServerConnectionEventReceived { event } => {
                self.steam_server_connected = Some(matches!(
                    event,
                    SteamworksSteamServerConnectionEvent::Connected
                ));
                self.last_steam_server_connection_event = Some(event.clone());
            }
            SteamworksServerOperation::ClientApproved { approval } => {
                if !self.authenticated_users.contains(&approval.user) {
                    self.authenticated_users.push(approval.user);
                }
                self.last_client_approval = Some(approval.clone());
            }
            SteamworksServerOperation::ClientDenied { denial } => {
                self.authenticated_users
                    .retain(|known| *known != denial.user);
                self.last_client_denial = Some(denial.clone());
            }
            SteamworksServerOperation::ClientKicked { kick } => {
                self.authenticated_users.retain(|known| *known != kick.user);
                self.last_client_kick = Some(kick.clone());
            }
            SteamworksServerOperation::ClientGroupStatusReceived { status } => {
                self.last_client_group_status = Some(status.clone());
            }
            SteamworksServerOperation::ProductSet { product } => {
                self.product = Some(product.clone());
            }
            SteamworksServerOperation::GameDescriptionSet { description } => {
                self.game_description = Some(description.clone());
            }
            SteamworksServerOperation::GameDataSet { data } => {
                self.game_data = Some(data.clone());
            }
            SteamworksServerOperation::DedicatedServerSet { dedicated } => {
                self.dedicated = Some(*dedicated);
            }
            SteamworksServerOperation::AnonymousLogonSubmitted => {
                self.anonymous_logon_submitted = true;
            }
            SteamworksServerOperation::AdvertiseServerActiveSet { active } => {
                self.advertise_server_active = Some(*active);
            }
            SteamworksServerOperation::ModDirSet { mod_dir } => {
                self.mod_dir = Some(mod_dir.clone());
            }
            SteamworksServerOperation::MapNameSet { map_name } => {
                self.map_name = Some(map_name.clone());
            }
            SteamworksServerOperation::ServerNameSet { server_name } => {
                self.server_name = Some(server_name.clone());
            }
            SteamworksServerOperation::MaxPlayersSet { count } => {
                self.max_players = Some(*count);
            }
            SteamworksServerOperation::GameTagsSet { tags } => {
                self.game_tags = Some(tags.clone());
            }
            SteamworksServerOperation::KeyValueSet { key, value } => {
                if let Some((_, known_value)) = self
                    .key_values
                    .iter_mut()
                    .find(|(known_key, _)| known_key == key)
                {
                    *known_value = value.clone();
                } else {
                    self.key_values.push((key.clone(), value.clone()));
                }
            }
            SteamworksServerOperation::AllKeyValuesCleared => {
                self.key_values.clear();
            }
            SteamworksServerOperation::PasswordProtectedSet { protected } => {
                self.password_protected = Some(*protected);
            }
            SteamworksServerOperation::BotPlayerCountSet { count } => {
                self.bot_player_count = Some(*count);
            }
            SteamworksServerOperation::IncomingPacketHandled { .. } => {}
            _ => {}
        }
    }

    fn logon_submitted(&self) -> bool {
        self.anonymous_logon_submitted
    }
}

/// Game-server client approval callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksServerClientApproval {
    /// Steam user approved to connect.
    pub user: steamworks::SteamId,
    /// Owner of the game license, which can differ from `user` for Family Sharing.
    pub owner: steamworks::SteamId,
}

/// Game-server client denial callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksServerClientDenial {
    /// Steam user denied by Steam.
    pub user: steamworks::SteamId,
    /// Denial reason reported by Steam.
    pub deny_reason: steamworks::DenyReason,
    /// Optional denial text reported by Steam.
    pub optional_text: String,
}

/// Game-server client kick callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksServerClientKick {
    /// Steam user kicked by Steam.
    pub user: steamworks::SteamId,
    /// Kick reason reported by Steam.
    pub deny_reason: steamworks::DenyReason,
}

/// Game-server group status callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksServerClientGroupStatus {
    /// Steam user whose group status was queried.
    pub user: steamworks::SteamId,
    /// Steam group ID.
    pub group: steamworks::SteamId,
    /// Whether the user is a member of the group.
    pub member: bool,
    /// Whether the user is an officer of the group.
    pub officer: bool,
}

/// A high-level command for Steam Game Server operations.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksServerCommand {
    /// Read the Steam ID of this game server.
    GetSteamId,
    /// Request an authentication session ticket for an entity identified by Steam ID.
    GetAuthenticationSessionTicket {
        /// Steam ID for the entity that will verify the ticket.
        steam_id: steamworks::SteamId,
    },
    /// Cancel a locally issued authentication ticket.
    CancelAuthenticationTicket {
        /// Ticket handle to cancel.
        ticket: steamworks::AuthTicket,
    },
    /// Begin validating a ticket received from another Steam user.
    BeginAuthenticationSession {
        /// Steam user that provided the ticket.
        user: steamworks::SteamId,
        /// Raw authentication ticket bytes.
        ticket: Vec<u8>,
    },
    /// End a session started with [`SteamworksServerCommand::BeginAuthenticationSession`].
    EndAuthenticationSession {
        /// Steam user whose authentication session should end.
        user: steamworks::SteamId,
    },
    /// Forward one shared-query-port packet to Steam.
    HandleIncomingPacket {
        /// Packet bytes received by the game server socket.
        data: Vec<u8>,
        /// Source address for the packet.
        addr: SocketAddrV4,
    },
    /// Set the game product identifier before server logon.
    SetProduct {
        /// Product identifier submitted to Steam.
        product: String,
    },
    /// Set the game description before server logon.
    SetGameDescription {
        /// Description submitted to Steam.
        description: String,
    },
    /// Set optional game data for server browser filtering.
    SetGameData {
        /// Game data string submitted to Steam.
        data: String,
    },
    /// Set whether this is a dedicated or listen server.
    SetDedicatedServer {
        /// Whether this is a dedicated server.
        dedicated: bool,
    },
    /// Submit anonymous server logon.
    LogOnAnonymous,
    /// Set whether Steam should advertise this server.
    SetAdvertiseServerActive {
        /// Whether this server should be advertised.
        active: bool,
    },
    /// Set the mod directory string.
    SetModDir {
        /// Mod directory submitted to Steam.
        mod_dir: String,
    },
    /// Set the map name reported in server browser data.
    SetMapName {
        /// Map name submitted to Steam.
        map_name: String,
    },
    /// Set the server name reported in server browser data.
    SetServerName {
        /// Server name submitted to Steam.
        server_name: String,
    },
    /// Set the maximum number of players.
    SetMaxPlayers {
        /// Maximum player count.
        count: i32,
    },
    /// Set game tags for server browser filtering.
    SetGameTags {
        /// Game tags submitted to Steam.
        tags: String,
    },
    /// Add or update a server rules key/value pair.
    SetKeyValue {
        /// Rule key.
        key: String,
        /// Rule value.
        value: String,
    },
    /// Clear all server rules key/value pairs.
    ClearAllKeyValues,
    /// Set whether this server is password protected.
    SetPasswordProtected {
        /// Whether this server requires a password.
        protected: bool,
    },
    /// Set the bot player count.
    SetBotPlayerCount {
        /// Bot player count.
        count: i32,
    },
}

impl SteamworksServerCommand {
    /// Creates a [`SteamworksServerCommand::GetAuthenticationSessionTicket`] command.
    pub fn get_authentication_session_ticket(steam_id: steamworks::SteamId) -> Self {
        Self::GetAuthenticationSessionTicket { steam_id }
    }

    /// Creates a [`SteamworksServerCommand::CancelAuthenticationTicket`] command.
    pub fn cancel_authentication_ticket(ticket: steamworks::AuthTicket) -> Self {
        Self::CancelAuthenticationTicket { ticket }
    }

    /// Creates a [`SteamworksServerCommand::BeginAuthenticationSession`] command.
    pub fn begin_authentication_session(
        user: steamworks::SteamId,
        ticket: impl Into<Vec<u8>>,
    ) -> Self {
        Self::BeginAuthenticationSession {
            user,
            ticket: ticket.into(),
        }
    }

    /// Creates a [`SteamworksServerCommand::EndAuthenticationSession`] command.
    pub fn end_authentication_session(user: steamworks::SteamId) -> Self {
        Self::EndAuthenticationSession { user }
    }

    /// Creates a [`SteamworksServerCommand::HandleIncomingPacket`] command.
    pub fn handle_incoming_packet(data: impl Into<Vec<u8>>, addr: SocketAddrV4) -> Self {
        Self::HandleIncomingPacket {
            data: data.into(),
            addr,
        }
    }

    /// Creates a [`SteamworksServerCommand::SetProduct`] command.
    pub fn set_product(product: impl Into<String>) -> Self {
        Self::SetProduct {
            product: product.into(),
        }
    }

    /// Creates a [`SteamworksServerCommand::SetGameDescription`] command.
    pub fn set_game_description(description: impl Into<String>) -> Self {
        Self::SetGameDescription {
            description: description.into(),
        }
    }

    /// Creates a [`SteamworksServerCommand::SetGameData`] command.
    pub fn set_game_data(data: impl Into<String>) -> Self {
        Self::SetGameData { data: data.into() }
    }

    /// Creates a [`SteamworksServerCommand::SetDedicatedServer`] command.
    pub fn set_dedicated_server(dedicated: bool) -> Self {
        Self::SetDedicatedServer { dedicated }
    }

    /// Creates a [`SteamworksServerCommand::SetAdvertiseServerActive`] command.
    pub fn set_advertise_server_active(active: bool) -> Self {
        Self::SetAdvertiseServerActive { active }
    }

    /// Creates a [`SteamworksServerCommand::SetModDir`] command.
    pub fn set_mod_dir(mod_dir: impl Into<String>) -> Self {
        Self::SetModDir {
            mod_dir: mod_dir.into(),
        }
    }

    /// Creates a [`SteamworksServerCommand::SetMapName`] command.
    pub fn set_map_name(map_name: impl Into<String>) -> Self {
        Self::SetMapName {
            map_name: map_name.into(),
        }
    }

    /// Creates a [`SteamworksServerCommand::SetServerName`] command.
    pub fn set_server_name(server_name: impl Into<String>) -> Self {
        Self::SetServerName {
            server_name: server_name.into(),
        }
    }

    /// Creates a [`SteamworksServerCommand::SetMaxPlayers`] command.
    pub fn set_max_players(count: i32) -> Self {
        Self::SetMaxPlayers { count }
    }

    /// Creates a [`SteamworksServerCommand::SetGameTags`] command.
    pub fn set_game_tags(tags: impl Into<String>) -> Self {
        Self::SetGameTags { tags: tags.into() }
    }

    /// Creates a [`SteamworksServerCommand::SetKeyValue`] command.
    pub fn set_key_value(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::SetKeyValue {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Creates a [`SteamworksServerCommand::SetPasswordProtected`] command.
    pub fn set_password_protected(protected: bool) -> Self {
        Self::SetPasswordProtected { protected }
    }

    /// Creates a [`SteamworksServerCommand::SetBotPlayerCount`] command.
    pub fn set_bot_player_count(count: i32) -> Self {
        Self::SetBotPlayerCount { count }
    }
}

/// A successfully submitted Steam Game Server operation or synchronous read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksServerOperation {
    /// The Steam ID of this game server was read.
    SteamIdRead {
        /// Steam ID reported by Steam.
        steam_id: steamworks::SteamId,
    },
    /// Authentication session ticket bytes were issued.
    AuthenticationSessionTicketIssued {
        /// Ticket handle that should be cancelled when no longer needed.
        ticket: steamworks::AuthTicket,
        /// Raw ticket bytes to send to the verifying entity.
        ticket_bytes: Vec<u8>,
        /// Steam ID used as the network identity for the verifier.
        steam_id: steamworks::SteamId,
    },
    /// A locally issued authentication ticket was cancelled.
    AuthenticationTicketCancelled {
        /// Ticket handle that was cancelled.
        ticket: steamworks::AuthTicket,
    },
    /// Authentication began for a remote user ticket.
    AuthenticationSessionStarted {
        /// Steam user whose ticket was accepted for validation.
        user: steamworks::SteamId,
    },
    /// Authentication ended for a remote user.
    AuthenticationSessionEnded {
        /// Steam user whose authentication session ended.
        user: steamworks::SteamId,
    },
    /// Authentication session ticket creation callback was observed.
    AuthenticationSessionTicketResponse {
        /// Ticket creation response reported by Steam.
        response: SteamworksAuthSessionTicketResponse,
    },
    /// Auth ticket validation callback was observed.
    AuthenticationTicketValidationReceived {
        /// Validation response reported by Steam.
        validation: SteamworksAuthTicketValidation,
    },
    /// Steam server connection state callback was observed.
    SteamServerConnectionEventReceived {
        /// Connection event reported by Steam.
        event: SteamworksSteamServerConnectionEvent,
    },
    /// Steam approved a game-server client.
    ClientApproved {
        /// Approval details.
        approval: SteamworksServerClientApproval,
    },
    /// Steam denied a game-server client.
    ClientDenied {
        /// Denial details.
        denial: SteamworksServerClientDenial,
    },
    /// Steam kicked a game-server client.
    ClientKicked {
        /// Kick details.
        kick: SteamworksServerClientKick,
    },
    /// Steam returned a group status result for a client.
    ClientGroupStatusReceived {
        /// Group status details.
        status: SteamworksServerClientGroupStatus,
    },
    /// A shared-query-port packet was forwarded to Steam.
    IncomingPacketHandled {
        /// Source address for the packet.
        addr: SocketAddrV4,
        /// Number of bytes forwarded.
        bytes: usize,
        /// Whether Steam accepted the packet.
        accepted: bool,
    },
    /// Product identifier was submitted.
    ProductSet {
        /// Product identifier submitted to Steam.
        product: String,
    },
    /// Game description was submitted.
    GameDescriptionSet {
        /// Description submitted to Steam.
        description: String,
    },
    /// Game data string was submitted.
    GameDataSet {
        /// Game data submitted to Steam.
        data: String,
    },
    /// Dedicated/listen server flag was submitted.
    DedicatedServerSet {
        /// Whether this is a dedicated server.
        dedicated: bool,
    },
    /// Anonymous server logon was submitted.
    AnonymousLogonSubmitted,
    /// Server advertisement flag was submitted.
    AdvertiseServerActiveSet {
        /// Whether this server should be advertised.
        active: bool,
    },
    /// Mod directory was submitted.
    ModDirSet {
        /// Mod directory submitted to Steam.
        mod_dir: String,
    },
    /// Map name was submitted.
    MapNameSet {
        /// Map name submitted to Steam.
        map_name: String,
    },
    /// Server name was submitted.
    ServerNameSet {
        /// Server name submitted to Steam.
        server_name: String,
    },
    /// Maximum player count was submitted.
    MaxPlayersSet {
        /// Maximum player count.
        count: i32,
    },
    /// Game tags were submitted.
    GameTagsSet {
        /// Tags submitted to Steam.
        tags: String,
    },
    /// Server rule key/value pair was submitted.
    KeyValueSet {
        /// Rule key.
        key: String,
        /// Rule value.
        value: String,
    },
    /// Server rules key/value pairs were cleared.
    AllKeyValuesCleared,
    /// Password-protected flag was submitted.
    PasswordProtectedSet {
        /// Whether this server requires a password.
        protected: bool,
    },
    /// Bot player count was submitted.
    BotPlayerCountSet {
        /// Bot player count.
        count: i32,
    },
}

/// Result message emitted by [`SteamworksServerPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksServerResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksServerOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksServerCommand,
        /// Failure reason.
        error: SteamworksServerError,
    },
}

/// Synchronous errors from [`SteamworksServerPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksServerError {
    /// No [`SteamworksServer`] resource exists.
    #[error("SteamworksServer resource is not available")]
    ServerUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steam Game Server command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A remote authentication session was requested with no ticket bytes.
    #[error("Steam Game Server command requires a non-empty authentication ticket")]
    EmptyTicket,
    /// A count field must be non-negative.
    #[error("Steam Game Server command field {field} must be non-negative, got {value}")]
    InvalidCount {
        /// Field that contained the invalid count.
        field: &'static str,
        /// Invalid count.
        value: i32,
    },
    /// Steam game tags must be non-empty and shorter than 128 bytes.
    #[error("Steam Game Server game tags must be non-empty and shorter than 128 bytes")]
    InvalidGameTags,
    /// The command must be submitted before the server logs on.
    #[error("Steam Game Server command {command} must be submitted before server logon")]
    CommandRequiresPreLogon {
        /// Command that must run before logon.
        command: &'static str,
    },
    /// The upstream Steamworks API rejected an authentication session.
    #[error("Steam Game Server authentication session failed: {source}")]
    AuthSession {
        /// Authentication session failure reason.
        #[source]
        source: SteamworksAuthSessionError,
    },
}

impl SteamworksServerError {
    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    fn invalid_count(field: &'static str, value: i32) -> Self {
        Self::InvalidCount { field, value }
    }

    fn auth_session(source: steamworks::AuthSessionError) -> Self {
        Self::AuthSession {
            source: source.into(),
        }
    }

    fn command_requires_pre_logon(command: &'static str) -> Self {
        Self::CommandRequiresPreLogon { command }
    }
}

/// Stores Steam Game Server callback handles so callbacks stay registered.
#[derive(Default, Resource)]
pub struct SteamworksServerCallbackRegistry {
    handles: Vec<steamworks::CallbackHandle>,
}

impl SteamworksServerCallbackRegistry {
    /// Registers a typed Steam Game Server callback and stores its handle.
    pub fn register<C, F>(&mut self, server: &SteamworksServer, callback: F)
    where
        C: steamworks::Callback,
        F: FnMut(C) + 'static + Send,
    {
        self.handles.push(server.register_callback(callback));
    }

    /// Drops every registered callback handle.
    pub fn clear(&mut self) {
        self.handles.clear();
    }

    /// Number of callback handles currently held.
    pub fn len(&self) -> usize {
        self.handles.len()
    }

    /// Returns true when no callback handles are held.
    pub fn is_empty(&self) -> bool {
        self.handles.is_empty()
    }
}

/// A Bevy plugin that integrates Steam Game Server callbacks into an app.
pub struct SteamworksServerPlugin {
    mode: SteamworksServerInitMode,
    failure_policy: SteamworksFailurePolicy,
    run_callbacks: bool,
    server: Mutex<Option<steamworks::Server>>,
}

impl SteamworksServerPlugin {
    /// Creates a plugin that initializes Steam Game Server from a config.
    pub fn new(config: SteamworksServerConfig) -> Self {
        Self::with_mode(SteamworksServerInitMode::Config(config))
    }

    /// Creates a plugin that does not initialize Steam Game Server.
    ///
    /// Use this when you insert [`SteamworksServer`] yourself, or when tests only
    /// need the plugin's schedule and message setup.
    pub fn manual() -> Self {
        Self::with_mode(SteamworksServerInitMode::Manual)
    }

    /// Initializes Steam Game Server immediately and wraps it.
    pub fn init(config: SteamworksServerConfig) -> Result<Self, SteamworksServerUnavailable> {
        validate_server_config(&config)?;
        let (server, _server_client) = steamworks::Server::init(
            config.ip,
            config.game_port,
            config.query_port,
            config.server_mode,
            &config.version,
        )
        .map_err(|source| SteamworksServerUnavailable::init_failed(config, source))?;
        Ok(Self::from_server(server))
    }

    /// Creates a plugin from an already initialized Steam Game Server.
    pub fn from_server(server: steamworks::Server) -> Self {
        Self {
            mode: SteamworksServerInitMode::Manual,
            failure_policy: SteamworksFailurePolicy::Panic,
            run_callbacks: true,
            server: Mutex::new(Some(server)),
        }
    }

    /// Sets the initialization failure policy.
    pub fn failure_policy(mut self, policy: SteamworksFailurePolicy) -> Self {
        self.failure_policy = policy;
        self
    }

    /// Keeps the Bevy app running when Steam Game Server cannot be initialized.
    ///
    /// The plugin will insert [`SteamworksServerUnavailable`] and emit a
    /// structured `tracing` error.
    pub fn log_and_continue(self) -> Self {
        self.failure_policy(SteamworksFailurePolicy::LogAndContinue)
    }

    /// Sets whether the plugin should automatically run Steam Game Server callbacks.
    pub fn run_callbacks(mut self, run_callbacks: bool) -> Self {
        self.run_callbacks = run_callbacks;
        self
    }

    fn with_mode(mode: SteamworksServerInitMode) -> Self {
        Self {
            mode,
            failure_policy: SteamworksFailurePolicy::Panic,
            run_callbacks: true,
            server: Mutex::new(None),
        }
    }

    fn initialize_server(&self) -> Result<steamworks::Server, SteamworksServerUnavailable> {
        let injected = self
            .server
            .lock()
            .expect("SteamworksServerPlugin server mutex was poisoned")
            .take();

        if let Some(server) = injected {
            return Ok(server);
        }

        match &self.mode {
            SteamworksServerInitMode::Config(config) => {
                validate_server_config(config)?;
                let (server, _server_client) = steamworks::Server::init(
                    config.ip,
                    config.game_port,
                    config.query_port,
                    config.server_mode,
                    &config.version,
                )
                .map_err(|source| {
                    SteamworksServerUnavailable::init_failed(config.clone(), source)
                })?;
                Ok(server)
            }
            SteamworksServerInitMode::Manual => {
                Err(SteamworksServerUnavailable::ManualServerMissing)
            }
        }
    }

    fn handle_unavailable(&self, app: &mut App, error: SteamworksServerUnavailable) {
        match self.failure_policy {
            SteamworksFailurePolicy::Panic => panic!("{error}"),
            SteamworksFailurePolicy::LogAndContinue => {
                tracing::error!(
                    target: "bevy_steamworks",
                    init_mode = ?self.mode,
                    error = %error,
                    "Steam Game Server unavailable"
                );
                app.insert_resource(error);
            }
        }
    }
}

impl From<steamworks::Server> for SteamworksServerPlugin {
    fn from(server: steamworks::Server) -> Self {
        Self::from_server(server)
    }
}

impl Plugin for SteamworksServerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksServerState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksServerCommand>()
            .add_message::<SteamworksServerResult>()
            .init_resource::<SteamworksServerCallbackRegistry>();

        if self.run_callbacks {
            app.configure_sets(First, SteamworksSystem::RunCallbacks)
                .add_systems(
                    First,
                    run_steam_server_callbacks
                        .in_set(SteamworksSystem::RunCallbacks)
                        .before(bevy_ecs::message::MessageUpdateSystems),
                );
        }

        app.configure_sets(
            First,
            SteamworksSystem::ProcessServerCommands
                .after(SteamworksSystem::RunCallbacks)
                .before(bevy_ecs::message::MessageUpdateSystems),
        )
        .add_systems(
            First,
            process_server_commands.in_set(SteamworksSystem::ProcessServerCommands),
        );

        if app.world().contains_resource::<SteamworksServer>() {
            tracing::debug!(
                target: "bevy_steamworks",
                init_mode = ?self.mode,
                "using existing SteamworksServer resource"
            );
            return;
        }

        match self.initialize_server() {
            Ok(server) => {
                tracing::info!(
                    target: "bevy_steamworks",
                    init_mode = ?self.mode,
                    "Steam Game Server initialized"
                );
                app.insert_resource(SteamworksServer::new(server));
            }
            Err(error) => self.handle_unavailable(app, error),
        }
    }
}

fn process_server_commands(
    server: Option<Res<SteamworksServer>>,
    mut state: ResMut<SteamworksServerState>,
    mut commands: ResMut<Messages<SteamworksServerCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksServerResult>,
) {
    process_server_steam_events(&mut state, &mut steam_events, &mut results);

    let Some(server) = server else {
        let error = SteamworksServerError::ServerUnavailable;
        for command in commands.drain() {
            state.record_error(error.clone());
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steam Game Server command failed"
            );
            results.write(SteamworksServerResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        match handle_server_command(&server, &state, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steam Game Server command"
                );
                results.write(SteamworksServerResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steam Game Server command failed"
                );
                results.write(SteamworksServerResult::Err { command, error });
            }
        }
    }
}

fn process_server_steam_events(
    state: &mut SteamworksServerState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksServerResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::AuthSessionTicketResponse(event) => {
                SteamworksServerOperation::AuthenticationSessionTicketResponse {
                    response: SteamworksAuthSessionTicketResponse {
                        ticket: event.ticket,
                        result: event.result,
                    },
                }
            }
            SteamworksEvent::ValidateAuthTicketResponse(event) => {
                SteamworksServerOperation::AuthenticationTicketValidationReceived {
                    validation: SteamworksAuthTicketValidation {
                        steam_id: event.steam_id,
                        owner_steam_id: event.owner_steam_id,
                        response: event.response.clone().map_err(Into::into),
                    },
                }
            }
            SteamworksEvent::SteamServersConnected(_) => {
                SteamworksServerOperation::SteamServerConnectionEventReceived {
                    event: SteamworksSteamServerConnectionEvent::Connected,
                }
            }
            SteamworksEvent::SteamServersDisconnected(event) => {
                SteamworksServerOperation::SteamServerConnectionEventReceived {
                    event: SteamworksSteamServerConnectionEvent::Disconnected {
                        reason: event.reason,
                    },
                }
            }
            SteamworksEvent::SteamServerConnectFailure(event) => {
                SteamworksServerOperation::SteamServerConnectionEventReceived {
                    event: SteamworksSteamServerConnectionEvent::ConnectFailure {
                        reason: event.reason,
                        still_retrying: event.still_retrying,
                    },
                }
            }
            SteamworksEvent::GSClientApprove(event) => SteamworksServerOperation::ClientApproved {
                approval: SteamworksServerClientApproval {
                    user: event.user,
                    owner: event.owner,
                },
            },
            SteamworksEvent::GSClientDeny(event) => SteamworksServerOperation::ClientDenied {
                denial: SteamworksServerClientDenial {
                    user: event.user,
                    deny_reason: event.deny_reason,
                    optional_text: event.optional_text.clone(),
                },
            },
            SteamworksEvent::GSClientKick(event) => SteamworksServerOperation::ClientKicked {
                kick: SteamworksServerClientKick {
                    user: event.user,
                    deny_reason: event.deny_reason,
                },
            },
            SteamworksEvent::GSClientGroupStatus(event) => {
                SteamworksServerOperation::ClientGroupStatusReceived {
                    status: SteamworksServerClientGroupStatus {
                        user: event.user,
                        group: event.group,
                        member: event.member,
                        officer: event.officer,
                    },
                }
            }
            _ => continue,
        };

        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steam Game Server callback"
        );
        results.write(SteamworksServerResult::Ok(operation));
    }
}

fn handle_server_command(
    server: &SteamworksServer,
    state: &SteamworksServerState,
    command: &SteamworksServerCommand,
) -> Result<SteamworksServerOperation, SteamworksServerError> {
    validate_server_command_for_state(command, state)?;

    Ok(match command {
        SteamworksServerCommand::GetSteamId => SteamworksServerOperation::SteamIdRead {
            steam_id: server.steam_id(),
        },
        SteamworksServerCommand::GetAuthenticationSessionTicket { steam_id } => {
            let (ticket, ticket_bytes) =
                server.authentication_session_ticket_with_steam_id(*steam_id);
            SteamworksServerOperation::AuthenticationSessionTicketIssued {
                ticket,
                ticket_bytes,
                steam_id: *steam_id,
            }
        }
        SteamworksServerCommand::CancelAuthenticationTicket { ticket } => {
            server.cancel_authentication_ticket(*ticket);
            SteamworksServerOperation::AuthenticationTicketCancelled { ticket: *ticket }
        }
        SteamworksServerCommand::BeginAuthenticationSession { user, ticket } => {
            server
                .begin_authentication_session(*user, ticket)
                .map_err(SteamworksServerError::auth_session)?;
            SteamworksServerOperation::AuthenticationSessionStarted { user: *user }
        }
        SteamworksServerCommand::EndAuthenticationSession { user } => {
            server.end_authentication_session(*user);
            SteamworksServerOperation::AuthenticationSessionEnded { user: *user }
        }
        SteamworksServerCommand::HandleIncomingPacket { data, addr } => {
            SteamworksServerOperation::IncomingPacketHandled {
                addr: *addr,
                bytes: data.len(),
                accepted: server.handle_incoming_packet(data, *addr),
            }
        }
        SteamworksServerCommand::SetProduct { product } => {
            server.set_product(product);
            SteamworksServerOperation::ProductSet {
                product: product.clone(),
            }
        }
        SteamworksServerCommand::SetGameDescription { description } => {
            server.set_game_description(description);
            SteamworksServerOperation::GameDescriptionSet {
                description: description.clone(),
            }
        }
        SteamworksServerCommand::SetGameData { data } => {
            server.set_game_data(data);
            SteamworksServerOperation::GameDataSet { data: data.clone() }
        }
        SteamworksServerCommand::SetDedicatedServer { dedicated } => {
            server.set_dedicated_server(*dedicated);
            SteamworksServerOperation::DedicatedServerSet {
                dedicated: *dedicated,
            }
        }
        SteamworksServerCommand::LogOnAnonymous => {
            server.log_on_anonymous();
            SteamworksServerOperation::AnonymousLogonSubmitted
        }
        SteamworksServerCommand::SetAdvertiseServerActive { active } => {
            server.set_advertise_server_active(*active);
            SteamworksServerOperation::AdvertiseServerActiveSet { active: *active }
        }
        SteamworksServerCommand::SetModDir { mod_dir } => {
            server.set_mod_dir(mod_dir);
            SteamworksServerOperation::ModDirSet {
                mod_dir: mod_dir.clone(),
            }
        }
        SteamworksServerCommand::SetMapName { map_name } => {
            server.set_map_name(map_name);
            SteamworksServerOperation::MapNameSet {
                map_name: map_name.clone(),
            }
        }
        SteamworksServerCommand::SetServerName { server_name } => {
            server.set_server_name(server_name);
            SteamworksServerOperation::ServerNameSet {
                server_name: server_name.clone(),
            }
        }
        SteamworksServerCommand::SetMaxPlayers { count } => {
            server.set_max_players(*count);
            SteamworksServerOperation::MaxPlayersSet { count: *count }
        }
        SteamworksServerCommand::SetGameTags { tags } => {
            server.set_game_tags(tags);
            SteamworksServerOperation::GameTagsSet { tags: tags.clone() }
        }
        SteamworksServerCommand::SetKeyValue { key, value } => {
            server.set_key_value(key, value);
            SteamworksServerOperation::KeyValueSet {
                key: key.clone(),
                value: value.clone(),
            }
        }
        SteamworksServerCommand::ClearAllKeyValues => {
            server.clear_all_key_values();
            SteamworksServerOperation::AllKeyValuesCleared
        }
        SteamworksServerCommand::SetPasswordProtected { protected } => {
            server.set_password_protected(*protected);
            SteamworksServerOperation::PasswordProtectedSet {
                protected: *protected,
            }
        }
        SteamworksServerCommand::SetBotPlayerCount { count } => {
            server.set_bot_player_count(*count);
            SteamworksServerOperation::BotPlayerCountSet { count: *count }
        }
    })
}

fn run_steam_server_callbacks(
    server: Option<Res<SteamworksServer>>,
    mut output: MessageWriter<SteamworksEvent>,
) {
    let Some(server) = server else {
        return;
    };

    server.process_callbacks(|callback| {
        output.write(SteamworksEvent::from(callback));
    });
}

fn validate_server_command(command: &SteamworksServerCommand) -> Result<(), SteamworksServerError> {
    match command {
        SteamworksServerCommand::BeginAuthenticationSession { ticket, .. } => {
            if ticket.is_empty() {
                Err(SteamworksServerError::EmptyTicket)
            } else {
                Ok(())
            }
        }
        SteamworksServerCommand::SetProduct { product } => {
            validate_steam_string("product", product)
        }
        SteamworksServerCommand::SetGameDescription { description } => {
            validate_steam_string("description", description)
        }
        SteamworksServerCommand::SetGameData { data } => validate_steam_string("data", data),
        SteamworksServerCommand::SetModDir { mod_dir } => validate_steam_string("mod_dir", mod_dir),
        SteamworksServerCommand::SetMapName { map_name } => {
            validate_steam_string("map_name", map_name)
        }
        SteamworksServerCommand::SetServerName { server_name } => {
            validate_steam_string("server_name", server_name)
        }
        SteamworksServerCommand::SetGameTags { tags } => {
            validate_steam_string("tags", tags)?;
            if tags.is_empty() || tags.len() >= 128 {
                Err(SteamworksServerError::InvalidGameTags)
            } else {
                Ok(())
            }
        }
        SteamworksServerCommand::SetKeyValue { key, value } => {
            validate_steam_string("key", key)?;
            validate_steam_string("value", value)
        }
        SteamworksServerCommand::SetMaxPlayers { count } => {
            validate_non_negative_count("count", *count)
        }
        SteamworksServerCommand::SetBotPlayerCount { count } => {
            validate_non_negative_count("count", *count)
        }
        _ => Ok(()),
    }
}

fn validate_server_command_for_state(
    command: &SteamworksServerCommand,
    state: &SteamworksServerState,
) -> Result<(), SteamworksServerError> {
    validate_server_command(command)?;

    if state.logon_submitted() {
        if let Some(command_name) = pre_logon_only_command_name(command) {
            return Err(SteamworksServerError::command_requires_pre_logon(
                command_name,
            ));
        }
    }

    Ok(())
}

fn pre_logon_only_command_name(command: &SteamworksServerCommand) -> Option<&'static str> {
    match command {
        SteamworksServerCommand::SetProduct { .. } => Some("SetProduct"),
        SteamworksServerCommand::SetGameDescription { .. } => Some("SetGameDescription"),
        _ => None,
    }
}

fn validate_server_config(
    config: &SteamworksServerConfig,
) -> Result<(), SteamworksServerUnavailable> {
    if config.version.as_bytes().contains(&0) {
        Err(SteamworksServerUnavailable::invalid_string("version"))
    } else {
        Ok(())
    }
}

fn validate_steam_string(field: &'static str, value: &str) -> Result<(), SteamworksServerError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksServerError::invalid_string(field))
    } else {
        Ok(())
    }
}

fn validate_non_negative_count(
    field: &'static str,
    value: i32,
) -> Result<(), SteamworksServerError> {
    if value < 0 {
        Err(SteamworksServerError::invalid_count(field, value))
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
    fn manual_mode_can_continue_without_server() {
        let mut app = App::new();

        app.add_plugins(SteamworksServerPlugin::manual().log_and_continue());

        assert!(app
            .world()
            .contains_resource::<SteamworksServerUnavailable>());
        assert!(app.world().contains_resource::<SteamworksServerState>());
        assert!(app
            .world()
            .contains_resource::<SteamworksServerCallbackRegistry>());
        assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksServerCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksServerResult>>());
        assert!(!app.world().contains_resource::<SteamworksServer>());

        app.update();
    }

    #[test]
    #[should_panic(expected = "manual Steam Game Server initialization was selected")]
    fn manual_mode_panics_by_default() {
        let mut app = App::new();

        app.add_plugins(SteamworksServerPlugin::manual());
    }

    #[test]
    fn invalid_version_can_continue_without_server() {
        let mut app = App::new();

        app.add_plugins(
            SteamworksServerPlugin::new(SteamworksServerConfig::new(
                Ipv4Addr::LOCALHOST,
                27015,
                27016,
                steamworks::ServerMode::Authentication,
                "bad\0version",
            ))
            .log_and_continue(),
        );

        assert_eq!(
            app.world().resource::<SteamworksServerUnavailable>(),
            &SteamworksServerUnavailable::InvalidString { field: "version" }
        );
        assert!(!app.world().contains_resource::<SteamworksServer>());
    }

    #[test]
    fn commands_fail_when_server_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksServerPlugin::manual().log_and_continue());
        app.world_mut()
            .resource_mut::<Messages<SteamworksServerCommand>>()
            .write(SteamworksServerCommand::GetSteamId);

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksServerResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksServerResult::Err {
                command: SteamworksServerCommand::GetSteamId,
                error: SteamworksServerError::ServerUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksServerState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksServerError::ServerUnavailable)
        );
    }

    #[test]
    fn server_callbacks_are_bridged_without_server() {
        let mut app = App::new();
        let user = steamworks::SteamId::from_raw(7);
        let owner = steamworks::SteamId::from_raw(8);
        let group = steamworks::SteamId::from_raw(9);

        app.add_plugins(SteamworksServerPlugin::manual().log_and_continue());
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::SteamServersConnected(
                steamworks::SteamServersConnected,
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::ValidateAuthTicketResponse(
                steamworks::ValidateAuthTicketResponse {
                    steam_id: user,
                    owner_steam_id: owner,
                    response: Ok(()),
                },
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::GSClientApprove(
                steamworks::GSClientApprove { user, owner },
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::ValidateAuthTicketResponse(
                steamworks::ValidateAuthTicketResponse {
                    steam_id: user,
                    owner_steam_id: owner,
                    response: Err(steamworks::AuthSessionValidateError::VACBanned),
                },
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::GSClientDeny(steamworks::GSClientDeny {
                user,
                deny_reason: steamworks::DenyReason::NoLicense,
                optional_text: "no license".to_string(),
            }));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::GSClientKick(steamworks::GSClientKick {
                user,
                deny_reason: steamworks::DenyReason::SteamConnectionLost,
            }));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::GSClientGroupStatus(
                steamworks::GSClientGroupStatus {
                    user,
                    group,
                    member: true,
                    officer: false,
                },
            ));

        app.update();

        let validation_failed = SteamworksAuthTicketValidation {
            steam_id: user,
            owner_steam_id: owner,
            response: Err(crate::SteamworksAuthSessionValidateError::VacBanned),
        };
        let approval = SteamworksServerClientApproval { user, owner };
        let denial = SteamworksServerClientDenial {
            user,
            deny_reason: steamworks::DenyReason::NoLicense,
            optional_text: "no license".to_string(),
        };
        let kick = SteamworksServerClientKick {
            user,
            deny_reason: steamworks::DenyReason::SteamConnectionLost,
        };
        let group_status = SteamworksServerClientGroupStatus {
            user,
            group,
            member: true,
            officer: false,
        };

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksServerResult>>();
        let drained = results.drain().collect::<Vec<_>>();
        assert_eq!(
            drained,
            vec![
                SteamworksServerResult::Ok(
                    SteamworksServerOperation::SteamServerConnectionEventReceived {
                        event: SteamworksSteamServerConnectionEvent::Connected,
                    },
                ),
                SteamworksServerResult::Ok(
                    SteamworksServerOperation::AuthenticationTicketValidationReceived {
                        validation: SteamworksAuthTicketValidation {
                            steam_id: user,
                            owner_steam_id: owner,
                            response: Ok(()),
                        },
                    },
                ),
                SteamworksServerResult::Ok(SteamworksServerOperation::ClientApproved {
                    approval: approval.clone(),
                }),
                SteamworksServerResult::Ok(
                    SteamworksServerOperation::AuthenticationTicketValidationReceived {
                        validation: validation_failed.clone(),
                    },
                ),
                SteamworksServerResult::Ok(SteamworksServerOperation::ClientDenied {
                    denial: denial.clone(),
                }),
                SteamworksServerResult::Ok(SteamworksServerOperation::ClientKicked {
                    kick: kick.clone(),
                }),
                SteamworksServerResult::Ok(SteamworksServerOperation::ClientGroupStatusReceived {
                    status: group_status.clone(),
                }),
            ]
        );

        let state = app.world().resource::<SteamworksServerState>();
        assert_eq!(state.steam_server_connected(), Some(true));
        assert_eq!(
            state.last_steam_server_connection_event(),
            Some(&SteamworksSteamServerConnectionEvent::Connected)
        );
        assert_eq!(
            state.last_auth_ticket_validation(),
            Some(&validation_failed)
        );
        assert_eq!(state.last_client_approval(), Some(&approval));
        assert_eq!(state.last_client_denial(), Some(&denial));
        assert_eq!(state.last_client_kick(), Some(&kick));
        assert_eq!(state.last_client_group_status(), Some(&group_status));
        assert!(state.authenticated_users().is_empty());
        assert_eq!(state.last_error(), None);
    }

    #[test]
    fn constructors_preserve_inputs() {
        let user = steamworks::SteamId::from_raw(7);
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 27015);

        assert_eq!(
            SteamworksServerCommand::get_authentication_session_ticket(user),
            SteamworksServerCommand::GetAuthenticationSessionTicket { steam_id: user }
        );
        assert_eq!(
            SteamworksServerCommand::begin_authentication_session(user, [1, 2, 3]),
            SteamworksServerCommand::BeginAuthenticationSession {
                user,
                ticket: vec![1, 2, 3],
            }
        );
        assert_eq!(
            SteamworksServerCommand::end_authentication_session(user),
            SteamworksServerCommand::EndAuthenticationSession { user }
        );
        assert_eq!(
            SteamworksServerCommand::handle_incoming_packet([255, 255, 255, 255], addr),
            SteamworksServerCommand::HandleIncomingPacket {
                data: vec![255, 255, 255, 255],
                addr,
            }
        );
        assert_eq!(
            SteamworksServerCommand::set_product("480"),
            SteamworksServerCommand::SetProduct {
                product: "480".to_string(),
            }
        );
        assert_eq!(
            SteamworksServerCommand::set_game_description("Spacewar"),
            SteamworksServerCommand::SetGameDescription {
                description: "Spacewar".to_string(),
            }
        );
        assert_eq!(
            SteamworksServerCommand::set_key_value("map", "arena"),
            SteamworksServerCommand::SetKeyValue {
                key: "map".to_string(),
                value: "arena".to_string(),
            }
        );
    }

    #[test]
    fn validation_rejects_inputs_that_would_panic_upstream() {
        assert_eq!(
            validate_server_command(&SteamworksServerCommand::set_product("bad\0product")),
            Err(SteamworksServerError::InvalidString { field: "product" })
        );
        assert_eq!(
            validate_server_command(&SteamworksServerCommand::set_key_value("bad\0key", "arena")),
            Err(SteamworksServerError::InvalidString { field: "key" })
        );
        assert_eq!(
            validate_server_command(&SteamworksServerCommand::set_key_value("map", "bad\0value")),
            Err(SteamworksServerError::InvalidString { field: "value" })
        );
        assert_eq!(
            validate_server_command(&SteamworksServerCommand::set_game_tags("")),
            Err(SteamworksServerError::InvalidGameTags)
        );
        assert_eq!(
            validate_server_command(&SteamworksServerCommand::set_game_tags("a".repeat(128))),
            Err(SteamworksServerError::InvalidGameTags)
        );
        assert_eq!(
            validate_server_command(&SteamworksServerCommand::set_max_players(-1)),
            Err(SteamworksServerError::InvalidCount {
                field: "count",
                value: -1,
            })
        );
        assert_eq!(
            validate_server_command(&SteamworksServerCommand::set_bot_player_count(-1)),
            Err(SteamworksServerError::InvalidCount {
                field: "count",
                value: -1,
            })
        );
        assert_eq!(
            validate_server_command(&SteamworksServerCommand::begin_authentication_session(
                steamworks::SteamId::from_raw(1),
                Vec::new(),
            )),
            Err(SteamworksServerError::EmptyTicket)
        );
    }

    #[test]
    fn validation_rejects_pre_logon_only_commands_after_logon() {
        let mut state = SteamworksServerState::default();

        assert_eq!(
            validate_server_command_for_state(&SteamworksServerCommand::set_product("480"), &state),
            Ok(())
        );

        state.record_operation(&SteamworksServerOperation::AnonymousLogonSubmitted);

        assert_eq!(
            validate_server_command_for_state(&SteamworksServerCommand::set_product("480"), &state),
            Err(SteamworksServerError::CommandRequiresPreLogon {
                command: "SetProduct",
            })
        );
        assert_eq!(
            validate_server_command_for_state(
                &SteamworksServerCommand::set_game_description("Spacewar"),
                &state
            ),
            Err(SteamworksServerError::CommandRequiresPreLogon {
                command: "SetGameDescription",
            })
        );
        assert_eq!(
            validate_server_command_for_state(
                &SteamworksServerCommand::set_server_name("Arena"),
                &state
            ),
            Ok(())
        );
    }

    #[test]
    fn state_records_server_operations() {
        let mut state = SteamworksServerState::default();
        let steam_id = steamworks::SteamId::from_raw(1);
        let user = steamworks::SteamId::from_raw(2);

        state.record_operation(&SteamworksServerOperation::SteamIdRead { steam_id });
        state.record_operation(&SteamworksServerOperation::AuthenticationSessionStarted { user });
        state.record_operation(&SteamworksServerOperation::ProductSet {
            product: "480".to_string(),
        });
        state.record_operation(&SteamworksServerOperation::GameDescriptionSet {
            description: "Spacewar".to_string(),
        });
        state.record_operation(&SteamworksServerOperation::GameDataSet {
            data: "mode=arena".to_string(),
        });
        state.record_operation(&SteamworksServerOperation::DedicatedServerSet { dedicated: true });
        state.record_operation(&SteamworksServerOperation::AnonymousLogonSubmitted);
        state.record_operation(&SteamworksServerOperation::AdvertiseServerActiveSet {
            active: true,
        });
        state.record_operation(&SteamworksServerOperation::ModDirSet {
            mod_dir: "spacewar".to_string(),
        });
        state.record_operation(&SteamworksServerOperation::MapNameSet {
            map_name: "arena".to_string(),
        });
        state.record_operation(&SteamworksServerOperation::ServerNameSet {
            server_name: "Test Server".to_string(),
        });
        state.record_operation(&SteamworksServerOperation::MaxPlayersSet { count: 16 });
        state.record_operation(&SteamworksServerOperation::GameTagsSet {
            tags: "arena,pvp".to_string(),
        });
        state.record_operation(&SteamworksServerOperation::KeyValueSet {
            key: "map".to_string(),
            value: "arena".to_string(),
        });
        state.record_operation(&SteamworksServerOperation::KeyValueSet {
            key: "map".to_string(),
            value: "arena2".to_string(),
        });
        state.record_operation(&SteamworksServerOperation::PasswordProtectedSet {
            protected: false,
        });
        state.record_operation(&SteamworksServerOperation::BotPlayerCountSet { count: 2 });

        assert_eq!(state.steam_id(), Some(steam_id));
        assert_eq!(state.authenticated_users(), &[user]);
        assert_eq!(state.product(), Some("480"));
        assert_eq!(state.game_description(), Some("Spacewar"));
        assert_eq!(state.game_data(), Some("mode=arena"));
        assert_eq!(state.dedicated(), Some(true));
        assert!(state.anonymous_logon_submitted());
        assert_eq!(state.advertise_server_active(), Some(true));
        assert_eq!(state.mod_dir(), Some("spacewar"));
        assert_eq!(state.map_name(), Some("arena"));
        assert_eq!(state.server_name(), Some("Test Server"));
        assert_eq!(state.max_players(), Some(16));
        assert_eq!(state.game_tags(), Some("arena,pvp"));
        assert_eq!(
            state.key_values(),
            &[("map".to_string(), "arena2".to_string())]
        );
        assert_eq!(state.password_protected(), Some(false));
        assert_eq!(state.bot_player_count(), Some(2));

        state.record_operation(&SteamworksServerOperation::AuthenticationSessionEnded { user });
        state.record_operation(&SteamworksServerOperation::AllKeyValuesCleared);

        assert!(state.authenticated_users().is_empty());
        assert!(state.key_values().is_empty());
    }

    #[test]
    fn server_callback_registry_tracks_handles() {
        let registry = SteamworksServerCallbackRegistry::default();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }
}
