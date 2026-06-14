use std::sync::{Arc, Mutex};

use bevy_ecs::prelude::Resource;

use super::SteamworksStatsResult;

#[derive(Clone, Debug, Default, Resource)]
pub(super) struct SteamworksStatsAsyncResults {
    queue: Arc<Mutex<Vec<SteamworksStatsResult>>>,
}

impl SteamworksStatsAsyncResults {
    pub(super) fn push(&self, result: SteamworksStatsResult) {
        self.queue
            .lock()
            .expect("Steamworks stats async result mutex was poisoned")
            .push(result);
    }

    pub(super) fn drain(&self) -> Vec<SteamworksStatsResult> {
        self.queue
            .lock()
            .expect("Steamworks stats async result mutex was poisoned")
            .drain(..)
            .collect()
    }
}
