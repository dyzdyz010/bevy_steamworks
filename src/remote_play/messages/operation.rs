use super::super::{SteamworksRemotePlaySessionInfo, SteamworksRemotePlaySessionSnapshot};

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
