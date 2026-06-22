use std::net::SocketAddrV4;

use bevy_ecs::message::Message;

use super::super::SteamworksServerLoginToken;

mod constructors;
mod debug;

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
    /// Enable or disable Steam master-server heartbeats.
    EnableHeartbeats {
        /// Whether Steam should send server heartbeats.
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
