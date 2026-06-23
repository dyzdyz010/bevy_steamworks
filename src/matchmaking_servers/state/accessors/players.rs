use std::time::Duration;

use crate::matchmaking_servers::*;

impl SteamworksMatchmakingServersState {
    /// Returns the most recent direct server player-details response.
    pub fn last_server_player_details(&self) -> Option<&SteamworksServerPlayerDetails> {
        self.last_server_player_details.as_ref()
    }

    /// Returns cached direct server player details by query ID.
    pub fn server_player_details(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<&SteamworksServerPlayerDetails> {
        self.server_player_details
            .iter()
            .find(|details| details.query == query)
    }

    /// Returns the target endpoint from cached direct player details.
    pub fn server_player_details_target(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<SteamworksServerQueryTarget> {
        self.server_player_details(query)
            .map(|details| details.target)
    }

    /// Returns cached direct player rows for one query.
    pub fn server_players(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<&[SteamworksServerPlayerInfo]> {
        self.server_player_details(query)
            .map(|details| details.players.as_slice())
    }

    /// Returns the number of cached direct player rows for one query.
    pub fn server_player_count(&self, query: SteamworksServerQueryId) -> Option<usize> {
        self.server_players(query).map(|players| players.len())
    }

    /// Returns the number of player rows in the most recent direct player-details response.
    pub fn last_server_player_count(&self) -> Option<usize> {
        self.last_server_player_details
            .as_ref()
            .map(|details| details.players.len())
    }

    /// Returns a cached direct player row by player name.
    pub fn server_player(
        &self,
        query: SteamworksServerQueryId,
        name: &str,
    ) -> Option<&SteamworksServerPlayerInfo> {
        self.server_players(query)
            .and_then(|players| players.iter().find(|player| player.name == name))
    }

    /// Returns whether cached direct player details contain a player name.
    pub fn server_has_player(&self, query: SteamworksServerQueryId, name: &str) -> Option<bool> {
        self.server_players(query)
            .map(|players| players.iter().any(|player| player.name == name))
    }

    /// Returns a cached direct player score by player name.
    pub fn server_player_score(&self, query: SteamworksServerQueryId, name: &str) -> Option<i32> {
        self.server_player(query, name).map(|player| player.score)
    }

    /// Returns cached direct player time-played by player name.
    pub fn server_player_time_played(
        &self,
        query: SteamworksServerQueryId,
        name: &str,
    ) -> Option<Duration> {
        self.server_player(query, name)
            .map(|player| player.time_played)
    }

    /// Returns the most recent direct server player-details query that failed.
    pub fn last_failed_server_player_details(&self) -> Option<SteamworksServerQueryId> {
        self.last_failed_server_player_details
    }

    /// Returns whether a direct player-details query has failed.
    pub fn server_player_details_failed(&self, query: SteamworksServerQueryId) -> bool {
        self.failed_server_player_details.contains(&query)
    }
}
