use thiserror::Error;

use super::super::SteamworksScreenshotLibraryError;

/// Synchronous errors from [`crate::SteamworksScreenshotsPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksScreenshotsError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// Screenshot dimensions must be positive.
    #[error("Steamworks screenshot dimensions must be positive, got {width}x{height}")]
    InvalidDimensions {
        /// Requested width.
        width: i32,
        /// Requested height.
        height: i32,
    },
    /// The upstream Steamworks API rejected the screenshot library add.
    #[error("Steamworks screenshot library add failed: {source}")]
    LibraryAddFailed {
        /// Failure reported by the upstream Steamworks wrapper.
        #[source]
        source: SteamworksScreenshotLibraryError,
    },
}

impl SteamworksScreenshotsError {
    pub(in crate::screenshots) fn library_add_failed(
        source: steamworks::screenshots::ScreenshotLibraryAddError,
    ) -> Self {
        Self::LibraryAddFailed {
            source: source.into(),
        }
    }
}
