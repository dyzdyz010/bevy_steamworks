use std::{
    io::{Read, Write},
    thread,
};

use super::{
    async_results::SteamworksRemoteStorageAsyncResults,
    messages::{
        SteamworksRemoteStorageCommand, SteamworksRemoteStorageError,
        SteamworksRemoteStorageOperation, SteamworksRemoteStorageResult,
    },
    types::{
        SteamworksRemoteStorageFileContents, SteamworksRemoteStorageFileWrite,
        SteamworksRemoteStorageFileWritten,
    },
};

pub(super) fn spawn_file_read(
    client: steamworks::Client,
    async_results: SteamworksRemoteStorageAsyncResults,
    request_id: u64,
    name: String,
) -> Result<(), SteamworksRemoteStorageError> {
    let command = SteamworksRemoteStorageCommand::ReadFile { name: name.clone() };
    let error_name = name.clone();
    thread::Builder::new()
        .name(format!("bevy_steamworks_remote_storage_read_{request_id}"))
        .spawn(move || {
            let result = read_file_on_worker(client, request_id, name, command);
            async_results.push(result);
        })
        .map(|_| ())
        .map_err(|source| {
            SteamworksRemoteStorageError::file_io(
                "remote_storage.file.read.spawn",
                request_id,
                error_name,
                source,
            )
        })
}

fn read_file_on_worker(
    client: steamworks::Client,
    request_id: u64,
    name: String,
    command: SteamworksRemoteStorageCommand,
) -> SteamworksRemoteStorageResult {
    let remote_storage = client.remote_storage();
    let file = remote_storage.file(&name);
    if !file.exists() {
        return SteamworksRemoteStorageResult::Err {
            command,
            error: SteamworksRemoteStorageError::file_unavailable_for_request(request_id, name),
        };
    }

    let mut data = Vec::new();
    if let Err(source) = file.read().read_to_end(&mut data) {
        return SteamworksRemoteStorageResult::Err {
            command,
            error: SteamworksRemoteStorageError::file_io(
                "remote_storage.file.read",
                request_id,
                name,
                source,
            ),
        };
    }

    SteamworksRemoteStorageResult::Ok(SteamworksRemoteStorageOperation::FileRead {
        contents: SteamworksRemoteStorageFileContents {
            request_id,
            name,
            data,
        },
    })
}

pub(super) fn spawn_file_write(
    client: steamworks::Client,
    async_results: SteamworksRemoteStorageAsyncResults,
    request_id: u64,
    write: SteamworksRemoteStorageFileWrite,
) -> Result<(), SteamworksRemoteStorageError> {
    let error_name = write.name.clone();
    thread::Builder::new()
        .name(format!("bevy_steamworks_remote_storage_write_{request_id}"))
        .spawn(move || {
            let result = write_file_on_worker(client, request_id, write);
            async_results.push(result);
        })
        .map(|_| ())
        .map_err(|source| {
            SteamworksRemoteStorageError::file_io(
                "remote_storage.file.write.spawn",
                request_id,
                error_name,
                source,
            )
        })
}

fn write_file_on_worker(
    client: steamworks::Client,
    request_id: u64,
    write: SteamworksRemoteStorageFileWrite,
) -> SteamworksRemoteStorageResult {
    let command = SteamworksRemoteStorageCommand::WriteFile {
        write: write.clone(),
    };
    let bytes = write.bytes();
    let remote_storage = client.remote_storage();
    let mut writer = remote_storage.file(&write.name).write();
    if let Err(source) = writer.write_all(&write.data) {
        return SteamworksRemoteStorageResult::Err {
            command,
            error: SteamworksRemoteStorageError::file_io(
                "remote_storage.file.write",
                request_id,
                write.name,
                source,
            ),
        };
    }
    drop(writer);

    SteamworksRemoteStorageResult::Ok(SteamworksRemoteStorageOperation::FileWritten {
        written: SteamworksRemoteStorageFileWritten {
            request_id,
            name: write.name,
            bytes,
        },
    })
}
