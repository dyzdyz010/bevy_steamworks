use bevy_ecs::prelude::Resource;

use super::{
    messages::{SteamworksRemotePlayError, SteamworksRemotePlayOperation},
    types::{
        SteamworksRemotePlayInvite, SteamworksRemotePlaySessionInfo,
        SteamworksRemotePlaySessionSnapshot,
    },
};

/// Runtime state for [`super::SteamworksRemotePlayPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksRemotePlayState {
    last_error: Option<SteamworksRemotePlayError>,
    sessions: Vec<SteamworksRemotePlaySessionSnapshot>,
    known_sessions: Vec<SteamworksRemotePlaySessionInfo>,
    observed_connected_sessions: Vec<steamworks::RemotePlaySessionId>,
    last_submitted_invite: Option<SteamworksRemotePlayInvite>,
    submitted_invite_count: u64,
}

impl SteamworksRemotePlayState {
    /// Returns the most recent synchronous error observed by the Remote Play plugin.
    pub fn last_error(&self) -> Option<&SteamworksRemotePlayError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent active Remote Play session list read through the plugin.
    ///
    /// Upstream `steamworks` does not expose session IDs from bulk session listing;
    /// use [`super::SteamworksRemotePlayCommand::GetSession`] with IDs from
    /// [`SteamworksRemotePlayOperation::SessionConnected`] or
    /// [`crate::SteamworksEvent::RemotePlayConnected`] when a stable ID is needed.
    pub fn sessions(&self) -> &[SteamworksRemotePlaySessionSnapshot] {
        &self.sessions
    }

    /// Returns session snapshots read through ID-based commands.
    pub fn known_sessions(&self) -> &[SteamworksRemotePlaySessionInfo] {
        &self.known_sessions
    }

    /// Returns one ID-based session snapshot read through the plugin.
    pub fn known_session(
        &self,
        session: steamworks::RemotePlaySessionId,
    ) -> Option<&SteamworksRemotePlaySessionInfo> {
        self.known_sessions
            .iter()
            .find(|known| known.session == session)
    }

    /// Returns session IDs observed as connected and not yet disconnected.
    ///
    /// This list is callback-driven and only reflects sessions observed while
    /// this plugin has been running. Use [`super::SteamworksRemotePlayCommand::ListSessions`]
    /// for a fresh bulk snapshot from Steam.
    pub fn observed_connected_sessions(&self) -> &[steamworks::RemotePlaySessionId] {
        &self.observed_connected_sessions
    }

    /// Returns whether a session has been observed as connected and not yet disconnected.
    pub fn is_session_observed_connected(&self, session: steamworks::RemotePlaySessionId) -> bool {
        self.observed_connected_sessions.contains(&session)
    }

    /// Returns the most recent Remote Play Together invite submitted through this command layer.
    pub fn last_submitted_invite(&self) -> Option<SteamworksRemotePlayInvite> {
        self.last_submitted_invite
    }

    /// Returns how many Remote Play Together invites this plugin successfully submitted.
    pub fn submitted_invite_count(&self) -> u64 {
        self.submitted_invite_count
    }

    pub(super) fn record_error(&mut self, error: SteamworksRemotePlayError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksRemotePlayOperation) {
        match operation {
            SteamworksRemotePlayOperation::SessionsListed { sessions } => {
                self.sessions.clone_from(sessions);
            }
            SteamworksRemotePlayOperation::SessionRead { session } => {
                if let Some(existing) = self
                    .known_sessions
                    .iter_mut()
                    .find(|known| known.session == session.session)
                {
                    *existing = session.clone();
                } else {
                    self.known_sessions.push(session.clone());
                }
            }
            SteamworksRemotePlayOperation::InviteSubmitted { session, friend } => {
                self.last_submitted_invite = Some(SteamworksRemotePlayInvite {
                    session: *session,
                    friend: *friend,
                });
                self.submitted_invite_count = self.submitted_invite_count.saturating_add(1);
            }
            SteamworksRemotePlayOperation::SessionConnected { session } => {
                if !self.observed_connected_sessions.contains(session) {
                    self.observed_connected_sessions.push(*session);
                }
            }
            SteamworksRemotePlayOperation::SessionDisconnected { session } => {
                self.observed_connected_sessions
                    .retain(|known| known != session);
                self.known_sessions
                    .retain(|known| known.session != *session);
            }
        }
    }
}
