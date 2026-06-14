use bevy_app::App;
use bevy_ecs::message::Messages;

use super::achievements::achievement_icon_fetched_operation;
use super::lifecycle::should_submit_store;
use super::*;

#[test]
fn stats_plugin_registers_resources_and_messages() {
    let mut app = App::new();

    app.add_plugins(SteamworksStatsPlugin::new());

    assert!(app.world().contains_resource::<SteamworksStatsSettings>());
    assert!(app.world().contains_resource::<SteamworksStatsState>());
    assert!(app
        .world()
        .contains_resource::<SteamworksStatsLeaderboardHandles>());
    assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksStatsCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksStatsResult>>());
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksStatsPlugin::new().request_current_user_stats_on_startup(false));
    app.world_mut()
        .resource_mut::<Messages<SteamworksStatsCommand>>()
        .write(SteamworksStatsCommand::get_stat_i32("total_kills"));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksStatsResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksStatsResult::Err {
            command: SteamworksStatsCommand::get_stat_i32("total_kills"),
            error: SteamworksStatsError::ClientUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksStatsState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksStatsError::ClientUnavailable)
    );
}

#[test]
fn stats_callbacks_are_bridged_without_client() {
    let mut app = App::new();
    let steam_id = steamworks::SteamId::from_raw(42);
    let game_id = steamworks::GameId::from_raw(480);

    app.add_plugins(SteamworksStatsPlugin::new().request_current_user_stats_on_startup(false));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::UserStatsReceived(
            steamworks::UserStatsReceived {
                steam_id,
                game_id,
                result: Ok(()),
            },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::UserStatsStored(
            steamworks::UserStatsStored {
                game_id,
                result: Err(steamworks::SteamError::PersistFailed),
            },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::UserAchievementStored(
            steamworks::UserAchievementStored {
                game_id,
                achievement_name: "ACH_WIN".to_owned(),
                current_progress: 5,
                max_progress: 10,
            },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::UserAchievementIconFetched(
            steamworks::UserAchievementIconFetched {
                game_id,
                achievement_name: "ACH_WIN".to_owned(),
                achieved: true,
                icon_handle: 99,
            },
        ));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksStatsResult>>();
    let drained = results.drain().collect::<Vec<_>>();
    let expected_received = SteamworksUserStatsReceived {
        steam_id,
        game_id,
        result: Ok(()),
    };
    let expected_stored = SteamworksUserStatsStored {
        game_id,
        result: Err(steamworks::SteamError::PersistFailed),
    };
    let expected_achievement = SteamworksUserAchievementStored {
        game_id,
        achievement_name: "ACH_WIN".to_owned(),
        current_progress: 5,
        max_progress: 10,
    };

    assert_eq!(
        drained,
        vec![
            SteamworksStatsResult::Ok(SteamworksStatsOperation::UserStatsReceived {
                callback: expected_received.clone(),
            }),
            SteamworksStatsResult::Ok(SteamworksStatsOperation::UserStatsStored {
                callback: expected_stored.clone(),
            }),
            SteamworksStatsResult::Ok(SteamworksStatsOperation::UserAchievementStored {
                callback: expected_achievement.clone(),
            }),
            SteamworksStatsResult::Ok(SteamworksStatsOperation::AchievementIconFetched {
                name: "ACH_WIN".to_owned(),
                achieved: true,
                icon_handle: 99,
                icon: SteamworksAchievementIconStatus::PendingOrUnavailable,
            }),
        ]
    );

    let state = app.world().resource::<SteamworksStatsState>();
    assert_eq!(state.last_user_stats_received(), Some(&expected_received));
    assert_eq!(state.last_user_stats_stored(), Some(&expected_stored));
    assert_eq!(
        state.last_user_achievement_stored(),
        Some(&expected_achievement)
    );
    assert_eq!(state.achievement_icon_callback_count(), 1);
    assert_eq!(state.last_error(), None);
}

#[test]
fn operation_requires_store_for_writes_only() {
    assert!(operation_requires_store(
        &SteamworksStatsOperation::AchievementUnlocked {
            name: "ACH_WIN".to_owned(),
        }
    ));
    assert!(operation_requires_store(
        &SteamworksStatsOperation::StatI32Set {
            name: "kills".to_owned(),
            value: 1,
        }
    ));
    assert!(!operation_requires_store(
        &SteamworksStatsOperation::StatI32Read {
            name: "kills".to_owned(),
            value: 1,
        }
    ));
    assert!(!operation_requires_store(
        &SteamworksStatsOperation::StatsStoreSubmitted
    ));
    assert!(!operation_requires_store(
        &SteamworksStatsOperation::AllStatsReset {
            achievements_too: true,
        }
    ));
}

#[test]
fn forced_store_bypasses_auto_store_setting() {
    let mut state = SteamworksStatsState::default();
    state.pending_store = true;
    let auto_store = SteamworksStatsSettings {
        auto_store: true,
        ..Default::default()
    };
    let manual_store = SteamworksStatsSettings {
        auto_store: false,
        ..Default::default()
    };

    assert!(should_submit_store(&auto_store, &state));
    assert!(!should_submit_store(&manual_store, &state));

    state.force_store = true;
    assert!(should_submit_store(&manual_store, &state));
}

#[test]
fn state_records_local_stats_and_named_achievements() {
    let mut state = SteamworksStatsState::default();

    state.record_operation(&SteamworksStatsOperation::StatI32Read {
        name: "kills".to_owned(),
        value: 1,
    });
    state.record_operation(&SteamworksStatsOperation::StatI32Set {
        name: "kills".to_owned(),
        value: 2,
    });
    state.record_operation(&SteamworksStatsOperation::StatF32Read {
        name: "accuracy".to_owned(),
        value: 0.5,
    });
    state.record_operation(&SteamworksStatsOperation::StatF32Set {
        name: "accuracy".to_owned(),
        value: 0.75,
    });
    state.record_operation(&SteamworksStatsOperation::AchievementRead {
        name: "ACH_WIN".to_owned(),
        achieved: false,
    });
    state.record_operation(&SteamworksStatsOperation::AchievementDisplayAttributeRead {
        name: "ACH_WIN".to_owned(),
        key: "name".to_owned(),
        value: "Winner".to_owned(),
    });
    state.record_operation(&SteamworksStatsOperation::AchievementDisplayAttributeRead {
        name: "ACH_WIN".to_owned(),
        key: "desc".to_owned(),
        value: "Win once".to_owned(),
    });
    state.record_operation(&SteamworksStatsOperation::AchievementDisplayAttributeRead {
        name: "ACH_WIN".to_owned(),
        key: "hidden".to_owned(),
        value: "1".to_owned(),
    });
    state.record_operation(&SteamworksStatsOperation::AchievementAndUnlockTimeRead {
        name: "ACH_WIN".to_owned(),
        achieved: true,
        unlock_time: 42,
    });
    state.record_operation(&SteamworksStatsOperation::AchievementAchievedPercentRead {
        name: "ACH_WIN".to_owned(),
        percent: 12.5,
    });
    state.record_operation(
        &SteamworksStatsOperation::AchievementGlobalPercentagesListed {
            offset: 0,
            total: 1,
            percentages: vec![SteamworksAchievementGlobalPercentage {
                api_name: "ACH_OTHER".to_owned(),
                percent: 99.0,
            }],
        },
    );
    state.record_operation(&SteamworksStatsOperation::AchievementUnlocked {
        name: "ACH_SECRET".to_owned(),
    });
    state.record_operation(&SteamworksStatsOperation::AchievementCleared {
        name: "ACH_SECRET".to_owned(),
    });

    assert_eq!(state.stat_i32("kills"), Some(2));
    assert_eq!(state.stat_f32("accuracy"), Some(0.75));
    assert_eq!(state.achievement_unlocked("ACH_WIN"), Some(true));
    assert_eq!(state.achievement_unlock_time("ACH_WIN"), Some(42));
    assert_eq!(
        state.achievement_display_attribute("ACH_WIN", "name"),
        Some("Winner")
    );
    assert_eq!(state.achievement_global_percent("ACH_WIN"), Some(12.5));
    assert_eq!(state.achievement_global_percent("ACH_OTHER"), Some(99.0));
    assert_eq!(
        state.last_global_achievement_percentages(),
        &[SteamworksAchievementGlobalPercentage {
            api_name: "ACH_OTHER".to_owned(),
            percent: 99.0,
        }]
    );
    assert_eq!(
        state.achievement("ACH_WIN"),
        Some(&SteamworksAchievementInfo {
            api_name: "ACH_WIN".to_owned(),
            display_name: Some("Winner".to_owned()),
            description: Some("Win once".to_owned()),
            hidden: Some(true),
            achieved: Some(true),
            unlock_time: Some(42),
        })
    );
    assert_eq!(state.achievement_unlocked("ACH_SECRET"), Some(false));
    assert_eq!(state.achievement_unlock_time("ACH_SECRET"), Some(0));

    state.record_operation(&SteamworksStatsOperation::AchievementNamesListed {
        offset: 0,
        total: 1,
        names: vec!["ACH_WIN".to_owned()],
    });
    assert_eq!(
        state.last_achievements(),
        &[SteamworksAchievementInfo {
            api_name: "ACH_WIN".to_owned(),
            ..Default::default()
        }]
    );
    assert_eq!(state.achievement_unlocked("ACH_WIN"), Some(true));
    assert_eq!(
        state.achievement_display_attribute("ACH_WIN", "desc"),
        Some("Win once")
    );

    state.record_operation(&SteamworksStatsOperation::AllStatsReset {
        achievements_too: false,
    });
    assert_eq!(state.stat_i32("kills"), None);
    assert_eq!(state.stat_f32("accuracy"), None);
    assert_eq!(state.achievement_unlocked("ACH_WIN"), Some(true));

    state.record_operation(&SteamworksStatsOperation::AllStatsReset {
        achievements_too: true,
    });
    assert_eq!(state.achievement_unlocked("ACH_WIN"), Some(false));
    assert_eq!(state.achievement_unlock_time("ACH_WIN"), Some(0));
}

#[test]
fn achievement_commands_preserve_inputs() {
    assert_eq!(
        SteamworksStatsCommand::list_achievement_names(),
        SteamworksStatsCommand::ListAchievementNames {
            offset: 0,
            limit: STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND,
        }
    );
    assert_eq!(
        SteamworksStatsCommand::list_achievement_names_page(8, 4),
        SteamworksStatsCommand::ListAchievementNames {
            offset: 8,
            limit: 4,
        }
    );
    assert_eq!(
        SteamworksStatsCommand::list_achievements(true, false),
        SteamworksStatsCommand::ListAchievements {
            include_display_attributes: true,
            include_unlock_state: false,
            offset: 0,
            limit: STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND,
        }
    );
    assert_eq!(
        SteamworksStatsCommand::list_achievements_page(false, true, 4, 12),
        SteamworksStatsCommand::ListAchievements {
            include_display_attributes: false,
            include_unlock_state: true,
            offset: 4,
            limit: 12,
        }
    );
    assert_eq!(
        SteamworksStatsCommand::get_achievement_icon("ACH_WIN"),
        SteamworksStatsCommand::GetAchievementIcon {
            name: "ACH_WIN".to_owned(),
        }
    );
    assert_eq!(
        SteamworksStatsCommand::list_achievement_global_percentages(),
        SteamworksStatsCommand::ListAchievementGlobalPercentages {
            offset: 0,
            limit: STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND,
        }
    );
    assert_eq!(
        SteamworksStatsCommand::list_achievement_global_percentages_page(6, 9),
        SteamworksStatsCommand::ListAchievementGlobalPercentages {
            offset: 6,
            limit: 9,
        }
    );
}

#[test]
fn leaderboard_commands_preserve_inputs() {
    let leaderboard = SteamworksLeaderboardId::from_raw(7);
    assert_eq!(SteamworksLeaderboardId::from_raw(7).raw(), 7);

    assert_eq!(
        SteamworksStatsCommand::find_leaderboard("daily_score"),
        SteamworksStatsCommand::FindLeaderboard {
            name: "daily_score".to_owned(),
        }
    );
    assert_eq!(
        SteamworksStatsCommand::find_or_create_leaderboard(
            "daily_score",
            SteamworksLeaderboardSortMethod::Descending,
            SteamworksLeaderboardDisplayType::Numeric,
        ),
        SteamworksStatsCommand::FindOrCreateLeaderboard {
            name: "daily_score".to_owned(),
            sort_method: SteamworksLeaderboardSortMethod::Descending,
            display_type: SteamworksLeaderboardDisplayType::Numeric,
        }
    );
    assert_eq!(
        SteamworksStatsCommand::get_leaderboard_info(leaderboard),
        SteamworksStatsCommand::GetLeaderboardInfo { leaderboard }
    );
    assert_eq!(
        SteamworksStatsCommand::upload_leaderboard_score(
            leaderboard,
            SteamworksLeaderboardUploadScoreMethod::KeepBest,
            10,
            vec![1, 2],
        ),
        SteamworksStatsCommand::UploadLeaderboardScore {
            leaderboard,
            method: SteamworksLeaderboardUploadScoreMethod::KeepBest,
            score: 10,
            details: vec![1, 2],
        }
    );
    assert_eq!(
        SteamworksStatsCommand::download_leaderboard_entries(
            leaderboard,
            SteamworksLeaderboardDataRequest::Global { start: 1, end: 10 },
            4,
        ),
        SteamworksStatsCommand::DownloadLeaderboardEntries {
            leaderboard,
            request: SteamworksLeaderboardDataRequest::Global { start: 1, end: 10 },
            max_details: 4,
        }
    );
    assert_eq!(
        SteamworksStatsCommand::download_leaderboard_entries_around_user(leaderboard, -2, 2, 0,),
        SteamworksStatsCommand::DownloadLeaderboardEntries {
            leaderboard,
            request: SteamworksLeaderboardDataRequest::GlobalAroundUser { start: -2, end: 2 },
            max_details: 0,
        }
    );
    assert_eq!(
        SteamworksStatsCommand::download_friends_leaderboard_entries(leaderboard, 0),
        SteamworksStatsCommand::DownloadLeaderboardEntries {
            leaderboard,
            request: SteamworksLeaderboardDataRequest::Friends,
            max_details: 0,
        }
    );
    assert_eq!(
        SteamworksStatsCommand::forget_leaderboard(leaderboard),
        SteamworksStatsCommand::ForgetLeaderboard { leaderboard }
    );
}

#[test]
fn leaderboard_validation_rejects_invalid_inputs() {
    assert_eq!(
        validate_stats_command(&SteamworksStatsCommand::find_leaderboard("bad\0name")),
        Err(SteamworksStatsError::InvalidString { field: "name" })
    );
    assert_eq!(
        validate_stats_command(&SteamworksStatsCommand::get_achievement_icon("bad\0name",)),
        Err(SteamworksStatsError::InvalidString { field: "name" })
    );
    assert_eq!(
        validate_stats_command(&SteamworksStatsCommand::list_achievements_page(
            false,
            false,
            0,
            STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND + 1,
        )),
        Err(SteamworksStatsError::TooManyAchievementEntries {
            requested: STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND + 1,
            max_supported: STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND,
        })
    );
    assert_eq!(
        validate_stats_command(
            &SteamworksStatsCommand::list_achievement_global_percentages_page(
                0,
                STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND + 1,
            )
        ),
        Err(SteamworksStatsError::TooManyAchievementEntries {
            requested: STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND + 1,
            max_supported: STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND,
        })
    );
    assert_eq!(
        validate_stats_command(&SteamworksStatsCommand::get_achievement_display_attribute(
            "ACH_WIN", "bad\0key",
        )),
        Err(SteamworksStatsError::InvalidString { field: "key" })
    );
    assert_eq!(
        validate_stats_command(&SteamworksStatsCommand::upload_leaderboard_score(
            SteamworksLeaderboardId::from_raw(1),
            SteamworksLeaderboardUploadScoreMethod::ForceUpdate,
            5,
            vec![0; STEAMWORKS_LEADERBOARD_MAX_DETAILS + 1],
        )),
        Err(SteamworksStatsError::TooManyLeaderboardDetails {
            requested: STEAMWORKS_LEADERBOARD_MAX_DETAILS + 1,
            max_supported: STEAMWORKS_LEADERBOARD_MAX_DETAILS,
        })
    );
    assert_eq!(
        validate_stats_command(&SteamworksStatsCommand::download_leaderboard_entries(
            SteamworksLeaderboardId::from_raw(1),
            SteamworksLeaderboardDataRequest::Global { start: 10, end: 5 },
            0,
        )),
        Err(SteamworksStatsError::InvalidLeaderboardRange { start: 10, end: 5 })
    );
    assert_eq!(
        validate_stats_command(&SteamworksStatsCommand::download_leaderboard_entries(
            SteamworksLeaderboardId::from_raw(1),
            SteamworksLeaderboardDataRequest::GlobalAroundUser { start: -2, end: 2 },
            0,
        )),
        Ok(())
    );
    assert_eq!(
        validate_stats_command(&SteamworksStatsCommand::download_leaderboard_entries(
            SteamworksLeaderboardId::from_raw(1),
            SteamworksLeaderboardDataRequest::Global {
                start: 0,
                end: STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND as i32,
            },
            0,
        )),
        Err(SteamworksStatsError::InvalidLeaderboardRange {
            start: 0,
            end: STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND as i32,
        })
    );
    assert_eq!(
        validate_stats_command(&SteamworksStatsCommand::download_leaderboard_entries(
            SteamworksLeaderboardId::from_raw(1),
            SteamworksLeaderboardDataRequest::GlobalAroundUser {
                start: -(STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND as i32),
                end: 0,
            },
            0,
        )),
        Err(SteamworksStatsError::TooManyLeaderboardEntries {
            requested: STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND + 1,
            max_supported: STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND,
        })
    );
}

#[test]
fn leaderboard_state_records_latest_info_and_entries() {
    let mut state = SteamworksStatsState::default();
    let leaderboard = SteamworksLeaderboardId::from_raw(3);
    let created_leaderboard = SteamworksLeaderboardId::from_raw(4);
    let info = SteamworksLeaderboardInfo {
        leaderboard,
        name: "daily_score".to_owned(),
        display_type: Some(SteamworksLeaderboardDisplayType::Numeric),
        sort_method: Some(SteamworksLeaderboardSortMethod::Descending),
        entry_count: 42,
    };
    let entry = SteamworksLeaderboardEntry {
        user: steamworks::SteamId::from_raw(123),
        global_rank: 1,
        score: 9000,
        details: vec![7, 8],
    };
    let upload = SteamworksLeaderboardScoreUploaded {
        score: 9000,
        was_changed: true,
        global_rank_new: 1,
        global_rank_previous: 3,
    };

    state.record_operation(&SteamworksStatsOperation::LeaderboardFindSubmitted {
        name: "daily_score".to_owned(),
    });
    state.record_operation(&SteamworksStatsOperation::LeaderboardFindCompleted {
        name: "daily_score".to_owned(),
        leaderboard: Some(leaderboard),
    });
    state.record_operation(
        &SteamworksStatsOperation::LeaderboardFindOrCreateSubmitted {
            name: "weekly_score".to_owned(),
            sort_method: SteamworksLeaderboardSortMethod::Descending,
            display_type: SteamworksLeaderboardDisplayType::Numeric,
        },
    );
    state.record_operation(
        &SteamworksStatsOperation::LeaderboardFindOrCreateCompleted {
            name: "weekly_score".to_owned(),
            leaderboard: Some(created_leaderboard),
        },
    );
    state.record_operation(&SteamworksStatsOperation::LeaderboardInfoRead { info: info.clone() });
    state.record_operation(&SteamworksStatsOperation::LeaderboardScoreUploadSubmitted {
        leaderboard,
        method: SteamworksLeaderboardUploadScoreMethod::KeepBest,
        score: 9000,
        details: vec![7, 8],
    });
    state.record_operation(&SteamworksStatsOperation::LeaderboardScoreUploaded {
        leaderboard,
        upload: Some(upload.clone()),
    });
    state.record_operation(
        &SteamworksStatsOperation::LeaderboardEntriesDownloadSubmitted {
            leaderboard,
            request: SteamworksLeaderboardDataRequest::Global { start: 1, end: 10 },
            max_details: 2,
        },
    );
    state.record_operation(&SteamworksStatsOperation::LeaderboardEntriesDownloaded {
        leaderboard,
        entries: vec![entry.clone()],
    });
    state.record_operation(&SteamworksStatsOperation::LeaderboardForgotten { leaderboard });

    assert_eq!(
        state.last_leaderboard_find_request(),
        Some(&SteamworksLeaderboardFindRequest {
            name: "daily_score".to_owned(),
        })
    );
    assert_eq!(
        state.last_leaderboard_find_result(),
        Some(&SteamworksLeaderboardFindResult {
            name: "daily_score".to_owned(),
            leaderboard: Some(leaderboard),
        })
    );
    assert_eq!(
        state.last_leaderboard_find_or_create_request(),
        Some(&SteamworksLeaderboardFindOrCreateRequest {
            name: "weekly_score".to_owned(),
            sort_method: SteamworksLeaderboardSortMethod::Descending,
            display_type: SteamworksLeaderboardDisplayType::Numeric,
        })
    );
    assert_eq!(
        state.last_leaderboard_find_or_create_result(),
        Some(&SteamworksLeaderboardFindOrCreateResult {
            name: "weekly_score".to_owned(),
            leaderboard: Some(created_leaderboard),
        })
    );
    assert_eq!(state.last_leaderboard_info(), Some(&info));
    assert_eq!(
        state.last_leaderboard_score_upload_request(),
        Some(&SteamworksLeaderboardScoreUploadRequest {
            leaderboard,
            method: SteamworksLeaderboardUploadScoreMethod::KeepBest,
            score: 9000,
            details: vec![7, 8],
        })
    );
    assert_eq!(
        state.last_leaderboard_score_upload_result(),
        Some(&SteamworksLeaderboardScoreUploadResult {
            leaderboard,
            upload: Some(upload),
        })
    );
    assert_eq!(
        state.last_leaderboard_entries_download_request(),
        Some(&SteamworksLeaderboardEntriesDownloadRequest {
            leaderboard,
            request: SteamworksLeaderboardDataRequest::Global { start: 1, end: 10 },
            max_details: 2,
        })
    );
    assert_eq!(
        state.last_leaderboard_entries_download_result(),
        Some(&SteamworksLeaderboardEntriesDownloadResult {
            leaderboard,
            entries: vec![entry.clone()],
        })
    );
    assert_eq!(state.last_leaderboard_entries(), &[entry]);
    assert_eq!(state.last_forgotten_leaderboard(), Some(leaderboard));
}

#[test]
fn achievement_state_records_catalog_and_icons() {
    let mut state = SteamworksStatsState::default();
    let achievement = SteamworksAchievementInfo {
        api_name: "ACH_WIN".to_owned(),
        display_name: Some("Winner".to_owned()),
        description: Some("Win once".to_owned()),
        hidden: Some(false),
        achieved: Some(true),
        unlock_time: Some(12),
    };
    let icon = SteamworksAchievementIcon {
        api_name: "ACH_WIN".to_owned(),
        width: 64,
        height: 64,
        rgba: vec![255; 64 * 64 * 4],
    };

    state.record_operation(&SteamworksStatsOperation::AchievementNamesListed {
        offset: 0,
        total: 1,
        names: vec!["ACH_ONE".to_owned()],
    });
    assert_eq!(
        state.last_achievements(),
        &[SteamworksAchievementInfo {
            api_name: "ACH_ONE".to_owned(),
            ..Default::default()
        }]
    );

    state.record_operation(&SteamworksStatsOperation::AchievementsListed {
        offset: 0,
        total: 1,
        achievements: vec![achievement.clone()],
    });
    assert_eq!(
        state.achievement_display_attribute("ACH_WIN", "name"),
        Some("Winner")
    );
    assert_eq!(
        state.achievement_display_attribute("ACH_WIN", "hidden"),
        Some("0")
    );
    state.record_operation(&SteamworksStatsOperation::AchievementIconRead {
        name: "ACH_WIN".to_owned(),
        icon: SteamworksAchievementIconStatus::Available(icon.clone()),
    });
    state.record_operation(&SteamworksStatsOperation::AchievementIconFetched {
        name: "ACH_WIN".to_owned(),
        achieved: true,
        icon_handle: 99,
        icon: SteamworksAchievementIconStatus::PendingOrUnavailable,
    });
    state.record_operation(
        &SteamworksStatsOperation::AchievementGlobalPercentagesListed {
            offset: 0,
            total: 1,
            percentages: vec![SteamworksAchievementGlobalPercentage {
                api_name: "ACH_WIN".to_owned(),
                percent: 12.5,
            }],
        },
    );

    assert_eq!(state.last_achievements(), &[achievement]);
    assert_eq!(state.last_achievement_icon(), Some(&icon));
    assert_eq!(state.achievement_icon_callback_count(), 1);
    assert_eq!(
        state.last_global_achievement_percentages(),
        &[SteamworksAchievementGlobalPercentage {
            api_name: "ACH_WIN".to_owned(),
            percent: 12.5,
        }]
    );
}

#[test]
fn global_stats_state_records_latest_values() {
    let mut state = SteamworksStatsState::default();
    let game_id = steamworks::GameId::from_raw(480);

    state.record_operation(&SteamworksStatsOperation::GlobalStatsReceived { game_id });
    state.record_operation(&SteamworksStatsOperation::GlobalStatI64Read {
        name: "total_kills".to_owned(),
        value: 123,
    });
    state.record_operation(&SteamworksStatsOperation::GlobalStatF64Read {
        name: "average_accuracy".to_owned(),
        value: 0.75,
    });
    state.record_operation(&SteamworksStatsOperation::GlobalStatHistoryI64Read {
        name: "daily_kills".to_owned(),
        values: vec![3, 2, 1],
    });
    state.record_operation(&SteamworksStatsOperation::GlobalStatHistoryF64Read {
        name: "daily_accuracy".to_owned(),
        values: vec![0.5, 0.6],
    });

    assert_eq!(state.last_global_stats_game_id(), Some(game_id));
    assert_eq!(
        state.last_global_stat_i64(),
        Some(&SteamworksGlobalStatValue {
            name: "total_kills".to_owned(),
            value: 123,
        })
    );
    assert_eq!(
        state.last_global_stat_f64(),
        Some(&SteamworksGlobalStatValue {
            name: "average_accuracy".to_owned(),
            value: 0.75,
        })
    );
    assert_eq!(
        state.last_global_stat_history_i64(),
        Some(&SteamworksGlobalStatHistory {
            name: "daily_kills".to_owned(),
            values: vec![3, 2, 1],
        })
    );
    assert_eq!(
        state.last_global_stat_history_f64(),
        Some(&SteamworksGlobalStatHistory {
            name: "daily_accuracy".to_owned(),
            values: vec![0.5, 0.6],
        })
    );

    state.record_operation(&SteamworksStatsOperation::GlobalStatsRequested { history_days: 7 });

    assert_eq!(state.last_global_stats_game_id(), None);
    assert_eq!(state.last_global_stat_i64(), None);
    assert_eq!(state.last_global_stat_f64(), None);
    assert_eq!(state.last_global_stat_history_i64(), None);
    assert_eq!(state.last_global_stat_history_f64(), None);
}

#[test]
fn achievement_icon_callback_operation_preserves_context() {
    let icon = SteamworksAchievementIconStatus::Available(achievement_icon_from_rgba(
        "ACH_WIN",
        vec![255; 64 * 64 * 4],
    ));
    let event = steamworks::UserAchievementIconFetched {
        game_id: steamworks::GameId::from_raw(480),
        achievement_name: "ACH_WIN".to_owned(),
        achieved: true,
        icon_handle: 42,
    };

    assert_eq!(
        achievement_icon_fetched_operation(&event, icon.clone()),
        SteamworksStatsOperation::AchievementIconFetched {
            name: "ACH_WIN".to_owned(),
            achieved: true,
            icon_handle: 42,
            icon,
        }
    );
    assert_eq!(
        SteamworksAchievementIconStatus::PendingOrUnavailable.as_icon(),
        None
    );
}
