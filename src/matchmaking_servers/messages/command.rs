use bevy_ecs::message::Message;

use super::super::{
    SteamworksServerListFilters, SteamworksServerListKind, SteamworksServerListRequestId,
};

/// A high-level command for Steam Matchmaking Servers workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksMatchmakingServersCommand {
    /// Request a Steam server list.
    RequestServerList {
        /// Steam app ID to query.
        app_id: steamworks::AppId,
        /// Server-list source.
        kind: SteamworksServerListKind,
        /// Filters applied to non-LAN server-list requests.
        filters: SteamworksServerListFilters,
    },
    /// Refresh an existing server-list request.
    RefreshServerList {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
    },
    /// Refresh one server in an existing server-list request.
    RefreshServer {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Server index inside the request.
        server: i32,
    },
    /// Read the number of servers currently known for a request.
    GetServerListCount {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
    },
    /// Read details for one server currently known for a request.
    GetServerDetails {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Server index inside the request.
        server: i32,
    },
    /// Read whether a request is still refreshing.
    IsServerListRefreshing {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
    },
    /// Release a server-list request handle.
    ReleaseServerList {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
    },
}

impl SteamworksMatchmakingServersCommand {
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
