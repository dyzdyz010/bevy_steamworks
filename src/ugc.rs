//! High-level Bevy ECS integration for Steam Workshop / UGC.
//!
//! This module builds on top of the upstream [`steamworks::UGC`] API. It keeps
//! common Workshop queries, subscriptions, downloads, and playtime tracking in
//! Bevy messages, while converting asynchronous Steam call results and download
//! callbacks into owned ECS-safe result messages.

use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

/// Maximum number of item IDs accepted by one UGC details or playtime command.
///
/// The raw Steam call takes a `u32` count and is not intended for unbounded
/// frame-loop payloads. Larger batches should be split by the caller.
pub const STEAMWORKS_UGC_MAX_ITEMS_PER_COMMAND: usize = 1_000;

/// Maximum item title bytes accepted before the trailing NUL terminator.
pub const STEAMWORKS_UGC_MAX_UPDATE_TITLE_BYTES: usize = 128;

/// Maximum item description bytes accepted before the trailing NUL terminator.
pub const STEAMWORKS_UGC_MAX_UPDATE_DESCRIPTION_BYTES: usize = 7_999;

/// Maximum developer metadata bytes accepted before the trailing NUL terminator.
pub const STEAMWORKS_UGC_MAX_UPDATE_METADATA_BYTES: usize = 4_999;

/// Maximum item tag bytes accepted by Steam.
pub const STEAMWORKS_UGC_MAX_UPDATE_TAG_BYTES: usize = 255;

/// Maximum key/value tag removals accepted by one item update.
pub const STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAG_REMOVALS: usize = 100;

/// Maximum key/value tag additions accepted by one item update.
pub const STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAGS: usize = 100;

/// Bevy plugin for high-level Steam Workshop / UGC commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksUgcCommand`] and [`SteamworksUgcResult`] messages and runs its
/// command processor in [`bevy_app::First`] after Steam callbacks. It also
/// mirrors Workshop download completion callbacks into UGC results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksUgcPlugin;

impl SteamworksUgcPlugin {
    /// Creates a UGC plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksUgcPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksUgcState>()
            .init_resource::<SteamworksUgcAsyncResults>()
            .init_resource::<SteamworksUgcUpdateWatches>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksUgcCommand>()
            .add_message::<SteamworksUgcResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessUgcCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_ugc_commands.in_set(SteamworksSystem::ProcessUgcCommands),
            );
    }
}

mod messages;
mod state;
mod types;

pub use messages::*;
pub use state::SteamworksUgcState;
pub use types::*;

#[derive(Clone, Debug, Default, Resource)]
struct SteamworksUgcAsyncResults {
    queue: Arc<Mutex<Vec<SteamworksUgcResult>>>,
}

impl SteamworksUgcAsyncResults {
    fn push(&self, result: SteamworksUgcResult) {
        self.queue
            .lock()
            .expect("Steamworks UGC async result mutex was poisoned")
            .push(result);
    }

    fn drain(&self) -> Vec<SteamworksUgcResult> {
        self.queue
            .lock()
            .expect("Steamworks UGC async result mutex was poisoned")
            .drain(..)
            .collect()
    }
}

#[derive(Clone, Debug, Default, Resource)]
struct SteamworksUgcUpdateWatches {
    storage: Arc<Mutex<SteamworksUgcUpdateWatchStorage>>,
}

impl SteamworksUgcUpdateWatches {
    fn insert(&self, request_id: u64, handle: steamworks::UpdateWatchHandle) {
        self.storage
            .lock()
            .expect("Steamworks UGC update watch storage mutex was poisoned")
            .insert(request_id, handle);
    }

    fn progress(&self, request_id: u64) -> Option<SteamworksUgcItemUpdateProgress> {
        self.storage
            .lock()
            .expect("Steamworks UGC update watch storage mutex was poisoned")
            .progress(request_id)
    }

    fn remove(&self, request_id: u64) -> bool {
        self.storage
            .lock()
            .expect("Steamworks UGC update watch storage mutex was poisoned")
            .remove(request_id)
    }

    fn len(&self) -> usize {
        self.storage
            .lock()
            .expect("Steamworks UGC update watch storage mutex was poisoned")
            .len()
    }
}

#[derive(Default)]
struct SteamworksUgcUpdateWatchStorage {
    watches: std::collections::HashMap<u64, steamworks::UpdateWatchHandle>,
}

impl std::fmt::Debug for SteamworksUgcUpdateWatchStorage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SteamworksUgcUpdateWatchStorage")
            .field("watch_count", &self.watches.len())
            .finish()
    }
}

impl SteamworksUgcUpdateWatchStorage {
    fn insert(&mut self, request_id: u64, handle: steamworks::UpdateWatchHandle) {
        self.watches.insert(request_id, handle);
    }

    fn progress(&self, request_id: u64) -> Option<SteamworksUgcItemUpdateProgress> {
        let handle = self.watches.get(&request_id)?;
        let (status, processed_bytes, total_bytes) = handle.progress();
        Some(SteamworksUgcItemUpdateProgress {
            request_id,
            status,
            processed_bytes,
            total_bytes,
        })
    }

    fn remove(&mut self, request_id: u64) -> bool {
        self.watches.remove(&request_id).is_some()
    }

    fn len(&self) -> usize {
        self.watches.len()
    }
}

fn process_ugc_commands(
    client: Option<Res<SteamworksClient>>,
    async_results: Res<SteamworksUgcAsyncResults>,
    update_watches: Res<SteamworksUgcUpdateWatches>,
    mut state: ResMut<SteamworksUgcState>,
    mut commands: ResMut<Messages<SteamworksUgcCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksUgcResult>,
) {
    for result in async_results.drain() {
        record_ugc_result(&mut state, &result);
        state.sync_active_item_updates(&update_watches);
        results.write(result);
    }

    process_ugc_steam_events(&mut state, &mut steam_events, &mut results);

    let Some(client) = client else {
        let error = SteamworksUgcError::ClientUnavailable;
        for command in commands.drain() {
            state.record_error(error.clone());
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks UGC command failed"
            );
            results.write(SteamworksUgcResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        if let Err(error) = validate_command(&command) {
            state.record_error(error.clone());
            state.sync_active_item_updates(&update_watches);
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks UGC command failed"
            );
            results.write(SteamworksUgcResult::Err { command, error });
            continue;
        }

        let request_id = async_command_request_id(&command, &mut state);
        match handle_ugc_command(
            &client,
            &async_results,
            &update_watches,
            command.clone(),
            request_id,
        ) {
            Ok(operation) => {
                state.record_operation(&operation);
                state.sync_active_item_updates(&update_watches);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks UGC command"
                );
                results.write(SteamworksUgcResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                state.sync_active_item_updates(&update_watches);
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks UGC command failed"
                );
                results.write(SteamworksUgcResult::Err { command, error });
            }
        }
    }
}

fn process_ugc_steam_events(
    state: &mut SteamworksUgcState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksUgcResult>,
) {
    for event in steam_events.read() {
        let SteamworksEvent::DownloadItemResult(event) = event else {
            continue;
        };

        let operation = SteamworksUgcOperation::DownloadItemResultReceived {
            result: SteamworksUgcDownloadItemResult {
                app_id: event.app_id,
                item: event.published_file_id,
                error: event.error,
            },
        };
        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks UGC callback"
        );
        results.write(SteamworksUgcResult::Ok(operation));
    }
}

fn record_ugc_result(state: &mut SteamworksUgcState, result: &SteamworksUgcResult) {
    match result {
        SteamworksUgcResult::Ok(operation) => state.record_operation(operation),
        SteamworksUgcResult::Err { error, .. } => {
            if error.async_request_id().is_some() {
                state.record_failed_async_operation();
            }
            state.record_error(error.clone());
        }
    }
}

fn async_command_request_id(
    command: &SteamworksUgcCommand,
    state: &mut SteamworksUgcState,
) -> Option<u64> {
    matches!(
        command,
        SteamworksUgcCommand::Query { .. }
            | SteamworksUgcCommand::CreateItem { .. }
            | SteamworksUgcCommand::SubmitItemUpdate { .. }
            | SteamworksUgcCommand::SubscribeItem { .. }
            | SteamworksUgcCommand::UnsubscribeItem { .. }
            | SteamworksUgcCommand::DeleteItem { .. }
            | SteamworksUgcCommand::StartPlaytimeTracking { .. }
            | SteamworksUgcCommand::StopPlaytimeTracking { .. }
            | SteamworksUgcCommand::StopPlaytimeTrackingForAllItems
    )
    .then(|| state.next_request_id())
}

fn handle_ugc_command(
    client: &SteamworksClient,
    async_results: &SteamworksUgcAsyncResults,
    update_watches: &SteamworksUgcUpdateWatches,
    command: SteamworksUgcCommand,
    request_id: Option<u64>,
) -> Result<SteamworksUgcOperation, SteamworksUgcError> {
    let ugc = client.ugc();
    match command {
        SteamworksUgcCommand::SuspendDownloads { suspend } => {
            ugc.suspend_downloads(suspend);
            Ok(SteamworksUgcOperation::DownloadsSuspended { suspend })
        }
        SteamworksUgcCommand::ListSubscribedItems {
            include_locally_disabled,
        } => {
            let items = ugc.subscribed_items(include_locally_disabled);
            Ok(SteamworksUgcOperation::SubscribedItemsListed {
                include_locally_disabled,
                items,
            })
        }
        SteamworksUgcCommand::GetItemState { item } => Ok(SteamworksUgcOperation::ItemStateRead {
            info: SteamworksUgcItemStateInfo {
                item,
                state: ugc.item_state(item),
            },
        }),
        SteamworksUgcCommand::GetItemDownloadInfo { item } => {
            Ok(SteamworksUgcOperation::ItemDownloadInfoRead {
                info: SteamworksUgcItemDownloadInfoResult {
                    item,
                    info: ugc
                        .item_download_info(item)
                        .map(
                            |(downloaded_bytes, total_bytes)| SteamworksUgcItemDownloadInfo {
                                downloaded_bytes,
                                total_bytes,
                            },
                        ),
                },
            })
        }
        SteamworksUgcCommand::GetItemInstallInfo { item } => {
            Ok(SteamworksUgcOperation::ItemInstallInfoRead {
                info: SteamworksUgcItemInstallInfoResult {
                    item,
                    info: ugc
                        .item_install_info(item)
                        .map(|info| SteamworksUgcItemInstallInfo {
                            folder: info.folder,
                            size_on_disk: info.size_on_disk,
                            timestamp: info.timestamp,
                        }),
                },
            })
        }
        SteamworksUgcCommand::DownloadItem {
            item,
            high_priority,
        } => {
            if !ugc.download_item(item, high_priority) {
                return Err(SteamworksUgcError::operation_failed("ugc.download_item"));
            }
            Ok(SteamworksUgcOperation::DownloadItemSubmitted {
                item,
                high_priority,
            })
        }
        SteamworksUgcCommand::Query { query } => {
            let request_id = request_id.expect("async UGC query command missing request id");
            let options = query.options().clone();
            let query_handle = create_query(&ugc, &query)?;
            let query_handle = apply_query_options(query_handle, &options)?;
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::Query {
                query: query.clone(),
            };
            let callback_query = query.clone();
            query_handle.fetch(move |result| {
                async_results.push(match result {
                    Ok(results) => {
                        SteamworksUgcResult::Ok(SteamworksUgcOperation::QueryCompleted {
                            request_id,
                            query: callback_query,
                            results: snapshot_query_results(&results, &options),
                        })
                    }
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.query.fetch",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::QueryRequested { request_id, query })
        }
        SteamworksUgcCommand::CreateItem { app_id, file_type } => {
            let request_id = request_id.expect("async UGC create command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::CreateItem { app_id, file_type };
            ugc.create_item(app_id, file_type, move |result| {
                async_results.push(match result {
                    Ok((item, legal)) => {
                        SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemCreated {
                            request_id,
                            item,
                            user_needs_to_accept_workshop_legal_agreement: legal,
                        })
                    }
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.create_item",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::ItemCreateRequested {
                request_id,
                app_id,
                file_type,
            })
        }
        SteamworksUgcCommand::SubmitItemUpdate {
            app_id,
            item,
            update,
        } => {
            let request_id = request_id.expect("async UGC item update command missing request id");
            let update_handle = ugc.start_item_update(app_id, item);
            let update_handle = apply_item_update(update_handle, &update)?;
            let async_results = async_results.clone();
            let update_watches_for_callback = update_watches.clone();
            let command = SteamworksUgcCommand::SubmitItemUpdate {
                app_id,
                item,
                update: update.clone(),
            };
            let watch = update_handle.submit(update.change_note.as_deref(), move |result| {
                update_watches_for_callback.remove(request_id);
                async_results.push(match result {
                    Ok((updated_item, legal)) => {
                        SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemUpdated {
                            request_id,
                            item: updated_item,
                            user_needs_to_accept_workshop_legal_agreement: legal,
                        })
                    }
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.submit_item_update",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            update_watches.insert(request_id, watch);
            Ok(SteamworksUgcOperation::ItemUpdateSubmitted {
                request_id,
                app_id,
                item,
                update,
            })
        }
        SteamworksUgcCommand::GetItemUpdateProgress { request_id } => {
            let progress = update_watches
                .progress(request_id)
                .ok_or(SteamworksUgcError::ItemUpdateNotFound { request_id })?;
            Ok(SteamworksUgcOperation::ItemUpdateProgressRead { progress })
        }
        SteamworksUgcCommand::ForgetItemUpdate { request_id } => {
            if update_watches.remove(request_id) {
                Ok(SteamworksUgcOperation::ItemUpdateForgotten { request_id })
            } else {
                Err(SteamworksUgcError::ItemUpdateNotFound { request_id })
            }
        }
        SteamworksUgcCommand::SubscribeItem { item } => {
            let request_id = request_id.expect("async UGC subscribe command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::SubscribeItem { item };
            ugc.subscribe_item(item, move |result| {
                async_results.push(match result {
                    Ok(()) => SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemSubscribed {
                        request_id,
                        item,
                    }),
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.subscribe_item",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::ItemSubscribeRequested { request_id, item })
        }
        SteamworksUgcCommand::UnsubscribeItem { item } => {
            let request_id = request_id.expect("async UGC unsubscribe command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::UnsubscribeItem { item };
            ugc.unsubscribe_item(item, move |result| {
                async_results.push(match result {
                    Ok(()) => SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemUnsubscribed {
                        request_id,
                        item,
                    }),
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.unsubscribe_item",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::ItemUnsubscribeRequested { request_id, item })
        }
        SteamworksUgcCommand::DeleteItem { item } => {
            let request_id = request_id.expect("async UGC delete command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::DeleteItem { item };
            ugc.delete_item(item, move |result| {
                async_results.push(match result {
                    Ok(()) => SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemDeleted {
                        request_id,
                        item,
                    }),
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.delete_item",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::ItemDeleteRequested { request_id, item })
        }
        SteamworksUgcCommand::StartPlaytimeTracking { items } => {
            let request_id =
                request_id.expect("async UGC start playtime command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::StartPlaytimeTracking {
                items: items.clone(),
            };
            let callback_items = items.clone();
            ugc.start_playtime_tracking(&items, move |result| {
                async_results.push(match result {
                    Ok(()) => {
                        SteamworksUgcResult::Ok(SteamworksUgcOperation::PlaytimeTrackingStarted {
                            request_id,
                            items: callback_items,
                        })
                    }
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.start_playtime_tracking",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::PlaytimeTrackingStartRequested { request_id, items })
        }
        SteamworksUgcCommand::StopPlaytimeTracking { items } => {
            let request_id =
                request_id.expect("async UGC stop playtime command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::StopPlaytimeTracking {
                items: items.clone(),
            };
            let callback_items = items.clone();
            ugc.stop_playtime_tracking(&items, move |result| {
                async_results.push(match result {
                    Ok(()) => {
                        SteamworksUgcResult::Ok(SteamworksUgcOperation::PlaytimeTrackingStopped {
                            request_id,
                            items: callback_items,
                        })
                    }
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.stop_playtime_tracking",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::PlaytimeTrackingStopRequested { request_id, items })
        }
        SteamworksUgcCommand::StopPlaytimeTrackingForAllItems => {
            let request_id =
                request_id.expect("async UGC stop all playtime command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksUgcCommand::StopPlaytimeTrackingForAllItems;
            ugc.stop_playtime_tracking_for_all_items(move |result| {
                async_results.push(match result {
                    Ok(()) => SteamworksUgcResult::Ok(
                        SteamworksUgcOperation::PlaytimeTrackingForAllItemsStopped { request_id },
                    ),
                    Err(source) => SteamworksUgcResult::Err {
                        command,
                        error: SteamworksUgcError::steam_error(
                            "ugc.stop_playtime_tracking_for_all_items",
                            Some(request_id),
                            source,
                        ),
                    },
                });
            });
            Ok(SteamworksUgcOperation::PlaytimeTrackingForAllItemsStopRequested { request_id })
        }
    }
}

fn create_query(
    ugc: &steamworks::UGC,
    query: &SteamworksUgcQuery,
) -> Result<steamworks::QueryHandle, SteamworksUgcError> {
    match query {
        SteamworksUgcQuery::All {
            query_type,
            item_type,
            app_ids,
            page,
            ..
        } => ugc.query_all(*query_type, *item_type, *app_ids, *page),
        SteamworksUgcQuery::User {
            account,
            list_type,
            item_type,
            sort_order,
            app_ids,
            page,
            ..
        } => ugc.query_user(
            *account,
            *list_type,
            *item_type,
            *sort_order,
            *app_ids,
            *page,
        ),
        SteamworksUgcQuery::Items { items, .. } => ugc.query_items(items.clone()),
    }
    .map_err(|_| SteamworksUgcError::CreateQueryFailed)
}

fn apply_item_update(
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

fn apply_query_options(
    mut query: steamworks::QueryHandle,
    options: &SteamworksUgcQueryOptions,
) -> Result<steamworks::QueryHandle, SteamworksUgcError> {
    validate_query_options(options)?;

    for tag in &options.required_tags {
        query = query.add_required_tag(tag);
    }
    for tag in &options.excluded_tags {
        query = query.add_excluded_tag(tag);
    }
    for (key, value) in &options.required_key_value_tags {
        query = query.add_required_key_value_tag(key, value);
    }
    if let Some(match_any_tag) = options.match_any_tag {
        query = query.set_match_any_tag(match_any_tag);
    }
    if let Some(language) = &options.language {
        query = query.set_language(language);
    }
    if let Some(seconds) = options.allow_cached_response_seconds {
        query = query.set_allow_cached_response(seconds);
    }
    if let Some(file_name) = &options.cloud_file_name_filter {
        query = query.set_cloud_file_name_filter(file_name);
    }
    if let Some(search_text) = &options.search_text {
        query = query.set_search_text(search_text);
    }
    if let Some(days) = options.ranked_by_trend_days {
        query = query.set_ranked_by_trend_days(days);
    }
    query = query.set_return_long_description(options.return_long_description);
    query = query.set_return_children(options.return_children);
    query = query.set_return_metadata(options.return_metadata);
    query = query.set_return_key_value_tags(options.return_key_value_tags);
    query = query.set_return_only_ids(options.return_only_ids);
    query = query.set_return_total_only(options.return_total_only);

    Ok(query)
}

fn snapshot_query_results(
    results: &steamworks::QueryResults<'_>,
    options: &SteamworksUgcQueryOptions,
) -> SteamworksUgcQueryResults {
    let items = (0..results.returned_results())
        .filter_map(|index| {
            results
                .get(index)
                .map(|result| snapshot_query_item(results, options, index, result))
        })
        .collect();

    SteamworksUgcQueryResults {
        was_cached: results.was_cached(),
        total_results: results.total_results(),
        returned_results: results.returned_results(),
        items,
    }
}

fn snapshot_query_item(
    results: &steamworks::QueryResults<'_>,
    options: &SteamworksUgcQueryOptions,
    index: u32,
    result: steamworks::QueryResult,
) -> SteamworksUgcItemDetails {
    SteamworksUgcItemDetails {
        published_file_id: result.published_file_id,
        creator_app_id: result.creator_app_id,
        consumer_app_id: result.consumer_app_id,
        title: result.title,
        description: result.description,
        owner: result.owner,
        time_created: result.time_created,
        time_updated: result.time_updated,
        time_added_to_user_list: result.time_added_to_user_list,
        visibility: result.visibility,
        banned: result.banned,
        accepted_for_use: result.accepted_for_use,
        tags: result.tags,
        tags_truncated: result.tags_truncated,
        file_name: result.file_name,
        file_type: result.file_type,
        file_size: result.file_size,
        url: result.url,
        num_upvotes: result.num_upvotes,
        num_downvotes: result.num_downvotes,
        score: result.score,
        num_children: result.num_children,
        preview_url: results.preview_url(index),
        statistics: options
            .statistics
            .iter()
            .filter_map(|statistic| {
                results
                    .statistic(index, *statistic)
                    .map(|value| SteamworksUgcStatistic {
                        statistic: *statistic,
                        value,
                    })
            })
            .collect(),
        metadata: options
            .return_metadata
            .then(|| results.get_metadata(index))
            .flatten(),
        children: options
            .return_children
            .then(|| results.get_children(index))
            .flatten(),
        key_value_tags: if options.return_key_value_tags {
            (0..results.key_value_tags(index))
                .filter_map(|tag_index| results.get_key_value_tag(index, tag_index))
                .collect()
        } else {
            Vec::new()
        },
    }
}

fn validate_command(command: &SteamworksUgcCommand) -> Result<(), SteamworksUgcError> {
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

fn validate_query(query: &SteamworksUgcQuery) -> Result<(), SteamworksUgcError> {
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

fn validate_query_options(options: &SteamworksUgcQueryOptions) -> Result<(), SteamworksUgcError> {
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

fn validate_item_update(update: &SteamworksUgcItemUpdate) -> Result<(), SteamworksUgcError> {
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

#[cfg(test)]
mod tests;
