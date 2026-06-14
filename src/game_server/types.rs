use std::{fmt, net::SocketAddrV4};

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

/// Login token for [`crate::SteamworksServerCommand::LogOn`].
///
/// The token is redacted from [`Debug`] output so command tracing does not leak it.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksServerLoginToken(String);

impl SteamworksServerLoginToken {
    /// Creates a login token wrapper.
    pub fn new(token: impl Into<String>) -> Self {
        Self(token.into())
    }

    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SteamworksServerLoginToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SteamworksServerLoginToken(<redacted>)")
    }
}

impl From<String> for SteamworksServerLoginToken {
    fn from(token: String) -> Self {
        Self::new(token)
    }
}

impl From<&str> for SteamworksServerLoginToken {
    fn from(token: &str) -> Self {
        Self::new(token)
    }
}

/// A shared-query outgoing packet produced by Steam Game Server.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksServerOutgoingPacket {
    /// Destination address for this packet.
    pub addr: SocketAddrV4,
    /// Packet bytes to send to `addr`.
    pub data: Vec<u8>,
}

impl fmt::Debug for SteamworksServerOutgoingPacket {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SteamworksServerOutgoingPacket")
            .field("addr", &self.addr)
            .field("data_len", &self.data.len())
            .finish()
    }
}

/// Auth session ticket issued through the game-server command layer.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksServerIssuedAuthSessionTicket {
    /// Ticket handle that should be cancelled when no longer needed.
    pub ticket: steamworks::AuthTicket,
    /// Raw ticket bytes returned by Steam.
    ///
    /// Treat this as credential material; avoid logging it or storing it longer than needed.
    pub ticket_bytes: Vec<u8>,
    /// Steam ID used as the network identity for the verifier.
    pub steam_id: steamworks::SteamId,
}

impl fmt::Debug for SteamworksServerIssuedAuthSessionTicket {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SteamworksServerIssuedAuthSessionTicket")
            .field("ticket", &self.ticket)
            .field("ticket_bytes_len", &self.ticket_bytes.len())
            .field("steam_id", &self.steam_id)
            .finish()
    }
}

/// Shared-query incoming packet forwarded to Steam Game Server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksServerIncomingPacket {
    /// Source address for the packet.
    pub addr: SocketAddrV4,
    /// Number of bytes forwarded.
    pub bytes: usize,
    /// Whether Steam accepted the packet.
    pub accepted: bool,
}
