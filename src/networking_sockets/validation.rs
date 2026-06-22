use super::{
    SteamworksConnectionRequestPolicy, SteamworksNetworkingSocketsCommand,
    SteamworksNetworkingSocketsConfigEntry, SteamworksNetworkingSocketsError,
    STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES,
    STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIG_ENTRIES,
    STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND,
    STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND,
    STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES,
    STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES,
};

pub(super) fn validate_command(
    command: &SteamworksNetworkingSocketsCommand,
) -> Result<(), SteamworksNetworkingSocketsError> {
    match command {
        SteamworksNetworkingSocketsCommand::CreateListenSocketIp { options, .. } => {
            validate_config_entries(options)
        }
        SteamworksNetworkingSocketsCommand::CreateListenSocketP2p {
            local_virtual_port,
            options,
        } => {
            validate_virtual_port(*local_virtual_port)?;
            validate_config_entries(options)
        }
        SteamworksNetworkingSocketsCommand::CreateHostedDedicatedServerListenSocket {
            options,
            ..
        } => validate_config_entries(options),
        SteamworksNetworkingSocketsCommand::ConnectByIpAddress { options, .. } => {
            validate_config_entries(options)
        }
        SteamworksNetworkingSocketsCommand::ConnectP2p {
            remote_virtual_port,
            options,
            ..
        } => {
            validate_virtual_port(*remote_virtual_port)?;
            validate_config_entries(options)
        }
        SteamworksNetworkingSocketsCommand::PollListenSocketEvents {
            max_events,
            request_policy,
            ..
        } => {
            validate_event_limit(*max_events)?;
            validate_request_policy(request_policy)
        }
        SteamworksNetworkingSocketsCommand::PollAllListenSocketEvents {
            max_events_per_socket,
            request_policy,
        } => {
            validate_event_limit(*max_events_per_socket)?;
            validate_request_policy(request_policy)
        }
        SteamworksNetworkingSocketsCommand::PollConnectionEvents { max_events, .. } => {
            validate_event_limit(*max_events)
        }
        SteamworksNetworkingSocketsCommand::PollAllConnectionEvents {
            max_events_per_connection,
        } => validate_event_limit(*max_events_per_connection),
        SteamworksNetworkingSocketsCommand::ReceiveMessages { batch_size, .. } => {
            validate_message_batch_size(*batch_size)
        }
        SteamworksNetworkingSocketsCommand::ReceiveAllMessages {
            batch_size_per_connection,
        } => validate_message_batch_size(*batch_size_per_connection),
        SteamworksNetworkingSocketsCommand::ReceivePollGroupMessages { batch_size, .. } => {
            validate_message_batch_size(*batch_size)
        }
        SteamworksNetworkingSocketsCommand::ReceiveAllPollGroupMessages {
            batch_size_per_poll_group,
        } => validate_message_batch_size(*batch_size_per_poll_group),
        SteamworksNetworkingSocketsCommand::SendMessage { data, .. } => {
            validate_message_payload_len(data.len())
        }
        SteamworksNetworkingSocketsCommand::SendMessages { messages } => {
            validate_send_message_batch_size(messages.len())?;
            for message in messages {
                validate_message_payload_len(message.data.len())?;
                validate_message_channel(message.channel)?;
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
        SteamworksNetworkingSocketsCommand::CloseConnection { debug, .. }
        | SteamworksNetworkingSocketsCommand::CloseAllConnections { debug, .. } => {
            if debug
                .as_ref()
                .is_some_and(|value| value.as_bytes().contains(&0))
            {
                return Err(SteamworksNetworkingSocketsError::invalid_string("debug"));
            }
            Ok(())
        }
        SteamworksNetworkingSocketsCommand::SetConnectionName { name, .. } => {
            if name.as_bytes().contains(&0) {
                return Err(SteamworksNetworkingSocketsError::invalid_string("name"));
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

fn validate_send_message_batch_size(
    batch_size: usize,
) -> Result<(), SteamworksNetworkingSocketsError> {
    if batch_size == 0 {
        return Err(SteamworksNetworkingSocketsError::EmptyMessageBatch);
    }
    if batch_size > STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND {
        return Err(SteamworksNetworkingSocketsError::SendBatchTooLarge {
            requested: batch_size,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND,
        });
    }
    Ok(())
}

fn validate_message_payload_len(bytes: usize) -> Result<(), SteamworksNetworkingSocketsError> {
    if bytes > STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES {
        return Err(SteamworksNetworkingSocketsError::MessageTooLarge {
            bytes,
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES,
        });
    }
    Ok(())
}

fn validate_message_channel(channel: i32) -> Result<(), SteamworksNetworkingSocketsError> {
    if channel < 0 {
        return Err(SteamworksNetworkingSocketsError::InvalidMessageChannel { channel });
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

fn validate_config_entries(
    options: &[SteamworksNetworkingSocketsConfigEntry],
) -> Result<(), SteamworksNetworkingSocketsError> {
    if options.len() > STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIG_ENTRIES {
        return Err(SteamworksNetworkingSocketsError::TooManyConfigEntries {
            requested: options.len(),
            max_supported: STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIG_ENTRIES,
        });
    }

    for (index, option) in options.iter().enumerate() {
        validate_config_entry(index, option)?;
    }

    Ok(())
}

fn validate_config_entry(
    index: usize,
    option: &SteamworksNetworkingSocketsConfigEntry,
) -> Result<(), SteamworksNetworkingSocketsError> {
    use steamworks::networking_types::NetworkingConfigDataType;

    match option {
        SteamworksNetworkingSocketsConfigEntry::Int32 { value, .. }
            if value.data_type() != NetworkingConfigDataType::Int32 =>
        {
            Err(SteamworksNetworkingSocketsError::InvalidConfigEntryType {
                index,
                expected: value.data_type(),
                actual: NetworkingConfigDataType::Int32,
            })
        }
        SteamworksNetworkingSocketsConfigEntry::Int64 { value, .. }
            if value.data_type() != NetworkingConfigDataType::Int64 =>
        {
            Err(SteamworksNetworkingSocketsError::InvalidConfigEntryType {
                index,
                expected: value.data_type(),
                actual: NetworkingConfigDataType::Int64,
            })
        }
        SteamworksNetworkingSocketsConfigEntry::Float { value, data }
            if value.data_type() != NetworkingConfigDataType::Float =>
        {
            Err(SteamworksNetworkingSocketsError::InvalidConfigEntryType {
                index,
                expected: value.data_type(),
                actual: NetworkingConfigDataType::Float,
            })
        }
        SteamworksNetworkingSocketsConfigEntry::Float { data, .. } if !data.is_finite() => {
            Err(SteamworksNetworkingSocketsError::InvalidConfigFloat { index })
        }
        SteamworksNetworkingSocketsConfigEntry::String { value, .. }
            if value.data_type() != NetworkingConfigDataType::String =>
        {
            Err(SteamworksNetworkingSocketsError::InvalidConfigEntryType {
                index,
                expected: value.data_type(),
                actual: NetworkingConfigDataType::String,
            })
        }
        SteamworksNetworkingSocketsConfigEntry::String { data, .. }
            if data.as_bytes().contains(&0) =>
        {
            Err(SteamworksNetworkingSocketsError::InvalidConfigString { index })
        }
        SteamworksNetworkingSocketsConfigEntry::Int32 { .. }
        | SteamworksNetworkingSocketsConfigEntry::Int64 { .. }
        | SteamworksNetworkingSocketsConfigEntry::Float { .. }
        | SteamworksNetworkingSocketsConfigEntry::String { .. } => Ok(()),
    }
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
