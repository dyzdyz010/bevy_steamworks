use std::{fmt, net::SocketAddrV4};

use bevy_ecs::message::Message;

use super::super::SteamworksServerLoginToken;

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
    /// Request an authentication session ticket for a specific networking identity.
    GetAuthenticationSessionTicketForIdentity {
        /// Networking identity for the entity that will verify the ticket.
        identity: steamworks::networking_types::NetworkingIdentity,
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
            Self::GetAuthenticationSessionTicketForIdentity { identity } => formatter
                .debug_struct("GetAuthenticationSessionTicketForIdentity")
                .field("identity", identity)
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
    /// Creates a [`SteamworksServerCommand::GetSteamId`] command.
    pub fn get_steam_id() -> Self {
        Self::GetSteamId
    }

    /// Creates a [`SteamworksServerCommand::GetAuthenticationSessionTicket`] command.
    pub fn get_authentication_session_ticket(steam_id: steamworks::SteamId) -> Self {
        Self::GetAuthenticationSessionTicket { steam_id }
    }

    /// Creates a [`SteamworksServerCommand::GetAuthenticationSessionTicketForIdentity`] command.
    pub fn get_authentication_session_ticket_for_identity(
        identity: steamworks::networking_types::NetworkingIdentity,
    ) -> Self {
        Self::GetAuthenticationSessionTicketForIdentity { identity }
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

    /// Creates a [`SteamworksServerCommand::LogOnAnonymous`] command.
    pub fn log_on_anonymous() -> Self {
        Self::LogOnAnonymous
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

    /// Creates a [`SteamworksServerCommand::ClearAllKeyValues`] command.
    pub fn clear_all_key_values() -> Self {
        Self::ClearAllKeyValues
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
