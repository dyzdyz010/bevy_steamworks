use crate::matchmaking_servers::*;

impl SteamworksMatchmakingServersState {
    /// Returns the most recent direct server-rules response.
    pub fn last_server_rules(&self) -> Option<&SteamworksServerRules> {
        self.last_server_rules.as_ref()
    }

    /// Returns cached direct server rules by query ID.
    pub fn server_rules(&self, query: SteamworksServerQueryId) -> Option<&SteamworksServerRules> {
        self.server_rules.iter().find(|rules| rules.query == query)
    }

    /// Returns the target endpoint from cached direct server rules.
    pub fn server_rules_target(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<SteamworksServerQueryTarget> {
        self.server_rules(query).map(|rules| rules.target)
    }

    /// Returns cached direct server rule rows for one query.
    pub fn server_rule_entries(
        &self,
        query: SteamworksServerQueryId,
    ) -> Option<&[SteamworksServerRule]> {
        self.server_rules(query).map(|rules| rules.rules.as_slice())
    }

    /// Returns the number of cached direct server rule rows for one query.
    pub fn server_rule_count(&self, query: SteamworksServerQueryId) -> Option<usize> {
        self.server_rule_entries(query).map(|rules| rules.len())
    }

    /// Returns the number of rule rows in the most recent direct server-rules response.
    pub fn last_server_rule_count(&self) -> Option<usize> {
        self.last_server_rules
            .as_ref()
            .map(|rules| rules.rules.len())
    }

    /// Returns one cached direct server rule, preserving a cached query without that key as `Some(None)`.
    pub fn server_rule(&self, query: SteamworksServerQueryId, key: &str) -> Option<Option<&str>> {
        self.server_rule_entries(query).map(|rules| {
            rules
                .iter()
                .find(|rule| rule.key == key)
                .map(|rule| rule.value.as_str())
        })
    }

    /// Returns whether cached direct server rules contain a key.
    pub fn server_has_rule(&self, query: SteamworksServerQueryId, key: &str) -> Option<bool> {
        self.server_rule_entries(query)
            .map(|rules| rules.iter().any(|rule| rule.key == key))
    }

    /// Returns the most recent direct server-rules query that failed.
    pub fn last_failed_server_rules(&self) -> Option<SteamworksServerQueryId> {
        self.last_failed_server_rules
    }

    /// Returns whether a direct server-rules query has failed.
    pub fn server_rules_failed(&self, query: SteamworksServerQueryId) -> bool {
        self.failed_server_rules.contains(&query)
    }
}
