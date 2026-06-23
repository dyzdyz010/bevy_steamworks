use std::fmt;

use super::{
    SteamworksNetworkingSocketsConfigEntry, SteamworksNetworkingSocketsConnectionName,
    SteamworksNetworkingSocketsMessage, SteamworksNetworkingSocketsOutboundMessage,
    SteamworksNetworkingSocketsPollGroupMessage,
};

impl fmt::Debug for SteamworksNetworkingSocketsConfigEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int32 { value, data } => formatter
                .debug_struct("Int32")
                .field("value", value)
                .field("data", data)
                .finish(),
            Self::Int64 { value, data } => formatter
                .debug_struct("Int64")
                .field("value", value)
                .field("data", data)
                .finish(),
            Self::Float { value, data } => formatter
                .debug_struct("Float")
                .field("value", value)
                .field("data", data)
                .finish(),
            Self::String { value, data } => formatter
                .debug_struct("String")
                .field("value", value)
                .field("data_len", &data.len())
                .finish(),
        }
    }
}

impl fmt::Debug for SteamworksNetworkingSocketsOutboundMessage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SteamworksNetworkingSocketsOutboundMessage")
            .field("connection", &self.connection)
            .field("send_flags", &self.send_flags)
            .field("channel", &self.channel)
            .field("data_len", &self.data.len())
            .field("user_data", &self.user_data)
            .finish()
    }
}

impl fmt::Debug for SteamworksNetworkingSocketsConnectionName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SteamworksNetworkingSocketsConnectionName")
            .field("connection", &self.connection)
            .field("name_len", &self.name.len())
            .finish()
    }
}

impl fmt::Debug for SteamworksNetworkingSocketsMessage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SteamworksNetworkingSocketsMessage")
            .field("connection", &self.connection)
            .field("peer", &self.peer)
            .field("data_len", &self.data.len())
            .field("channel", &self.channel)
            .field("send_flags", &self.send_flags)
            .field("message_number", &self.message_number)
            .field("connection_user_data", &self.connection_user_data)
            .finish()
    }
}

impl fmt::Debug for SteamworksNetworkingSocketsPollGroupMessage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SteamworksNetworkingSocketsPollGroupMessage")
            .field("poll_group", &self.poll_group)
            .field("peer", &self.peer)
            .field("data_len", &self.data.len())
            .field("channel", &self.channel)
            .field("send_flags", &self.send_flags)
            .field("message_number", &self.message_number)
            .field("connection_user_data", &self.connection_user_data)
            .finish()
    }
}
