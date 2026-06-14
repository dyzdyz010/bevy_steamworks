use std::sync::{Arc, Mutex};

use bevy_ecs::prelude::Resource;

use super::SteamworksUgcItemUpdateProgress;

#[derive(Clone, Debug, Default, Resource)]
pub(super) struct SteamworksUgcUpdateWatches {
    storage: Arc<Mutex<SteamworksUgcUpdateWatchStorage>>,
}

impl SteamworksUgcUpdateWatches {
    pub(super) fn insert(&self, request_id: u64, handle: steamworks::UpdateWatchHandle) {
        self.storage
            .lock()
            .expect("Steamworks UGC update watch storage mutex was poisoned")
            .insert(request_id, handle);
    }

    pub(super) fn progress(&self, request_id: u64) -> Option<SteamworksUgcItemUpdateProgress> {
        self.storage
            .lock()
            .expect("Steamworks UGC update watch storage mutex was poisoned")
            .progress(request_id)
    }

    pub(super) fn remove(&self, request_id: u64) -> bool {
        self.storage
            .lock()
            .expect("Steamworks UGC update watch storage mutex was poisoned")
            .remove(request_id)
    }

    pub(super) fn len(&self) -> usize {
        self.storage
            .lock()
            .expect("Steamworks UGC update watch storage mutex was poisoned")
            .len()
    }
}

#[derive(Default)]
struct SteamworksUgcUpdateWatchStorage {
    watches: std::collections::HashMap<u64, steamworks::UpdateWatchHandle>,
}

impl std::fmt::Debug for SteamworksUgcUpdateWatchStorage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SteamworksUgcUpdateWatchStorage")
            .field("watch_count", &self.watches.len())
            .finish()
    }
}

impl SteamworksUgcUpdateWatchStorage {
    pub(super) fn insert(&mut self, request_id: u64, handle: steamworks::UpdateWatchHandle) {
        self.watches.insert(request_id, handle);
    }

    pub(super) fn progress(&self, request_id: u64) -> Option<SteamworksUgcItemUpdateProgress> {
        let handle = self.watches.get(&request_id)?;
        let (status, processed_bytes, total_bytes) = handle.progress();
        Some(SteamworksUgcItemUpdateProgress {
            request_id,
            status,
            processed_bytes,
            total_bytes,
        })
    }

    pub(super) fn remove(&mut self, request_id: u64) -> bool {
        self.watches.remove(&request_id).is_some()
    }

    pub(super) fn len(&self) -> usize {
        self.watches.len()
    }
}
