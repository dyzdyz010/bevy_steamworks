use bevy_ecs::prelude::Resource;

use super::SteamworksServer;

/// Stores Steam Game Server callback handles so callbacks stay registered.
#[derive(Default, Resource)]
pub struct SteamworksServerCallbackRegistry {
    handles: Vec<steamworks::CallbackHandle>,
}

impl SteamworksServerCallbackRegistry {
    /// Registers a typed Steam Game Server callback and stores its handle.
    pub fn register<C, F>(&mut self, server: &SteamworksServer, callback: F)
    where
        C: steamworks::Callback,
        F: FnMut(C) + 'static + Send,
    {
        self.handles.push(server.register_callback(callback));
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
