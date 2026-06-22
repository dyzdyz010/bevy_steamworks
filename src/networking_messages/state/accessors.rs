use super::SteamworksNetworkingMessagesState;
use crate::networking_messages::{
    SteamworksNetworkingMessage, SteamworksNetworkingMessagesConnectionInfo,
    SteamworksNetworkingMessagesError, SteamworksNetworkingMessagesSessionRequestInfo,
};

impl SteamworksNetworkingMessagesState {
    /// Returns the most recent synchronous command error observed by the plugin.
    pub fn last_error(&self) -> Option<&SteamworksNetworkingMessagesError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent batch of received messages.
    pub fn received_messages(&self) -> &[SteamworksNetworkingMessage] {
        &self.received_messages
    }

    /// Returns the most recent connection info snapshot read through the plugin.
    pub fn last_connection_info(&self) -> Option<&SteamworksNetworkingMessagesConnectionInfo> {
        self.last_connection_info.as_ref()
    }

    /// Returns the most recent incoming session request observed by the callback.
    pub fn last_session_request(&self) -> Option<&SteamworksNetworkingMessagesSessionRequestInfo> {
        self.last_session_request.as_ref()
    }

    /// Returns the most recent session failure observed by the callback.
    pub fn last_session_failure(&self) -> Option<&SteamworksNetworkingMessagesConnectionInfo> {
        self.last_session_failure.as_ref()
    }

    /// Returns whether incoming session requests are currently auto-accepted.
    pub fn auto_accept_session_requests(&self) -> bool {
        *self
            .auto_accept_session_requests
            .lock()
            .expect("Steamworks Networking Messages policy mutex was poisoned")
    }

    /// Returns the number of successful send commands observed through the plugin.
    pub fn sent_count(&self) -> u64 {
        self.sent_count
    }

    /// Returns the number of received messages observed through the plugin.
    pub fn received_count(&self) -> u64 {
        self.received_count
    }

    /// Returns the number of incoming session requests observed by the plugin.
    pub fn session_request_count(&self) -> u64 {
        self.session_request_count
    }

    /// Returns the number of session failures observed by the plugin.
    pub fn session_failure_count(&self) -> u64 {
        self.session_failure_count
    }
}
