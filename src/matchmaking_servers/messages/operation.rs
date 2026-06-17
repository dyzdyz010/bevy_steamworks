use super::super::{
    SteamworksGameServerItem, SteamworksServerListFilters, SteamworksServerListKind,
    SteamworksServerListRequestId, SteamworksServerListResponse, SteamworksServerPing,
    SteamworksServerPlayerDetails, SteamworksServerQueryId, SteamworksServerQueryInfo,
    SteamworksServerRules,
};

/// A successfully submitted Matchmaking Servers operation or callback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksMatchmakingServersOperation {
    /// A direct single-server query was submitted.
    ServerQuerySubmitted {
        /// Submitted query context.
        query: SteamworksServerQueryInfo,
    },
    /// A direct server ping returned a server snapshot.
    ServerPingResponded {
        /// Ping snapshot.
        ping: SteamworksServerPing,
    },
    /// A direct server ping failed.
    ServerPingFailed {
        /// Plugin-owned query ID.
        query: SteamworksServerQueryId,
    },
    /// A direct player-details query returned all player rows and completed.
    ServerPlayerDetailsReceived {
        /// Player-details snapshot.
        details: SteamworksServerPlayerDetails,
    },
    /// A direct player-details query failed.
    ServerPlayerDetailsFailed {
        /// Plugin-owned query ID.
        query: SteamworksServerQueryId,
    },
    /// A direct server-rules query returned all rules and completed.
    ServerRulesReceived {
        /// Server-rules snapshot.
        rules: SteamworksServerRules,
    },
    /// A direct server-rules query failed.
    ServerRulesFailed {
        /// Plugin-owned query ID.
        query: SteamworksServerQueryId,
    },
    /// A server-list request was submitted.
    ServerListRequested {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Steam app ID queried.
        app_id: steamworks::AppId,
        /// Server-list source.
        kind: SteamworksServerListKind,
        /// Filters applied to the request.
        filters: SteamworksServerListFilters,
    },
    /// A server responded to a server-list request.
    ServerResponded {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Server index inside the request.
        server_index: i32,
        /// Snapshot of the server.
        server: SteamworksGameServerItem,
    },
    /// A server failed to respond to a server-list request.
    ServerFailedToRespond {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Server index inside the request.
        server_index: i32,
    },
    /// A server-list refresh completed.
    ServerListRefreshCompleted {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Completion response.
        response: SteamworksServerListResponse,
    },
    /// A server-list refresh was submitted.
    ServerListRefreshSubmitted {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
    },
    /// A single-server refresh was submitted.
    ServerRefreshSubmitted {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Server index inside the request.
        server_index: i32,
    },
    /// Server count was read from a request.
    ServerListCountRead {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Server count reported by Steam.
        count: i32,
    },
    /// Server details were read from a request.
    ServerDetailsRead {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Server index inside the request.
        server_index: i32,
        /// Server snapshot.
        server: SteamworksGameServerItem,
    },
    /// Refreshing state was read from a request.
    ServerListRefreshingRead {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
        /// Whether the request is currently refreshing.
        refreshing: bool,
    },
    /// A server-list request was released.
    ServerListReleased {
        /// Plugin-owned request ID.
        request: SteamworksServerListRequestId,
    },
}
