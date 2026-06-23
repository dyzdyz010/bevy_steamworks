use crate::ugc::*;

impl SteamworksUgcState {
    /// Returns Workshop item detail snapshots cached from completed full queries.
    pub fn item_details(&self) -> &[SteamworksUgcItemDetails] {
        &self.item_details
    }

    /// Returns cached Workshop item details for one item.
    pub fn item_detail(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<&SteamworksUgcItemDetails> {
        self.item_details
            .iter()
            .find(|details| details.published_file_id == item)
    }

    /// Returns a cached Workshop item's creator app ID, preserving a read with no value as `Some(None)`.
    pub fn item_creator_app_id(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<Option<steamworks::AppId>> {
        self.item_detail(item).map(|details| details.creator_app_id)
    }

    /// Returns a cached Workshop item's consumer app ID, preserving a read with no value as `Some(None)`.
    pub fn item_consumer_app_id(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<Option<steamworks::AppId>> {
        self.item_detail(item)
            .map(|details| details.consumer_app_id)
    }

    /// Returns a cached Workshop item's title.
    pub fn item_title(&self, item: steamworks::PublishedFileId) -> Option<&str> {
        self.item_detail(item).map(|details| details.title.as_str())
    }

    /// Returns a cached Workshop item's description.
    pub fn item_description(&self, item: steamworks::PublishedFileId) -> Option<&str> {
        self.item_detail(item)
            .map(|details| details.description.as_str())
    }

    /// Returns a cached Workshop item's tags.
    pub fn item_tags(&self, item: steamworks::PublishedFileId) -> Option<&[String]> {
        self.item_detail(item)
            .map(|details| details.tags.as_slice())
    }

    /// Returns a cached Workshop item's preview URL, preserving a read with no URL as `Some(None)`.
    pub fn item_preview_url(&self, item: steamworks::PublishedFileId) -> Option<Option<&str>> {
        self.item_detail(item)
            .map(|details| details.preview_url.as_deref())
    }

    /// Returns a cached Workshop item's content descriptors.
    pub fn item_content_descriptors(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<&[crate::ugc::SteamworksUgcContentDescriptor]> {
        self.item_detail(item)
            .map(|details| details.content_descriptors.as_slice())
    }

    /// Returns cached Workshop item statistics.
    pub fn item_statistics(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<&[SteamworksUgcStatistic]> {
        self.item_detail(item)
            .map(|details| details.statistics.as_slice())
    }

    /// Returns one cached Workshop item statistic, preserving a cached item without that statistic as `Some(None)`.
    pub fn item_statistic(
        &self,
        item: steamworks::PublishedFileId,
        statistic: steamworks::UGCStatisticType,
    ) -> Option<Option<u64>> {
        self.item_detail(item).map(|details| {
            details
                .statistics
                .iter()
                .find(|entry| entry.statistic == statistic)
                .map(|entry| entry.value)
        })
    }

    /// Returns cached Workshop item metadata, preserving a read with no metadata as `Some(None)`.
    pub fn item_metadata(&self, item: steamworks::PublishedFileId) -> Option<Option<&[u8]>> {
        self.item_detail(item)
            .map(|details| details.metadata.as_deref())
    }

    /// Returns cached Workshop child item IDs, preserving a read with no child list as `Some(None)`.
    pub fn item_children(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<Option<&[steamworks::PublishedFileId]>> {
        self.item_detail(item)
            .map(|details| details.children.as_deref())
    }

    /// Returns cached Workshop item key/value tags.
    pub fn item_key_value_tags(
        &self,
        item: steamworks::PublishedFileId,
    ) -> Option<&[(String, String)]> {
        self.item_detail(item)
            .map(|details| details.key_value_tags.as_slice())
    }

    /// Returns one cached Workshop item key/value tag, preserving a cached item without that key as `Some(None)`.
    pub fn item_key_value_tag(
        &self,
        item: steamworks::PublishedFileId,
        key: &str,
    ) -> Option<Option<&str>> {
        self.item_detail(item).map(|details| {
            details
                .key_value_tags
                .iter()
                .find(|(entry_key, _)| entry_key == key)
                .map(|(_, value)| value.as_str())
        })
    }
}
