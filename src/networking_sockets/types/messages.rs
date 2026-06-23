use super::*;

/// Sent-message snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsSentMessage {
    /// Connection sent on.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Assigned message number.
    pub message_number: u64,
    /// Payload size in bytes.
    pub bytes: usize,
}

/// Owned outbound Networking Sockets message submitted by a batch send command.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsOutboundMessage {
    /// Connection to send on.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Delivery flags.
    pub send_flags: steamworks::networking_types::SendFlags,
    /// Message lane/channel.
    pub channel: i32,
    /// Payload copied into an upstream message before sending.
    pub data: Vec<u8>,
    /// Application user data attached to the outgoing message.
    pub user_data: i64,
}

impl SteamworksNetworkingSocketsOutboundMessage {
    /// Creates an outbound message for channel `0` with no user data.
    pub fn new(
        connection: SteamworksNetworkingSocketsConnectionId,
        send_flags: steamworks::networking_types::SendFlags,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            connection,
            send_flags,
            channel: 0,
            data: data.into(),
            user_data: 0,
        }
    }

    /// Sets the message lane/channel.
    pub fn with_channel(mut self, channel: i32) -> Self {
        self.channel = channel;
        self
    }

    /// Sets application user data on the outgoing message.
    pub fn with_user_data(mut self, user_data: i64) -> Self {
        self.user_data = user_data;
        self
    }
}

/// Per-message result returned by a batch send command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingSocketsMessageSendResult {
    /// Connection that this message targeted.
    pub connection: SteamworksNetworkingSocketsConnectionId,
    /// Delivery flags used by this message.
    pub send_flags: steamworks::networking_types::SendFlags,
    /// Message lane/channel.
    pub channel: i32,
    /// Payload size in bytes.
    pub bytes: usize,
    /// Application user data that was attached to the outgoing message.
    pub user_data: i64,
    /// Message number assigned by Steam, or the per-message Steam send error.
    pub result: Result<u64, steamworks::SteamError>,
}
