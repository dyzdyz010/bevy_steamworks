use std::time::Duration;

use super::super::{SteamworksTimelineEventInfo, SteamworksTimelineGameMode};

/// A Steam Timeline operation accepted by this command layer.
///
/// The operation has been submitted to the upstream `steamworks` wrapper. Steam
/// may still no-op the request when the runtime Timeline interface is
/// unavailable.
#[derive(Clone, Debug, PartialEq)]
pub enum SteamworksTimelineOperation {
    /// Timeline game mode was submitted.
    GameModeSet {
        /// Game mode submitted to Steam.
        mode: SteamworksTimelineGameMode,
    },
    /// Timeline state description was submitted.
    StateDescriptionSet {
        /// Tooltip text.
        description: String,
        /// Duration over which Steam should apply the change.
        duration: Duration,
    },
    /// Timeline state description clear was submitted.
    StateDescriptionCleared {
        /// Duration over which Steam should apply the change.
        duration: Duration,
    },
    /// Timeline event was submitted.
    TimelineEventAdded {
        /// Event details submitted to Steam.
        event: SteamworksTimelineEventInfo,
    },
}
