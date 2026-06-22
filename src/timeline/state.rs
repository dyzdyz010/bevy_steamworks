use bevy_ecs::prelude::Resource;

use super::{
    messages::SteamworksTimelineError,
    types::{
        SteamworksTimelineEventInfo, SteamworksTimelineGameMode, SteamworksTimelineStateDescription,
    },
};

mod accessors;
mod operations;

/// Runtime state for [`super::SteamworksTimelinePlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksTimelineState {
    last_error: Option<SteamworksTimelineError>,
    game_mode: Option<SteamworksTimelineGameMode>,
    state_description: Option<SteamworksTimelineStateDescription>,
    last_event: Option<SteamworksTimelineEventInfo>,
    event_count: u64,
}
