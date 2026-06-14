use std::ops::Deref;

use bevy_ecs::prelude::Resource;

/// A Bevy resource wrapping [`steamworks::Client`].
#[derive(Clone, Resource)]
pub struct SteamworksClient(steamworks::Client);

impl SteamworksClient {
    /// Creates a Bevy resource from an initialized Steamworks client.
    pub fn new(client: steamworks::Client) -> Self {
        Self(client)
    }

    /// Returns the underlying Steamworks client.
    pub fn inner(&self) -> &steamworks::Client {
        &self.0
    }

    /// Clones the underlying Steamworks client handle.
    pub fn clone_inner(&self) -> steamworks::Client {
        self.0.clone()
    }
}

impl From<steamworks::Client> for SteamworksClient {
    fn from(client: steamworks::Client) -> Self {
        Self::new(client)
    }
}

impl Deref for SteamworksClient {
    type Target = steamworks::Client;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}
