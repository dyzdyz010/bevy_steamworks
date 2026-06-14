use std::sync::{Arc, Mutex};

use bevy_ecs::prelude::Resource;

use super::SteamworksUgcResult;

#[derive(Clone, Debug, Default, Resource)]
pub(super) struct SteamworksUgcAsyncResults {
    queue: Arc<Mutex<Vec<SteamworksUgcResult>>>,
}

impl SteamworksUgcAsyncResults {
    pub(super) fn push(&self, result: SteamworksUgcResult) {
        self.queue
            .lock()
            .expect("Steamworks UGC async result mutex was poisoned")
            .push(result);
    }

    pub(super) fn drain(&self) -> Vec<SteamworksUgcResult> {
        self.queue
            .lock()
            .expect("Steamworks UGC async result mutex was poisoned")
            .drain(..)
            .collect()
    }
}
