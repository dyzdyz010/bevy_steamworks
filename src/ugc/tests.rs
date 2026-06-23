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

fn test_item_details(
    item: steamworks::PublishedFileId,
    title: impl Into<String>,
) -> SteamworksUgcItemDetails {
    SteamworksUgcItemDetails {
        published_file_id: item,
        creator_app_id: Some(steamworks::AppId(480)),
        consumer_app_id: Some(steamworks::AppId(480)),
        title: title.into(),
        description: "Description".to_owned(),
        owner: steamworks::SteamId::from_raw(1),
        time_created: 1,
        time_updated: 2,
        time_added_to_user_list: 3,
        visibility: steamworks::PublishedFileVisibility::Public,
        banned: false,
        accepted_for_use: true,
        tags: vec!["tag".to_owned()],
        tags_truncated: false,
        file_name: "file.dat".to_owned(),
        file_type: steamworks::FileType::Community,
        file_size: 1024,
        url: "https://example.invalid/item".to_owned(),
        num_upvotes: 10,
        num_downvotes: 1,
        score: 0.9,
        num_children: 0,
        preview_url: Some("https://example.invalid/preview.png".to_owned()),
        content_descriptors: vec![SteamworksUgcContentDescriptor::AnyMatureContent],
        statistics: vec![SteamworksUgcStatistic {
            statistic: steamworks::UGCStatisticType::Subscriptions,
            value: 7,
        }],
        metadata: Some(b"metadata".to_vec()),
        children: Some(Vec::new()),
        key_value_tags: vec![("mode".to_owned(), "arena".to_owned())],
    }
}

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
fn game_server_workshop_init_fails_when_server_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksUgcPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksUgcCommand>>()
        .write(SteamworksUgcCommand::init_workshop_for_game_server(
            steamworks::AppId(480),
            "workshop_server",
        ));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksUgcResult>>();
    let drained = results.drain().collect::<Vec<_>>();
    let command = SteamworksUgcCommand::init_workshop_for_game_server(
        steamworks::AppId(480),
        "workshop_server",
    );

    assert_eq!(
        drained,
        vec![SteamworksUgcResult::Err {
            command,
            error: SteamworksUgcError::ServerUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksUgcState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksUgcError::ServerUnavailable)
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
        validate_command(&SteamworksUgcCommand::init_workshop_for_game_server(
            SteamworksUgcWorkshopDepotId::from_raw(0),
            "workshop",
        )),
        Err(SteamworksUgcError::InvalidWorkshopDepot)
    );
    assert_eq!(
        validate_command(&SteamworksUgcCommand::init_workshop_for_game_server(
            steamworks::AppId(480),
            "bad\0folder",
        )),
        Err(SteamworksUgcError::InvalidString { field: "folder" })
    );

    assert_eq!(
        validate_command(&SteamworksUgcCommand::query(SteamworksUgcQuery::items(
            Vec::<steamworks::PublishedFileId>::new()
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
                result: successful.clone(),
            }),
            SteamworksUgcResult::Ok(SteamworksUgcOperation::DownloadItemResultReceived {
                result: failed.clone(),
            }),
        ]
    );

    let state = app.world().resource::<SteamworksUgcState>();
    assert_eq!(state.last_download_item_result(), Some(&failed));
    assert_eq!(state.download_item_results(), &[failed.clone()]);
    assert_eq!(state.download_item_result(item), Some(&failed));
    assert_eq!(state.download_item_failed(item), Some(true));
    assert_eq!(state.last_error(), None);
}

#[test]
fn constructors_preserve_inputs() {
    let item = steamworks::PublishedFileId(42);
    let query = SteamworksUgcQuery::item(42_u64).with_options(
        SteamworksUgcQueryOptions::new()
            .with_metadata(true)
            .with_key_value_tags(true),
    );
    let direct_query = SteamworksUgcQuery::all(
        steamworks::UGCQueryType::RankedByTextSearch,
        steamworks::UGCType::Items,
        steamworks::AppIDs::ConsumerAppId(steamworks::AppId(480)),
        1,
    )
    .with_required_tag("arena")
    .with_excluded_tag("draft")
    .with_required_key_value_tag("mode", "ranked")
    .with_match_any_tag(true)
    .with_language("english")
    .with_allow_cached_response(60)
    .with_cloud_file_name_filter("save.dat")
    .with_search_text("space battle")
    .with_ranked_by_trend_days(7)
    .with_long_description(true)
    .with_children(true)
    .with_metadata(true)
    .with_key_value_tags(true)
    .with_additional_previews(true)
    .with_return_only_ids(true)
    .with_return_total_only(true)
    .with_statistic(steamworks::UGCStatisticType::Subscriptions);
    assert_eq!(
        direct_query.options(),
        &SteamworksUgcQueryOptions::new()
            .with_required_tag("arena")
            .with_excluded_tag("draft")
            .with_required_key_value_tag("mode", "ranked")
            .with_match_any_tag(true)
            .with_language("english")
            .with_allow_cached_response(60)
            .with_cloud_file_name_filter("save.dat")
            .with_search_text("space battle")
            .with_ranked_by_trend_days(7)
            .with_long_description(true)
            .with_children(true)
            .with_metadata(true)
            .with_key_value_tags(true)
            .with_additional_previews(true)
            .with_return_only_ids(true)
            .with_return_total_only(true)
            .with_statistic(steamworks::UGCStatisticType::Subscriptions)
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
    assert_eq!(SteamworksUgcWorkshopDepotId::from_raw(480).raw(), 480);
    assert_eq!(
        SteamworksUgcWorkshopDepotId::from(steamworks::AppId(480)),
        SteamworksUgcWorkshopDepotId::from_raw(480)
    );
    assert_eq!(
        SteamworksUgcQuery::items([42_u64]),
        SteamworksUgcQuery::Items {
            items: vec![item],
            options: SteamworksUgcQueryOptions::new(),
        }
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
        SteamworksUgcCommand::download_item(42_u64, true),
        SteamworksUgcCommand::DownloadItem {
            item,
            high_priority: true,
        }
    );
    assert_eq!(
        SteamworksUgcCommand::start_playtime_tracking([42_u64]),
        SteamworksUgcCommand::StartPlaytimeTracking { items: vec![item] }
    );
    assert_eq!(
        SteamworksUgcCommand::get_item_state(42_u64),
        SteamworksUgcCommand::GetItemState { item }
    );
    assert_eq!(
        SteamworksUgcCommand::get_item_download_info(42_u64),
        SteamworksUgcCommand::GetItemDownloadInfo { item }
    );
    assert_eq!(
        SteamworksUgcCommand::get_item_install_info(42_u64),
        SteamworksUgcCommand::GetItemInstallInfo { item }
    );
    assert_eq!(
        SteamworksUgcCommand::create_item(480_u32, steamworks::FileType::Community),
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
        SteamworksUgcCommand::submit_item_update(480_u32, 42_u64, update.clone(),),
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
        SteamworksUgcCommand::subscribe_item(42_u64),
        SteamworksUgcCommand::SubscribeItem { item }
    );
    assert_eq!(
        SteamworksUgcCommand::unsubscribe_item(42_u64),
        SteamworksUgcCommand::UnsubscribeItem { item }
    );
    assert_eq!(
        SteamworksUgcCommand::delete_item(42_u64),
        SteamworksUgcCommand::DeleteItem { item }
    );
    assert_eq!(
        SteamworksUgcCommand::stop_playtime_tracking([42_u64]),
        SteamworksUgcCommand::StopPlaytimeTracking { items: vec![item] }
    );
    assert_eq!(
        SteamworksUgcCommand::stop_playtime_tracking_for_all_items(),
        SteamworksUgcCommand::StopPlaytimeTrackingForAllItems
    );
    assert_eq!(
        SteamworksUgcCommand::init_workshop_for_game_server(steamworks::AppId(480), "workshop"),
        SteamworksUgcCommand::InitWorkshopForGameServer {
            workshop_depot: SteamworksUgcWorkshopDepotId::from_raw(480),
            folder: "workshop".to_owned(),
        }
    );
}

#[test]
fn state_records_operations_without_unbounded_query_history() {
    let mut state = SteamworksUgcState::default();
    let item = steamworks::PublishedFileId(42);
    let first_detail = test_item_details(item, "First title");
    let second_detail = test_item_details(item, "Second title");
    let first = SteamworksUgcQueryResults {
        was_cached: false,
        total_results: 1,
        returned_results: 1,
        items: vec![first_detail],
    };
    let second = SteamworksUgcQueryResults {
        was_cached: true,
        total_results: 2,
        returned_results: 1,
        items: vec![second_detail.clone()],
    };

    state.record_operation(&SteamworksUgcOperation::SubscribedItemsListed {
        include_locally_disabled: false,
        items: vec![item],
    });
    state.record_operation(&SteamworksUgcOperation::QueryRequested {
        request_id: 0,
        query: SteamworksUgcQuery::item(item),
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
    state.record_operation(&SteamworksUgcOperation::ItemDownloadInfoRead {
        info: SteamworksUgcItemDownloadInfoResult {
            item,
            info: Some(SteamworksUgcItemDownloadInfo {
                downloaded_bytes: 10,
                total_bytes: 100,
            }),
        },
    });
    state.record_operation(&SteamworksUgcOperation::ItemInstallInfoRead {
        info: SteamworksUgcItemInstallInfoResult {
            item,
            info: Some(SteamworksUgcItemInstallInfo {
                folder: "workshop/item".to_owned(),
                size_on_disk: 2048,
                timestamp: 1234,
            }),
        },
    });
    state.record_operation(&SteamworksUgcOperation::DownloadItemSubmitted {
        item,
        high_priority: false,
    });
    state.record_operation(&SteamworksUgcOperation::DownloadItemResultReceived {
        result: SteamworksUgcDownloadItemResult {
            app_id: steamworks::AppId(480),
            item,
            error: None,
        },
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
    assert_eq!(state.item_details(), &[second_detail.clone()]);
    assert_eq!(state.item_detail(item), Some(&second_detail));
    assert_eq!(
        state.item_creator_app_id(item),
        Some(Some(steamworks::AppId(480)))
    );
    assert_eq!(
        state.item_consumer_app_id(item),
        Some(Some(steamworks::AppId(480)))
    );
    assert_eq!(state.item_title(item), Some("Second title"));
    assert_eq!(state.item_description(item), Some("Description"));
    assert_eq!(state.item_tags(item), Some(["tag".to_owned()].as_slice()));
    assert_eq!(
        state.item_preview_url(item),
        Some(Some("https://example.invalid/preview.png"))
    );
    assert_eq!(
        state.item_content_descriptors(item),
        Some([SteamworksUgcContentDescriptor::AnyMatureContent].as_slice())
    );
    assert_eq!(
        state.item_statistics(item),
        Some(
            [SteamworksUgcStatistic {
                statistic: steamworks::UGCStatisticType::Subscriptions,
                value: 7,
            }]
            .as_slice()
        )
    );
    assert_eq!(
        state.item_statistic(item, steamworks::UGCStatisticType::Subscriptions),
        Some(Some(7))
    );
    assert_eq!(
        state.item_statistic(item, steamworks::UGCStatisticType::Favorites),
        Some(None)
    );
    assert_eq!(
        state.item_metadata(item),
        Some(Some(b"metadata".as_slice()))
    );
    assert_eq!(state.item_children(item), Some(Some([].as_slice())));
    assert_eq!(
        state.item_key_value_tags(item),
        Some([("mode".to_owned(), "arena".to_owned())].as_slice())
    );
    assert_eq!(state.item_key_value_tag(item, "mode"), Some(Some("arena")));
    assert_eq!(state.item_key_value_tag(item, "missing"), Some(None));
    assert_eq!(
        state.item_state(item),
        Some(&SteamworksUgcItemStateInfo {
            item,
            state: steamworks::ItemState::SUBSCRIBED,
        })
    );
    assert_eq!(
        state.item_download_info(item),
        Some(&SteamworksUgcItemDownloadInfoResult {
            item,
            info: Some(SteamworksUgcItemDownloadInfo {
                downloaded_bytes: 10,
                total_bytes: 100,
            }),
        })
    );
    assert_eq!(
        state.item_install_info(item),
        Some(&SteamworksUgcItemInstallInfoResult {
            item,
            info: Some(SteamworksUgcItemInstallInfo {
                folder: "workshop/item".to_owned(),
                size_on_disk: 2048,
                timestamp: 1234,
            }),
        })
    );
    assert_eq!(
        state.download_item_result(item),
        Some(&SteamworksUgcDownloadItemResult {
            app_id: steamworks::AppId(480),
            item,
            error: None,
        })
    );
    assert_eq!(state.download_item_failed(item), Some(false));
    assert_eq!(
        state.query_request(0),
        Some(&SteamworksUgcQueryRequest {
            request_id: 0,
            query: SteamworksUgcQuery::item(item),
        })
    );
    assert_eq!(
        state.query_result(1),
        Some(&SteamworksUgcQueryResult {
            request_id: 1,
            query: SteamworksUgcQuery::item(item),
            results: second.clone(),
        })
    );
    assert_eq!(state.query_requests().len(), 4);
    assert_eq!(state.query_results().len(), 2);
    assert_eq!(
        state.query_total_result(2),
        Some(&SteamworksUgcQueryTotalResult {
            request_id: 2,
            query: SteamworksUgcQuery::item(item),
            total: SteamworksUgcQueryTotal { total_results: 42 },
        })
    );
    assert_eq!(
        state.query_ids_result(3),
        Some(&SteamworksUgcQueryIdsResult {
            request_id: 3,
            query: SteamworksUgcQuery::item(item),
            ids: SteamworksUgcQueryIds { items: vec![item] },
        })
    );

    state.record_operation(&SteamworksUgcOperation::ItemDeleted {
        request_id: 3,
        item,
    });
    state.record_operation(&SteamworksUgcOperation::GameServerWorkshopInitialized {
        workshop_depot: SteamworksUgcWorkshopDepotId::from_raw(480),
        folder: "workshop".to_owned(),
    });

    assert!(state.subscribed_items().is_empty());
    assert!(state.item_details().is_empty());
    assert_eq!(state.item_detail(item), None);
    assert_eq!(state.item_creator_app_id(item), None);
    assert_eq!(state.item_consumer_app_id(item), None);
    assert_eq!(state.item_title(item), None);
    assert_eq!(state.item_description(item), None);
    assert_eq!(state.item_tags(item), None);
    assert_eq!(state.item_preview_url(item), None);
    assert_eq!(state.item_content_descriptors(item), None);
    assert_eq!(state.item_statistics(item), None);
    assert_eq!(
        state.item_statistic(item, steamworks::UGCStatisticType::Subscriptions),
        None
    );
    assert_eq!(state.item_metadata(item), None);
    assert_eq!(state.item_children(item), None);
    assert_eq!(state.item_key_value_tags(item), None);
    assert_eq!(state.item_key_value_tag(item, "mode"), None);
    assert_eq!(state.item_state(item), None);
    assert_eq!(state.item_download_info(item), None);
    assert_eq!(state.item_install_info(item), None);
    assert_eq!(state.download_item_result(item), None);
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
    assert_eq!(
        state.last_game_server_workshop_init(),
        Some(&SteamworksUgcGameServerWorkshopInit {
            workshop_depot: SteamworksUgcWorkshopDepotId::from_raw(480),
            folder: "workshop".to_owned(),
        })
    );
}

#[test]
fn download_item_result_cache_is_bounded() {
    let mut state = SteamworksUgcState::default();

    for raw in 1..=(super::state::STEAMWORKS_UGC_STATE_ITEM_CACHE_LIMIT as u64 + 1) {
        let item = steamworks::PublishedFileId(raw);
        state.record_operation(&SteamworksUgcOperation::DownloadItemResultReceived {
            result: SteamworksUgcDownloadItemResult {
                app_id: steamworks::AppId(480),
                item,
                error: (raw % 2 == 0).then_some(steamworks::SteamError::PersistFailed),
            },
        });
    }

    assert_eq!(
        state.download_item_results().len(),
        super::state::STEAMWORKS_UGC_STATE_ITEM_CACHE_LIMIT
    );
    assert_eq!(
        state.download_item_result(steamworks::PublishedFileId(1)),
        None
    );
    assert_eq!(
        state.download_item_failed(steamworks::PublishedFileId(2)),
        Some(true)
    );
}

#[test]
fn item_detail_cache_is_bounded() {
    let mut state = SteamworksUgcState::default();

    for raw in 1..=(super::state::STEAMWORKS_UGC_STATE_ITEM_CACHE_LIMIT as u64 + 1) {
        let item = steamworks::PublishedFileId(raw);
        state.record_operation(&SteamworksUgcOperation::QueryCompleted {
            request_id: raw,
            query: SteamworksUgcQuery::item(item),
            results: SteamworksUgcQueryResults {
                was_cached: false,
                total_results: 1,
                returned_results: 1,
                items: vec![test_item_details(item, format!("Item {raw}"))],
            },
        });
    }

    assert_eq!(
        state.item_details().len(),
        super::state::STEAMWORKS_UGC_STATE_ITEM_CACHE_LIMIT
    );
    assert_eq!(state.item_detail(steamworks::PublishedFileId(1)), None);
    assert!(state.item_detail(steamworks::PublishedFileId(2)).is_some());
}

#[test]
fn query_result_caches_are_bounded() {
    let mut state = SteamworksUgcState::default();

    for raw in 1..=(super::state::STEAMWORKS_UGC_STATE_ITEM_CACHE_LIMIT as u64 + 1) {
        let item = steamworks::PublishedFileId(raw);
        let query = SteamworksUgcQuery::item(item);

        state.record_operation(&SteamworksUgcOperation::QueryRequested {
            request_id: raw,
            query: query.clone(),
        });
        state.record_operation(&SteamworksUgcOperation::QueryCompleted {
            request_id: raw,
            query: query.clone(),
            results: SteamworksUgcQueryResults {
                was_cached: false,
                total_results: 1,
                returned_results: 1,
                items: vec![test_item_details(item, format!("Item {raw}"))],
            },
        });
        state.record_operation(&SteamworksUgcOperation::QueryTotalCompleted {
            request_id: raw,
            query: query.clone(),
            total: SteamworksUgcQueryTotal {
                total_results: raw as u32,
            },
        });
        state.record_operation(&SteamworksUgcOperation::QueryIdsCompleted {
            request_id: raw,
            query,
            ids: SteamworksUgcQueryIds { items: vec![item] },
        });
    }

    assert_eq!(
        state.query_requests().len(),
        super::state::STEAMWORKS_UGC_STATE_ITEM_CACHE_LIMIT
    );
    assert_eq!(
        state.query_results().len(),
        super::state::STEAMWORKS_UGC_STATE_ITEM_CACHE_LIMIT
    );
    assert_eq!(
        state.query_total_results().len(),
        super::state::STEAMWORKS_UGC_STATE_ITEM_CACHE_LIMIT
    );
    assert_eq!(
        state.query_ids_results().len(),
        super::state::STEAMWORKS_UGC_STATE_ITEM_CACHE_LIMIT
    );

    assert_eq!(state.query_request(1), None);
    assert_eq!(state.query_result(1), None);
    assert_eq!(state.query_total_result(1), None);
    assert_eq!(state.query_ids_result(1), None);
    assert!(state.query_request(2).is_some());
    assert!(state.query_result(2).is_some());
    assert!(state.query_total_result(2).is_some());
    assert!(state.query_ids_result(2).is_some());
}
