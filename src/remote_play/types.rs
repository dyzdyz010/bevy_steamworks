/// Snapshot of one Steam Remote Play session returned by bulk listing.
///
/// The upstream `steamworks` API does not expose the session ID from
/// [`steamworks::RemotePlay::sessions`]. Session IDs are available from
/// [`super::SteamworksRemotePlayOperation::SessionConnected`] or
/// [`crate::SteamworksEvent::RemotePlayConnected`] and can then be queried with
/// [`super::SteamworksRemotePlayCommand::GetSession`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksRemotePlaySessionSnapshot {
    /// Steam user associated with this session.
    pub user: steamworks::SteamId,
    /// Client device name, or `None` if the session has expired.
    pub client_name: Option<String>,
    /// Client device form factor, or `None` if unknown or expired.
    pub client_form_factor: Option<steamworks::SteamDeviceFormFactor>,
    /// Client resolution, or `None` if the session has expired.
    pub client_resolution: Option<(u32, u32)>,
}

/// Snapshot of one Steam Remote Play session with a known session ID.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksRemotePlaySessionInfo {
    /// Remote Play session ID.
    pub session: steamworks::RemotePlaySessionId,
    /// Steam user associated with this session.
    pub user: steamworks::SteamId,
    /// Client device name, or `None` if the session has expired.
    pub client_name: Option<String>,
    /// Client device form factor, or `None` if unknown or expired.
    pub client_form_factor: Option<steamworks::SteamDeviceFormFactor>,
    /// Client resolution, or `None` if the session has expired.
    pub client_resolution: Option<(u32, u32)>,
}

/// Remote Play Together invite accepted by Steam.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SteamworksRemotePlayInvite {
    /// Session context supplied by the caller.
    pub session: steamworks::RemotePlaySessionId,
    /// Friend Steam ID invited.
    pub friend: steamworks::SteamId,
}
