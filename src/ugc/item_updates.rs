use super::{SteamworksUgcError, SteamworksUgcItemUpdate};

pub(super) fn apply_item_update(
    mut handle: steamworks::UpdateHandle,
    update: &SteamworksUgcItemUpdate,
) -> Result<steamworks::UpdateHandle, SteamworksUgcError> {
    if let Some(title) = &update.title {
        handle = handle.title(title);
    }
    if let Some(description) = &update.description {
        handle = handle.description(description);
    }
    if let Some(language) = &update.language {
        handle = handle.language(language);
    }
    if let Some(path) = &update.preview_path {
        handle = handle.preview_path(path);
    }
    if let Some(path) = &update.content_path {
        handle = handle.content_path(path);
    }
    if let Some(metadata) = &update.metadata {
        handle = handle.metadata(metadata);
    }
    if let Some(visibility) = update.visibility {
        handle = handle.visibility(visibility);
    }
    if let Some(tags) = &update.tags {
        handle = handle.tags(tags.tags.clone(), tags.allow_admin_tags);
    }
    if update.remove_all_key_value_tags {
        handle = handle.remove_all_key_value_tags();
    }
    for key in &update.remove_key_value_tags {
        handle = handle.remove_key_value_tag(key);
    }
    for (key, value) in &update.add_key_value_tags {
        handle = handle.add_key_value_tag(key, value);
    }
    for descriptor in &update.add_content_descriptors {
        handle = handle.add_content_descriptor((*descriptor).into());
    }
    for descriptor in &update.remove_content_descriptors {
        handle = handle.remove_content_descriptor((*descriptor).into());
    }

    Ok(handle)
}
