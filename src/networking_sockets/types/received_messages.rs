use super::*;

/// Owned snapshot of one received Networking Sockets message.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsMessage {
    /// Connection that received the message.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Remote peer identity carried by Steam's message.
    pub peer: steamworks::networking_types::NetworkingIdentity,
    /// Message payload copied from Steam's message handle.
    pub data: Vec<u8>,
    /// Message lane/channel.
    pub channel: i32,
    /// Message flags reported by Steam.
    pub send_flags: steamworks::networking_types::SendFlags,
    /// Message number assigned by the sender.
    pub message_number: u64,
    /// Connection user data captured by Steam for this message.
    pub connection_user_data: i64,
}

/// Message batch received from one connection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsConnectionMessages {
    /// Connection that was received from.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Owned message snapshots.
    pub messages: Vec<SteamworksNetworkingSocketsMessage>,
}

/// Owned snapshot of one message received through a poll group.
///
/// The upstream safe wrapper does not expose the raw connection handle carried
/// by poll-group messages. Use [`crate::SteamworksNetworkingSocketsCommand::SetConnectionUserData`]
/// to attach an application-level connection identifier if you need to map
/// poll-group messages back to game state.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsPollGroupMessage {
    /// Poll group that received the message.
    pub poll_group: SteamworksNetworkingSocketsPollGroupId,
    /// Remote peer identity carried by Steam's message.
    pub peer: steamworks::networking_types::NetworkingIdentity,
    /// Message payload copied from Steam's message handle.
    pub data: Vec<u8>,
    /// Message lane/channel.
    pub channel: i32,
    /// Message flags reported by Steam.
    pub send_flags: steamworks::networking_types::SendFlags,
    /// Message number assigned by the sender.
    pub message_number: u64,
    /// Connection user data captured by Steam for this message.
    pub connection_user_data: i64,
}

/// Message batch received from one poll group.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsPollGroupMessages {
    /// Poll group that was received from.
    pub poll_group: SteamworksNetworkingSocketsPollGroupId,
    /// Owned message snapshots.
    pub messages: Vec<SteamworksNetworkingSocketsPollGroupMessage>,
}
