use bevy_ecs::message::Message;

mod constructors;

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
