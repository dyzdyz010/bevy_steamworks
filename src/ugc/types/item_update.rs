use std::path::PathBuf;

/// Tags to set on a Workshop item update.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SteamworksUgcItemUpdateTags {
    /// Tags to set.
    pub tags: Vec<String>,
    /// Whether Steam may apply admin-only tags.
    pub allow_admin_tags: bool,
}

/// Mature-content descriptor used by Workshop item updates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SteamworksUgcContentDescriptor {
    /// Some nudity or sexual content.
    NudityOrSexualContent,
    /// Frequent violence or gore.
    FrequentViolenceOrGore,
    /// Adult-only sexual content.
    AdultOnlySexualContent,
    /// Frequent nudity or sexual content.
    GratuitousSexualContent,
    /// General mature content.
    AnyMatureContent,
}

impl From<SteamworksUgcContentDescriptor> for steamworks::UGCContentDescriptorID {
    fn from(value: SteamworksUgcContentDescriptor) -> Self {
        match value {
            SteamworksUgcContentDescriptor::NudityOrSexualContent => Self::NudityOrSexualContent,
            SteamworksUgcContentDescriptor::FrequentViolenceOrGore => Self::FrequentViolenceOrGore,
            SteamworksUgcContentDescriptor::AdultOnlySexualContent => Self::AdultOnlySexualContent,
            SteamworksUgcContentDescriptor::GratuitousSexualContent => {
                Self::GratuitousSexualContent
            }
            SteamworksUgcContentDescriptor::AnyMatureContent => Self::AnyMatureContent,
        }
    }
}

/// Options applied to one Workshop item update before submission.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SteamworksUgcItemUpdate {
    /// New title.
    pub title: Option<String>,
    /// New description.
    pub description: Option<String>,
    /// Update language.
    pub language: Option<String>,
    /// Preview image path.
    pub preview_path: Option<PathBuf>,
    /// Content directory path.
    pub content_path: Option<PathBuf>,
    /// Developer metadata.
    pub metadata: Option<String>,
    /// Item visibility.
    pub visibility: Option<steamworks::PublishedFileVisibility>,
    /// Replacement tag list.
    pub tags: Option<SteamworksUgcItemUpdateTags>,
    /// Key/value tags to add.
    pub add_key_value_tags: Vec<(String, String)>,
    /// Key/value tag keys to remove.
    pub remove_key_value_tags: Vec<String>,
    /// Whether all key/value tags should be removed before adding requested tags.
    pub remove_all_key_value_tags: bool,
    /// Content descriptors to add.
    pub add_content_descriptors: Vec<SteamworksUgcContentDescriptor>,
    /// Content descriptors to remove.
    pub remove_content_descriptors: Vec<SteamworksUgcContentDescriptor>,
    /// Optional change note sent with the update submission.
    pub change_note: Option<String>,
}

impl SteamworksUgcItemUpdate {
    /// Creates an empty item update.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the item title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the item description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the update language.
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Sets the preview image path.
    pub fn with_preview_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.preview_path = Some(path.into());
        self
    }

    /// Sets the content directory path.
    pub fn with_content_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.content_path = Some(path.into());
        self
    }

    /// Sets developer metadata.
    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }

    /// Sets item visibility.
    pub fn with_visibility(mut self, visibility: steamworks::PublishedFileVisibility) -> Self {
        self.visibility = Some(visibility);
        self
    }

    /// Replaces item tags.
    pub fn with_tags<I, S>(mut self, tags: I, allow_admin_tags: bool) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tags = Some(SteamworksUgcItemUpdateTags {
            tags: tags.into_iter().map(Into::into).collect(),
            allow_admin_tags,
        });
        self
    }

    /// Adds one key/value tag.
    pub fn with_key_value_tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.add_key_value_tags.push((key.into(), value.into()));
        self
    }

    /// Removes all key/value tags with the given key.
    pub fn with_removed_key_value_tag(mut self, key: impl Into<String>) -> Self {
        self.remove_key_value_tags.push(key.into());
        self
    }

    /// Removes all key/value tags before applying added key/value tags.
    pub fn with_remove_all_key_value_tags(mut self) -> Self {
        self.remove_all_key_value_tags = true;
        self
    }

    /// Adds one content descriptor.
    pub fn with_added_content_descriptor(
        mut self,
        descriptor: SteamworksUgcContentDescriptor,
    ) -> Self {
        self.add_content_descriptors.push(descriptor);
        self
    }

    /// Removes one content descriptor.
    pub fn with_removed_content_descriptor(
        mut self,
        descriptor: SteamworksUgcContentDescriptor,
    ) -> Self {
        self.remove_content_descriptors.push(descriptor);
        self
    }

    /// Sets the change note submitted with this update.
    pub fn with_change_note(mut self, change_note: impl Into<String>) -> Self {
        self.change_note = Some(change_note.into());
        self
    }
}

/// Progress snapshot for a submitted Workshop item update.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUgcItemUpdateProgress {
    /// Plugin request ID returned by the update submission.
    pub request_id: u64,
    /// Current update status.
    pub status: steamworks::UpdateStatus,
    /// Bytes processed so far.
    pub processed_bytes: u64,
    /// Total bytes expected by Steam.
    pub total_bytes: u64,
}
