use std::path::PathBuf;

use thiserror::Error;

/// Screenshot library submission accepted by Steam.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksSubmittedScreenshot {
    /// Steam screenshot handle.
    pub handle: steamworks::screenshots::ScreenshotHandle,
    /// Screenshot image file path submitted.
    pub filename: PathBuf,
    /// Optional thumbnail image file path submitted.
    pub thumbnail_filename: Option<PathBuf>,
    /// Screenshot width in pixels.
    pub width: i32,
    /// Screenshot height in pixels.
    pub height: i32,
}

/// Screenshot ready callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksScreenshotReady {
    /// Screenshot handle, or the error reported by Steam.
    pub local_handle:
        Result<steamworks::screenshots::ScreenshotHandle, SteamworksScreenshotReadyError>,
}

/// Cloneable, comparable mirror of upstream
/// [`steamworks::screenshots::ScreenshotLibraryAddError`].
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum SteamworksScreenshotLibraryError {
    /// Steam failed to save the screenshot file for an unspecified reason.
    #[error("the screenshot file could not be saved")]
    SavingFailed,
    /// One of the provided paths was invalid or could not be canonicalized.
    #[error("invalid screenshot path")]
    InvalidPath,
}

impl From<steamworks::screenshots::ScreenshotLibraryAddError> for SteamworksScreenshotLibraryError {
    fn from(error: steamworks::screenshots::ScreenshotLibraryAddError) -> Self {
        match error {
            steamworks::screenshots::ScreenshotLibraryAddError::SavingFailed => Self::SavingFailed,
            steamworks::screenshots::ScreenshotLibraryAddError::InvalidPath => Self::InvalidPath,
        }
    }
}

/// Cloneable, comparable mirror of upstream
/// [`steamworks::screenshots::ScreenshotReadyError`].
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum SteamworksScreenshotReadyError {
    /// The screenshot could not be loaded or parsed.
    #[error("the screenshot could not be loaded or parsed")]
    Fail,
    /// The screenshot could not be saved to disk.
    #[error("the screenshot could not be saved to disk")]
    IoFailure,
}

impl From<steamworks::screenshots::ScreenshotReadyError> for SteamworksScreenshotReadyError {
    fn from(error: steamworks::screenshots::ScreenshotReadyError) -> Self {
        match error {
            steamworks::screenshots::ScreenshotReadyError::Fail => Self::Fail,
            steamworks::screenshots::ScreenshotReadyError::IoFailure => Self::IoFailure,
        }
    }
}
