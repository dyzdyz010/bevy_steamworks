use std::time::Duration;

use crate::matchmaking_servers::*;

impl SteamworksMatchmakingServersState {
    /// Returns the most recent direct server ping response.
    pub fn last_server_ping(&self) -> Option<&SteamworksServerPing> {
        self.last_server_ping.as_ref()
    }

    /// Returns a cached direct server ping response by query ID.
    pub fn server_ping(&self, query: SteamworksServerQueryId) -> Option<&SteamworksServerPing> {
        self.server_pings.iter().find(|ping| ping.query == query)
    }

    /// Returns the target endpoint from a cached direct ping response.
    pub fn server_ping_target(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<SteamworksServerQueryTarget> {
        self.server_ping(query).map(|ping| ping.target)
    }

    /// Returns the server item from a cached direct ping response.
    pub fn server_ping_server(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<&SteamworksGameServerItem> {
        self.server_ping(query).map(|ping| &ping.server)
    }

    /// Returns the ping duration from a cached direct ping response.
    pub fn server_ping_latency(&self, query: SteamworksServerQueryId) -> Option<Duration> {
        self.server_ping_server(query).map(|server| server.ping)
    }

    /// Returns the server name from a cached direct ping response.
    pub fn server_ping_server_name(&self, query: SteamworksServerQueryId) -> Option<&str> {
        self.server_ping_server(query)
            .map(|server| server.server_name.as_str())
    }

    /// Returns the map name from a cached direct ping response.
    pub fn server_ping_map(&self, query: SteamworksServerQueryId) -> Option<&str> {
        self.server_ping_server(query)
            .map(|server| server.map.as_str())
    }

    /// Returns the current player count from a cached direct ping response.
    pub fn server_ping_player_count(&self, query: SteamworksServerQueryId) -> Option<i32> {
        self.server_ping_server(query).map(|server| server.players)
    }

    /// Returns the maximum player count from a cached direct ping response.
    pub fn server_ping_max_players(&self, query: SteamworksServerQueryId) -> Option<i32> {
        self.server_ping_server(query)
            .map(|server| server.max_players)
    }

    /// Returns the most recent direct server ping query that failed.
    pub fn last_failed_server_ping(&self) -> Option<SteamworksServerQueryId> {
        self.last_failed_server_ping
    }

    /// Returns whether a direct server ping query has failed.
    pub fn server_ping_failed(&self, query: SteamworksServerQueryId) -> bool {
        self.failed_server_pings.contains(&query)
    }
}
