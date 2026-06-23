use std::fmt;

use super::SteamworksServerOperation;

impl fmt::Debug for SteamworksServerOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SteamIdRead { steam_id } => formatter
                .debug_struct("SteamIdRead")
                .field("steam_id", steam_id)
                .finish(),
            Self::AuthenticationSessionTicketIssued {
                ticket,
                ticket_bytes,
                steam_id,
            } => formatter
                .debug_struct("AuthenticationSessionTicketIssued")
                .field("ticket", ticket)
                .field("ticket_bytes_len", &ticket_bytes.len())
                .field("steam_id", steam_id)
                .finish(),
            Self::AuthenticationSessionTicketForIdentityIssued {
                ticket,
                ticket_bytes,
                identity,
            } => formatter
                .debug_struct("AuthenticationSessionTicketForIdentityIssued")
                .field("ticket", ticket)
                .field("ticket_bytes_len", &ticket_bytes.len())
                .field("identity", identity)
                .finish(),
            Self::AuthenticationTicketCancelled { ticket } => formatter
                .debug_struct("AuthenticationTicketCancelled")
                .field("ticket", ticket)
                .finish(),
            Self::AuthenticationSessionStarted { user } => formatter
                .debug_struct("AuthenticationSessionStarted")
                .field("user", user)
                .finish(),
            Self::AuthenticationSessionEnded { user } => formatter
                .debug_struct("AuthenticationSessionEnded")
                .field("user", user)
                .finish(),
            Self::AuthenticationSessionTicketResponse { response } => formatter
                .debug_struct("AuthenticationSessionTicketResponse")
                .field("response", response)
                .finish(),
            Self::AuthenticationTicketValidationReceived { validation } => formatter
                .debug_struct("AuthenticationTicketValidationReceived")
                .field("validation", validation)
                .finish(),
            Self::SteamServerConnectionEventReceived { event } => formatter
                .debug_struct("SteamServerConnectionEventReceived")
                .field("event", event)
                .finish(),
            Self::ClientApproved { approval } => formatter
                .debug_struct("ClientApproved")
                .field("approval", approval)
                .finish(),
            Self::ClientDenied { denial } => formatter
                .debug_struct("ClientDenied")
                .field("denial", denial)
                .finish(),
            Self::ClientKicked { kick } => formatter
                .debug_struct("ClientKicked")
                .field("kick", kick)
                .finish(),
            Self::ClientGroupStatusReceived { status } => formatter
                .debug_struct("ClientGroupStatusReceived")
                .field("status", status)
                .finish(),
            Self::IncomingPacketHandled {
                addr,
                bytes,
                accepted,
            } => formatter
                .debug_struct("IncomingPacketHandled")
                .field("addr", addr)
                .field("bytes", bytes)
                .field("accepted", accepted)
                .finish(),
            Self::ProductSet { product } => formatter
                .debug_struct("ProductSet")
                .field("product", product)
                .finish(),
            Self::GameDescriptionSet { description } => formatter
                .debug_struct("GameDescriptionSet")
                .field("description", description)
                .finish(),
            Self::GameDataSet { data } => formatter
                .debug_struct("GameDataSet")
                .field("data", data)
                .finish(),
            Self::DedicatedServerSet { dedicated } => formatter
                .debug_struct("DedicatedServerSet")
                .field("dedicated", dedicated)
                .finish(),
            Self::AnonymousLogonSubmitted => formatter.write_str("AnonymousLogonSubmitted"),
            Self::TokenLogonSubmitted => formatter.write_str("TokenLogonSubmitted"),
            Self::AdvertiseServerActiveSet { active } => formatter
                .debug_struct("AdvertiseServerActiveSet")
                .field("active", active)
                .finish(),
            Self::HeartbeatsEnabled { active } => formatter
                .debug_struct("HeartbeatsEnabled")
                .field("active", active)
                .finish(),
            Self::ModDirSet { mod_dir } => formatter
                .debug_struct("ModDirSet")
                .field("mod_dir", mod_dir)
                .finish(),
            Self::MapNameSet { map_name } => formatter
                .debug_struct("MapNameSet")
                .field("map_name", map_name)
                .finish(),
            Self::ServerNameSet { server_name } => formatter
                .debug_struct("ServerNameSet")
                .field("server_name", server_name)
                .finish(),
            Self::MaxPlayersSet { count } => formatter
                .debug_struct("MaxPlayersSet")
                .field("count", count)
                .finish(),
            Self::GameTagsSet { tags } => formatter
                .debug_struct("GameTagsSet")
                .field("tags", tags)
                .finish(),
            Self::KeyValueSet { key, value } => formatter
                .debug_struct("KeyValueSet")
                .field("key", key)
                .field("value", value)
                .finish(),
            Self::AllKeyValuesCleared => formatter.write_str("AllKeyValuesCleared"),
            Self::PasswordProtectedSet { protected } => formatter
                .debug_struct("PasswordProtectedSet")
                .field("protected", protected)
                .finish(),
            Self::BotPlayerCountSet { count } => formatter
                .debug_struct("BotPlayerCountSet")
                .field("count", count)
                .finish(),
            Self::OutgoingPacketsDrained { packets } => formatter
                .debug_struct("OutgoingPacketsDrained")
                .field("packets", packets)
                .finish(),
        }
    }
}
