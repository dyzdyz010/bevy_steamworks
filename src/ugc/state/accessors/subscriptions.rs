use crate::ugc::*;

impl SteamworksUgcState {
    /// Returns the most recent subscribed Workshop item list.
    pub fn subscribed_items(&self) -> &[steamworks::PublishedFileId] {
        &self.subscribed_items
    }

    /// Returns the number of items in the most recent subscribed Workshop item list.
    pub fn subscribed_item_count(&self) -> usize {
        self.subscribed_items.len()
    }

    /// Returns whether the most recent subscribed item list contains an item.
    pub fn is_item_subscribed(&self, item: steamworks::PublishedFileId) -> bool {
        self.subscribed_items.contains(&item)
    }
}
