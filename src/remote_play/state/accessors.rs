use super::SteamworksRemotePlayState;
use crate::remote_play::{
    SteamworksRemotePlayError, SteamworksRemotePlayInvite, SteamworksRemotePlaySessionInfo,
    SteamworksRemotePlaySessionSnapshot,
};

impl SteamworksRemotePlayState {
    /// Returns the most recent synchronous error observed by the Remote Play plugin.
    pub fn last_error(&self) -> Option<&SteamworksRemotePlayError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent active Remote Play session list read through the plugin.
    ///
    /// Upstream `steamworks` does not expose session IDs from bulk session listing;
    /// use [`crate::SteamworksRemotePlayCommand::GetSession`] with IDs from
    /// [`crate::SteamworksRemotePlayOperation::SessionConnected`] or
    /// [`crate::SteamworksEvent::RemotePlayConnected`] when a stable ID is needed.
    pub fn sessions(&self) -> &[SteamworksRemotePlaySessionSnapshot] {
        &self.sessions
    }

    /// Returns how many sessions were reported by the most recent bulk session list.
    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    /// Returns bulk session snapshots for one Steam user.
    pub fn sessions_for_user(
        &self,
        user: steamworks::SteamId,
    ) -> impl Iterator<Item = &SteamworksRemotePlaySessionSnapshot> + '_ {
        self.sessions
            .iter()
            .filter(move |session| session.user == user)
    }

    /// Returns the most recent bulk session snapshot for one Steam user.
    pub fn latest_session_for_user(
        &self,
        user: steamworks::SteamId,
    ) -> Option<&SteamworksRemotePlaySessionSnapshot> {
        self.sessions
            .iter()
            .rev()
            .find(|session| session.user == user)
    }

    /// Returns session snapshots read through ID-based commands.
    pub fn known_sessions(&self) -> &[SteamworksRemotePlaySessionInfo] {
        &self.known_sessions
    }

    /// Returns how many ID-based session snapshots are currently cached.
    pub fn known_session_count(&self) -> usize {
        self.known_sessions.len()
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

    /// Returns whether an ID-based session snapshot is currently cached.
    pub fn has_known_session(&self, session: steamworks::RemotePlaySessionId) -> bool {
        self.known_session(session).is_some()
    }

    /// Returns known ID-based session snapshots for one Steam user.
    pub fn known_sessions_for_user(
        &self,
        user: steamworks::SteamId,
    ) -> impl Iterator<Item = &SteamworksRemotePlaySessionInfo> + '_ {
        self.known_sessions
            .iter()
            .filter(move |session| session.user == user)
    }

    /// Returns the most recent known ID-based session snapshot for one Steam user.
    pub fn latest_known_session_for_user(
        &self,
        user: steamworks::SteamId,
    ) -> Option<&SteamworksRemotePlaySessionInfo> {
        self.known_sessions
            .iter()
            .rev()
            .find(|session| session.user == user)
    }

    /// Returns the latest known client name for a session, preserving an expired/no-name session as `Some(None)`.
    pub fn session_client_name(
        &self,
        session: steamworks::RemotePlaySessionId,
    ) -> Option<Option<&str>> {
        self.known_session(session)
            .map(|session| session.client_name.as_deref())
    }

    /// Returns the latest known client form factor for a session, preserving an unknown/expired form factor as `Some(None)`.
    pub fn session_client_form_factor(
        &self,
        session: steamworks::RemotePlaySessionId,
    ) -> Option<Option<steamworks::SteamDeviceFormFactor>> {
        self.known_session(session)
            .map(|session| session.client_form_factor)
    }

    /// Returns the latest known client resolution for a session, preserving an expired/no-resolution session as `Some(None)`.
    pub fn session_client_resolution(
        &self,
        session: steamworks::RemotePlaySessionId,
    ) -> Option<Option<(u32, u32)>> {
        self.known_session(session)
            .map(|session| session.client_resolution)
    }

    /// Returns the latest known Steam user for a session.
    pub fn session_user(
        &self,
        session: steamworks::RemotePlaySessionId,
    ) -> Option<steamworks::SteamId> {
        self.known_session(session).map(|session| session.user)
    }

    /// Returns session IDs observed as connected and not yet disconnected.
    ///
    /// This list is callback-driven and only reflects sessions observed while
    /// this plugin has been running. Use [`crate::SteamworksRemotePlayCommand::ListSessions`]
    /// for a fresh bulk snapshot from Steam.
    pub fn observed_connected_sessions(&self) -> &[steamworks::RemotePlaySessionId] {
        &self.observed_connected_sessions
    }

    /// Returns how many sessions are callback-observed as connected.
    pub fn observed_connected_session_count(&self) -> usize {
        self.observed_connected_sessions.len()
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
}
