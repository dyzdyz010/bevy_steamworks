use super::SteamworksNetworkingSocketsCommand;

impl std::fmt::Debug for SteamworksNetworkingSocketsCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitAuthentication => f.write_str("InitAuthentication"),
            Self::GetAuthenticationStatus => f.write_str("GetAuthenticationStatus"),
            Self::CreateListenSocketIp {
                local_address,
                options,
            } => f
                .debug_struct("CreateListenSocketIp")
                .field("local_address", local_address)
                .field("options", options)
                .finish(),
            Self::CreateListenSocketP2p {
                local_virtual_port,
                options,
            } => f
                .debug_struct("CreateListenSocketP2p")
                .field("local_virtual_port", local_virtual_port)
                .field("options", options)
                .finish(),
            Self::CreateHostedDedicatedServerListenSocket {
                local_virtual_port,
                options,
            } => f
                .debug_struct("CreateHostedDedicatedServerListenSocket")
                .field("local_virtual_port", local_virtual_port)
                .field("options", options)
                .finish(),
            Self::ConnectByIpAddress { address, options } => f
                .debug_struct("ConnectByIpAddress")
                .field("address", address)
                .field("options", options)
                .finish(),
            Self::ConnectP2p {
                identity,
                remote_virtual_port,
                options,
            } => f
                .debug_struct("ConnectP2p")
                .field("identity", identity)
                .field("remote_virtual_port", remote_virtual_port)
                .field("options", options)
                .finish(),
            Self::CreatePollGroup => f.write_str("CreatePollGroup"),
            Self::CreateServerPollGroup => f.write_str("CreateServerPollGroup"),
            Self::PollListenSocketEvents {
                listen_socket,
                max_events,
                request_policy,
            } => f
                .debug_struct("PollListenSocketEvents")
                .field("listen_socket", listen_socket)
                .field("max_events", max_events)
                .field("request_policy", request_policy)
                .finish(),
            Self::PollAllListenSocketEvents {
                max_events_per_socket,
                request_policy,
            } => f
                .debug_struct("PollAllListenSocketEvents")
                .field("max_events_per_socket", max_events_per_socket)
                .field("request_policy", request_policy)
                .finish(),
            Self::PollConnectionEvents {
                connection,
                max_events,
            } => f
                .debug_struct("PollConnectionEvents")
                .field("connection", connection)
                .field("max_events", max_events)
                .finish(),
            Self::PollAllConnectionEvents {
                max_events_per_connection,
            } => f
                .debug_struct("PollAllConnectionEvents")
                .field("max_events_per_connection", max_events_per_connection)
                .finish(),
            Self::GetConnectionInfo { connection } => f
                .debug_struct("GetConnectionInfo")
                .field("connection", connection)
                .finish(),
            Self::GetConnectionUserData { connection } => f
                .debug_struct("GetConnectionUserData")
                .field("connection", connection)
                .finish(),
            Self::GetRealtimeConnectionStatus { connection, lanes } => f
                .debug_struct("GetRealtimeConnectionStatus")
                .field("connection", connection)
                .field("lanes", lanes)
                .finish(),
            Self::SendMessage {
                connection,
                send_flags,
                data,
            } => f
                .debug_struct("SendMessage")
                .field("connection", connection)
                .field("send_flags", send_flags)
                .field("data_len", &data.len())
                .finish(),
            Self::SendMessages { messages } => f
                .debug_struct("SendMessages")
                .field("messages", messages)
                .finish(),
            Self::ReceiveMessages {
                connection,
                batch_size,
            } => f
                .debug_struct("ReceiveMessages")
                .field("connection", connection)
                .field("batch_size", batch_size)
                .finish(),
            Self::ReceiveAllMessages {
                batch_size_per_connection,
            } => f
                .debug_struct("ReceiveAllMessages")
                .field("batch_size_per_connection", batch_size_per_connection)
                .finish(),
            Self::ReceivePollGroupMessages {
                poll_group,
                batch_size,
            } => f
                .debug_struct("ReceivePollGroupMessages")
                .field("poll_group", poll_group)
                .field("batch_size", batch_size)
                .finish(),
            Self::ReceiveAllPollGroupMessages {
                batch_size_per_poll_group,
            } => f
                .debug_struct("ReceiveAllPollGroupMessages")
                .field("batch_size_per_poll_group", batch_size_per_poll_group)
                .finish(),
            Self::FlushMessages { connection } => f
                .debug_struct("FlushMessages")
                .field("connection", connection)
                .finish(),
            Self::FlushAllMessages => f.write_str("FlushAllMessages"),
            Self::SetConnectionPollGroup {
                connection,
                poll_group,
            } => f
                .debug_struct("SetConnectionPollGroup")
                .field("connection", connection)
                .field("poll_group", poll_group)
                .finish(),
            Self::ClearConnectionPollGroup { connection } => f
                .debug_struct("ClearConnectionPollGroup")
                .field("connection", connection)
                .finish(),
            Self::ConfigureConnectionLanes {
                connection,
                lane_priorities,
                lane_weights,
            } => f
                .debug_struct("ConfigureConnectionLanes")
                .field("connection", connection)
                .field("lane_priorities", lane_priorities)
                .field("lane_weights", lane_weights)
                .finish(),
            Self::SetConnectionUserData {
                connection,
                user_data,
            } => f
                .debug_struct("SetConnectionUserData")
                .field("connection", connection)
                .field("user_data", user_data)
                .finish(),
            Self::SetConnectionName { connection, name } => f
                .debug_struct("SetConnectionName")
                .field("connection", connection)
                .field("name_len", &name.len())
                .finish(),
            Self::CloseConnection {
                connection,
                reason,
                debug,
                enable_linger,
            } => f
                .debug_struct("CloseConnection")
                .field("connection", connection)
                .field("reason", reason)
                .field("debug", debug)
                .field("enable_linger", enable_linger)
                .finish(),
            Self::CloseListenSocket { listen_socket } => f
                .debug_struct("CloseListenSocket")
                .field("listen_socket", listen_socket)
                .finish(),
            Self::ClosePollGroup { poll_group } => f
                .debug_struct("ClosePollGroup")
                .field("poll_group", poll_group)
                .finish(),
        }
    }
}
