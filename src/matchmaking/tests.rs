use bevy_app::{App, Plugin};
use bevy_ecs::message::Messages;

use crate::SteamworksEvent;

use super::async_results::SteamworksMatchmakingAsyncResults;
use super::validation::{validate_command, validate_filter};
use super::*;

#[test]
fn matchmaking_plugin_registers_resources_and_messages() {
    let mut app = App::new();

    app.add_plugins(SteamworksMatchmakingPlugin::new());

    assert!(app
        .world()
        .contains_resource::<SteamworksMatchmakingState>());
    assert!(app
        .world()
        .contains_resource::<SteamworksMatchmakingAsyncResults>());
    assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksMatchmakingCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksMatchmakingResult>>());
}

#[test]
fn plugin_name_matches_matchmaking_type_path_for_bevy_tracking() {
    let plugin = SteamworksMatchmakingPlugin::new();

    assert_eq!(
        plugin.name(),
        std::any::type_name::<SteamworksMatchmakingPlugin>()
    );
    assert_eq!(
        plugin.name(),
        "bevy_steamworks::matchmaking::SteamworksMatchmakingPlugin"
    );
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksMatchmakingPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksMatchmakingCommand>>()
        .write(SteamworksMatchmakingCommand::request_lobby_list(
            SteamworksLobbyListFilter::new(),
        ));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksMatchmakingResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksMatchmakingResult::Err {
            command: SteamworksMatchmakingCommand::request_lobby_list(
                SteamworksLobbyListFilter::new()
            ),
            error: SteamworksMatchmakingError::ClientUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksMatchmakingState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksMatchmakingError::ClientUnavailable)
    );
}

#[test]
fn validation_rejects_interior_nul() {
    let command = SteamworksMatchmakingCommand::set_lobby_data(
        steamworks::LobbyId::from_raw(1),
        "mode\0bad",
        "dm",
    );

    assert_eq!(
        validate_command(&command),
        Err(SteamworksMatchmakingError::InvalidString { field: "key" })
    );

    let filter = SteamworksLobbyListFilter::new().with_string(
        "mode",
        "dm\0bad",
        steamworks::StringFilterKind::Equal,
    );

    assert_eq!(
        validate_filter(&filter),
        Err(SteamworksMatchmakingError::InvalidString { field: "value" })
    );
}

#[test]
fn validation_rejects_steam_assert_inputs() {
    assert_eq!(
        validate_command(&SteamworksMatchmakingCommand::create_lobby(
            steamworks::LobbyType::Private,
            251
        )),
        Err(SteamworksMatchmakingError::MaxLobbyMembersExceeded {
            requested: 251,
            max_supported: MAX_LOBBY_MEMBERS,
        })
    );

    assert_eq!(
        validate_command(&SteamworksMatchmakingCommand::send_lobby_chat_message(
            steamworks::LobbyId::from_raw(1),
            Vec::new()
        )),
        Err(SteamworksMatchmakingError::InvalidChatMessageLength {
            requested: 0,
            max_supported: MAX_LOBBY_CHAT_MESSAGE_BYTES,
        })
    );

    let filter = SteamworksLobbyListFilter::new().with_max_results(MAX_LOBBY_LIST_RESULTS + 1);
    assert_eq!(
        validate_filter(&filter),
        Err(SteamworksMatchmakingError::MaxLobbyListResultsExceeded {
            requested: MAX_LOBBY_LIST_RESULTS + 1,
            max_supported: MAX_LOBBY_LIST_RESULTS,
        })
    );
}

#[test]
fn debug_redacts_lobby_chat_payload_bytes() {
    let lobby = steamworks::LobbyId::from_raw(1);
    let command = SteamworksMatchmakingCommand::send_lobby_chat_message(lobby, vec![1, 2, 3]);
    let operation = SteamworksMatchmakingOperation::LobbyChatEntryRead {
        lobby,
        chat_id: 7,
        data: vec![4, 5, 6],
    };
    let entry = SteamworksLobbyChatEntry {
        lobby,
        chat_id: 7,
        data: vec![7, 8, 9],
    };

    let command_debug = format!("{command:?}");
    let operation_debug = format!("{operation:?}");
    let entry_debug = format!("{entry:?}");

    assert!(command_debug.contains("data_len: 3"));
    assert!(!command_debug.contains("[1, 2, 3]"));
    assert!(operation_debug.contains("data_len: 3"));
    assert!(!operation_debug.contains("[4, 5, 6]"));
    assert!(entry_debug.contains("data_len: 3"));
    assert!(!entry_debug.contains("[7, 8, 9]"));
}

#[test]
fn async_success_operations_preserve_request_context() {
    let filter = SteamworksLobbyListFilter::new().with_max_results(2);
    let lobbies = vec![steamworks::LobbyId::from_raw(11)];

    assert_eq!(
        SteamworksMatchmakingOperation::LobbyListReceived {
            request_id: 7,
            filter: filter.clone(),
            lobbies: lobbies.clone(),
        },
        SteamworksMatchmakingOperation::LobbyListReceived {
            request_id: 7,
            filter,
            lobbies,
        }
    );

    let requested_lobby = steamworks::LobbyId::from_raw(22);
    let joined_lobby = steamworks::LobbyId::from_raw(33);
    assert_eq!(
        SteamworksMatchmakingOperation::LobbyJoined {
            request_id: 8,
            requested_lobby,
            lobby: joined_lobby,
        },
        SteamworksMatchmakingOperation::LobbyJoined {
            request_id: 8,
            requested_lobby,
            lobby: joined_lobby,
        }
    );
}

#[test]
fn state_records_matchmaking_operations_without_unbounded_history() {
    let mut state = SteamworksMatchmakingState::default();
    let filter = SteamworksLobbyListFilter::new().with_max_results(2);
    let first_lobby = steamworks::LobbyId::from_raw(11);
    let second_lobby = steamworks::LobbyId::from_raw(22);
    let user = steamworks::SteamId::from_raw(33);
    let owner = steamworks::SteamId::from_raw(44);
    let server = SteamworksLobbyGameServer {
        address: "127.0.0.1:27015".parse().expect("valid socket address"),
        steam_id: Some(owner),
    };

    state.record_operation(&SteamworksMatchmakingOperation::LobbyListRequested {
        request_id: 1,
        filter: filter.clone(),
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyListReceived {
        request_id: 1,
        filter: filter.clone(),
        lobbies: vec![first_lobby, second_lobby],
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyCreateRequested {
        request_id: 2,
        lobby_type: steamworks::LobbyType::FriendsOnly,
        max_members: 4,
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyCreated {
        request_id: 2,
        lobby_type: steamworks::LobbyType::FriendsOnly,
        max_members: 4,
        lobby: first_lobby,
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyJoinRequested {
        request_id: 3,
        lobby: second_lobby,
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyJoined {
        request_id: 3,
        requested_lobby: second_lobby,
        lobby: second_lobby,
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyJoined {
        request_id: 3,
        requested_lobby: second_lobby,
        lobby: second_lobby,
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyLeft { lobby: first_lobby });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyDataCountRead {
        lobby: second_lobby,
        count: 2,
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyDataRead {
        lobby: second_lobby,
        key: "mode".to_owned(),
        value: Some("dm".to_owned()),
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyDataByIndexRead {
        lobby: second_lobby,
        index: 1,
        entry: Some(("map".to_owned(), "arena".to_owned())),
    });
    state.record_operation(&SteamworksMatchmakingOperation::AllLobbyDataRead {
        lobby: second_lobby,
        entries: vec![
            ("mode".to_owned(), "dm".to_owned()),
            ("map".to_owned(), "arena".to_owned()),
        ],
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyDataSet {
        lobby: second_lobby,
        key: "mode".to_owned(),
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyDataDeleted {
        lobby: second_lobby,
        key: "old".to_owned(),
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyMemberDataSet {
        lobby: second_lobby,
        key: "loadout".to_owned(),
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyMemberDataRead {
        lobby: second_lobby,
        user,
        key: "rank".to_owned(),
        value: Some("gold".to_owned()),
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyMemberLimitRead {
        lobby: second_lobby,
        limit: Some(8),
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyOwnerRead {
        lobby: second_lobby,
        owner,
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyMemberCountRead {
        lobby: second_lobby,
        count: 3,
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyMembersListed {
        lobby: second_lobby,
        members: vec![user, owner],
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyJoinableSet {
        lobby: second_lobby,
        joinable: false,
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyChatMessageSent {
        lobby: second_lobby,
        len: 5,
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyChatEntryRead {
        lobby: second_lobby,
        chat_id: 7,
        data: vec![1, 2, 3],
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyGameServerSet {
        lobby: second_lobby,
        server: server.clone(),
    });
    state.record_operation(&SteamworksMatchmakingOperation::LobbyGameServerRead {
        lobby: second_lobby,
        server: Some(server.clone()),
    });

    assert_eq!(
        state.last_lobby_list_request(),
        Some(&SteamworksLobbyListRequest {
            request_id: 1,
            filter: filter.clone(),
        })
    );
    assert_eq!(state.last_lobby_list(), &[first_lobby, second_lobby]);
    assert_eq!(
        state.last_lobby_create_request(),
        Some(&SteamworksLobbyCreateRequest {
            request_id: 2,
            lobby_type: steamworks::LobbyType::FriendsOnly,
            max_members: 4,
        })
    );
    assert_eq!(
        state.last_lobby_join_request(),
        Some(&SteamworksMatchmakingLobbyJoinRequest {
            request_id: 3,
            lobby: second_lobby,
        })
    );
    assert_eq!(
        state.last_created_lobby(),
        Some(&SteamworksLobbyCreated {
            request_id: 2,
            lobby_type: steamworks::LobbyType::FriendsOnly,
            max_members: 4,
            lobby: first_lobby,
        })
    );
    assert_eq!(
        state.last_joined_lobby(),
        Some(&SteamworksLobbyJoined {
            request_id: 3,
            requested_lobby: second_lobby,
            lobby: second_lobby,
        })
    );
    assert_eq!(state.last_left_lobby(), Some(first_lobby));
    assert_eq!(state.joined_lobbies(), &[second_lobby]);
    assert_eq!(
        state.last_lobby_data_count(),
        Some(&SteamworksLobbyDataCount {
            lobby: second_lobby,
            count: 2,
        })
    );
    assert_eq!(
        state.last_lobby_data(),
        Some(&SteamworksLobbyDataValue {
            lobby: second_lobby,
            key: "mode".to_owned(),
            value: Some("dm".to_owned()),
        })
    );
    assert_eq!(
        state.last_lobby_data_entry(),
        Some(&SteamworksLobbyDataEntry {
            lobby: second_lobby,
            index: 1,
            entry: Some(("map".to_owned(), "arena".to_owned())),
        })
    );
    assert_eq!(
        state.last_all_lobby_data(),
        Some(&SteamworksLobbyDataEntries {
            lobby: second_lobby,
            entries: vec![
                ("mode".to_owned(), "dm".to_owned()),
                ("map".to_owned(), "arena".to_owned()),
            ],
        })
    );
    assert_eq!(
        state.last_lobby_data_set(),
        Some(&SteamworksLobbyDataMutation {
            lobby: second_lobby,
            key: "mode".to_owned(),
        })
    );
    assert_eq!(
        state.last_lobby_data_deleted(),
        Some(&SteamworksLobbyDataMutation {
            lobby: second_lobby,
            key: "old".to_owned(),
        })
    );
    assert_eq!(
        state.last_lobby_member_data_set(),
        Some(&SteamworksLobbyDataMutation {
            lobby: second_lobby,
            key: "loadout".to_owned(),
        })
    );
    assert_eq!(
        state.last_lobby_member_data(),
        Some(&SteamworksLobbyMemberDataValue {
            lobby: second_lobby,
            user,
            key: "rank".to_owned(),
            value: Some("gold".to_owned()),
        })
    );
    assert_eq!(
        state.last_lobby_member_limit(),
        Some(&SteamworksLobbyMemberLimit {
            lobby: second_lobby,
            limit: Some(8),
        })
    );
    assert_eq!(
        state.last_lobby_owner(),
        Some(&SteamworksLobbyOwner {
            lobby: second_lobby,
            owner,
        })
    );
    assert_eq!(
        state.last_lobby_member_count(),
        Some(&SteamworksLobbyMemberCount {
            lobby: second_lobby,
            count: 3,
        })
    );
    assert_eq!(
        state.last_lobby_members(),
        Some(&SteamworksLobbyMembers {
            lobby: second_lobby,
            members: vec![user, owner],
        })
    );
    assert_eq!(
        state.last_lobby_joinability(),
        Some(&SteamworksLobbyJoinability {
            lobby: second_lobby,
            joinable: false,
        })
    );
    assert_eq!(
        state.last_lobby_chat_message_sent(),
        Some(&SteamworksLobbyChatMessageSent {
            lobby: second_lobby,
            len: 5,
        })
    );
    assert_eq!(
        state.last_lobby_chat_entry(),
        Some(&SteamworksLobbyChatEntry {
            lobby: second_lobby,
            chat_id: 7,
            data: vec![1, 2, 3],
        })
    );
    assert_eq!(
        state.last_lobby_game_server_set(),
        Some(&SteamworksLobbyGameServerAssignment {
            lobby: second_lobby,
            server: server.clone(),
        })
    );
    assert_eq!(
        state.last_lobby_game_server(),
        Some(&SteamworksLobbyGameServerLookup {
            lobby: second_lobby,
            server: Some(server),
        })
    );
}

#[test]
fn lobby_callbacks_are_bridged_without_client() {
    let mut app = App::new();
    let lobby = steamworks::LobbyId::from_raw(11);
    let user = steamworks::SteamId::from_raw(22);
    let maker = steamworks::SteamId::from_raw(33);

    app.add_plugins(SteamworksMatchmakingPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::LobbyCreated(steamworks::LobbyCreated {
            result: 1,
            lobby,
        }));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::LobbyEnter(steamworks::LobbyEnter {
            lobby,
            chat_permissions: 0,
            blocked: false,
            chat_room_enter_response: steamworks::ChatRoomEnterResponse::Success,
        }));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::LobbyChatMsg(steamworks::LobbyChatMsg {
            lobby,
            user,
            chat_entry_type: steamworks::ChatEntryType::ChatMsg,
            chat_id: 5,
        }));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::LobbyChatUpdate(
            steamworks::LobbyChatUpdate {
                lobby,
                user_changed: user,
                making_change: maker,
                member_state_change: steamworks::ChatMemberStateChange::Entered,
            },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::LobbyDataUpdate(
            steamworks::LobbyDataUpdate {
                lobby,
                member: user,
                success: true,
            },
        ));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksMatchmakingResult>>();
    let drained = results.drain().collect::<Vec<_>>();
    let expected_created = SteamworksLobbyCreatedCallback { result: 1, lobby };
    let expected_enter = SteamworksLobbyEnterCallback {
        lobby,
        chat_permissions: 0,
        blocked: false,
        chat_room_enter_response: steamworks::ChatRoomEnterResponse::Success,
    };
    let expected_message = SteamworksLobbyChatMessage {
        lobby,
        user,
        chat_entry_type: steamworks::ChatEntryType::ChatMsg,
        chat_id: 5,
    };
    let expected_chat_update = SteamworksLobbyChatUpdate {
        lobby,
        user_changed: user,
        making_change: maker,
        member_state_change: steamworks::ChatMemberStateChange::Entered,
    };
    let expected_data_update = SteamworksLobbyDataUpdate {
        lobby,
        member: user,
        success: true,
    };

    assert_eq!(
        drained,
        vec![
            SteamworksMatchmakingResult::Ok(
                SteamworksMatchmakingOperation::LobbyCreateCallbackReceived {
                    callback: expected_created.clone(),
                },
            ),
            SteamworksMatchmakingResult::Ok(
                SteamworksMatchmakingOperation::LobbyEnterCallbackReceived {
                    callback: expected_enter.clone(),
                },
            ),
            SteamworksMatchmakingResult::Ok(
                SteamworksMatchmakingOperation::LobbyChatMessageReceived {
                    message: expected_message.clone(),
                },
            ),
            SteamworksMatchmakingResult::Ok(
                SteamworksMatchmakingOperation::LobbyChatUpdateReceived {
                    update: expected_chat_update.clone(),
                },
            ),
            SteamworksMatchmakingResult::Ok(
                SteamworksMatchmakingOperation::LobbyDataUpdateReceived {
                    update: expected_data_update.clone(),
                },
            ),
        ]
    );

    let state = app.world().resource::<SteamworksMatchmakingState>();
    assert_eq!(state.joined_lobbies(), &[lobby]);
    assert_eq!(state.last_lobby_created_callback(), Some(&expected_created));
    assert_eq!(state.last_lobby_enter_callback(), Some(&expected_enter));
    assert_eq!(state.last_lobby_chat_message(), Some(&expected_message));
    assert_eq!(state.last_lobby_chat_update(), Some(&expected_chat_update));
    assert_eq!(state.last_lobby_data_update(), Some(&expected_data_update));
    assert_eq!(state.last_error(), None);
}
