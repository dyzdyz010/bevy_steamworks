use bevy_ecs::message::Message;

use super::super::SteamworksNetworkingPeer;

mod constructors;

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
