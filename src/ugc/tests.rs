use std::path::PathBuf;

use bevy_app::{App, Plugin};
use bevy_ecs::message::Messages;

use crate::SteamworksEvent;

use super::async_results::SteamworksUgcAsyncResults;
use super::update_watches::SteamworksUgcUpdateWatches;
use super::validation::{
    validate_command, validate_item_update, validate_query, validate_query_options,
};
use super::*;

#[test]
fn ugc_plugin_registers_resources_and_messages() {
    let mut app = App::new();

    app.add_plugins(SteamworksUgcPlugin::new());

    assert!(app.world().contains_resource::<SteamworksUgcState>());
    assert!(app.world().contains_resource::<SteamworksUgcAsyncResults>());
    assert!(app
        .world()
        .contains_resource::<SteamworksUgcUpdateWatches>());
    assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksUgcCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksUgcResult>>());
}

#[test]
fn plugin_name_matches_ugc_type_path_for_bevy_tracking() {
    let plugin = SteamworksUgcPlugin::new();

    assert_eq!(plugin.name(), std::any::type_name::<SteamworksUgcPlugin>());
    assert_eq!(plugin.name(), "bevy_steamworks::ugc::SteamworksUgcPlugin");
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksUgcPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksUgcCommand>>()
        .write(SteamworksUgcCommand::list_subscribed_items(false));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksUgcResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksUgcResult::Err {
            command: SteamworksUgcCommand::list_subscribed_items(false),
            error: SteamworksUgcError::ClientUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksUgcState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksUgcError::ClientUnavailable)
    );
}

#[test]
fn validation_rejects_invalid_inputs() {
    assert_eq!(
        validate_command(&SteamworksUgcCommand::GetItemState {
            item: steamworks::PublishedFileId(0),
        }),
        Err(SteamworksUgcError::InvalidItemId)
    );

    assert_eq!(
        validate_command(&SteamworksUgcCommand::query(SteamworksUgcQuery::items(
            Vec::new()
        ))),
        Err(SteamworksUgcError::EmptyItemList)
    );

    let too_many = vec![steamworks::PublishedFileId(1); STEAMWORKS_UGC_MAX_ITEMS_PER_COMMAND + 1];
    assert_eq!(
        validate_command(&SteamworksUgcCommand::query(SteamworksUgcQuery::items(
            too_many
        ))),
        Err(SteamworksUgcError::TooManyItems {
            requested: STEAMWORKS_UGC_MAX_ITEMS_PER_COMMAND + 1,
            max_supported: STEAMWORKS_UGC_MAX_ITEMS_PER_COMMAND,
        })
    );

    assert_eq!(
        validate_query(&SteamworksUgcQuery::all(
            steamworks::UGCQueryType::RankedByVote,
            steamworks::UGCType::Items,
            steamworks::AppIDs::ConsumerAppId(steamworks::AppId(480)),
            0,
        )),
        Err(SteamworksUgcError::InvalidPage)
    );

    let invalid_options = [
        (
            SteamworksUgcQueryOptions::new().with_required_tag("bad\0tag"),
            "required_tag",
        ),
        (
            SteamworksUgcQueryOptions::new().with_excluded_tag("bad\0tag"),
            "excluded_tag",
        ),
        (
            SteamworksUgcQueryOptions::new().with_required_key_value_tag("bad\0key", "value"),
            "required_key_value_tag.key",
        ),
        (
            SteamworksUgcQueryOptions::new().with_required_key_value_tag("key", "bad\0value"),
            "required_key_value_tag.value",
        ),
        (
            SteamworksUgcQueryOptions::new().with_language("en\0bad"),
            "language",
        ),
        (
            SteamworksUgcQueryOptions::new().with_cloud_file_name_filter("save\0bad.dat"),
            "cloud_file_name_filter",
        ),
        (
            SteamworksUgcQueryOptions::new().with_search_text("bad\0search"),
            "search_text",
        ),
    ];

    for (options, field) in invalid_options {
        assert_eq!(
            validate_query_options(&options),
            Err(SteamworksUgcError::InvalidString { field })
        );
    }

    assert_eq!(
        validate_item_update(&SteamworksUgcItemUpdate::new()),
        Err(SteamworksUgcError::EmptyItemUpdate)
    );
    assert_eq!(
        validate_item_update(
            &SteamworksUgcItemUpdate::new()
                .with_title("x".repeat(STEAMWORKS_UGC_MAX_UPDATE_TITLE_BYTES + 1)),
        ),
        Err(SteamworksUgcError::StringTooLong {
            field: "title",
            requested: STEAMWORKS_UGC_MAX_UPDATE_TITLE_BYTES + 1,
            max_supported: STEAMWORKS_UGC_MAX_UPDATE_TITLE_BYTES,
        })
    );
    assert_eq!(
        validate_item_update(&SteamworksUgcItemUpdate::new().with_tags(["bad,tag"], false),),
        Err(SteamworksUgcError::InvalidTagText {
            tag: "bad,tag".to_owned(),
        })
    );
    assert_eq!(
        validate_item_update(&SteamworksUgcItemUpdate {
            remove_key_value_tags: vec![
                "key".to_owned();
                STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAG_REMOVALS + 1
            ],
            ..SteamworksUgcItemUpdate::new().with_title("Title")
        }),
        Err(SteamworksUgcError::TooManyKeyValueTagRemovals {
            requested: STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAG_REMOVALS + 1,
            max_supported: STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAG_REMOVALS,
        })
    );
    assert_eq!(
        validate_item_update(
            &SteamworksUgcItemUpdate::new().with_key_value_tag("bad-key", "value"),
        ),
        Err(SteamworksUgcError::InvalidKeyValueTagKey {
            key: "bad-key".to_owned(),
        })
    );
    assert_eq!(
        validate_item_update(
            &SteamworksUgcItemUpdate::new()
                .with_key_value_tag("key", "x".repeat(STEAMWORKS_UGC_MAX_UPDATE_TAG_BYTES + 1),)
        ),
        Err(SteamworksUgcError::StringTooLong {
            field: "key_value_tag.value",
            requested: STEAMWORKS_UGC_MAX_UPDATE_TAG_BYTES + 1,
            max_supported: STEAMWORKS_UGC_MAX_UPDATE_TAG_BYTES,
        })
    );
    assert_eq!(
        validate_item_update(&SteamworksUgcItemUpdate {
            add_key_value_tags: vec![
                ("key".to_owned(), "value".to_owned());
                STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAGS + 1
            ],
            ..SteamworksUgcItemUpdate::new().with_title("Title")
        }),
        Err(SteamworksUgcError::TooManyKeyValueTags {
            requested: STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAGS + 1,
            max_supported: STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAGS,
        })
    );
    let missing_path = PathBuf::from("target/__missing_bevy_steamworks_ugc_update_path__");
    assert_eq!(
        validate_item_update(
            &SteamworksUgcItemUpdate::new().with_content_path(missing_path.clone()),
        ),
        Err(SteamworksUgcError::InvalidPath {
            field: "content_path",
            path: missing_path,
        })
    );
}

#[test]
fn download_item_callbacks_are_bridged_without_client() {
    let mut app = App::new();
    let item = steamworks::PublishedFileId(42);
    let app_id = steamworks::AppId(480);

    app.add_plugins(SteamworksUgcPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::DownloadItemResult(
            steamworks::DownloadItemResult {
                app_id,
                published_file_id: item,
                error: None,
            },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::DownloadItemResult(
            steamworks::DownloadItemResult {
                app_id,
                published_file_id: item,
                error: Some(steamworks::SteamError::PersistFailed),
            },
        ));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksUgcResult>>();
    let drained = results.drain().collect::<Vec<_>>();
    let successful = SteamworksUgcDownloadItemResult {
        app_id,
        item,
        error: None,
    };
    let failed = SteamworksUgcDownloadItemResult {
        app_id,
        item,
        error: Some(steamworks::SteamError::PersistFailed),
    };

    assert_eq!(
        drained,
        vec![
            SteamworksUgcResult::Ok(SteamworksUgcOperation::DownloadItemResultReceived {
                result: successful,
            }),
            SteamworksUgcResult::Ok(SteamworksUgcOperation::DownloadItemResultReceived {
                result: failed.clone(),
            }),
        ]
    );

    let state = app.world().resource::<SteamworksUgcState>();
    assert_eq!(state.last_download_item_result(), Some(&failed));
    assert_eq!(state.last_error(), None);
}

#[test]
fn constructors_preserve_inputs() {
    let item = steamworks::PublishedFileId(42);
    let query = SteamworksUgcQuery::item(item).with_options(
        SteamworksUgcQueryOptions::new()
            .with_metadata(true)
            .with_key_value_tags(true),
    );
    assert!(
        SteamworksUgcQueryOptions::new()
            .with_additional_previews(true)
            .return_additional_previews
    );
    assert_eq!(
        SteamworksUgcContentDescriptor::from(
            steamworks::UGCContentDescriptorID::AdultOnlySexualContent
        ),
        SteamworksUgcContentDescriptor::AdultOnlySexualContent
    );

    assert_eq!(
        SteamworksUgcCommand::query(query.clone()),
        SteamworksUgcCommand::Query {
            query: query.clone()
        }
    );
    assert_eq!(
        SteamworksUgcCommand::query_total(query.clone()),
        SteamworksUgcCommand::QueryTotal {
            query: query.clone()
        }
    );
    assert_eq!(
        SteamworksUgcCommand::query_ids(query.clone()),
        SteamworksUgcCommand::QueryIds { query }
    );
    assert_eq!(
        SteamworksUgcCommand::download_item(item, true),
        SteamworksUgcCommand::DownloadItem {
            item,
            high_priority: true,
        }
    );
    assert_eq!(
        SteamworksUgcCommand::start_playtime_tracking(vec![item]),
        SteamworksUgcCommand::StartPlaytimeTracking { items: vec![item] }
    );
    assert_eq!(
        SteamworksUgcCommand::get_item_state(item),
        SteamworksUgcCommand::GetItemState { item }
    );
    assert_eq!(
        SteamworksUgcCommand::get_item_download_info(item),
        SteamworksUgcCommand::GetItemDownloadInfo { item }
    );
    assert_eq!(
        SteamworksUgcCommand::get_item_install_info(item),
        SteamworksUgcCommand::GetItemInstallInfo { item }
    );
    assert_eq!(
        SteamworksUgcCommand::create_item(steamworks::AppId(480), steamworks::FileType::Community),
        SteamworksUgcCommand::CreateItem {
            app_id: steamworks::AppId(480),
            file_type: steamworks::FileType::Community,
        }
    );
    let update = SteamworksUgcItemUpdate::new()
        .with_title("Title")
        .with_description("Description")
        .with_language("english")
        .with_metadata("metadata")
        .with_visibility(steamworks::PublishedFileVisibility::Private)
        .with_tags(["tag"], false)
        .with_key_value_tag("mode", "arena")
        .with_removed_key_value_tag("old")
        .with_remove_all_key_value_tags()
        .with_added_content_descriptor(SteamworksUgcContentDescriptor::AnyMatureContent)
        .with_removed_content_descriptor(SteamworksUgcContentDescriptor::FrequentViolenceOrGore)
        .with_change_note("Updated metadata");
    assert_eq!(
        SteamworksUgcCommand::submit_item_update(steamworks::AppId(480), item, update.clone(),),
        SteamworksUgcCommand::SubmitItemUpdate {
            app_id: steamworks::AppId(480),
            item,
            update,
        }
    );
    assert_eq!(
        SteamworksUgcCommand::get_item_update_progress(9),
        SteamworksUgcCommand::GetItemUpdateProgress { request_id: 9 }
    );
    assert_eq!(
        SteamworksUgcCommand::forget_item_update(9),
        SteamworksUgcCommand::ForgetItemUpdate { request_id: 9 }
    );
    assert_eq!(
        SteamworksUgcCommand::delete_item(item),
        SteamworksUgcCommand::DeleteItem { item }
    );
    assert_eq!(
        SteamworksUgcCommand::stop_playtime_tracking(vec![item]),
        SteamworksUgcCommand::StopPlaytimeTracking { items: vec![item] }
    );
    assert_eq!(
        SteamworksUgcCommand::stop_playtime_tracking_for_all_items(),
        SteamworksUgcCommand::StopPlaytimeTrackingForAllItems
    );
}

#[test]
fn state_records_operations_without_unbounded_query_history() {
    let mut state = SteamworksUgcState::default();
    let item = steamworks::PublishedFileId(42);
    let first = SteamworksUgcQueryResults {
        was_cached: false,
        total_results: 1,
        returned_results: 1,
        items: Vec::new(),
    };
    let second = SteamworksUgcQueryResults {
        was_cached: true,
        total_results: 2,
        returned_results: 0,
        items: Vec::new(),
    };

    state.record_operation(&SteamworksUgcOperation::SubscribedItemsListed {
        include_locally_disabled: false,
        items: vec![item],
    });
    state.record_operation(&SteamworksUgcOperation::QueryCompleted {
        request_id: 0,
        query: SteamworksUgcQuery::item(item),
        results: first,
    });
    state.record_operation(&SteamworksUgcOperation::QueryCompleted {
        request_id: 1,
        query: SteamworksUgcQuery::item(item),
        results: second.clone(),
    });
    state.record_operation(&SteamworksUgcOperation::QueryTotalCompleted {
        request_id: 2,
        query: SteamworksUgcQuery::item(item),
        total: SteamworksUgcQueryTotal { total_results: 42 },
    });
    state.record_operation(&SteamworksUgcOperation::QueryIdsCompleted {
        request_id: 3,
        query: SteamworksUgcQuery::item(item),
        ids: SteamworksUgcQueryIds { items: vec![item] },
    });
    state.record_operation(&SteamworksUgcOperation::ItemStateRead {
        info: SteamworksUgcItemStateInfo {
            item,
            state: steamworks::ItemState::SUBSCRIBED,
        },
    });
    state.record_operation(&SteamworksUgcOperation::DownloadItemSubmitted {
        item,
        high_priority: false,
    });
    state.record_operation(&SteamworksUgcOperation::ItemSubscribed {
        request_id: 2,
        item,
    });
    state.record_operation(&SteamworksUgcOperation::ItemUpdated {
        request_id: 4,
        item,
        user_needs_to_accept_workshop_legal_agreement: false,
    });
    state.record_operation(&SteamworksUgcOperation::ItemUpdateProgressRead {
        progress: SteamworksUgcItemUpdateProgress {
            request_id: 4,
            status: steamworks::UpdateStatus::UploadingContent,
            processed_bytes: 10,
            total_bytes: 100,
        },
    });
    state.record_operation(&SteamworksUgcOperation::ItemUnsubscribed {
        request_id: 3,
        item,
    });

    assert!(state.subscribed_items().is_empty());
    assert_eq!(state.last_query(), Some(&second));
    assert_eq!(
        state.last_query_total(),
        Some(&SteamworksUgcQueryTotal { total_results: 42 })
    );
    assert_eq!(
        state.last_query_ids(),
        Some(&SteamworksUgcQueryIds { items: vec![item] })
    );
    assert_eq!(
        state.last_item_state(),
        Some(&SteamworksUgcItemStateInfo {
            item,
            state: steamworks::ItemState::SUBSCRIBED,
        })
    );
    assert_eq!(state.submitted_downloads(), 1);
    assert_eq!(state.successful_async_operations(), 7);
    assert_eq!(state.failed_async_operations(), 0);
    assert_eq!(state.completed_async_operations(), 7);
    assert_eq!(
        state.last_item_update_progress(),
        Some(&SteamworksUgcItemUpdateProgress {
            request_id: 4,
            status: steamworks::UpdateStatus::UploadingContent,
            processed_bytes: 10,
            total_bytes: 100,
        })
    );
}
