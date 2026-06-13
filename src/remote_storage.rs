//! High-level Bevy ECS integration for Steam Remote Storage.
//!
//! This module builds on top of the upstream [`steamworks::RemoteStorage`] API.
//! It intentionally avoids the upstream blocking file reader/writer helpers in
//! Bevy systems; games can still access those through [`crate::SteamworksClient`]
//! when they can move file IO out of the frame-critical path.

use std::sync::{Arc, Mutex};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksSystem};

/// Bevy plugin for high-level Steam Remote Storage commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksRemoteStorageCommand`] and [`SteamworksRemoteStorageResult`]
/// messages and runs its command processor in [`bevy_app::First`] after Steam
/// callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksRemoteStoragePlugin;

impl SteamworksRemoteStoragePlugin {
    /// Creates a Remote Storage plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksRemoteStoragePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksRemoteStorageState>()
            .init_resource::<SteamworksRemoteStorageAsyncResults>()
            .add_message::<SteamworksRemoteStorageCommand>()
            .add_message::<SteamworksRemoteStorageResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessRemoteStorageCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_remote_storage_commands
                    .in_set(SteamworksSystem::ProcessRemoteStorageCommands),
            );
    }
}

/// Runtime state for [`SteamworksRemoteStoragePlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksRemoteStorageState {
    last_error: Option<SteamworksRemoteStorageError>,
    cloud_info: Option<SteamworksRemoteStorageCloudInfo>,
    files: Vec<SteamworksRemoteStorageFileSummary>,
    last_file_info: Option<SteamworksRemoteStorageFileInfo>,
    last_shared_file: Option<SteamworksRemoteStorageSharedFile>,
    share_count: u64,
    next_request_id: u64,
}

impl SteamworksRemoteStorageState {
    /// Returns the most recent synchronous or async error observed by the plugin.
    pub fn last_error(&self) -> Option<&SteamworksRemoteStorageError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent Cloud availability snapshot.
    pub fn cloud_info(&self) -> Option<&SteamworksRemoteStorageCloudInfo> {
        self.cloud_info.as_ref()
    }

    /// Returns the most recent Steam Cloud file list.
    pub fn files(&self) -> &[SteamworksRemoteStorageFileSummary] {
        &self.files
    }

    /// Returns the most recent file metadata snapshot read through the plugin.
    pub fn last_file_info(&self) -> Option<&SteamworksRemoteStorageFileInfo> {
        self.last_file_info.as_ref()
    }

    /// Returns the most recent file share completed through the plugin.
    pub fn last_shared_file(&self) -> Option<&SteamworksRemoteStorageSharedFile> {
        self.last_shared_file.as_ref()
    }

    /// Returns the number of completed file shares observed through the plugin.
    pub fn share_count(&self) -> u64 {
        self.share_count
    }

    fn record_error(&mut self, error: SteamworksRemoteStorageError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksRemoteStorageOperation) {
        match operation {
            SteamworksRemoteStorageOperation::CloudInfoRead { info } => {
                self.cloud_info = Some(info.clone());
            }
            SteamworksRemoteStorageOperation::CloudEnabledForAppRead { enabled } => {
                if let Some(info) = &mut self.cloud_info {
                    info.app_enabled = *enabled;
                }
            }
            SteamworksRemoteStorageOperation::CloudEnabledForAccountRead { enabled } => {
                if let Some(info) = &mut self.cloud_info {
                    info.account_enabled = *enabled;
                }
            }
            SteamworksRemoteStorageOperation::CloudEnabledForAppSet { enabled } => {
                if let Some(info) = &mut self.cloud_info {
                    info.app_enabled = *enabled;
                }
            }
            SteamworksRemoteStorageOperation::FilesListed { files } => {
                self.files.clone_from(files);
            }
            SteamworksRemoteStorageOperation::FileInfoRead { info } => {
                self.last_file_info = Some(info.clone());
            }
            SteamworksRemoteStorageOperation::FileDeleted { name, deleted } => {
                if *deleted {
                    self.files.retain(|file| file.name != *name);
                    if self
                        .last_file_info
                        .as_ref()
                        .is_some_and(|info| info.name == *name)
                    {
                        self.last_file_info = None;
                    }
                }
            }
            SteamworksRemoteStorageOperation::FileForgotten { name, forgotten } => {
                if *forgotten {
                    if let Some(info) = &mut self.last_file_info {
                        if info.name == *name {
                            info.persisted = false;
                        }
                    }
                }
            }
            SteamworksRemoteStorageOperation::FileSyncPlatformsRead { name, platforms }
            | SteamworksRemoteStorageOperation::FileSyncPlatformsSet { name, platforms } => {
                if let Some(info) = &mut self.last_file_info {
                    if info.name != *name {
                        return;
                    }
                    info.sync_platforms = *platforms;
                }
            }
            SteamworksRemoteStorageOperation::FileShared { shared_file } => {
                self.last_shared_file = Some(shared_file.clone());
                self.share_count = self.share_count.saturating_add(1);
            }
            SteamworksRemoteStorageOperation::FileShareRequested { .. } => {}
        }
    }

    fn next_request_id(&mut self) -> u64 {
        let request_id = self.next_request_id;
        self.next_request_id = self.next_request_id.wrapping_add(1);
        request_id
    }
}

#[derive(Clone, Debug, Default, Resource)]
struct SteamworksRemoteStorageAsyncResults {
    queue: Arc<Mutex<Vec<SteamworksRemoteStorageResult>>>,
}

impl SteamworksRemoteStorageAsyncResults {
    fn push(&self, result: SteamworksRemoteStorageResult) {
        self.queue
            .lock()
            .expect("Steamworks Remote Storage async result mutex was poisoned")
            .push(result);
    }

    fn drain(&self) -> Vec<SteamworksRemoteStorageResult> {
        self.queue
            .lock()
            .expect("Steamworks Remote Storage async result mutex was poisoned")
            .drain(..)
            .collect()
    }
}

/// Steam Cloud availability for the current app and account.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SteamworksRemoteStorageCloudInfo {
    /// Whether Steam Cloud is enabled for this app.
    pub app_enabled: bool,
    /// Whether Steam Cloud is enabled for this Steam account.
    pub account_enabled: bool,
}

/// Name and size of a file in Steam Remote Storage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksRemoteStorageFileSummary {
    /// File name in Steam Remote Storage.
    pub name: String,
    /// File size in bytes.
    pub size_bytes: u64,
}

/// Metadata for one Steam Remote Storage file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksRemoteStorageFileInfo {
    /// File name in Steam Remote Storage.
    pub name: String,
    /// Whether Steam reports the file currently exists.
    pub exists: bool,
    /// Whether Steam reports the file is persisted in Cloud storage.
    pub persisted: bool,
    /// Steam file timestamp, as Unix epoch seconds.
    pub timestamp: i64,
    /// Platforms the file is configured to sync to.
    pub sync_platforms: steamworks::RemoteStoragePlatforms,
}

/// Opaque handle returned by Steam after a successful file share.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SteamworksRemoteStorageFileShareHandle(u64);

impl SteamworksRemoteStorageFileShareHandle {
    /// Creates a file share handle from a raw Steam handle value.
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw Steam handle value.
    pub const fn raw(self) -> u64 {
        self.0
    }
}

/// A file share completed through Steam Remote Storage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksRemoteStorageSharedFile {
    /// Plugin-assigned request ID for correlating submitted and completed work.
    pub request_id: u64,
    /// File name that was shared.
    pub name: String,
    /// Steam handle for the shared file.
    pub handle: SteamworksRemoteStorageFileShareHandle,
}

/// A high-level command for Steam Remote Storage metadata and sharing workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksRemoteStorageCommand {
    /// Read a snapshot of Cloud availability.
    GetCloudInfo,
    /// Read whether Steam Cloud is enabled for this app.
    IsCloudEnabledForApp,
    /// Read whether Steam Cloud is enabled for this account.
    IsCloudEnabledForAccount,
    /// Set whether Steam Cloud is enabled for this app.
    SetCloudEnabledForApp {
        /// Enabled value to submit.
        enabled: bool,
    },
    /// List Steam Cloud files.
    ListFiles,
    /// Read metadata for one file.
    GetFileInfo {
        /// File name in Steam Remote Storage.
        name: String,
    },
    /// Delete one file locally and remotely.
    DeleteFile {
        /// File name in Steam Remote Storage.
        name: String,
    },
    /// Forget one file remotely while keeping it locally.
    ForgetFile {
        /// File name in Steam Remote Storage.
        name: String,
    },
    /// Read the platforms a file syncs to.
    GetSyncPlatforms {
        /// File name in Steam Remote Storage.
        name: String,
    },
    /// Set the platforms a file syncs to.
    SetSyncPlatforms {
        /// File name in Steam Remote Storage.
        name: String,
        /// Platforms to submit to Steam.
        platforms: steamworks::RemoteStoragePlatforms,
    },
    /// Share one file through Steam Remote Storage.
    ShareFile {
        /// File name in Steam Remote Storage.
        name: String,
    },
}

impl SteamworksRemoteStorageCommand {
    /// Creates a [`SteamworksRemoteStorageCommand::SetCloudEnabledForApp`] command.
    pub fn set_cloud_enabled_for_app(enabled: bool) -> Self {
        Self::SetCloudEnabledForApp { enabled }
    }

    /// Creates a [`SteamworksRemoteStorageCommand::GetFileInfo`] command.
    pub fn get_file_info(name: impl Into<String>) -> Self {
        Self::GetFileInfo { name: name.into() }
    }

    /// Creates a [`SteamworksRemoteStorageCommand::DeleteFile`] command.
    pub fn delete_file(name: impl Into<String>) -> Self {
        Self::DeleteFile { name: name.into() }
    }

    /// Creates a [`SteamworksRemoteStorageCommand::ForgetFile`] command.
    pub fn forget_file(name: impl Into<String>) -> Self {
        Self::ForgetFile { name: name.into() }
    }

    /// Creates a [`SteamworksRemoteStorageCommand::GetSyncPlatforms`] command.
    pub fn get_sync_platforms(name: impl Into<String>) -> Self {
        Self::GetSyncPlatforms { name: name.into() }
    }

    /// Creates a [`SteamworksRemoteStorageCommand::SetSyncPlatforms`] command.
    pub fn set_sync_platforms(
        name: impl Into<String>,
        platforms: steamworks::RemoteStoragePlatforms,
    ) -> Self {
        Self::SetSyncPlatforms {
            name: name.into(),
            platforms,
        }
    }

    /// Creates a [`SteamworksRemoteStorageCommand::ShareFile`] command.
    pub fn share_file(name: impl Into<String>) -> Self {
        Self::ShareFile { name: name.into() }
    }
}

/// A successfully processed Steam Remote Storage operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksRemoteStorageOperation {
    /// Cloud availability was read.
    CloudInfoRead {
        /// Cloud availability snapshot.
        info: SteamworksRemoteStorageCloudInfo,
    },
    /// App-level Cloud enabled state was read.
    CloudEnabledForAppRead {
        /// Whether Steam Cloud is enabled for this app.
        enabled: bool,
    },
    /// Account-level Cloud enabled state was read.
    CloudEnabledForAccountRead {
        /// Whether Steam Cloud is enabled for this account.
        enabled: bool,
    },
    /// App-level Cloud enabled state was submitted.
    CloudEnabledForAppSet {
        /// Enabled value submitted to Steam.
        enabled: bool,
    },
    /// Files were listed.
    FilesListed {
        /// File summaries returned by Steam.
        files: Vec<SteamworksRemoteStorageFileSummary>,
    },
    /// File metadata was read.
    FileInfoRead {
        /// File metadata snapshot.
        info: SteamworksRemoteStorageFileInfo,
    },
    /// File delete was submitted.
    FileDeleted {
        /// File name submitted.
        name: String,
        /// Whether Steam reported that a file was actually deleted.
        deleted: bool,
    },
    /// File forget was submitted.
    FileForgotten {
        /// File name submitted.
        name: String,
        /// Whether Steam reported that a file was actually forgotten.
        forgotten: bool,
    },
    /// File sync platforms were read.
    FileSyncPlatformsRead {
        /// File name inspected.
        name: String,
        /// Platforms reported by Steam.
        platforms: steamworks::RemoteStoragePlatforms,
    },
    /// File sync platforms were set.
    FileSyncPlatformsSet {
        /// File name submitted.
        name: String,
        /// Platforms submitted to Steam.
        platforms: steamworks::RemoteStoragePlatforms,
    },
    /// File sharing was requested.
    FileShareRequested {
        /// Plugin-assigned request ID.
        request_id: u64,
        /// File name submitted.
        name: String,
    },
    /// File sharing completed successfully.
    FileShared {
        /// Shared file details.
        shared_file: SteamworksRemoteStorageSharedFile,
    },
}

/// Result message emitted by [`SteamworksRemoteStoragePlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksRemoteStorageResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksRemoteStorageOperation),
    /// The command failed synchronously or through a Steam async call result.
    Err {
        /// Command that failed.
        command: SteamworksRemoteStorageCommand,
        /// Failure reason.
        error: SteamworksRemoteStorageError,
    },
}

/// Synchronous and async errors from [`SteamworksRemoteStoragePlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksRemoteStorageError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks Remote Storage command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// The upstream Steamworks API returned an explicit Steam error.
    #[error("Steamworks Remote Storage operation {operation} failed: {source}")]
    SteamError {
        /// Operation that failed.
        operation: &'static str,
        /// Plugin request ID for async operations.
        request_id: Option<u64>,
        /// Steamworks error.
        source: steamworks::SteamError,
    },
    /// The requested file is not available in Steam Remote Storage.
    #[error("Steamworks Remote Storage file is not available: {name}")]
    FileUnavailable {
        /// File name submitted.
        name: String,
    },
    /// The upstream Steamworks API rejected the operation.
    #[error("Steamworks Remote Storage operation failed: {operation}")]
    OperationFailed {
        /// Operation that failed.
        operation: &'static str,
    },
}

impl SteamworksRemoteStorageError {
    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    fn steam_error(
        operation: &'static str,
        request_id: Option<u64>,
        source: steamworks::SteamError,
    ) -> Self {
        Self::SteamError {
            operation,
            request_id,
            source,
        }
    }

    fn file_unavailable(name: impl Into<String>) -> Self {
        Self::FileUnavailable { name: name.into() }
    }

    fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }
}

fn process_remote_storage_commands(
    client: Option<Res<SteamworksClient>>,
    async_results: Res<SteamworksRemoteStorageAsyncResults>,
    mut state: ResMut<SteamworksRemoteStorageState>,
    mut commands: ResMut<Messages<SteamworksRemoteStorageCommand>>,
    mut results: MessageWriter<SteamworksRemoteStorageResult>,
) {
    for result in async_results.drain() {
        record_remote_storage_result(&mut state, &result);
        results.write(result);
    }

    let Some(client) = client else {
        let error = SteamworksRemoteStorageError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks Remote Storage command failed"
            );
            results.write(SteamworksRemoteStorageResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        if let Err(error) = validate_command(&command) {
            state.record_error(error.clone());
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks Remote Storage command failed"
            );
            results.write(SteamworksRemoteStorageResult::Err { command, error });
            continue;
        }

        let request_id = async_command_request_id(&command, &mut state);
        match handle_remote_storage_command(&client, &async_results, command.clone(), request_id) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks Remote Storage command"
                );
                results.write(SteamworksRemoteStorageResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks Remote Storage command failed"
                );
                results.write(SteamworksRemoteStorageResult::Err { command, error });
            }
        }
    }
}

fn record_remote_storage_result(
    state: &mut SteamworksRemoteStorageState,
    result: &SteamworksRemoteStorageResult,
) {
    match result {
        SteamworksRemoteStorageResult::Ok(operation) => state.record_operation(operation),
        SteamworksRemoteStorageResult::Err { error, .. } => state.record_error(error.clone()),
    }
}

fn async_command_request_id(
    command: &SteamworksRemoteStorageCommand,
    state: &mut SteamworksRemoteStorageState,
) -> Option<u64> {
    matches!(command, SteamworksRemoteStorageCommand::ShareFile { .. })
        .then(|| state.next_request_id())
}

fn handle_remote_storage_command(
    client: &SteamworksClient,
    async_results: &SteamworksRemoteStorageAsyncResults,
    command: SteamworksRemoteStorageCommand,
    request_id: Option<u64>,
) -> Result<SteamworksRemoteStorageOperation, SteamworksRemoteStorageError> {
    let remote_storage = client.remote_storage();
    match command {
        SteamworksRemoteStorageCommand::GetCloudInfo => {
            Ok(SteamworksRemoteStorageOperation::CloudInfoRead {
                info: snapshot_cloud_info(&remote_storage),
            })
        }
        SteamworksRemoteStorageCommand::IsCloudEnabledForApp => {
            Ok(SteamworksRemoteStorageOperation::CloudEnabledForAppRead {
                enabled: remote_storage.is_cloud_enabled_for_app(),
            })
        }
        SteamworksRemoteStorageCommand::IsCloudEnabledForAccount => Ok(
            SteamworksRemoteStorageOperation::CloudEnabledForAccountRead {
                enabled: remote_storage.is_cloud_enabled_for_account(),
            },
        ),
        SteamworksRemoteStorageCommand::SetCloudEnabledForApp { enabled } => {
            remote_storage.set_cloud_enabled_for_app(enabled);
            Ok(SteamworksRemoteStorageOperation::CloudEnabledForAppSet { enabled })
        }
        SteamworksRemoteStorageCommand::ListFiles => {
            let files = remote_storage
                .files()
                .into_iter()
                .map(|file| SteamworksRemoteStorageFileSummary {
                    name: file.name,
                    size_bytes: file.size,
                })
                .collect();
            Ok(SteamworksRemoteStorageOperation::FilesListed { files })
        }
        SteamworksRemoteStorageCommand::GetFileInfo { name } => {
            let file = remote_storage.file(&name);
            Ok(SteamworksRemoteStorageOperation::FileInfoRead {
                info: snapshot_file_info(name, &file),
            })
        }
        SteamworksRemoteStorageCommand::DeleteFile { name } => {
            let deleted = remote_storage.file(&name).delete();
            Ok(SteamworksRemoteStorageOperation::FileDeleted { name, deleted })
        }
        SteamworksRemoteStorageCommand::ForgetFile { name } => {
            let forgotten = remote_storage.file(&name).forget();
            Ok(SteamworksRemoteStorageOperation::FileForgotten { name, forgotten })
        }
        SteamworksRemoteStorageCommand::GetSyncPlatforms { name } => {
            let platforms = remote_storage.file(&name).get_sync_platforms();
            Ok(SteamworksRemoteStorageOperation::FileSyncPlatformsRead { name, platforms })
        }
        SteamworksRemoteStorageCommand::SetSyncPlatforms { name, platforms } => {
            let file = remote_storage.file(&name);
            if !file.exists() {
                return Err(SteamworksRemoteStorageError::file_unavailable(name));
            }
            file.set_sync_platforms(platforms);
            let applied = file.get_sync_platforms();
            if applied != platforms {
                return Err(SteamworksRemoteStorageError::operation_failed(
                    "remote_storage.file.set_sync_platforms",
                ));
            }
            Ok(SteamworksRemoteStorageOperation::FileSyncPlatformsSet { name, platforms })
        }
        SteamworksRemoteStorageCommand::ShareFile { name } => {
            let request_id = request_id.expect("async Remote Storage command missing request id");
            let async_results = async_results.clone();
            let command = SteamworksRemoteStorageCommand::ShareFile { name: name.clone() };
            let callback_name = name.clone();
            remote_storage.file(&name).share(move |result| {
                let result = match result {
                    Ok(handle) => SteamworksRemoteStorageResult::Ok(
                        SteamworksRemoteStorageOperation::FileShared {
                            shared_file: SteamworksRemoteStorageSharedFile {
                                request_id,
                                name: callback_name,
                                handle: SteamworksRemoteStorageFileShareHandle::from_raw(handle),
                            },
                        },
                    ),
                    Err(source) => SteamworksRemoteStorageResult::Err {
                        command,
                        error: SteamworksRemoteStorageError::steam_error(
                            "remote_storage.file.share",
                            Some(request_id),
                            source,
                        ),
                    },
                };
                async_results.push(result);
            });
            Ok(SteamworksRemoteStorageOperation::FileShareRequested { request_id, name })
        }
    }
}

fn snapshot_cloud_info(
    remote_storage: &steamworks::RemoteStorage,
) -> SteamworksRemoteStorageCloudInfo {
    SteamworksRemoteStorageCloudInfo {
        app_enabled: remote_storage.is_cloud_enabled_for_app(),
        account_enabled: remote_storage.is_cloud_enabled_for_account(),
    }
}

fn snapshot_file_info(
    name: String,
    file: &steamworks::SteamFile,
) -> SteamworksRemoteStorageFileInfo {
    SteamworksRemoteStorageFileInfo {
        name,
        exists: file.exists(),
        persisted: file.is_persisted(),
        timestamp: file.timestamp(),
        sync_platforms: file.get_sync_platforms(),
    }
}

fn validate_command(
    command: &SteamworksRemoteStorageCommand,
) -> Result<(), SteamworksRemoteStorageError> {
    match command {
        SteamworksRemoteStorageCommand::GetFileInfo { name }
        | SteamworksRemoteStorageCommand::DeleteFile { name }
        | SteamworksRemoteStorageCommand::ForgetFile { name }
        | SteamworksRemoteStorageCommand::GetSyncPlatforms { name }
        | SteamworksRemoteStorageCommand::SetSyncPlatforms { name, .. }
        | SteamworksRemoteStorageCommand::ShareFile { name } => validate_steam_string("name", name),
        SteamworksRemoteStorageCommand::GetCloudInfo
        | SteamworksRemoteStorageCommand::IsCloudEnabledForApp
        | SteamworksRemoteStorageCommand::IsCloudEnabledForAccount
        | SteamworksRemoteStorageCommand::SetCloudEnabledForApp { .. }
        | SteamworksRemoteStorageCommand::ListFiles => Ok(()),
    }
}

fn validate_steam_string(
    field: &'static str,
    value: &str,
) -> Result<(), SteamworksRemoteStorageError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksRemoteStorageError::invalid_string(field))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    #[test]
    fn remote_storage_plugin_registers_resources_and_messages() {
        let mut app = App::new();

        app.add_plugins(SteamworksRemoteStoragePlugin::new());

        assert!(app
            .world()
            .contains_resource::<SteamworksRemoteStorageState>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksRemoteStorageCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksRemoteStorageResult>>());
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksRemoteStoragePlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksRemoteStorageCommand>>()
            .write(SteamworksRemoteStorageCommand::GetCloudInfo);

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksRemoteStorageResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksRemoteStorageResult::Err {
                command: SteamworksRemoteStorageCommand::GetCloudInfo,
                error: SteamworksRemoteStorageError::ClientUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksRemoteStorageState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksRemoteStorageError::ClientUnavailable)
        );
    }

    #[test]
    fn validation_rejects_interior_nul() {
        let command = SteamworksRemoteStorageCommand::get_file_info("save\0bad.dat");

        assert_eq!(
            validate_command(&command),
            Err(SteamworksRemoteStorageError::InvalidString { field: "name" })
        );

        let command = SteamworksRemoteStorageCommand::share_file("save\0bad.dat");

        assert_eq!(
            validate_command(&command),
            Err(SteamworksRemoteStorageError::InvalidString { field: "name" })
        );
    }

    #[test]
    fn async_share_commands_get_unique_request_ids() {
        let mut state = SteamworksRemoteStorageState::default();
        let command = SteamworksRemoteStorageCommand::share_file("save.dat");

        assert_eq!(async_command_request_id(&command, &mut state), Some(0));
        assert_eq!(async_command_request_id(&command, &mut state), Some(1));
        assert_eq!(
            async_command_request_id(&SteamworksRemoteStorageCommand::GetCloudInfo, &mut state),
            None
        );
    }

    #[test]
    fn state_records_remote_storage_operations_without_unbounded_share_history() {
        let mut state = SteamworksRemoteStorageState::default();
        let platforms =
            steamworks::RemoteStoragePlatforms::WINDOWS | steamworks::RemoteStoragePlatforms::LINUX;

        state.record_operation(&SteamworksRemoteStorageOperation::CloudEnabledForAppRead {
            enabled: true,
        });
        assert!(state.cloud_info().is_none());

        state.record_operation(&SteamworksRemoteStorageOperation::CloudInfoRead {
            info: SteamworksRemoteStorageCloudInfo {
                app_enabled: true,
                account_enabled: false,
            },
        });
        state.record_operation(&SteamworksRemoteStorageOperation::FilesListed {
            files: vec![SteamworksRemoteStorageFileSummary {
                name: "save.dat".to_owned(),
                size_bytes: 10,
            }],
        });
        state.record_operation(&SteamworksRemoteStorageOperation::FileInfoRead {
            info: SteamworksRemoteStorageFileInfo {
                name: "save.dat".to_owned(),
                exists: true,
                persisted: true,
                timestamp: 7,
                sync_platforms: steamworks::RemoteStoragePlatforms::WINDOWS,
            },
        });
        state.record_operation(&SteamworksRemoteStorageOperation::FileSyncPlatformsSet {
            name: "save.dat".to_owned(),
            platforms,
        });
        state.record_operation(&SteamworksRemoteStorageOperation::FileShared {
            shared_file: SteamworksRemoteStorageSharedFile {
                request_id: 0,
                name: "save.dat".to_owned(),
                handle: SteamworksRemoteStorageFileShareHandle::from_raw(11),
            },
        });
        state.record_operation(&SteamworksRemoteStorageOperation::FileShared {
            shared_file: SteamworksRemoteStorageSharedFile {
                request_id: 1,
                name: "save2.dat".to_owned(),
                handle: SteamworksRemoteStorageFileShareHandle::from_raw(12),
            },
        });

        assert_eq!(
            state.cloud_info(),
            Some(&SteamworksRemoteStorageCloudInfo {
                app_enabled: true,
                account_enabled: false,
            })
        );
        assert_eq!(
            state.files(),
            &[SteamworksRemoteStorageFileSummary {
                name: "save.dat".to_owned(),
                size_bytes: 10,
            }]
        );
        assert_eq!(
            state.last_file_info().map(|info| info.sync_platforms),
            Some(platforms)
        );
        state.record_operation(&SteamworksRemoteStorageOperation::FileForgotten {
            name: "save.dat".to_owned(),
            forgotten: true,
        });
        assert_eq!(state.files().len(), 1);
        assert_eq!(
            state.last_file_info().map(|info| info.persisted),
            Some(false)
        );
        assert_eq!(state.share_count(), 2);
        assert_eq!(
            state.last_shared_file(),
            Some(&SteamworksRemoteStorageSharedFile {
                request_id: 1,
                name: "save2.dat".to_owned(),
                handle: SteamworksRemoteStorageFileShareHandle::from_raw(12),
            })
        );
    }
}
