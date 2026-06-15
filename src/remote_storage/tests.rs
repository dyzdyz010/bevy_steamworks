use bevy_app::{App, Plugin};
use bevy_ecs::message::Messages;

use super::async_results::SteamworksRemoteStorageAsyncResults;
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
        .contains_resource::<SteamworksRemoteStorageAsyncResults>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksRemoteStorageCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksRemoteStorageResult>>());
}

#[test]
fn plugin_name_matches_remote_storage_type_path_for_bevy_tracking() {
    let plugin = SteamworksRemoteStoragePlugin::new();

    assert_eq!(
        plugin.name(),
        std::any::type_name::<SteamworksRemoteStoragePlugin>()
    );
    assert_eq!(
        plugin.name(),
        "bevy_steamworks::remote_storage::SteamworksRemoteStoragePlugin"
    );
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksRemoteStoragePlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksRemoteStorageCommand>>()
        .write(SteamworksRemoteStorageCommand::get_cloud_info());

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
fn constructors_preserve_inputs() {
    let platforms =
        steamworks::RemoteStoragePlatforms::WINDOWS | steamworks::RemoteStoragePlatforms::LINUX;

    assert_eq!(
        SteamworksRemoteStorageCommand::get_cloud_info(),
        SteamworksRemoteStorageCommand::GetCloudInfo
    );
    assert_eq!(
        SteamworksRemoteStorageCommand::is_cloud_enabled_for_app(),
        SteamworksRemoteStorageCommand::IsCloudEnabledForApp
    );
    assert_eq!(
        SteamworksRemoteStorageCommand::is_cloud_enabled_for_account(),
        SteamworksRemoteStorageCommand::IsCloudEnabledForAccount
    );
    assert_eq!(
        SteamworksRemoteStorageCommand::set_cloud_enabled_for_app(true),
        SteamworksRemoteStorageCommand::SetCloudEnabledForApp { enabled: true }
    );
    assert_eq!(
        SteamworksRemoteStorageCommand::list_files(),
        SteamworksRemoteStorageCommand::ListFiles
    );
    assert_eq!(
        SteamworksRemoteStorageCommand::get_file_info("save.dat"),
        SteamworksRemoteStorageCommand::GetFileInfo {
            name: "save.dat".to_owned(),
        }
    );
    assert_eq!(
        SteamworksRemoteStorageCommand::read_file("save.dat"),
        SteamworksRemoteStorageCommand::ReadFile {
            name: "save.dat".to_owned(),
        }
    );
    assert_eq!(
        SteamworksRemoteStorageCommand::write_file("save.dat", b"payload".to_vec()),
        SteamworksRemoteStorageCommand::WriteFile {
            write: SteamworksRemoteStorageFileWrite::new("save.dat", b"payload".to_vec()),
        }
    );
    assert_eq!(
        SteamworksRemoteStorageCommand::delete_file("save.dat"),
        SteamworksRemoteStorageCommand::DeleteFile {
            name: "save.dat".to_owned(),
        }
    );
    assert_eq!(
        SteamworksRemoteStorageCommand::forget_file("save.dat"),
        SteamworksRemoteStorageCommand::ForgetFile {
            name: "save.dat".to_owned(),
        }
    );
    assert_eq!(
        SteamworksRemoteStorageCommand::get_sync_platforms("save.dat"),
        SteamworksRemoteStorageCommand::GetSyncPlatforms {
            name: "save.dat".to_owned(),
        }
    );
    assert_eq!(
        SteamworksRemoteStorageCommand::set_sync_platforms("save.dat", platforms),
        SteamworksRemoteStorageCommand::SetSyncPlatforms {
            name: "save.dat".to_owned(),
            platforms,
        }
    );
    assert_eq!(
        SteamworksRemoteStorageCommand::share_file("save.dat"),
        SteamworksRemoteStorageCommand::ShareFile {
            name: "save.dat".to_owned(),
        }
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
    state.record_operation(&SteamworksRemoteStorageOperation::FileRead {
        contents: SteamworksRemoteStorageFileContents {
            request_id: 2,
            name: "save.dat".to_owned(),
            data: b"payload".to_vec(),
        },
    });
    state.record_operation(&SteamworksRemoteStorageOperation::FileWritten {
        written: SteamworksRemoteStorageFileWritten {
            request_id: 3,
            name: "save2.dat".to_owned(),
            bytes: 7,
        },
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
        state.last_file_info().map(|info| info.sync_platforms),
        Some(platforms)
    );
    assert_eq!(
        state.last_file_contents(),
        Some(&SteamworksRemoteStorageFileContents {
            request_id: 2,
            name: "save.dat".to_owned(),
            data: b"payload".to_vec(),
        })
    );
    assert_eq!(
        state.last_file_written(),
        Some(&SteamworksRemoteStorageFileWritten {
            request_id: 3,
            name: "save2.dat".to_owned(),
            bytes: 7,
        })
    );
    assert_eq!(state.read_count(), 1);
    assert_eq!(state.write_count(), 1);
    assert_eq!(
        state.files(),
        &[
            SteamworksRemoteStorageFileSummary {
                name: "save.dat".to_owned(),
                size_bytes: 10,
            },
            SteamworksRemoteStorageFileSummary {
                name: "save2.dat".to_owned(),
                size_bytes: 7,
            },
        ]
    );
    state.record_operation(&SteamworksRemoteStorageOperation::FileForgotten {
        name: "save.dat".to_owned(),
        forgotten: true,
    });
    assert_eq!(state.files().len(), 2);
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

#[test]
fn debug_redacts_remote_storage_file_payloads() {
    let command = SteamworksRemoteStorageCommand::write_file("save.dat", b"secret".to_vec());
    let write = SteamworksRemoteStorageFileWrite::new("save.dat", b"secret".to_vec());
    let contents = SteamworksRemoteStorageFileContents {
        request_id: 1,
        name: "save.dat".to_owned(),
        data: b"secret".to_vec(),
    };
    let operation = SteamworksRemoteStorageOperation::FileRead {
        contents: contents.clone(),
    };
    let result = SteamworksRemoteStorageResult::Ok(operation.clone());

    for debug in [
        format!("{command:?}"),
        format!("{write:?}"),
        format!("{contents:?}"),
        format!("{operation:?}"),
        format!("{result:?}"),
    ] {
        assert!(debug.contains("data_len: 6"));
        assert!(!debug.contains("secret"));
    }
}
