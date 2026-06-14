use std::sync::{Arc, Mutex};

use bevy_ecs::prelude::Resource;

use super::SteamworksMatchmakingResult;

#[derive(Clone, Debug, Default, Resource)]
pub(super) struct SteamworksMatchmakingAsyncResults {
    queue: Arc<Mutex<Vec<SteamworksMatchmakingResult>>>,
}

impl SteamworksMatchmakingAsyncResults {
    pub(super) fn push(&self, result: SteamworksMatchmakingResult) {
        self.queue
            .lock()
            .expect("Steamworks matchmaking async result mutex was poisoned")
            .push(result);
    }

    pub(super) fn drain(&self) -> Vec<SteamworksMatchmakingResult> {
        self.queue
            .lock()
            .expect("Steamworks matchmaking async result mutex was poisoned")
            .drain(..)
            .collect()
    }
}
