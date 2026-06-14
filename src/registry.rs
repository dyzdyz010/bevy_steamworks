use bevy_ecs::prelude::Resource;
use steamworks::{Callback, CallbackHandle};

use crate::SteamworksClient;

/// Stores Steamworks callback handles so callbacks stay registered.
#[derive(Default, Resource)]
pub struct SteamworksCallbackRegistry {
    handles: Vec<CallbackHandle>,
}

impl SteamworksCallbackRegistry {
    /// Registers a typed Steamworks callback and stores its handle.
    pub fn register<C, F>(&mut self, client: &SteamworksClient, callback: F)
    where
        C: Callback,
        F: FnMut(C) + Send + 'static,
    {
        self.handles.push(client.register_callback(callback));
    }

    /// Drops every registered callback handle.
    pub fn clear(&mut self) {
        self.handles.clear();
    }

    /// Number of callback handles currently held.
    pub fn len(&self) -> usize {
        self.handles.len()
    }

    /// Returns true when no callback handles are held.
    pub fn is_empty(&self) -> bool {
        self.handles.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn callback_registry_tracks_handles() {
        let registry = SteamworksCallbackRegistry::default();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }
}
