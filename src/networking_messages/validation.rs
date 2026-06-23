use super::{
    messages::{SteamworksNetworkingMessagesCommand, SteamworksNetworkingMessagesError},
    types::SteamworksNetworkingPeer,
    STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE,
};

pub(super) fn validate_command(
    command: &SteamworksNetworkingMessagesCommand,
) -> Result<(), SteamworksNetworkingMessagesError> {
    match command {
        SteamworksNetworkingMessagesCommand::SendMessage { peer, channel, .. } => {
            validate_peer(peer)?;
            validate_channel(*channel)
        }
        SteamworksNetworkingMessagesCommand::ReceiveMessages {
            channel,
            batch_size,
        } => {
            validate_channel(*channel)?;
            if *batch_size == 0 {
                return Err(SteamworksNetworkingMessagesError::InvalidBatchSize);
            }
            if *batch_size > STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE {
                return Err(SteamworksNetworkingMessagesError::BatchSizeTooLarge {
                    batch_size: *batch_size,
                    max_batch_size: STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE,
                });
            }
            Ok(())
        }
        SteamworksNetworkingMessagesCommand::GetSessionConnectionInfo { peer } => {
            validate_peer(peer)
        }
        SteamworksNetworkingMessagesCommand::SetSessionRequestDecision { decision } => {
            validate_peer(&decision.peer)
        }
        SteamworksNetworkingMessagesCommand::ClearSessionRequestDecision { peer } => {
            validate_peer(peer)
        }
        SteamworksNetworkingMessagesCommand::SetAutoAcceptSessionRequests { .. } => Ok(()),
    }
}

fn validate_peer(peer: &SteamworksNetworkingPeer) -> Result<(), SteamworksNetworkingMessagesError> {
    match peer {
        SteamworksNetworkingPeer::SteamId(id) if id.is_invalid() => {
            Err(SteamworksNetworkingMessagesError::InvalidIdentity)
        }
        SteamworksNetworkingPeer::Identity(identity) if identity.is_invalid() => {
            Err(SteamworksNetworkingMessagesError::InvalidIdentity)
        }
        _ => Ok(()),
    }
}

fn validate_channel(channel: u32) -> Result<(), SteamworksNetworkingMessagesError> {
    if channel > i32::MAX as u32 {
        return Err(SteamworksNetworkingMessagesError::InvalidChannel { channel });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_rejects_invalid_inputs() {
        assert_eq!(
            validate_command(
                &SteamworksNetworkingMessagesCommand::send_message_to_steam_id(
                    steamworks::SteamId::from_raw(0),
                    steamworks::networking_types::SendFlags::RELIABLE,
                    0,
                    vec![1],
                )
            ),
            Err(SteamworksNetworkingMessagesError::InvalidIdentity)
        );
        assert_eq!(
            validate_command(
                &SteamworksNetworkingMessagesCommand::accept_session_requests_from(
                    steamworks::SteamId::from_raw(0),
                )
            ),
            Err(SteamworksNetworkingMessagesError::InvalidIdentity)
        );
        assert_eq!(
            validate_command(
                &SteamworksNetworkingMessagesCommand::clear_session_request_decision(
                    steamworks::SteamId::from_raw(0),
                )
            ),
            Err(SteamworksNetworkingMessagesError::InvalidIdentity)
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingMessagesCommand::receive_messages(
                i32::MAX as u32 + 1,
                1,
            )),
            Err(SteamworksNetworkingMessagesError::InvalidChannel {
                channel: i32::MAX as u32 + 1,
            })
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingMessagesCommand::receive_messages(0, 0)),
            Err(SteamworksNetworkingMessagesError::InvalidBatchSize)
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingMessagesCommand::receive_messages(
                0,
                STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE + 1,
            )),
            Err(SteamworksNetworkingMessagesError::BatchSizeTooLarge {
                batch_size: STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE + 1,
                max_batch_size: STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE,
            })
        );
    }
}
