use std::net::Ipv4Addr;

use super::super::super::{
    SteamworksServerListFilters, SteamworksServerListKind, SteamworksServerListRequestId,
    SteamworksServerQueryTarget,
};
use super::SteamworksMatchmakingServersCommand;

impl SteamworksMatchmakingServersCommand {
    /// Creates a direct server ping command.
    pub fn ping_server(address: Ipv4Addr, query_port: u16) -> Self {
        Self::PingServer {
            target: SteamworksServerQueryTarget {
                address,
                query_port,
            },
        }
    }

    /// Creates a direct player-details query command.
    pub fn query_player_details(address: Ipv4Addr, query_port: u16) -> Self {
        Self::QueryPlayerDetails {
            target: SteamworksServerQueryTarget {
                address,
                query_port,
            },
        }
    }

    /// Creates a direct server-rules query command.
    pub fn query_server_rules(address: Ipv4Addr, query_port: u16) -> Self {
        Self::QueryServerRules {
            target: SteamworksServerQueryTarget {
                address,
                query_port,
            },
        }
    }

    /// Creates a LAN server-list request command.
    pub fn request_lan_server_list(app_id: impl Into<steamworks::AppId>) -> Self {
        Self::RequestServerList {
            app_id: app_id.into(),
            kind: SteamworksServerListKind::Lan,
            filters: SteamworksServerListFilters::new(),
        }
    }

    /// Creates an Internet server-list request command.
    pub fn request_internet_server_list(
        app_id: impl Into<steamworks::AppId>,
        filters: SteamworksServerListFilters,
    ) -> Self {
        Self::RequestServerList {
            app_id: app_id.into(),
            kind: SteamworksServerListKind::Internet,
            filters,
        }
    }

    /// Creates a favorites server-list request command.
    pub fn request_favorites_server_list(
        app_id: impl Into<steamworks::AppId>,
        filters: SteamworksServerListFilters,
    ) -> Self {
        Self::RequestServerList {
            app_id: app_id.into(),
            kind: SteamworksServerListKind::Favorites,
            filters,
        }
    }

    /// Creates a history server-list request command.
    pub fn request_history_server_list(
        app_id: impl Into<steamworks::AppId>,
        filters: SteamworksServerListFilters,
    ) -> Self {
        Self::RequestServerList {
            app_id: app_id.into(),
            kind: SteamworksServerListKind::History,
            filters,
        }
    }

    /// Creates a friends server-list request command.
    pub fn request_friends_server_list(
        app_id: impl Into<steamworks::AppId>,
        filters: SteamworksServerListFilters,
    ) -> Self {
        Self::RequestServerList {
            app_id: app_id.into(),
            kind: SteamworksServerListKind::Friends,
            filters,
        }
    }

    /// Creates a server-list refresh command.
    pub fn refresh_server_list(request: SteamworksServerListRequestId) -> Self {
        Self::RefreshServerList { request }
    }

    /// Creates a single-server refresh command.
    pub fn refresh_server(request: SteamworksServerListRequestId, server: i32) -> Self {
        Self::RefreshServer { request, server }
    }

    /// Creates a server-list count read command.
    pub fn get_server_list_count(request: SteamworksServerListRequestId) -> Self {
        Self::GetServerListCount { request }
    }

    /// Creates a server details read command.
    pub fn get_server_details(request: SteamworksServerListRequestId, server: i32) -> Self {
        Self::GetServerDetails { request, server }
    }

    /// Creates a server-list refreshing state read command.
    pub fn is_server_list_refreshing(request: SteamworksServerListRequestId) -> Self {
        Self::IsServerListRefreshing { request }
    }

    /// Creates a server-list release command.
    pub fn release_server_list(request: SteamworksServerListRequestId) -> Self {
        Self::ReleaseServerList { request }
    }
}
