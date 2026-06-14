use thiserror::Error;

/// Synchronous errors from [`crate::SteamworksAppsPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksAppsError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks apps command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
}

impl SteamworksAppsError {
    pub(in crate::apps) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }
}
