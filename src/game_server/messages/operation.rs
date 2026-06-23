use std::net::SocketAddrV4;

use crate::user::{
    SteamworksAuthSessionTicketResponse, SteamworksAuthTicketValidation,
    SteamworksSteamServerConnectionEvent,
};

use super::super::{
    SteamworksServerClientApproval, SteamworksServerClientDenial,
    SteamworksServerClientGroupStatus, SteamworksServerClientKick, SteamworksServerOutgoingPacket,
};

mod debug;

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
    /// Authentication session ticket bytes were issued for a networking identity.
    AuthenticationSessionTicketForIdentityIssued {
        /// Ticket handle that should be cancelled when no longer needed.
        ticket: steamworks::AuthTicket,
        /// Raw ticket bytes to send to the verifying entity.
        ticket_bytes: Vec<u8>,
        /// Networking identity used for the verifier.
        identity: steamworks::networking_types::NetworkingIdentity,
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
    /// Steam master-server heartbeat flag was submitted.
    HeartbeatsEnabled {
        /// Whether Steam should send server heartbeats.
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
