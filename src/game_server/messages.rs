use std::{fmt, net::SocketAddrV4};

use bevy_ecs::message::Message;
use thiserror::Error;

use crate::user::{
    SteamworksAuthSessionError, SteamworksAuthSessionTicketResponse,
    SteamworksAuthTicketValidation, SteamworksSteamServerConnectionEvent,
};

use super::*;

/// A high-level command for Steam Game Server operations.
#[derive(Clone, Message, PartialEq, Eq)]
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
    /// Submit token-based server logon.
    LogOn {
        /// Redacted login token.
        token: SteamworksServerLoginToken,
    },
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
    /// Drain queued shared-query outgoing packets from Steam.
    DrainOutgoingPackets,
}

impl fmt::Debug for SteamworksServerCommand {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GetSteamId => formatter.write_str("GetSteamId"),
            Self::GetAuthenticationSessionTicket { steam_id } => formatter
                .debug_struct("GetAuthenticationSessionTicket")
                .field("steam_id", steam_id)
                .finish(),
            Self::CancelAuthenticationTicket { ticket } => formatter
                .debug_struct("CancelAuthenticationTicket")
                .field("ticket", ticket)
                .finish(),
            Self::BeginAuthenticationSession { user, ticket } => formatter
                .debug_struct("BeginAuthenticationSession")
                .field("user", user)
                .field("ticket_len", &ticket.len())
                .finish(),
            Self::EndAuthenticationSession { user } => formatter
                .debug_struct("EndAuthenticationSession")
                .field("user", user)
                .finish(),
            Self::HandleIncomingPacket { data, addr } => formatter
                .debug_struct("HandleIncomingPacket")
                .field("data_len", &data.len())
                .field("addr", addr)
                .finish(),
            Self::SetProduct { product } => formatter
                .debug_struct("SetProduct")
                .field("product", product)
                .finish(),
            Self::SetGameDescription { description } => formatter
                .debug_struct("SetGameDescription")
                .field("description", description)
                .finish(),
            Self::SetGameData { data } => formatter
                .debug_struct("SetGameData")
                .field("data", data)
                .finish(),
            Self::SetDedicatedServer { dedicated } => formatter
                .debug_struct("SetDedicatedServer")
                .field("dedicated", dedicated)
                .finish(),
            Self::LogOnAnonymous => formatter.write_str("LogOnAnonymous"),
            Self::LogOn { token } => formatter
                .debug_struct("LogOn")
                .field("token", token)
                .finish(),
            Self::SetAdvertiseServerActive { active } => formatter
                .debug_struct("SetAdvertiseServerActive")
                .field("active", active)
                .finish(),
            Self::SetModDir { mod_dir } => formatter
                .debug_struct("SetModDir")
                .field("mod_dir", mod_dir)
                .finish(),
            Self::SetMapName { map_name } => formatter
                .debug_struct("SetMapName")
                .field("map_name", map_name)
                .finish(),
            Self::SetServerName { server_name } => formatter
                .debug_struct("SetServerName")
                .field("server_name", server_name)
                .finish(),
            Self::SetMaxPlayers { count } => formatter
                .debug_struct("SetMaxPlayers")
                .field("count", count)
                .finish(),
            Self::SetGameTags { tags } => formatter
                .debug_struct("SetGameTags")
                .field("tags", tags)
                .finish(),
            Self::SetKeyValue { key, value } => formatter
                .debug_struct("SetKeyValue")
                .field("key", key)
                .field("value", value)
                .finish(),
            Self::ClearAllKeyValues => formatter.write_str("ClearAllKeyValues"),
            Self::SetPasswordProtected { protected } => formatter
                .debug_struct("SetPasswordProtected")
                .field("protected", protected)
                .finish(),
            Self::SetBotPlayerCount { count } => formatter
                .debug_struct("SetBotPlayerCount")
                .field("count", count)
                .finish(),
            Self::DrainOutgoingPackets => formatter.write_str("DrainOutgoingPackets"),
        }
    }
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

    /// Creates a [`SteamworksServerCommand::LogOn`] command.
    pub fn log_on(token: impl Into<SteamworksServerLoginToken>) -> Self {
        Self::LogOn {
            token: token.into(),
        }
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

    /// Creates a [`SteamworksServerCommand::DrainOutgoingPackets`] command.
    pub fn drain_outgoing_packets() -> Self {
        Self::DrainOutgoingPackets
    }
}

/// A successfully submitted Steam Game Server operation or synchronous read.
#[derive(Clone, PartialEq, Eq)]
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
    /// Token-based server logon was submitted.
    TokenLogonSubmitted,
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
    /// Shared-query outgoing packets were drained from Steam.
    OutgoingPacketsDrained {
        /// Packets to send through the game server socket.
        packets: Vec<SteamworksServerOutgoingPacket>,
    },
}

impl fmt::Debug for SteamworksServerOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SteamIdRead { steam_id } => formatter
                .debug_struct("SteamIdRead")
                .field("steam_id", steam_id)
                .finish(),
            Self::AuthenticationSessionTicketIssued {
                ticket,
                ticket_bytes,
                steam_id,
            } => formatter
                .debug_struct("AuthenticationSessionTicketIssued")
                .field("ticket", ticket)
                .field("ticket_bytes_len", &ticket_bytes.len())
                .field("steam_id", steam_id)
                .finish(),
            Self::AuthenticationTicketCancelled { ticket } => formatter
                .debug_struct("AuthenticationTicketCancelled")
                .field("ticket", ticket)
                .finish(),
            Self::AuthenticationSessionStarted { user } => formatter
                .debug_struct("AuthenticationSessionStarted")
                .field("user", user)
                .finish(),
            Self::AuthenticationSessionEnded { user } => formatter
                .debug_struct("AuthenticationSessionEnded")
                .field("user", user)
                .finish(),
            Self::AuthenticationSessionTicketResponse { response } => formatter
                .debug_struct("AuthenticationSessionTicketResponse")
                .field("response", response)
                .finish(),
            Self::AuthenticationTicketValidationReceived { validation } => formatter
                .debug_struct("AuthenticationTicketValidationReceived")
                .field("validation", validation)
                .finish(),
            Self::SteamServerConnectionEventReceived { event } => formatter
                .debug_struct("SteamServerConnectionEventReceived")
                .field("event", event)
                .finish(),
            Self::ClientApproved { approval } => formatter
                .debug_struct("ClientApproved")
                .field("approval", approval)
                .finish(),
            Self::ClientDenied { denial } => formatter
                .debug_struct("ClientDenied")
                .field("denial", denial)
                .finish(),
            Self::ClientKicked { kick } => formatter
                .debug_struct("ClientKicked")
                .field("kick", kick)
                .finish(),
            Self::ClientGroupStatusReceived { status } => formatter
                .debug_struct("ClientGroupStatusReceived")
                .field("status", status)
                .finish(),
            Self::IncomingPacketHandled {
                addr,
                bytes,
                accepted,
            } => formatter
                .debug_struct("IncomingPacketHandled")
                .field("addr", addr)
                .field("bytes", bytes)
                .field("accepted", accepted)
                .finish(),
            Self::ProductSet { product } => formatter
                .debug_struct("ProductSet")
                .field("product", product)
                .finish(),
            Self::GameDescriptionSet { description } => formatter
                .debug_struct("GameDescriptionSet")
                .field("description", description)
                .finish(),
            Self::GameDataSet { data } => formatter
                .debug_struct("GameDataSet")
                .field("data", data)
                .finish(),
            Self::DedicatedServerSet { dedicated } => formatter
                .debug_struct("DedicatedServerSet")
                .field("dedicated", dedicated)
                .finish(),
            Self::AnonymousLogonSubmitted => formatter.write_str("AnonymousLogonSubmitted"),
            Self::TokenLogonSubmitted => formatter.write_str("TokenLogonSubmitted"),
            Self::AdvertiseServerActiveSet { active } => formatter
                .debug_struct("AdvertiseServerActiveSet")
                .field("active", active)
                .finish(),
            Self::ModDirSet { mod_dir } => formatter
                .debug_struct("ModDirSet")
                .field("mod_dir", mod_dir)
                .finish(),
            Self::MapNameSet { map_name } => formatter
                .debug_struct("MapNameSet")
                .field("map_name", map_name)
                .finish(),
            Self::ServerNameSet { server_name } => formatter
                .debug_struct("ServerNameSet")
                .field("server_name", server_name)
                .finish(),
            Self::MaxPlayersSet { count } => formatter
                .debug_struct("MaxPlayersSet")
                .field("count", count)
                .finish(),
            Self::GameTagsSet { tags } => formatter
                .debug_struct("GameTagsSet")
                .field("tags", tags)
                .finish(),
            Self::KeyValueSet { key, value } => formatter
                .debug_struct("KeyValueSet")
                .field("key", key)
                .field("value", value)
                .finish(),
            Self::AllKeyValuesCleared => formatter.write_str("AllKeyValuesCleared"),
            Self::PasswordProtectedSet { protected } => formatter
                .debug_struct("PasswordProtectedSet")
                .field("protected", protected)
                .finish(),
            Self::BotPlayerCountSet { count } => formatter
                .debug_struct("BotPlayerCountSet")
                .field("count", count)
                .finish(),
            Self::OutgoingPacketsDrained { packets } => formatter
                .debug_struct("OutgoingPacketsDrained")
                .field("packets", packets)
                .finish(),
        }
    }
}

/// Result message emitted by [`crate::SteamworksServerPlugin`].
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

/// Synchronous errors from [`crate::SteamworksServerPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksServerError {
    /// No [`crate::SteamworksServer`] resource exists.
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
    /// Token-based server logon was requested with no token.
    #[error("Steam Game Server token logon requires a non-empty token")]
    EmptyLogonToken,
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
    /// A server logon command was submitted after logon had already been submitted.
    #[error("Steam Game Server logon has already been submitted")]
    LogonAlreadySubmitted,
    /// The upstream Steamworks API rejected an authentication session.
    #[error("Steam Game Server authentication session failed: {source}")]
    AuthSession {
        /// Authentication session failure reason.
        #[source]
        source: SteamworksAuthSessionError,
    },
}

impl SteamworksServerError {
    pub(super) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    pub(super) fn invalid_count(field: &'static str, value: i32) -> Self {
        Self::InvalidCount { field, value }
    }

    pub(super) fn auth_session(source: steamworks::AuthSessionError) -> Self {
        Self::AuthSession {
            source: source.into(),
        }
    }

    pub(super) fn command_requires_pre_logon(command: &'static str) -> Self {
        Self::CommandRequiresPreLogon { command }
    }
}
