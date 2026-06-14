use super::{
    SteamworksConnectionRequestPolicy, SteamworksNetworkingSocketsCommand,
    SteamworksNetworkingSocketsError, STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES,
    STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND,
    STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND,
    STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES,
    STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES,
};

pub(super) fn validate_command(
    command: &SteamworksNetworkingSocketsCommand,
) -> Result<(), SteamworksNetworkingSocketsError> {
    match command {
        SteamworksNetworkingSocketsCommand::CreateListenSocketP2p { local_virtual_port } => {
            validate_virtual_port(*local_virtual_port)
        }
        SteamworksNetworkingSocketsCommand::ConnectP2p {
            remote_virtual_port,
            ..
        } => validate_virtual_port(*remote_virtual_port),
        SteamworksNetworkingSocketsCommand::PollListenSocketEvents {
            max_events,
            request_policy,
            ..
        } => {
            validate_event_limit(*max_events)?;
            validate_request_policy(request_policy)
        }
        SteamworksNetworkingSocketsCommand::PollConnectionEvents { max_events, .. } => {
            validate_event_limit(*max_events)
        }
        SteamworksNetworkingSocketsCommand::ReceiveMessages { batch_size, .. } => {
            validate_message_batch_size(*batch_size)
        }
        SteamworksNetworkingSocketsCommand::ReceivePollGroupMessages { batch_size, .. } => {
            validate_message_batch_size(*batch_size)
        }
        SteamworksNetworkingSocketsCommand::SendMessage { data, .. } => {
            if data.len() > STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES {
                return Err(SteamworksNetworkingSocketsError::MessageTooLarge {
                    bytes: data.len(),
                    max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES,
                });
            }
            Ok(())
        }
        SteamworksNetworkingSocketsCommand::GetRealtimeConnectionStatus { lanes, .. } => {
            if *lanes > STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES {
                return Err(SteamworksNetworkingSocketsError::InvalidLaneCount {
                    lanes: *lanes,
                    max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES,
                });
            }
            Ok(())
        }
        SteamworksNetworkingSocketsCommand::ConfigureConnectionLanes {
            lane_priorities,
            lane_weights,
            ..
        } => validate_lane_configuration(lane_priorities, lane_weights),
        SteamworksNetworkingSocketsCommand::CloseConnection { debug, .. } => {
            if debug
                .as_ref()
                .is_some_and(|value| value.as_bytes().contains(&0))
            {
                return Err(SteamworksNetworkingSocketsError::invalid_string("debug"));
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn validate_event_limit(max_events: usize) -> Result<(), SteamworksNetworkingSocketsError> {
    if max_events == 0 {
        return Err(SteamworksNetworkingSocketsError::InvalidEventLimit);
    }
    if max_events > STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND {
        return Err(SteamworksNetworkingSocketsError::TooManyEvents {
            requested: max_events,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND,
        });
    }
    Ok(())
}

fn validate_message_batch_size(batch_size: usize) -> Result<(), SteamworksNetworkingSocketsError> {
    if batch_size == 0 {
        return Err(SteamworksNetworkingSocketsError::InvalidBatchSize);
    }
    if batch_size > STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND {
        return Err(SteamworksNetworkingSocketsError::BatchSizeTooLarge {
            requested: batch_size,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND,
        });
    }
    Ok(())
}

fn validate_lane_configuration(
    lane_priorities: &[i32],
    lane_weights: &[u16],
) -> Result<(), SteamworksNetworkingSocketsError> {
    if lane_priorities.is_empty() || lane_priorities.len() != lane_weights.len() {
        return Err(SteamworksNetworkingSocketsError::InvalidLaneConfiguration {
            priorities: lane_priorities.len(),
            weights: lane_weights.len(),
        });
    }
    if lane_priorities.len() > STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES {
        return Err(SteamworksNetworkingSocketsError::TooManyConfiguredLanes {
            requested: lane_priorities.len(),
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES,
        });
    }
    Ok(())
}

fn validate_virtual_port(port: i32) -> Result<(), SteamworksNetworkingSocketsError> {
    if port < 0 {
        return Err(SteamworksNetworkingSocketsError::InvalidVirtualPort { port });
    }
    Ok(())
}

fn validate_request_policy(
    policy: &SteamworksConnectionRequestPolicy,
) -> Result<(), SteamworksNetworkingSocketsError> {
    match policy {
        SteamworksConnectionRequestPolicy::Reject { debug, .. }
            if debug
                .as_ref()
                .is_some_and(|value| value.as_bytes().contains(&0)) =>
        {
            Err(SteamworksNetworkingSocketsError::invalid_string(
                "request_policy.debug",
            ))
        }
        _ => Ok(()),
    }
}
