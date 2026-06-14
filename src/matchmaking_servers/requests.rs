use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bevy_ecs::prelude::Resource;

use crate::SteamworksClient;

use super::{
    SteamworksMatchmakingServersResult, SteamworksServerListReleaseError,
    SteamworksServerListRequestId,
};

#[derive(Clone, Debug, Default, Resource)]
pub(super) struct SteamworksMatchmakingServersAsyncResults {
    queue: Arc<Mutex<Vec<SteamworksMatchmakingServersResult>>>,
}

impl SteamworksMatchmakingServersAsyncResults {
    pub(super) fn push(&self, result: SteamworksMatchmakingServersResult) {
        self.queue
            .lock()
            .expect("Steamworks matchmaking servers async result mutex was poisoned")
            .push(result);
    }

    pub(super) fn drain(&self) -> Vec<SteamworksMatchmakingServersResult> {
        self.queue
            .lock()
            .expect("Steamworks matchmaking servers async result mutex was poisoned")
            .drain(..)
            .collect()
    }
}

#[derive(Clone, Debug, Default, Resource)]
pub(super) struct SteamworksMatchmakingServerListRequests {
    storage: Arc<Mutex<SteamworksMatchmakingServerListRequestStorage>>,
}

impl SteamworksMatchmakingServerListRequests {
    pub(super) fn insert(
        &self,
        request: SteamworksServerListRequestId,
        client: &SteamworksClient,
        handle: Arc<Mutex<steamworks::ServerListRequest>>,
    ) {
        self.storage
            .lock()
            .expect("Steamworks server-list request storage mutex was poisoned")
            .insert(request, client.clone_inner(), handle);
    }

    pub(super) fn get(
        &self,
        request: SteamworksServerListRequestId,
    ) -> Option<Arc<Mutex<steamworks::ServerListRequest>>> {
        self.storage
            .lock()
            .expect("Steamworks server-list request storage mutex was poisoned")
            .get(request)
    }

    pub(super) fn remove(
        &self,
        request: SteamworksServerListRequestId,
    ) -> Option<Arc<Mutex<steamworks::ServerListRequest>>> {
        self.storage
            .lock()
            .expect("Steamworks server-list request storage mutex was poisoned")
            .remove(request)
    }

    pub(super) fn len(&self) -> usize {
        self.storage
            .lock()
            .expect("Steamworks server-list request storage mutex was poisoned")
            .len()
    }
}

#[derive(Default)]
struct SteamworksMatchmakingServerListRequestStorage {
    requests: HashMap<SteamworksServerListRequestId, SteamworksServerListRequestHandle>,
}

impl std::fmt::Debug for SteamworksMatchmakingServerListRequestStorage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SteamworksMatchmakingServerListRequestStorage")
            .field("request_count", &self.requests.len())
            .finish()
    }
}

impl SteamworksMatchmakingServerListRequestStorage {
    pub(super) fn insert(
        &mut self,
        request: SteamworksServerListRequestId,
        client: steamworks::Client,
        handle: Arc<Mutex<steamworks::ServerListRequest>>,
    ) {
        self.requests.insert(
            request,
            SteamworksServerListRequestHandle {
                handle,
                _client: client,
            },
        );
    }

    pub(super) fn get(
        &self,
        request: SteamworksServerListRequestId,
    ) -> Option<Arc<Mutex<steamworks::ServerListRequest>>> {
        self.requests
            .get(&request)
            .map(|request| request.handle.clone())
    }

    pub(super) fn remove(
        &mut self,
        request: SteamworksServerListRequestId,
    ) -> Option<Arc<Mutex<steamworks::ServerListRequest>>> {
        self.requests.remove(&request).map(|request| request.handle)
    }

    pub(super) fn len(&self) -> usize {
        self.requests.len()
    }
}

impl Drop for SteamworksMatchmakingServerListRequestStorage {
    fn drop(&mut self) {
        for (request_id, request) in self.requests.drain() {
            match request.handle.lock() {
                Ok(mut request) => match request.release() {
                    Ok(()) => {
                        tracing::debug!(
                            target: "bevy_steamworks",
                            request = ?request_id,
                            "released Steamworks server-list request during plugin shutdown"
                        );
                    }
                    Err(source) => {
                        let reason = SteamworksServerListReleaseError::from(source);
                        tracing::error!(
                            target: "bevy_steamworks",
                            request = ?request_id,
                            reason = ?reason,
                            "failed to release Steamworks server-list request during plugin shutdown"
                        );
                    }
                },
                Err(_) => {
                    tracing::error!(
                        target: "bevy_steamworks",
                        request = ?request_id,
                        "failed to lock Steamworks server-list request during plugin shutdown"
                    );
                }
            }
        }
    }
}

#[derive(Clone)]
struct SteamworksServerListRequestHandle {
    handle: Arc<Mutex<steamworks::ServerListRequest>>,
    _client: steamworks::Client,
}
