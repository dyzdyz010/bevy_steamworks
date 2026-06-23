use crate::game_server::*;

impl SteamworksServerState {
    /// Returns the most recent product string submitted through this command layer.
    pub fn product(&self) -> Option<&str> {
        self.product.as_deref()
    }

    /// Returns the most recent game description submitted through this command layer.
    pub fn game_description(&self) -> Option<&str> {
        self.game_description.as_deref()
    }

    /// Returns the most recent game data string submitted through this command layer.
    pub fn game_data(&self) -> Option<&str> {
        self.game_data.as_deref()
    }

    /// Returns the most recent dedicated/listen server flag submitted through this command layer.
    pub fn dedicated(&self) -> Option<bool> {
        self.dedicated
    }

    /// Returns whether anonymous logon was submitted through this command layer.
    pub fn anonymous_logon_submitted(&self) -> bool {
        self.anonymous_logon_submitted
    }

    /// Returns whether token-based logon was submitted through this command layer.
    pub fn token_logon_submitted(&self) -> bool {
        self.token_logon_submitted
    }

    /// Returns whether any server logon command was submitted through this command layer.
    pub fn logon_submitted(&self) -> bool {
        self.anonymous_logon_submitted || self.token_logon_submitted
    }

    /// Returns the most recent advertise-server-active flag submitted through this command layer.
    pub fn advertise_server_active(&self) -> Option<bool> {
        self.advertise_server_active
    }

    /// Returns the most recent heartbeat-active flag submitted through this command layer.
    pub fn heartbeats_active(&self) -> Option<bool> {
        self.heartbeats_active
    }

    /// Returns the most recent mod dir submitted through this command layer.
    pub fn mod_dir(&self) -> Option<&str> {
        self.mod_dir.as_deref()
    }

    /// Returns the most recent map name submitted through this command layer.
    pub fn map_name(&self) -> Option<&str> {
        self.map_name.as_deref()
    }

    /// Returns the most recent server name submitted through this command layer.
    pub fn server_name(&self) -> Option<&str> {
        self.server_name.as_deref()
    }

    /// Returns the most recent maximum player count submitted through this command layer.
    pub fn max_players(&self) -> Option<i32> {
        self.max_players
    }

    /// Returns the most recent game tags string submitted through this command layer.
    pub fn game_tags(&self) -> Option<&str> {
        self.game_tags.as_deref()
    }

    /// Returns key/value rules submitted through this command layer.
    pub fn key_values(&self) -> &[(String, String)] {
        &self.key_values
    }

    /// Returns a server browser rule value submitted through this command layer.
    pub fn key_value(&self, key: &str) -> Option<&str> {
        self.key_values
            .iter()
            .find_map(|(known_key, value)| (known_key == key).then_some(value.as_str()))
    }

    /// Returns the most recent password-protected flag submitted through this command layer.
    pub fn password_protected(&self) -> Option<bool> {
        self.password_protected
    }

    /// Returns the most recent bot player count submitted through this command layer.
    pub fn bot_player_count(&self) -> Option<i32> {
        self.bot_player_count
    }
}
