use std::sync::{Arc, Mutex};

use bevy_ecs::prelude::Resource;

use super::*;

mod accessors;
mod operations;

/// Runtime state for [`crate::SteamworksNetworkingMessagesPlugin`].
#[derive(Clone, Debug, Resource)]
pub struct SteamworksNetworkingMessagesState {
    last_error: Option<SteamworksNetworkingMessagesError>,
    received_messages: Vec<SteamworksNetworkingMessage>,
    last_connection_info: Option<SteamworksNetworkingMessagesConnectionInfo>,
    last_session_request: Option<SteamworksNetworkingMessagesSessionRequestInfo>,
    last_session_failure: Option<SteamworksNetworkingMessagesConnectionInfo>,
    sent_count: u64,
    received_count: u64,
    session_request_count: u64,
    session_failure_count: u64,
    callbacks_registered: bool,
    auto_accept_session_requests: Arc<Mutex<bool>>,
    callback_results: Arc<Mutex<Vec<SteamworksNetworkingMessagesResult>>>,
}

impl Default for SteamworksNetworkingMessagesState {
    fn default() -> Self {
        Self::new(true)
    }
}

impl SteamworksNetworkingMessagesState {
    /// Creates state with the requested session request policy.
    pub fn new(auto_accept_session_requests: bool) -> Self {
        Self {
            last_error: None,
            received_messages: Vec::new(),
            last_connection_info: None,
            last_session_request: None,
            last_session_failure: None,
            sent_count: 0,
            received_count: 0,
            session_request_count: 0,
            session_failure_count: 0,
            callbacks_registered: false,
            auto_accept_session_requests: Arc::new(Mutex::new(auto_accept_session_requests)),
            callback_results: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
