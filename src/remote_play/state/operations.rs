use super::SteamworksRemotePlayState;
use crate::remote_play::{
    SteamworksRemotePlayError, SteamworksRemotePlayInvite, SteamworksRemotePlayOperation,
};

impl SteamworksRemotePlayState {
    pub(in crate::remote_play) fn record_error(&mut self, error: SteamworksRemotePlayError) {
        self.last_error = Some(error);
    }

    pub(in crate::remote_play) fn record_operation(
        &mut self,
        operation: &SteamworksRemotePlayOperation,
    ) {
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
