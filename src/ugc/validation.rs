use std::path::Path;

use super::{
    SteamworksUgcCommand, SteamworksUgcError, SteamworksUgcItemUpdate, SteamworksUgcQuery,
    SteamworksUgcQueryOptions, STEAMWORKS_UGC_MAX_ITEMS_PER_COMMAND,
    STEAMWORKS_UGC_MAX_UPDATE_DESCRIPTION_BYTES, STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAGS,
    STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAG_REMOVALS, STEAMWORKS_UGC_MAX_UPDATE_METADATA_BYTES,
    STEAMWORKS_UGC_MAX_UPDATE_TAG_BYTES, STEAMWORKS_UGC_MAX_UPDATE_TITLE_BYTES,
};

pub(super) fn validate_command(command: &SteamworksUgcCommand) -> Result<(), SteamworksUgcError> {
    match command {
        SteamworksUgcCommand::SuspendDownloads { .. }
        | SteamworksUgcCommand::ListSubscribedItems { .. }
        | SteamworksUgcCommand::CreateItem { .. }
        | SteamworksUgcCommand::GetItemUpdateProgress { .. }
        | SteamworksUgcCommand::ForgetItemUpdate { .. }
        | SteamworksUgcCommand::StopPlaytimeTrackingForAllItems => Ok(()),
        SteamworksUgcCommand::GetItemState { item }
        | SteamworksUgcCommand::GetItemDownloadInfo { item }
        | SteamworksUgcCommand::GetItemInstallInfo { item }
        | SteamworksUgcCommand::DownloadItem { item, .. }
        | SteamworksUgcCommand::SubscribeItem { item }
        | SteamworksUgcCommand::UnsubscribeItem { item }
        | SteamworksUgcCommand::DeleteItem { item } => validate_item(*item),
        SteamworksUgcCommand::SubmitItemUpdate { item, update, .. } => {
            validate_item(*item)?;
            validate_item_update(update)
        }
        SteamworksUgcCommand::Query { query } => validate_query(query),
        SteamworksUgcCommand::StartPlaytimeTracking { items }
        | SteamworksUgcCommand::StopPlaytimeTracking { items } => validate_items(items),
    }
}

pub(super) fn validate_query(query: &SteamworksUgcQuery) -> Result<(), SteamworksUgcError> {
    match query {
        SteamworksUgcQuery::All { page, options, .. }
        | SteamworksUgcQuery::User { page, options, .. } => {
            if *page == 0 {
                return Err(SteamworksUgcError::InvalidPage);
            }
            validate_query_options(options)
        }
        SteamworksUgcQuery::Items { items, options } => {
            validate_items(items)?;
            validate_query_options(options)
        }
    }
}

pub(super) fn validate_query_options(
    options: &SteamworksUgcQueryOptions,
) -> Result<(), SteamworksUgcError> {
    for tag in &options.required_tags {
        validate_steam_string("required_tag", tag)?;
    }
    for tag in &options.excluded_tags {
        validate_steam_string("excluded_tag", tag)?;
    }
    for (key, value) in &options.required_key_value_tags {
        validate_steam_string("required_key_value_tag.key", key)?;
        validate_steam_string("required_key_value_tag.value", value)?;
    }
    if let Some(language) = &options.language {
        validate_steam_string("language", language)?;
    }
    if let Some(file_name) = &options.cloud_file_name_filter {
        validate_steam_string("cloud_file_name_filter", file_name)?;
    }
    if let Some(search_text) = &options.search_text {
        validate_steam_string("search_text", search_text)?;
    }
    Ok(())
}

pub(super) fn validate_item_update(
    update: &SteamworksUgcItemUpdate,
) -> Result<(), SteamworksUgcError> {
    if item_update_is_empty(update) {
        return Err(SteamworksUgcError::EmptyItemUpdate);
    }

    if let Some(title) = &update.title {
        validate_bounded_steam_string("title", title, STEAMWORKS_UGC_MAX_UPDATE_TITLE_BYTES)?;
    }
    if let Some(description) = &update.description {
        validate_bounded_steam_string(
            "description",
            description,
            STEAMWORKS_UGC_MAX_UPDATE_DESCRIPTION_BYTES,
        )?;
    }
    if let Some(language) = &update.language {
        validate_steam_string("language", language)?;
    }
    if let Some(path) = &update.preview_path {
        validate_update_path("preview_path", path)?;
    }
    if let Some(path) = &update.content_path {
        validate_update_path("content_path", path)?;
    }
    if let Some(metadata) = &update.metadata {
        validate_bounded_steam_string(
            "metadata",
            metadata,
            STEAMWORKS_UGC_MAX_UPDATE_METADATA_BYTES,
        )?;
    }
    if let Some(tags) = &update.tags {
        for tag in &tags.tags {
            validate_update_tag(tag)?;
        }
    }
    if update.add_key_value_tags.len() > STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAGS {
        return Err(SteamworksUgcError::TooManyKeyValueTags {
            requested: update.add_key_value_tags.len(),
            max_supported: STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAGS,
        });
    }
    for (key, value) in &update.add_key_value_tags {
        validate_key_value_tag_key(key)?;
        validate_bounded_steam_string(
            "key_value_tag.value",
            value,
            STEAMWORKS_UGC_MAX_UPDATE_TAG_BYTES,
        )?;
    }
    if update.remove_key_value_tags.len() > STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAG_REMOVALS {
        return Err(SteamworksUgcError::TooManyKeyValueTagRemovals {
            requested: update.remove_key_value_tags.len(),
            max_supported: STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAG_REMOVALS,
        });
    }
    for key in &update.remove_key_value_tags {
        validate_steam_string("remove_key_value_tag", key)?;
    }
    if let Some(change_note) = &update.change_note {
        validate_steam_string("change_note", change_note)?;
    }

    Ok(())
}

fn item_update_is_empty(update: &SteamworksUgcItemUpdate) -> bool {
    update.title.is_none()
        && update.description.is_none()
        && update.language.is_none()
        && update.preview_path.is_none()
        && update.content_path.is_none()
        && update.metadata.is_none()
        && update.visibility.is_none()
        && update.tags.is_none()
        && update.add_key_value_tags.is_empty()
        && update.remove_key_value_tags.is_empty()
        && !update.remove_all_key_value_tags
        && update.add_content_descriptors.is_empty()
        && update.remove_content_descriptors.is_empty()
        && update.change_note.is_none()
}

fn validate_bounded_steam_string(
    field: &'static str,
    value: &str,
    max_supported: usize,
) -> Result<(), SteamworksUgcError> {
    validate_steam_string(field, value)?;
    if value.len() > max_supported {
        Err(SteamworksUgcError::StringTooLong {
            field,
            requested: value.len(),
            max_supported,
        })
    } else {
        Ok(())
    }
}

fn validate_update_path(field: &'static str, path: &Path) -> Result<(), SteamworksUgcError> {
    let path = path
        .canonicalize()
        .map_err(|_| SteamworksUgcError::InvalidPath {
            field,
            path: path.to_path_buf(),
        })?;
    validate_steam_string(field, &path.to_string_lossy())
}

fn validate_update_tag(tag: &str) -> Result<(), SteamworksUgcError> {
    validate_steam_string("tag", tag)?;
    if tag.len() > STEAMWORKS_UGC_MAX_UPDATE_TAG_BYTES
        || tag.contains(',')
        || !tag.bytes().all(|byte| (0x20..=0x7e).contains(&byte))
    {
        return Err(SteamworksUgcError::InvalidTagText {
            tag: tag.to_owned(),
        });
    }

    Ok(())
}

fn validate_key_value_tag_key(key: &str) -> Result<(), SteamworksUgcError> {
    validate_bounded_steam_string(
        "key_value_tag.key",
        key,
        STEAMWORKS_UGC_MAX_UPDATE_TAG_BYTES,
    )?;
    if key.is_empty()
        || !key
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        return Err(SteamworksUgcError::InvalidKeyValueTagKey {
            key: key.to_owned(),
        });
    }

    Ok(())
}

fn validate_items(items: &[steamworks::PublishedFileId]) -> Result<(), SteamworksUgcError> {
    if items.is_empty() {
        return Err(SteamworksUgcError::EmptyItemList);
    }
    if items.len() > STEAMWORKS_UGC_MAX_ITEMS_PER_COMMAND {
        return Err(SteamworksUgcError::TooManyItems {
            requested: items.len(),
            max_supported: STEAMWORKS_UGC_MAX_ITEMS_PER_COMMAND,
        });
    }
    for item in items {
        validate_item(*item)?;
    }
    Ok(())
}

fn validate_item(item: steamworks::PublishedFileId) -> Result<(), SteamworksUgcError> {
    if item.0 == 0 {
        Err(SteamworksUgcError::InvalidItemId)
    } else {
        Ok(())
    }
}

fn validate_steam_string(field: &'static str, value: &str) -> Result<(), SteamworksUgcError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksUgcError::invalid_string(field))
    } else {
        Ok(())
    }
}
