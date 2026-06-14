//! High-level Bevy ECS integration for Steam Remote Storage.
//!
//! This module builds on top of the upstream [`steamworks::RemoteStorage`] API.
//! It intentionally avoids the upstream blocking file reader/writer helpers in
//! Bevy systems; games can still access those through [`crate::SteamworksClient`]
//! when they can move file IO out of the frame-critical path.

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{MessageWriter, Messages},
    prelude::{Res, ResMut},
    schedule::IntoScheduleConfigs,
};

use crate::{SteamworksClient, SteamworksSystem};

mod async_results;
mod messages;
mod state;
#[cfg(test)]
mod tests;
mod types;

use async_results::SteamworksRemoteStorageAsyncResults;
pub use messages::*;
pub use state::SteamworksRemoteStorageState;
pub use types::*;

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
