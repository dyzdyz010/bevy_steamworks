use super::SteamworksRemotePlayCommand;

impl SteamworksRemotePlayCommand {
    /// Creates a [`crate::SteamworksRemotePlayCommand::ListSessions`] command.
    pub fn list_sessions() -> Self {
        Self::ListSessions
    }

    /// Creates a [`crate::SteamworksRemotePlayCommand::GetSession`] command.
    pub fn get_session(session: steamworks::RemotePlaySessionId) -> Self {
        Self::GetSession { session }
    }

    /// Creates a [`crate::SteamworksRemotePlayCommand::Invite`] command.
    pub fn invite(session: steamworks::RemotePlaySessionId, friend: steamworks::SteamId) -> Self {
        Self::Invite { session, friend }
    }
}
