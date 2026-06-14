use super::{
    messages::{SteamworksNetworkingCommand, SteamworksNetworkingError},
    STEAMWORKS_P2P_MAX_READ_PACKET_BYTES,
};

pub(super) fn validate_command(
    command: &SteamworksNetworkingCommand,
) -> Result<(), SteamworksNetworkingError> {
    match command {
        SteamworksNetworkingCommand::AcceptP2pSession { user }
        | SteamworksNetworkingCommand::CloseP2pSession { user }
        | SteamworksNetworkingCommand::GetP2pSessionState { user } => validate_steam_id(*user),
        SteamworksNetworkingCommand::SendP2pPacket {
            remote,
            send_type,
            channel,
            data,
        } => {
            validate_steam_id(*remote)?;
            validate_channel(*channel)?;
            let max_bytes = send_type.max_packet_bytes();
            if data.len() > max_bytes {
                return Err(SteamworksNetworkingError::PacketTooLarge {
                    bytes: data.len(),
                    max_bytes,
                });
            }
            Ok(())
        }
        SteamworksNetworkingCommand::GetAvailablePacketSize { channel } => {
            validate_channel(*channel)
        }
        SteamworksNetworkingCommand::ReadP2pPacket { channel, max_bytes } => {
            validate_channel(*channel)?;
            if *max_bytes == 0 {
                return Err(SteamworksNetworkingError::InvalidReadBufferSize);
            }
            if *max_bytes > STEAMWORKS_P2P_MAX_READ_PACKET_BYTES {
                return Err(SteamworksNetworkingError::ReadBufferTooLarge {
                    max_bytes: *max_bytes,
                    max_supported: STEAMWORKS_P2P_MAX_READ_PACKET_BYTES,
                });
            }
            Ok(())
        }
    }
}

fn validate_steam_id(user: steamworks::SteamId) -> Result<(), SteamworksNetworkingError> {
    if user.raw() == 0 {
        return Err(SteamworksNetworkingError::InvalidSteamId);
    }
    Ok(())
}

fn validate_channel(channel: u32) -> Result<(), SteamworksNetworkingError> {
    if channel > i32::MAX as u32 {
        return Err(SteamworksNetworkingError::InvalidChannel { channel });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        SteamworksP2pSendType, STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES,
        STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES,
    };

    fn user() -> steamworks::SteamId {
        steamworks::SteamId::from_raw(42)
    }

    #[test]
    fn validation_rejects_invalid_inputs() {
        assert_eq!(
            validate_command(&SteamworksNetworkingCommand::accept_p2p_session(
                steamworks::SteamId::from_raw(0),
            )),
            Err(SteamworksNetworkingError::InvalidSteamId)
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingCommand::get_available_packet_size(
                i32::MAX as u32 + 1,
            )),
            Err(SteamworksNetworkingError::InvalidChannel {
                channel: i32::MAX as u32 + 1,
            })
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingCommand::send_p2p_packet(
                user(),
                SteamworksP2pSendType::Unreliable,
                0,
                vec![0; STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES + 1],
            )),
            Err(SteamworksNetworkingError::PacketTooLarge {
                bytes: STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES + 1,
                max_bytes: STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES,
            })
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingCommand::send_p2p_packet(
                user(),
                SteamworksP2pSendType::Reliable,
                0,
                vec![0; STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES + 1],
            )),
            Err(SteamworksNetworkingError::PacketTooLarge {
                bytes: STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES + 1,
                max_bytes: STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES,
            })
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingCommand::read_p2p_packet(0, 0)),
            Err(SteamworksNetworkingError::InvalidReadBufferSize)
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingCommand::read_p2p_packet(
                0,
                STEAMWORKS_P2P_MAX_READ_PACKET_BYTES + 1,
            )),
            Err(SteamworksNetworkingError::ReadBufferTooLarge {
                max_bytes: STEAMWORKS_P2P_MAX_READ_PACKET_BYTES + 1,
                max_supported: STEAMWORKS_P2P_MAX_READ_PACKET_BYTES,
            })
        );
    }
}
