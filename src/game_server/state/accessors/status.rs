use crate::game_server::*;

impl SteamworksServerState {
    /// Returns the most recent synchronous error observed by the server plugin.
    pub fn last_error(&self) -> Option<&SteamworksServerError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent Steam ID read for this game server.
    pub fn steam_id(&self) -> Option<steamworks::SteamId> {
        self.steam_id
    }

    /// Returns the latest known Steam server connection state.
    ///
    /// This is updated by Steam server connection callbacks.
    pub fn steam_server_connected(&self) -> Option<bool> {
        self.steam_server_connected
    }
}
