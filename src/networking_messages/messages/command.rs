use bevy_ecs::message::Message;

use super::super::SteamworksNetworkingPeer;

/// A high-level command for Steam Networking Messages workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksNetworkingMessagesCommand {
    /// Send one payload to a peer on a channel.
    SendMessage {
        /// Remote peer.
        peer: SteamworksNetworkingPeer,
        /// Delivery flags.
        send_flags: steamworks::networking_types::SendFlags,
        /// Routing channel.
        channel: u32,
        /// Payload to send.
        data: Vec<u8>,
    },
    /// Receive up to `batch_size` messages from one channel.
    ReceiveMessages {
        /// Routing channel.
        channel: u32,
        /// Maximum number of messages to receive.
        batch_size: usize,
    },
    /// Read connection information for one peer.
    GetSessionConnectionInfo {
        /// Peer to inspect.
        peer: SteamworksNetworkingPeer,
    },
    /// Set whether future incoming session requests are accepted in the callback.
    SetAutoAcceptSessionRequests {
        /// Whether session requests should be accepted.
        enabled: bool,
    },
}

impl SteamworksNetworkingMessagesCommand {
    /// Creates a [`crate::SteamworksNetworkingMessagesCommand::SendMessage`] command.
    pub fn send_message(
        peer: SteamworksNetworkingPeer,
        send_flags: steamworks::networking_types::SendFlags,
        channel: u32,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self::SendMessage {
            peer,
            send_flags,
            channel,
            data: data.into(),
        }
    }

    /// Creates a command that sends a payload to a Steam user.
    pub fn send_message_to_steam_id(
        steam_id: steamworks::SteamId,
        send_flags: steamworks::networking_types::SendFlags,
        channel: u32,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self::send_message(
            SteamworksNetworkingPeer::steam_id(steam_id),
            send_flags,
            channel,
            data,
        )
    }

    /// Creates a [`crate::SteamworksNetworkingMessagesCommand::ReceiveMessages`] command.
    pub fn receive_messages(channel: u32, batch_size: usize) -> Self {
        Self::ReceiveMessages {
            channel,
            batch_size,
        }
    }

    /// Creates a [`crate::SteamworksNetworkingMessagesCommand::GetSessionConnectionInfo`] command.
    pub fn get_session_connection_info(peer: SteamworksNetworkingPeer) -> Self {
        Self::GetSessionConnectionInfo { peer }
    }

    /// Creates a [`crate::SteamworksNetworkingMessagesCommand::SetAutoAcceptSessionRequests`] command.
    pub fn set_auto_accept_session_requests(enabled: bool) -> Self {
        Self::SetAutoAcceptSessionRequests { enabled }
    }
}
