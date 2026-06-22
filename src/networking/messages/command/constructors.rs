use super::super::super::SteamworksP2pSendType;
use super::SteamworksNetworkingCommand;

impl SteamworksNetworkingCommand {
    /// Creates a [`crate::SteamworksNetworkingCommand::AcceptP2pSession`] command.
    pub fn accept_p2p_session(user: steamworks::SteamId) -> Self {
        Self::AcceptP2pSession { user }
    }

    /// Creates a [`crate::SteamworksNetworkingCommand::CloseP2pSession`] command.
    pub fn close_p2p_session(user: steamworks::SteamId) -> Self {
        Self::CloseP2pSession { user }
    }

    /// Creates a [`crate::SteamworksNetworkingCommand::GetP2pSessionState`] command.
    pub fn get_p2p_session_state(user: steamworks::SteamId) -> Self {
        Self::GetP2pSessionState { user }
    }

    /// Creates a [`crate::SteamworksNetworkingCommand::SendP2pPacket`] command.
    pub fn send_p2p_packet(
        remote: steamworks::SteamId,
        send_type: SteamworksP2pSendType,
        channel: u32,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self::SendP2pPacket {
            remote,
            send_type,
            channel,
            data: data.into(),
        }
    }

    /// Creates a [`crate::SteamworksNetworkingCommand::GetAvailablePacketSize`] command.
    pub fn get_available_packet_size(channel: u32) -> Self {
        Self::GetAvailablePacketSize { channel }
    }

    /// Creates a [`crate::SteamworksNetworkingCommand::ReadP2pPacket`] command.
    pub fn read_p2p_packet(channel: u32, max_bytes: usize) -> Self {
        Self::ReadP2pPacket { channel, max_bytes }
    }
}
