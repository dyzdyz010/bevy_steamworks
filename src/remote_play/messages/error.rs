use thiserror::Error;

/// Synchronous errors from [`crate::SteamworksRemotePlayPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksRemotePlayError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// The upstream Steamworks API rejected the invite.
    #[error("Steamworks Remote Play Together invite failed")]
    InviteFailed,
}
