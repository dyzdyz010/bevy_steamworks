use super::types::{
    SteamworksNetworkingMessage, SteamworksNetworkingMessagesConnectionInfo,
    SteamworksNetworkingMessagesRealtimeInfo,
};

pub(super) fn snapshot_networking_message(
    message: steamworks::networking_types::NetworkingMessage,
) -> SteamworksNetworkingMessage {
    SteamworksNetworkingMessage {
        peer: message.identity_peer(),
        data: message.data().to_vec(),
        channel: message.channel(),
        send_flags: message.send_flags(),
        message_number: u64::from(message.message_number()),
        connection_user_data: message.connection_user_data(),
    }
}

pub(super) fn snapshot_session_connection_info(
    state: steamworks::networking_types::NetworkingConnectionState,
    info: Option<&steamworks::networking_types::NetConnectionInfo>,
    realtime: Option<&steamworks::networking_types::NetConnectionRealTimeInfo>,
) -> SteamworksNetworkingMessagesConnectionInfo {
    SteamworksNetworkingMessagesConnectionInfo {
        state,
        remote: info.and_then(|info| info.identity_remote()),
        user_data: info.map(|info| info.user_data()),
        end_reason: info.and_then(|info| info.end_reason()),
        realtime: realtime.map(snapshot_realtime_info),
    }
}

fn snapshot_realtime_info(
    info: &steamworks::networking_types::NetConnectionRealTimeInfo,
) -> SteamworksNetworkingMessagesRealtimeInfo {
    SteamworksNetworkingMessagesRealtimeInfo {
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
    }
}
