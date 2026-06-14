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
