use bevy_ecs::message::Message;

use super::super::{
    SteamworksServerListFilters, SteamworksServerListKind, SteamworksServerListRequestId,
    SteamworksServerQueryTarget,
};

mod constructors;

/// A high-level command for Steam Matchmaking Servers workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksMatchmakingServersCommand {
    /// Ping one server directly.
    PingServer {
        /// Target server endpoint.
        target: SteamworksServerQueryTarget,
    },
    /// Query player details from one server directly.
    QueryPlayerDetails {
        /// Target server endpoint.
        target: SteamworksServerQueryTarget,
    },
    /// Query server rules from one server directly.
    QueryServerRules {
        /// Target server endpoint.
        target: SteamworksServerQueryTarget,
    },
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
