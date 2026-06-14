use bevy_ecs::message::Message;
use thiserror::Error;

use super::types::{SteamworksRemotePlaySessionInfo, SteamworksRemotePlaySessionSnapshot};

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
    /// Creates a [`SteamworksRemotePlayCommand::GetSession`] command.
    pub fn get_session(session: steamworks::RemotePlaySessionId) -> Self {
        Self::GetSession { session }
    }

    /// Creates a [`SteamworksRemotePlayCommand::Invite`] command.
    pub fn invite(session: steamworks::RemotePlaySessionId, friend: steamworks::SteamId) -> Self {
        Self::Invite { session, friend }
    }
}

/// A successfully submitted Steam Remote Play operation or synchronous read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksRemotePlayOperation {
    /// Active Remote Play sessions were listed.
    SessionsListed {
        /// Session snapshots.
        sessions: Vec<SteamworksRemotePlaySessionSnapshot>,
    },
    /// One Remote Play session was read.
    SessionRead {
        /// Session snapshot.
        session: SteamworksRemotePlaySessionInfo,
    },
    /// A Remote Play Together invite was submitted.
    InviteSubmitted {
        /// Session context supplied by the caller.
        session: steamworks::RemotePlaySessionId,
        /// Friend Steam ID invited.
        friend: steamworks::SteamId,
    },
    /// A Remote Play session connected callback was observed.
    SessionConnected {
        /// Session that connected.
        session: steamworks::RemotePlaySessionId,
    },
    /// A Remote Play session disconnected callback was observed.
    SessionDisconnected {
        /// Session that disconnected.
        session: steamworks::RemotePlaySessionId,
    },
}

/// Result message emitted by [`super::SteamworksRemotePlayPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksRemotePlayResult {
    /// The command or observed callback was processed successfully.
    Ok(SteamworksRemotePlayOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksRemotePlayCommand,
        /// Failure reason.
        error: SteamworksRemotePlayError,
    },
}

/// Synchronous errors from [`super::SteamworksRemotePlayPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksRemotePlayError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// The upstream Steamworks API rejected the invite.
    #[error("Steamworks Remote Play Together invite failed")]
    InviteFailed,
}
