use crate::matchmaking_servers::*;

impl SteamworksMatchmakingServersState {
    /// Returns the most recent direct server query submitted through this plugin.
    pub fn last_server_query(&self) -> Option<SteamworksServerQueryInfo> {
        self.last_server_query
    }

    /// Returns a cached direct server query context by query ID.
    pub fn server_query(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<SteamworksServerQueryInfo> {
        self.server_queries
            .iter()
            .find(|info| info.query == query)
            .copied()
    }

    /// Returns the target endpoint for a cached direct server query.
    pub fn server_query_target(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<SteamworksServerQueryTarget> {
        self.server_query(query).map(|info| info.target)
    }

    /// Returns the kind of a cached direct server query.
    pub fn server_query_kind(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<SteamworksServerQueryKind> {
        self.server_query(query).map(|info| info.kind)
    }
}
