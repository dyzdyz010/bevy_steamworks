use super::{
    SteamworksNetworkingSocketsConnectionId, SteamworksNetworkingSocketsConnectionInfo,
    SteamworksNetworkingSocketsMessage, SteamworksNetworkingSocketsPollGroupId,
    SteamworksNetworkingSocketsPollGroupMessage, SteamworksNetworkingSocketsRealtimeLaneStatus,
    SteamworksNetworkingSocketsRealtimeStatus,
};

pub(super) fn snapshot_connection_info(
    connection: SteamworksNetworkingSocketsConnectionId,
    info: steamworks::networking_types::NetConnectionInfo,
) -> SteamworksNetworkingSocketsConnectionInfo {
    SteamworksNetworkingSocketsConnectionInfo {
        connection,
        state: info
            .state()
            .unwrap_or(steamworks::networking_types::NetworkingConnectionState::None),
        remote: info.identity_remote(),
        user_data: info.user_data(),
        end_reason: info.end_reason(),
    }
}

pub(super) fn snapshot_realtime_status(
    connection: SteamworksNetworkingSocketsConnectionId,
    info: steamworks::networking_types::NetConnectionRealTimeInfo,
    lanes: Vec<steamworks::networking_types::NetConnectionRealTimeLaneStatus>,
) -> SteamworksNetworkingSocketsRealtimeStatus {
    SteamworksNetworkingSocketsRealtimeStatus {
        connection,
        connection_state: info
            .connection_state()
            .unwrap_or(steamworks::networking_types::NetworkingConnectionState::None),
        ping: info.ping(),
        connection_quality_local: info.connection_quality_local(),
        connection_quality_remote: info.connection_quality_remote(),
        out_packets_per_sec: info.out_packets_per_sec(),
        out_bytes_per_sec: info.out_bytes_per_sec(),
        in_packets_per_sec: info.in_packets_per_sec(),
        in_bytes_per_sec: info.in_bytes_per_sec(),
        send_rate_bytes_per_sec: info.send_rate_bytes_per_sec(),
        pending_unreliable: info.pending_unreliable(),
        pending_reliable: info.pending_reliable(),
        sent_unacked_reliable: info.sent_unacked_reliable(),
        queued_send_bytes: info.queued_send_bytes(),
        lanes: lanes
            .into_iter()
            .map(|lane| SteamworksNetworkingSocketsRealtimeLaneStatus {
                pending_unreliable: lane.pending_unreliable(),
                pending_reliable: lane.pending_reliable(),
                sent_unacked_reliable: lane.sent_unacked_reliable(),
                queued_send_bytes: lane.queued_send_bytes(),
            })
            .collect(),
    }
}

pub(super) fn snapshot_message(
    connection: SteamworksNetworkingSocketsConnectionId,
    message: steamworks::networking_types::NetworkingMessage,
) -> SteamworksNetworkingSocketsMessage {
    SteamworksNetworkingSocketsMessage {
        connection,
        peer: message.identity_peer(),
        data: message.data().to_vec(),
        channel: message.channel(),
        send_flags: message.send_flags(),
        message_number: u64::from(message.message_number()),
        connection_user_data: message.connection_user_data(),
    }
}

pub(super) fn snapshot_poll_group_message(
    poll_group: SteamworksNetworkingSocketsPollGroupId,
    message: steamworks::networking_types::NetworkingMessage,
) -> SteamworksNetworkingSocketsPollGroupMessage {
    SteamworksNetworkingSocketsPollGroupMessage {
        poll_group,
        peer: message.identity_peer(),
        data: message.data().to_vec(),
        channel: message.channel(),
        send_flags: message.send_flags(),
        message_number: u64::from(message.message_number()),
        connection_user_data: message.connection_user_data(),
    }
}
