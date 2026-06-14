use std::time::Duration;

/// Steam Timeline game mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksTimelineGameMode {
    /// The player is fully loaded into the game and playing.
    Playing,
    /// The player is in a multiplayer lobby.
    Staging,
    /// The player is in the game's main menu or a pause menu.
    Menus,
    /// The player is waiting for a loading screen.
    LoadingScreen,
}

impl SteamworksTimelineGameMode {
    pub(super) fn to_steam(self) -> steamworks::TimelineGameMode {
        match self {
            Self::Playing => steamworks::TimelineGameMode::Playing,
            Self::Staging => steamworks::TimelineGameMode::Staging,
            Self::Menus => steamworks::TimelineGameMode::Menus,
            Self::LoadingScreen => steamworks::TimelineGameMode::LoadingScreen,
        }
    }
}

/// Steam Timeline event clip priority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksTimelineEventClipPriority {
    /// This event is not appropriate as a clip.
    None,
    /// The user may want to make a clip around this event.
    Standard,
    /// The player is likely to want a clip around this event.
    Featured,
}

impl SteamworksTimelineEventClipPriority {
    pub(super) fn to_steam(self) -> steamworks::TimelineEventClipPriority {
        match self {
            Self::None => steamworks::TimelineEventClipPriority::None,
            Self::Standard => steamworks::TimelineEventClipPriority::Standard,
            Self::Featured => steamworks::TimelineEventClipPriority::Featured,
        }
    }
}

/// Timeline state description tracked by this command layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksTimelineStateDescription {
    /// Timeline tooltip text.
    pub description: String,
    /// Duration over which Steam should apply the change.
    pub duration: Duration,
}

/// Timeline event submitted through this command layer.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksTimelineEventInfo {
    /// Icon identifier configured for the game in Steamworks.
    pub icon: String,
    /// Event title.
    pub title: String,
    /// Event description.
    pub description: String,
    /// Event priority. Higher priority events are shown more prominently by Steam.
    pub priority: u32,
    /// Start offset in seconds relative to now.
    pub start_offset_seconds: f32,
    /// Event duration.
    pub duration: Duration,
    /// Clip priority submitted to Steam.
    pub clip_priority: SteamworksTimelineEventClipPriority,
}
