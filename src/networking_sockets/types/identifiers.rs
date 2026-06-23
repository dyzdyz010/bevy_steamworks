/// Opaque ID for a listen socket owned by [`crate::SteamworksNetworkingSocketsPlugin`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SteamworksListenSocketId(u64);

impl SteamworksListenSocketId {
    /// Creates an ID from a raw integer.
    pub fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw integer backing this ID.
    pub fn raw(self) -> u64 {
        self.0
    }
}

/// Opaque ID for a connection owned by [`crate::SteamworksNetworkingSocketsPlugin`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SteamworksNetworkingSocketsConnectionId(u64);

impl SteamworksNetworkingSocketsConnectionId {
    /// Creates an ID from a raw integer.
    pub fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw integer backing this ID.
    pub fn raw(self) -> u64 {
        self.0
    }
}

/// Opaque ID for a poll group owned by [`crate::SteamworksNetworkingSocketsPlugin`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SteamworksNetworkingSocketsPollGroupId(u64);

impl SteamworksNetworkingSocketsPollGroupId {
    /// Creates an ID from a raw integer.
    pub fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw integer backing this ID.
    pub fn raw(self) -> u64 {
        self.0
    }
}
