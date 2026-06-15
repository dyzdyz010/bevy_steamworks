use bevy_ecs::message::Message;

/// A high-level command for Steam Remote Play workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksRemotePlayCommand {
    /// Read all active Remote Play sessions.
    ListSessions,
    /// Read one Remote Play session snapshot.
    GetSession {
        /// Session to inspect.
        session: steamworks::RemotePlaySessionId,
    },
    /// Invite a friend to join using Remote Play Together.
    ///
    /// The current upstream Rust wrapper exposes invites through
    /// [`steamworks::RemotePlaySession`]. The session ID is retained in the
    /// result as caller-provided context, but Steam's invite acceptance is not
    /// proof that the invite was session-specific.
    Invite {
        /// Session context used to access the upstream invite helper.
        session: steamworks::RemotePlaySessionId,
        /// Friend Steam ID to invite.
        friend: steamworks::SteamId,
    },
}

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
