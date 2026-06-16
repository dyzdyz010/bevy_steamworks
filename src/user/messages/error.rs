use thiserror::Error;

use super::super::SteamworksAuthSessionError;

/// Synchronous errors from [`crate::SteamworksUserPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksUserError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks user command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A remote authentication session was requested with no ticket bytes.
    #[error("Steamworks user command requires a non-empty authentication ticket")]
    EmptyTicket,
    /// An authentication ticket was requested for an invalid networking identity.
    #[error("Steamworks user command requires a valid networking identity")]
    InvalidNetworkingIdentity,
    /// The upstream Steamworks API rejected an authentication session.
    #[error("Steamworks authentication session failed: {source}")]
    AuthSession {
        /// Authentication session failure reason.
        #[source]
        source: SteamworksAuthSessionError,
    },
}

impl SteamworksUserError {
    pub(in crate::user) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    pub(in crate::user) fn auth_session(source: steamworks::AuthSessionError) -> Self {
        Self::AuthSession {
            source: source.into(),
        }
    }
}
