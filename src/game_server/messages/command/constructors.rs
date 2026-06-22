use std::net::SocketAddrV4;

use crate::game_server::SteamworksServerLoginToken;

use super::SteamworksServerCommand;

impl SteamworksServerCommand {
    /// Creates a [`SteamworksServerCommand::GetSteamId`] command.
    pub fn get_steam_id() -> Self {
        Self::GetSteamId
    }

    /// Creates a [`SteamworksServerCommand::GetAuthenticationSessionTicket`] command.
    pub fn get_authentication_session_ticket(steam_id: steamworks::SteamId) -> Self {
        Self::GetAuthenticationSessionTicket { steam_id }
    }

    /// Creates a [`SteamworksServerCommand::GetAuthenticationSessionTicketForIdentity`] command.
    pub fn get_authentication_session_ticket_for_identity(
        identity: impl Into<steamworks::networking_types::NetworkingIdentity>,
    ) -> Self {
        Self::GetAuthenticationSessionTicketForIdentity {
            identity: identity.into(),
        }
    }

    /// Creates a [`SteamworksServerCommand::CancelAuthenticationTicket`] command.
    pub fn cancel_authentication_ticket(ticket: steamworks::AuthTicket) -> Self {
        Self::CancelAuthenticationTicket { ticket }
    }

    /// Creates a [`SteamworksServerCommand::BeginAuthenticationSession`] command.
    pub fn begin_authentication_session(
        user: steamworks::SteamId,
        ticket: impl Into<Vec<u8>>,
    ) -> Self {
        Self::BeginAuthenticationSession {
            user,
            ticket: ticket.into(),
        }
    }

    /// Creates a [`SteamworksServerCommand::EndAuthenticationSession`] command.
    pub fn end_authentication_session(user: steamworks::SteamId) -> Self {
        Self::EndAuthenticationSession { user }
    }

    /// Creates a [`SteamworksServerCommand::HandleIncomingPacket`] command.
    pub fn handle_incoming_packet(data: impl Into<Vec<u8>>, addr: SocketAddrV4) -> Self {
        Self::HandleIncomingPacket {
            data: data.into(),
            addr,
        }
    }

    /// Creates a [`SteamworksServerCommand::SetProduct`] command.
    pub fn set_product(product: impl Into<String>) -> Self {
        Self::SetProduct {
            product: product.into(),
        }
    }

    /// Creates a [`SteamworksServerCommand::SetGameDescription`] command.
    pub fn set_game_description(description: impl Into<String>) -> Self {
        Self::SetGameDescription {
            description: description.into(),
        }
    }

    /// Creates a [`SteamworksServerCommand::SetGameData`] command.
    pub fn set_game_data(data: impl Into<String>) -> Self {
        Self::SetGameData { data: data.into() }
    }

    /// Creates a [`SteamworksServerCommand::SetDedicatedServer`] command.
    pub fn set_dedicated_server(dedicated: bool) -> Self {
        Self::SetDedicatedServer { dedicated }
    }

    /// Creates a [`SteamworksServerCommand::LogOnAnonymous`] command.
    pub fn log_on_anonymous() -> Self {
        Self::LogOnAnonymous
    }

    /// Creates a [`SteamworksServerCommand::LogOn`] command.
    pub fn log_on(token: impl Into<SteamworksServerLoginToken>) -> Self {
        Self::LogOn {
            token: token.into(),
        }
    }

    /// Creates a [`SteamworksServerCommand::SetAdvertiseServerActive`] command.
    pub fn set_advertise_server_active(active: bool) -> Self {
        Self::SetAdvertiseServerActive { active }
    }

    /// Creates a [`SteamworksServerCommand::EnableHeartbeats`] command.
    pub fn enable_heartbeats(active: bool) -> Self {
        Self::EnableHeartbeats { active }
    }

    /// Creates a [`SteamworksServerCommand::SetModDir`] command.
    pub fn set_mod_dir(mod_dir: impl Into<String>) -> Self {
        Self::SetModDir {
            mod_dir: mod_dir.into(),
        }
    }

    /// Creates a [`SteamworksServerCommand::SetMapName`] command.
    pub fn set_map_name(map_name: impl Into<String>) -> Self {
        Self::SetMapName {
            map_name: map_name.into(),
        }
    }

    /// Creates a [`SteamworksServerCommand::SetServerName`] command.
    pub fn set_server_name(server_name: impl Into<String>) -> Self {
        Self::SetServerName {
            server_name: server_name.into(),
        }
    }

    /// Creates a [`SteamworksServerCommand::SetMaxPlayers`] command.
    pub fn set_max_players(count: i32) -> Self {
        Self::SetMaxPlayers { count }
    }

    /// Creates a [`SteamworksServerCommand::SetGameTags`] command.
    pub fn set_game_tags(tags: impl Into<String>) -> Self {
        Self::SetGameTags { tags: tags.into() }
    }

    /// Creates a [`SteamworksServerCommand::SetKeyValue`] command.
    pub fn set_key_value(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::SetKeyValue {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Creates a [`SteamworksServerCommand::ClearAllKeyValues`] command.
    pub fn clear_all_key_values() -> Self {
        Self::ClearAllKeyValues
    }

    /// Creates a [`SteamworksServerCommand::SetPasswordProtected`] command.
    pub fn set_password_protected(protected: bool) -> Self {
        Self::SetPasswordProtected { protected }
    }

    /// Creates a [`SteamworksServerCommand::SetBotPlayerCount`] command.
    pub fn set_bot_player_count(count: i32) -> Self {
        Self::SetBotPlayerCount { count }
    }

    /// Creates a [`SteamworksServerCommand::DrainOutgoingPackets`] command.
    pub fn drain_outgoing_packets() -> Self {
        Self::DrainOutgoingPackets
    }
}
