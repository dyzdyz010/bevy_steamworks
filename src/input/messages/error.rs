use thiserror::Error;

/// Synchronous errors from [`crate::SteamworksInputPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksInputError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks Input command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A handle passed to Steam is zero.
    #[error("Steamworks Input command field {field} contains an invalid zero handle")]
    InvalidHandle {
        /// Field that contained the invalid handle.
        field: &'static str,
    },
    /// Steam Input initialization returned false.
    #[error("Steam Input initialization failed")]
    InitFailed,
    /// Steam Input rejected the action manifest path.
    #[error("Steam Input rejected the action manifest path")]
    ActionManifestFileRejected,
    /// Steam Input returned an invalid zero handle for a lookup.
    #[error("Steam Input returned an invalid zero handle for {operation}")]
    InvalidHandleReturned {
        /// Lookup operation that returned an invalid handle.
        operation: &'static str,
    },
    /// Steam Input could not show the binding panel.
    #[error("Steam Input binding panel is unavailable")]
    BindingPanelUnavailable,
}

impl SteamworksInputError {
    pub(in crate::input) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    pub(in crate::input) fn invalid_handle(field: &'static str) -> Self {
        Self::InvalidHandle { field }
    }

    pub(in crate::input) fn invalid_handle_returned(operation: &'static str) -> Self {
        Self::InvalidHandleReturned { operation }
    }
}
