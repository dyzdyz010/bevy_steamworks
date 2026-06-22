use super::super::super::SteamworksNetworkingPeer;
use super::SteamworksNetworkingMessagesCommand;

impl SteamworksNetworkingMessagesCommand {
    /// Creates a [`crate::SteamworksNetworkingMessagesCommand::SendMessage`] command.
    pub fn send_message(
        peer: impl Into<SteamworksNetworkingPeer>,
        send_flags: steamworks::networking_types::SendFlags,
        channel: u32,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self::SendMessage {
            peer: peer.into(),
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
    pub fn get_session_connection_info(peer: impl Into<SteamworksNetworkingPeer>) -> Self {
        Self::GetSessionConnectionInfo { peer: peer.into() }
    }

    /// Creates a [`crate::SteamworksNetworkingMessagesCommand::SetAutoAcceptSessionRequests`] command.
    pub fn set_auto_accept_session_requests(enabled: bool) -> Self {
        Self::SetAutoAcceptSessionRequests { enabled }
    }
}
