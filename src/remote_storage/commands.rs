use bevy_ecs::{
    message::{MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::SteamworksClient;

use super::{
    async_results::SteamworksRemoteStorageAsyncResults,
    file_io::{spawn_file_read, spawn_file_write},
    messages::{
        SteamworksRemoteStorageCommand, SteamworksRemoteStorageError,
        SteamworksRemoteStorageOperation, SteamworksRemoteStorageResult,
    },
    snapshots::{snapshot_cloud_info, snapshot_file_info, snapshot_file_summary},
    state::SteamworksRemoteStorageState,
    types::{SteamworksRemoteStorageFileShareHandle, SteamworksRemoteStorageSharedFile},
    validation::validate_command,
};

pub(super) fn process_remote_storage_commands(
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
    matches!(
        command,
        SteamworksRemoteStorageCommand::ReadFile { .. }
            | SteamworksRemoteStorageCommand::WriteFile { .. }
            | SteamworksRemoteStorageCommand::ShareFile { .. }
    )
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
                .map(snapshot_file_summary)
                .collect();
            Ok(SteamworksRemoteStorageOperation::FilesListed { files })
        }
        SteamworksRemoteStorageCommand::GetFileInfo { name } => {
            let file = remote_storage.file(&name);
            Ok(SteamworksRemoteStorageOperation::FileInfoRead {
                info: snapshot_file_info(name, &file),
            })
        }
        SteamworksRemoteStorageCommand::GetFileExists { name } => {
            let exists = remote_storage.file(&name).exists();
            Ok(SteamworksRemoteStorageOperation::FileExistsRead { name, exists })
        }
        SteamworksRemoteStorageCommand::IsFilePersisted { name } => {
            let persisted = remote_storage.file(&name).is_persisted();
            Ok(SteamworksRemoteStorageOperation::FilePersistedRead { name, persisted })
        }
        SteamworksRemoteStorageCommand::GetFileTimestamp { name } => {
            let timestamp = remote_storage.file(&name).timestamp();
            Ok(SteamworksRemoteStorageOperation::FileTimestampRead { name, timestamp })
        }
        SteamworksRemoteStorageCommand::ReadFile { name } => {
            let request_id = request_id.expect("async Remote Storage command missing request id");
            spawn_file_read(
                client.clone_inner(),
                async_results.clone(),
                request_id,
                name.clone(),
            )?;
            Ok(SteamworksRemoteStorageOperation::FileReadRequested { request_id, name })
        }
        SteamworksRemoteStorageCommand::WriteFile { write } => {
            let request_id = request_id.expect("async Remote Storage command missing request id");
            let bytes = write.bytes();
            spawn_file_write(
                client.clone_inner(),
                async_results.clone(),
                request_id,
                write.clone(),
            )?;
            Ok(SteamworksRemoteStorageOperation::FileWriteRequested {
                request_id,
                name: write.name,
                bytes,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn async_share_commands_get_unique_request_ids() {
        let mut state = SteamworksRemoteStorageState::default();
        let command = SteamworksRemoteStorageCommand::share_file("save.dat");

        assert_eq!(async_command_request_id(&command, &mut state), Some(0));
        assert_eq!(async_command_request_id(&command, &mut state), Some(1));
        assert_eq!(
            async_command_request_id(
                &SteamworksRemoteStorageCommand::read_file("save.dat"),
                &mut state
            ),
            Some(2)
        );
        assert_eq!(
            async_command_request_id(
                &SteamworksRemoteStorageCommand::write_file("save.dat", b"payload"),
                &mut state
            ),
            Some(3)
        );
        assert_eq!(
            async_command_request_id(&SteamworksRemoteStorageCommand::GetCloudInfo, &mut state),
            None
        );
    }
}
