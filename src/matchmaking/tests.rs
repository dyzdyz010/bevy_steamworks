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
fn command_constructors_preserve_inputs() {
    let filter = SteamworksLobbyListFilter::new().with_max_results(2);
    let lobby = steamworks::LobbyId::from_raw(11);
    let user = steamworks::SteamId::from_raw(22);
    let server_id = steamworks::SteamId::from_raw(33);
    let address = "127.0.0.1:27015".parse().expect("valid socket address");

    assert_eq!(
        SteamworksMatchmakingCommand::request_lobby_list(filter.clone()),
        SteamworksMatchmakingCommand::RequestLobbyList { filter }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::create_lobby(steamworks::LobbyType::Private, 4),
        SteamworksMatchmakingCommand::CreateLobby {
            lobby_type: steamworks::LobbyType::Private,
            max_members: 4,
        }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::join_lobby(lobby),
        SteamworksMatchmakingCommand::JoinLobby { lobby }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::leave_lobby(lobby),
        SteamworksMatchmakingCommand::LeaveLobby { lobby }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::get_lobby_data_count(lobby),
        SteamworksMatchmakingCommand::GetLobbyDataCount { lobby }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::get_lobby_data(lobby, "mode"),
        SteamworksMatchmakingCommand::GetLobbyData {
            lobby,
            key: "mode".to_owned(),
        }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::get_lobby_data_by_index(lobby, 3),
        SteamworksMatchmakingCommand::GetLobbyDataByIndex { lobby, index: 3 }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::get_all_lobby_data(lobby),
        SteamworksMatchmakingCommand::GetAllLobbyData { lobby }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::set_lobby_data(lobby, "mode", "dm"),
        SteamworksMatchmakingCommand::SetLobbyData {
            lobby,
            key: "mode".to_owned(),
            value: "dm".to_owned(),
        }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::delete_lobby_data(lobby, "mode"),
        SteamworksMatchmakingCommand::DeleteLobbyData {
            lobby,
            key: "mode".to_owned(),
        }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::set_lobby_member_data(lobby, "loadout", "rail"),
        SteamworksMatchmakingCommand::SetLobbyMemberData {
            lobby,
            key: "loadout".to_owned(),
            value: "rail".to_owned(),
        }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::get_lobby_member_data(lobby, user, "rank"),
        SteamworksMatchmakingCommand::GetLobbyMemberData {
            lobby,
            user,
            key: "rank".to_owned(),
        }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::get_lobby_member_limit(lobby),
        SteamworksMatchmakingCommand::GetLobbyMemberLimit { lobby }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::get_lobby_owner(lobby),
        SteamworksMatchmakingCommand::GetLobbyOwner { lobby }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::get_lobby_member_count(lobby),
        SteamworksMatchmakingCommand::GetLobbyMemberCount { lobby }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::list_lobby_members(lobby),
        SteamworksMatchmakingCommand::ListLobbyMembers { lobby }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::set_lobby_joinable(lobby, false),
        SteamworksMatchmakingCommand::SetLobbyJoinable {
            lobby,
            joinable: false,
        }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::send_lobby_chat_message(lobby, b"hello".to_vec()),
        SteamworksMatchmakingCommand::SendLobbyChatMessage {
            lobby,
            data: b"hello".to_vec(),
        }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::get_lobby_chat_entry(lobby, 7, 128),
        SteamworksMatchmakingCommand::GetLobbyChatEntry {
            lobby,
            chat_id: 7,
            max_bytes: 128,
        }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::set_lobby_game_server(lobby, address, Some(server_id)),
        SteamworksMatchmakingCommand::SetLobbyGameServer {
            lobby,
            address,
            steam_id: Some(server_id),
        }
    );
    assert_eq!(
        SteamworksMatchmakingCommand::get_lobby_game_server(lobby),
        SteamworksMatchmakingCommand::GetLobbyGameServer { lobby }
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
    assert_eq!(state.last_lobby_list_count(), 2);
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
    assert_eq!(state.last_joined_lobby_id(), Some(second_lobby));
    assert_eq!(state.last_left_lobby(), Some(first_lobby));
    assert_eq!(state.joined_lobbies(), &[second_lobby]);
    assert_eq!(state.joined_lobby_count(), 1);
    assert!(state.is_lobby_joined(second_lobby));
    assert!(!state.is_lobby_joined(first_lobby));
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
            server: Some(server.clone()),
        })
    );
    assert_eq!(state.lobby_list_requests().len(), 1);
    assert_eq!(state.lobby_list_request(1), state.last_lobby_list_request());
    assert_eq!(
        state.lobby_list_result(1),
        Some(&SteamworksLobbyListResult {
            request_id: 1,
            filter,
            lobbies: vec![first_lobby, second_lobby],
        })
    );
    assert_eq!(
        state.lobby_create_request(2),
        state.last_lobby_create_request()
    );
    assert_eq!(state.lobby_join_request(3), state.last_lobby_join_request());
    assert_eq!(state.created_lobby(2), state.last_created_lobby());
    assert_eq!(state.joined_lobby_result(3), state.last_joined_lobby());
    assert_eq!(
        state.lobby_data_count(second_lobby),
        state.last_lobby_data_count()
    );
    assert_eq!(state.lobby_data_count_value(second_lobby), Some(2));
    assert_eq!(
        state.lobby_data_value(second_lobby, "mode"),
        state.last_lobby_data()
    );
    assert_eq!(state.lobby_data(second_lobby, "mode"), Some(Some("dm")));
    assert_eq!(state.lobby_data(second_lobby, "map"), Some(Some("arena")));
    assert_eq!(state.lobby_data(second_lobby, "missing"), Some(None));
    assert_eq!(state.lobby_data(first_lobby, "mode"), None);
    assert_eq!(state.has_lobby_data(second_lobby, "mode"), Some(true));
    assert_eq!(state.has_lobby_data(second_lobby, "missing"), Some(false));
    assert_eq!(state.has_lobby_data(first_lobby, "mode"), None);
    assert_eq!(
        state.lobby_data_entry(second_lobby, 1),
        state.last_lobby_data_entry()
    );
    assert_eq!(
        state.all_lobby_data(second_lobby),
        state.last_all_lobby_data()
    );
    assert_eq!(
        state.all_lobby_data_value(second_lobby, "map"),
        Some("arena")
    );
    assert_eq!(state.all_lobby_data_value(second_lobby, "missing"), None);
    assert_eq!(
        state.lobby_data_set(second_lobby, "mode"),
        state.last_lobby_data_set()
    );
    assert_eq!(
        state.lobby_data_deletion(second_lobby, "old"),
        state.last_lobby_data_deleted()
    );
    assert_eq!(
        state.lobby_member_data_set(second_lobby, "loadout"),
        state.last_lobby_member_data_set()
    );
    assert_eq!(
        state.lobby_member_data_value(second_lobby, user, "rank"),
        state.last_lobby_member_data()
    );
    assert_eq!(
        state.lobby_member_data(second_lobby, user, "rank"),
        Some(Some("gold"))
    );
    assert_eq!(state.lobby_member_data(second_lobby, user, "missing"), None);
    assert_eq!(
        state.has_lobby_member_data(second_lobby, user, "rank"),
        Some(true)
    );
    assert_eq!(
        state.has_lobby_member_data(second_lobby, user, "missing"),
        None
    );
    assert_eq!(
        state.lobby_member_limit(second_lobby),
        state.last_lobby_member_limit()
    );
    assert_eq!(state.lobby_member_limit_value(second_lobby), Some(Some(8)));
    assert_eq!(state.lobby_owner(second_lobby), state.last_lobby_owner());
    assert_eq!(state.lobby_owner_id(second_lobby), Some(owner));
    assert_eq!(
        state.lobby_member_count(second_lobby),
        state.last_lobby_member_count()
    );
    assert_eq!(state.lobby_member_count_value(second_lobby), Some(3));
    assert_eq!(
        state.lobby_members(second_lobby),
        state.last_lobby_members()
    );
    assert_eq!(
        state.lobby_member_ids(second_lobby),
        Some([user, owner].as_slice())
    );
    assert_eq!(state.has_lobby_member(second_lobby, user), Some(true));
    assert_eq!(
        state.has_lobby_member(second_lobby, steamworks::SteamId::from_raw(55)),
        Some(false)
    );
    assert_eq!(state.has_lobby_member(first_lobby, user), None);
    assert_eq!(
        state.lobby_joinability(second_lobby),
        state.last_lobby_joinability()
    );
    assert_eq!(state.lobby_joinable(second_lobby), Some(false));
    assert_eq!(
        state.lobby_chat_message_sent(second_lobby),
        state.last_lobby_chat_message_sent()
    );
    assert_eq!(
        state.lobby_chat_entry(second_lobby, 7),
        state.last_lobby_chat_entry()
    );
    assert_eq!(
        state.lobby_chat_entry_data(second_lobby, 7),
        Some([1, 2, 3].as_slice())
    );
    assert_eq!(state.lobby_chat_entry_len(second_lobby, 7), Some(3));
    assert_eq!(
        state.last_lobby_chat_entry_data(),
        Some([1, 2, 3].as_slice())
    );
    assert_eq!(
        state.lobby_game_server_assignment(second_lobby),
        state.last_lobby_game_server_set()
    );
    assert_eq!(
        state.lobby_game_server(second_lobby),
        state.last_lobby_game_server()
    );
    assert_eq!(state.has_lobby_game_server(second_lobby), Some(true));
    assert_eq!(
        state.lobby_game_server_address(second_lobby),
        Some(Some(server.address))
    );
    assert_eq!(
        state.lobby_game_server_steam_id(second_lobby),
        Some(Some(owner))
    );
}

#[test]
fn matchmaking_state_lookup_caches_are_bounded() {
    let mut state = SteamworksMatchmakingState::default();
    let limit = 1_024;

    for index in 0..(limit + 4) {
        state.record_operation(&SteamworksMatchmakingOperation::LobbyDataRead {
            lobby: steamworks::LobbyId::from_raw((index + 1) as u64),
            key: format!("key-{index}"),
            value: Some(format!("value-{index}")),
        });
    }

    assert_eq!(state.lobby_data_values().len(), limit);
    assert!(state
        .lobby_data_value(steamworks::LobbyId::from_raw(1), "key-0")
        .is_none());
    assert_eq!(
        state.lobby_data_value(
            steamworks::LobbyId::from_raw((limit + 4) as u64),
            format!("key-{}", limit + 3)
        ),
        Some(&SteamworksLobbyDataValue {
            lobby: steamworks::LobbyId::from_raw((limit + 4) as u64),
            key: format!("key-{}", limit + 3),
            value: Some(format!("value-{}", limit + 3)),
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
