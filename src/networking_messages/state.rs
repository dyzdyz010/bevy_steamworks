use std::sync::{Arc, Mutex};

use bevy_ecs::prelude::Resource;

use super::*;

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

    pub(super) fn record_error(&mut self, error: SteamworksNetworkingMessagesError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksNetworkingMessagesOperation) {
        match operation {
            SteamworksNetworkingMessagesOperation::MessageSent { .. } => {
                self.sent_count = self.sent_count.saturating_add(1);
            }
            SteamworksNetworkingMessagesOperation::MessagesReceived { messages, .. } => {
                self.received_count = self
                    .received_count
                    .saturating_add(messages.len().try_into().unwrap_or(u64::MAX));
                self.received_messages.clone_from(messages);
            }
            SteamworksNetworkingMessagesOperation::SessionConnectionInfoRead { info, .. } => {
                self.last_connection_info = Some(info.clone());
            }
            SteamworksNetworkingMessagesOperation::AutoAcceptSessionRequestsSet { enabled } => {
                self.set_auto_accept_session_requests(*enabled);
            }
            SteamworksNetworkingMessagesOperation::SessionRequestReceived { request } => {
                self.session_request_count = self.session_request_count.saturating_add(1);
                self.last_session_request = Some(request.clone());
            }
            SteamworksNetworkingMessagesOperation::SessionFailed { info } => {
                self.session_failure_count = self.session_failure_count.saturating_add(1);
                self.last_session_failure = Some(info.clone());
            }
        }
    }

    pub(super) fn callbacks_registered(&self) -> bool {
        self.callbacks_registered
    }

    pub(super) fn mark_callbacks_registered(&mut self) {
        self.callbacks_registered = true;
    }

    pub(super) fn auto_accept_session_requests_policy(&self) -> Arc<Mutex<bool>> {
        self.auto_accept_session_requests.clone()
    }

    pub(super) fn callback_results_queue(
        &self,
    ) -> Arc<Mutex<Vec<SteamworksNetworkingMessagesResult>>> {
        self.callback_results.clone()
    }
    pub(super) fn set_auto_accept_session_requests(&self, enabled: bool) {
        *self
            .auto_accept_session_requests
            .lock()
            .expect("Steamworks Networking Messages policy mutex was poisoned") = enabled;
    }

    pub(super) fn drain_callback_results(&self) -> Vec<SteamworksNetworkingMessagesResult> {
        self.callback_results
            .lock()
            .expect("Steamworks Networking Messages callback queue mutex was poisoned")
            .drain(..)
            .collect()
    }
}
