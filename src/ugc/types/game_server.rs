/// Workshop depot ID used when initializing UGC for a Steam Game Server.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SteamworksUgcWorkshopDepotId(u32);

impl SteamworksUgcWorkshopDepotId {
    /// Creates a Workshop depot ID from its raw Steam value.
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw Steam depot ID value.
    pub const fn raw(self) -> u32 {
        self.0
    }
}

impl From<u32> for SteamworksUgcWorkshopDepotId {
    fn from(raw: u32) -> Self {
        Self::from_raw(raw)
    }
}

impl From<steamworks::AppId> for SteamworksUgcWorkshopDepotId {
    fn from(app_id: steamworks::AppId) -> Self {
        Self::from_raw(app_id.0)
    }
}

/// Successful Steam Game Server Workshop initialization submitted through UGC.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUgcGameServerWorkshopInit {
    /// Workshop depot submitted to Steam.
    pub workshop_depot: SteamworksUgcWorkshopDepotId,
    /// Local folder Steam should use for game-server Workshop content.
    pub folder: String,
}
