use std::path::PathBuf;

use super::super::SteamworksScreenshotReady;

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
    /// [`crate::SteamworksScreenshotsOperation::ScreenshotReady`].
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
