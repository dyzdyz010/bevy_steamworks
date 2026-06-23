use std::sync::{Arc, Mutex};

use super::{
    push_bounded_received_messages, push_bounded_session_failure, push_bounded_session_request,
    upsert_bounded_session_decision, SteamworksNetworkingMessagesState,
};
use crate::networking_messages::{
    SteamworksNetworkingMessagesError, SteamworksNetworkingMessagesOperation,
    SteamworksNetworkingMessagesResult, SteamworksNetworkingMessagesSessionDecision,
    SteamworksNetworkingPeer,
};

impl SteamworksNetworkingMessagesState {
    pub(in crate::networking_messages) fn record_error(
        &mut self,
        error: SteamworksNetworkingMessagesError,
    ) {
        self.last_error = Some(error);
    }

    pub(in crate::networking_messages) fn record_operation(
        &mut self,
        operation: &SteamworksNetworkingMessagesOperation,
    ) {
        match operation {
            SteamworksNetworkingMessagesOperation::MessageSent { .. } => {
                self.sent_count = self.sent_count.saturating_add(1);
            }
            SteamworksNetworkingMessagesOperation::MessagesReceived { messages, .. } => {
                self.received_count = self
                    .received_count
                    .saturating_add(messages.len().try_into().unwrap_or(u64::MAX));
                push_bounded_received_messages(&mut self.recent_received_messages, messages);
                self.received_messages.clone_from(messages);
            }
            SteamworksNetworkingMessagesOperation::SessionConnectionInfoRead { info, .. } => {
                self.last_connection_info = Some(info.clone());
            }
            SteamworksNetworkingMessagesOperation::AutoAcceptSessionRequestsSet { enabled } => {
                self.set_auto_accept_session_requests(*enabled);
            }
            SteamworksNetworkingMessagesOperation::SessionRequestDecisionSet { decision } => {
                self.set_session_request_decision(decision.clone());
            }
            SteamworksNetworkingMessagesOperation::SessionRequestDecisionCleared { peer } => {
                self.clear_session_request_decision(peer);
            }
            SteamworksNetworkingMessagesOperation::SessionRequestReceived { request } => {
                self.session_request_count = self.session_request_count.saturating_add(1);
                if request.accepted {
                    self.session_accept_count = self.session_accept_count.saturating_add(1);
                } else {
                    self.session_reject_count = self.session_reject_count.saturating_add(1);
                }
                push_bounded_session_request(&mut self.session_requests, request.clone());
                self.last_session_request = Some(request.clone());
            }
            SteamworksNetworkingMessagesOperation::SessionFailed { info } => {
                self.session_failure_count = self.session_failure_count.saturating_add(1);
                push_bounded_session_failure(&mut self.session_failures, info.clone());
                self.last_session_failure = Some(info.clone());
            }
        }
    }

    pub(in crate::networking_messages) fn callbacks_registered(&self) -> bool {
        self.callbacks_registered
    }

    pub(in crate::networking_messages) fn mark_callbacks_registered(&mut self) {
        self.callbacks_registered = true;
    }

    pub(in crate::networking_messages) fn auto_accept_session_requests_policy(
        &self,
    ) -> Arc<Mutex<bool>> {
        self.auto_accept_session_requests.clone()
    }

    pub(in crate::networking_messages) fn session_request_decision_policy(
        &self,
    ) -> Arc<Mutex<Vec<SteamworksNetworkingMessagesSessionDecision>>> {
        self.session_request_decisions.clone()
    }

    pub(in crate::networking_messages) fn callback_results_queue(
        &self,
    ) -> Arc<Mutex<Vec<SteamworksNetworkingMessagesResult>>> {
        self.callback_results.clone()
    }

    pub(in crate::networking_messages) fn set_auto_accept_session_requests(&self, enabled: bool) {
        *self
            .auto_accept_session_requests
            .lock()
            .expect("Steamworks Networking Messages policy mutex was poisoned") = enabled;
    }

    pub(in crate::networking_messages) fn set_session_request_decision(
        &self,
        decision: SteamworksNetworkingMessagesSessionDecision,
    ) {
        upsert_bounded_session_decision(
            &mut self
                .session_request_decisions
                .lock()
                .expect("Steamworks Networking Messages decision mutex was poisoned"),
            decision,
        );
    }

    pub(in crate::networking_messages) fn clear_session_request_decision(
        &self,
        peer: &SteamworksNetworkingPeer,
    ) {
        self.session_request_decisions
            .lock()
            .expect("Steamworks Networking Messages decision mutex was poisoned")
            .retain(|decision| &decision.peer != peer);
    }

    pub(in crate::networking_messages) fn drain_callback_results(
        &self,
    ) -> Vec<SteamworksNetworkingMessagesResult> {
        self.callback_results
            .lock()
            .expect("Steamworks Networking Messages callback queue mutex was poisoned")
            .drain(..)
            .collect()
    }
}
