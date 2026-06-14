use std::path::PathBuf;

use bevy_ecs::message::Message;
use thiserror::Error;

use super::types::{SteamworksScreenshotLibraryError, SteamworksScreenshotReady};

/// A high-level command for Steam screenshot workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksScreenshotsCommand {
    /// Set whether this app handles Steam screenshot requests itself.
    ///
    /// When enabled, Steam emits [`crate::SteamworksEvent::ScreenshotRequested`]
    /// and [`SteamworksScreenshotsOperation::ScreenshotRequested`], and the game
    /// is expected to capture and submit a screenshot.
    HookScreenshots {
        /// Whether screenshots should be hooked by the app.
        hook: bool,
    },
    /// Read whether this app is currently hooking Steam screenshots.
    IsScreenshotsHooked,
    /// Trigger a Steam screenshot.
    ///
    /// Depending on hook state, Steam may emit
    /// [`crate::SteamworksEvent::ScreenshotRequested`] /
    /// [`SteamworksScreenshotsOperation::ScreenshotRequested`] and later
    /// [`crate::SteamworksEvent::ScreenshotReady`] /
    /// [`SteamworksScreenshotsOperation::ScreenshotReady`].
    TriggerScreenshot,
    /// Add an existing screenshot image file to the user's Steam screenshot library.
    ///
    /// This submits the request and returns a handle immediately if Steam accepts
    /// it. Final save confirmation arrives later through both
    /// [`crate::SteamworksEvent::ScreenshotReady`] and
    /// [`SteamworksScreenshotsOperation::ScreenshotReady`].
    ///
    /// The upstream wrapper canonicalizes the provided paths before submitting
    /// them to Steam, so use local paths and keep this command low-frequency.
    AddScreenshotToLibrary {
        /// Screenshot image file path.
        filename: PathBuf,
        /// Optional thumbnail image file path.
        thumbnail_filename: Option<PathBuf>,
        /// Screenshot width in pixels.
        width: i32,
        /// Screenshot height in pixels.
        height: i32,
    },
}

impl SteamworksScreenshotsCommand {
    /// Creates a [`SteamworksScreenshotsCommand::HookScreenshots`] command.
    pub fn hook_screenshots(hook: bool) -> Self {
        Self::HookScreenshots { hook }
    }

    /// Creates a [`SteamworksScreenshotsCommand::AddScreenshotToLibrary`] command.
    pub fn add_screenshot_to_library(
        filename: impl Into<PathBuf>,
        thumbnail_filename: Option<impl Into<PathBuf>>,
        width: i32,
        height: i32,
    ) -> Self {
        Self::AddScreenshotToLibrary {
            filename: filename.into(),
            thumbnail_filename: thumbnail_filename.map(Into::into),
            width,
            height,
        }
    }
}

/// A successfully submitted Steam screenshot operation or synchronous read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksScreenshotsOperation {
    /// Screenshot hook state was set.
    ScreenshotsHookSet {
        /// Hook state submitted to Steam.
        hook: bool,
    },
    /// Screenshot hook state was read.
    ScreenshotsHookedRead {
        /// Whether screenshots are hooked by the app.
        hooked: bool,
    },
    /// A Steam screenshot was triggered.
    ScreenshotTriggered,
    /// A screenshot library add request was accepted by Steam.
    ///
    /// Final save confirmation arrives later through both
    /// [`crate::SteamworksEvent::ScreenshotReady`] and
    /// [`SteamworksScreenshotsOperation::ScreenshotReady`].
    ScreenshotLibraryAddSubmitted {
        /// Steam screenshot handle.
        handle: steamworks::screenshots::ScreenshotHandle,
        /// Screenshot image file path submitted.
        filename: PathBuf,
        /// Optional thumbnail image file path submitted.
        thumbnail_filename: Option<PathBuf>,
        /// Screenshot width in pixels.
        width: i32,
        /// Screenshot height in pixels.
        height: i32,
    },
    /// Steam requested a screenshot from this app.
    ScreenshotRequested {
        /// Total number of screenshot request callbacks observed by this plugin.
        count: u64,
    },
    /// Steam reported a screenshot ready result.
    ScreenshotReady {
        /// Callback snapshot.
        ready: SteamworksScreenshotReady,
    },
}

/// Result message emitted by [`super::SteamworksScreenshotsPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksScreenshotsResult {
    /// The command or observed callback was processed successfully.
    Ok(SteamworksScreenshotsOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksScreenshotsCommand,
        /// Failure reason.
        error: SteamworksScreenshotsError,
    },
}

/// Synchronous errors from [`super::SteamworksScreenshotsPlugin`].
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
    pub(super) fn library_add_failed(
        source: steamworks::screenshots::ScreenshotLibraryAddError,
    ) -> Self {
        Self::LibraryAddFailed {
            source: source.into(),
        }
    }
}
