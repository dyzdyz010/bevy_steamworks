use std::fmt;

use super::SteamworksServerCommand;

impl fmt::Debug for SteamworksServerCommand {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GetSteamId => formatter.write_str("GetSteamId"),
            Self::GetAuthenticationSessionTicket { steam_id } => formatter
                .debug_struct("GetAuthenticationSessionTicket")
                .field("steam_id", steam_id)
                .finish(),
            Self::GetAuthenticationSessionTicketForIdentity { identity } => formatter
                .debug_struct("GetAuthenticationSessionTicketForIdentity")
                .field("identity", identity)
                .finish(),
            Self::CancelAuthenticationTicket { ticket } => formatter
                .debug_struct("CancelAuthenticationTicket")
                .field("ticket", ticket)
                .finish(),
            Self::BeginAuthenticationSession { user, ticket } => formatter
                .debug_struct("BeginAuthenticationSession")
                .field("user", user)
                .field("ticket_len", &ticket.len())
                .finish(),
            Self::EndAuthenticationSession { user } => formatter
                .debug_struct("EndAuthenticationSession")
                .field("user", user)
                .finish(),
            Self::HandleIncomingPacket { data, addr } => formatter
                .debug_struct("HandleIncomingPacket")
                .field("data_len", &data.len())
                .field("addr", addr)
                .finish(),
            Self::SetProduct { product } => formatter
                .debug_struct("SetProduct")
                .field("product", product)
                .finish(),
            Self::SetGameDescription { description } => formatter
                .debug_struct("SetGameDescription")
                .field("description", description)
                .finish(),
            Self::SetGameData { data } => formatter
                .debug_struct("SetGameData")
                .field("data", data)
                .finish(),
            Self::SetDedicatedServer { dedicated } => formatter
                .debug_struct("SetDedicatedServer")
                .field("dedicated", dedicated)
                .finish(),
            Self::LogOnAnonymous => formatter.write_str("LogOnAnonymous"),
            Self::LogOn { token } => formatter
                .debug_struct("LogOn")
                .field("token", token)
                .finish(),
            Self::SetAdvertiseServerActive { active } => formatter
                .debug_struct("SetAdvertiseServerActive")
                .field("active", active)
                .finish(),
            Self::EnableHeartbeats { active } => formatter
                .debug_struct("EnableHeartbeats")
                .field("active", active)
                .finish(),
            Self::SetModDir { mod_dir } => formatter
                .debug_struct("SetModDir")
                .field("mod_dir", mod_dir)
                .finish(),
            Self::SetMapName { map_name } => formatter
                .debug_struct("SetMapName")
                .field("map_name", map_name)
                .finish(),
            Self::SetServerName { server_name } => formatter
                .debug_struct("SetServerName")
                .field("server_name", server_name)
                .finish(),
            Self::SetMaxPlayers { count } => formatter
                .debug_struct("SetMaxPlayers")
                .field("count", count)
                .finish(),
            Self::SetGameTags { tags } => formatter
                .debug_struct("SetGameTags")
                .field("tags", tags)
                .finish(),
            Self::SetKeyValue { key, value } => formatter
                .debug_struct("SetKeyValue")
                .field("key", key)
                .field("value", value)
                .finish(),
            Self::ClearAllKeyValues => formatter.write_str("ClearAllKeyValues"),
            Self::SetPasswordProtected { protected } => formatter
                .debug_struct("SetPasswordProtected")
                .field("protected", protected)
                .finish(),
            Self::SetBotPlayerCount { count } => formatter
                .debug_struct("SetBotPlayerCount")
                .field("count", count)
                .finish(),
            Self::DrainOutgoingPackets => formatter.write_str("DrainOutgoingPackets"),
        }
    }
}
