use thiserror::Error;

/// Synchronous errors from [`crate::SteamworksUtilsPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksUtilsError {
    /// No compatible Steamworks resource exists for the requested utility command.
    ///
    /// Read-only utility queries can use either [`crate::SteamworksClient`] or
    /// [`crate::SteamworksServer`]. Overlay, warning-callback, and gamepad text
    /// input commands still require [`crate::SteamworksClient`].
    #[error("Steamworks Utils resource is not available for this command")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks utils command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A floating text input rectangle had non-positive dimensions.
    #[error("Steamworks floating text input size {width}x{height} must be positive")]
    InvalidFloatingTextInputBounds {
        /// Requested width.
        width: i32,
        /// Requested height.
        height: i32,
    },
}

impl SteamworksUtilsError {
    pub(in crate::utils) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }
}
