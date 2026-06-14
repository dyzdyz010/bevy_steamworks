use thiserror::Error;

/// Synchronous errors from [`crate::SteamworksTimelinePlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksTimelineError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks Timeline command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A floating-point value is not finite.
    #[error("Steamworks Timeline command field {field} must be finite")]
    InvalidFloat {
        /// Field whose value was invalid.
        field: &'static str,
    },
}

impl SteamworksTimelineError {
    pub(in crate::timeline) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    pub(in crate::timeline) fn invalid_float(field: &'static str) -> Self {
        Self::InvalidFloat { field }
    }
}
