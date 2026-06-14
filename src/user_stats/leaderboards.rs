use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bevy_ecs::prelude::Resource;

use super::SteamworksLeaderboardId;

#[derive(Clone, Debug, Default, Resource)]
pub(super) struct SteamworksStatsLeaderboardHandles {
    storage: Arc<Mutex<SteamworksStatsLeaderboardHandleStorage>>,
}

impl SteamworksStatsLeaderboardHandles {
    pub(super) fn insert(&self, leaderboard: steamworks::Leaderboard) -> SteamworksLeaderboardId {
        self.storage
            .lock()
            .expect("Steamworks leaderboard handle storage mutex was poisoned")
            .insert(leaderboard)
    }

    pub(super) fn get(&self, id: SteamworksLeaderboardId) -> Option<steamworks::Leaderboard> {
        self.storage
            .lock()
            .expect("Steamworks leaderboard handle storage mutex was poisoned")
            .get(id)
    }

    pub(super) fn remove(&self, id: SteamworksLeaderboardId) -> Option<steamworks::Leaderboard> {
        self.storage
            .lock()
            .expect("Steamworks leaderboard handle storage mutex was poisoned")
            .remove(id)
    }

    pub(super) fn len(&self) -> usize {
        self.storage
            .lock()
            .expect("Steamworks leaderboard handle storage mutex was poisoned")
            .len()
    }
}

#[derive(Debug)]
struct SteamworksStatsLeaderboardHandleStorage {
    next_id: u64,
    handles: HashMap<SteamworksLeaderboardId, steamworks::Leaderboard>,
}

impl Default for SteamworksStatsLeaderboardHandleStorage {
    fn default() -> Self {
        Self {
            next_id: 1,
            handles: HashMap::default(),
        }
    }
}

impl SteamworksStatsLeaderboardHandleStorage {
    pub(super) fn insert(
        &mut self,
        leaderboard: steamworks::Leaderboard,
    ) -> SteamworksLeaderboardId {
        if let Some((id, _)) = self
            .handles
            .iter()
            .find(|(_, known)| known.raw() == leaderboard.raw())
        {
            return *id;
        }

        let id = SteamworksLeaderboardId::from_raw(self.next_id);
        self.next_id = self.next_id.saturating_add(1).max(1);
        self.handles.insert(id, leaderboard);
        id
    }

    pub(super) fn get(&self, id: SteamworksLeaderboardId) -> Option<steamworks::Leaderboard> {
        self.handles.get(&id).cloned()
    }

    pub(super) fn remove(
        &mut self,
        id: SteamworksLeaderboardId,
    ) -> Option<steamworks::Leaderboard> {
        self.handles.remove(&id)
    }

    pub(super) fn len(&self) -> usize {
        self.handles.len()
    }
}
