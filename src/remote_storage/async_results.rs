use std::sync::{Arc, Mutex};

use bevy_ecs::prelude::Resource;

use super::messages::SteamworksRemoteStorageResult;

#[derive(Clone, Debug, Default, Resource)]
pub(super) struct SteamworksRemoteStorageAsyncResults {
    queue: Arc<Mutex<Vec<SteamworksRemoteStorageResult>>>,
}

impl SteamworksRemoteStorageAsyncResults {
    pub(super) fn push(&self, result: SteamworksRemoteStorageResult) {
        self.queue
            .lock()
            .expect("Steamworks Remote Storage async result mutex was poisoned")
            .push(result);
    }

    pub(super) fn drain(&self) -> Vec<SteamworksRemoteStorageResult> {
        self.queue
            .lock()
            .expect("Steamworks Remote Storage async result mutex was poisoned")
            .drain(..)
            .collect()
    }
}
