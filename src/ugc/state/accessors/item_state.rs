use crate::ugc::*;

impl SteamworksUgcState {
    /// Returns the most recent item state snapshot.
    pub fn last_item_state(&self) -> Option<&SteamworksUgcItemStateInfo> {
        self.last_item_state.as_ref()
    }

    /// Returns cached state flags for one Workshop item.
    pub fn item_state(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<&SteamworksUgcItemStateInfo> {
        self.item_states.iter().find(|info| info.item == item)
    }

    /// Returns cached state flags for one Workshop item.
    pub fn item_state_flags(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<steamworks::ItemState> {
        self.item_state(item).map(|info| info.state)
    }

    /// Returns whether cached item state flags contain a flag.
    pub fn item_state_contains(
        &self,
        item: steamworks::PublishedFileId,
        flag: steamworks::ItemState,
    ) -> Option<bool> {
        self.item_state_flags(item)
            .map(|state| state.contains(flag))
    }

    /// Returns whether cached item state says the item is subscribed.
    pub fn item_state_subscribed(&self, item: steamworks::PublishedFileId) -> Option<bool> {
        self.item_state_contains(item, steamworks::ItemState::SUBSCRIBED)
    }

    /// Returns whether cached item state says the item is installed.
    pub fn item_state_installed(&self, item: steamworks::PublishedFileId) -> Option<bool> {
        self.item_state_contains(item, steamworks::ItemState::INSTALLED)
    }

    /// Returns whether cached item state says the item needs an update.
    pub fn item_state_needs_update(&self, item: steamworks::PublishedFileId) -> Option<bool> {
        self.item_state_contains(item, steamworks::ItemState::NEEDS_UPDATE)
    }

    /// Returns whether cached item state says the item is downloading.
    pub fn item_state_downloading(&self, item: steamworks::PublishedFileId) -> Option<bool> {
        self.item_state_contains(item, steamworks::ItemState::DOWNLOADING)
    }

    /// Returns whether cached item state says the item download is pending.
    pub fn item_state_download_pending(&self, item: steamworks::PublishedFileId) -> Option<bool> {
        self.item_state_contains(item, steamworks::ItemState::DOWNLOAD_PENDING)
    }
}
