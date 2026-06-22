use bevy_ecs::prelude::Resource;

use crate::cache::trim_oldest;

use super::{
    messages::SteamworksTimelineError,
    types::{
        SteamworksTimelineEventInfo, SteamworksTimelineGameMode, SteamworksTimelineStateDescription,
    },
};

mod accessors;
mod operations;

pub(in crate::timeline) const STEAMWORKS_TIMELINE_STATE_EVENT_CACHE_LIMIT: usize = 1_024;

/// Runtime state for [`super::SteamworksTimelinePlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksTimelineState {
    last_error: Option<SteamworksTimelineError>,
    game_mode: Option<SteamworksTimelineGameMode>,
    state_description: Option<SteamworksTimelineStateDescription>,
    events: Vec<SteamworksTimelineEventInfo>,
    last_event: Option<SteamworksTimelineEventInfo>,
    event_count: u64,
}

pub(super) fn push_timeline_event(
    events: &mut Vec<SteamworksTimelineEventInfo>,
    event: SteamworksTimelineEventInfo,
) {
    events.push(event);
    trim_oldest(events, STEAMWORKS_TIMELINE_STATE_EVENT_CACHE_LIMIT);
}
