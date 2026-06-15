use bevy_app::{App, Plugin};
use bevy_ecs::message::Messages;

use crate::SteamworksEvent;

use super::*;

#[test]
fn friends_plugin_registers_resources_and_messages() {
    let mut app = App::new();

    app.add_plugins(SteamworksFriendsPlugin::new());

    assert!(app.world().contains_resource::<SteamworksFriendsState>());
    assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksFriendsCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksFriendsResult>>());
}

#[test]
fn plugin_name_matches_friends_type_path_for_bevy_tracking() {
    let plugin = SteamworksFriendsPlugin::new();

    assert_eq!(
        plugin.name(),
        std::any::type_name::<SteamworksFriendsPlugin>()
    );
    assert_eq!(
        plugin.name(),
        "bevy_steamworks::friends::SteamworksFriendsPlugin"
    );
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksFriendsPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksFriendsCommand>>()
        .write(SteamworksFriendsCommand::get_persona_name());

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksFriendsResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksFriendsResult::Err {
            command: SteamworksFriendsCommand::GetPersonaName,
            error: SteamworksFriendsError::ClientUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksFriendsState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksFriendsError::ClientUnavailable)
    );
}

#[test]
fn constructors_preserve_inputs() {
    let friend = steamworks::SteamId::from_raw(2);
    let lobby = steamworks::LobbyId::from_raw(3);
    let app_id = steamworks::AppId(480);
    let flags = steamworks::FriendFlags::IMMEDIATE;

    assert_eq!(
        SteamworksFriendsCommand::get_persona_name(),
        SteamworksFriendsCommand::GetPersonaName
    );
    assert_eq!(
        SteamworksFriendsCommand::list_friends(flags),
        SteamworksFriendsCommand::ListFriends { flags }
    );
    assert_eq!(
        SteamworksFriendsCommand::list_coplay_friends(),
        SteamworksFriendsCommand::ListCoplayFriends
    );
    assert_eq!(
        SteamworksFriendsCommand::get_friend(friend),
        SteamworksFriendsCommand::GetFriend { steam_id: friend }
    );
    assert_eq!(
        SteamworksFriendsCommand::request_user_information(friend, true),
        SteamworksFriendsCommand::RequestUserInformation {
            steam_id: friend,
            name_only: true,
        }
    );
    assert_eq!(
        SteamworksFriendsCommand::set_rich_presence("status", "In Match"),
        SteamworksFriendsCommand::SetRichPresence {
            key: "status".to_owned(),
            value: Some("In Match".to_owned()),
        }
    );
    assert_eq!(
        SteamworksFriendsCommand::clear_rich_presence_key("status"),
        SteamworksFriendsCommand::SetRichPresence {
            key: "status".to_owned(),
            value: None,
        }
    );
    assert_eq!(
        SteamworksFriendsCommand::clear_rich_presence(),
        SteamworksFriendsCommand::ClearRichPresence
    );
    assert_eq!(
        SteamworksFriendsCommand::get_friend_rich_presence(friend, "connect"),
        SteamworksFriendsCommand::GetFriendRichPresence {
            steam_id: friend,
            key: "connect".to_owned(),
        }
    );
    assert_eq!(
        SteamworksFriendsCommand::activate_game_overlay("Friends"),
        SteamworksFriendsCommand::ActivateGameOverlay {
            dialog: "Friends".to_owned(),
        }
    );
    assert_eq!(
        SteamworksFriendsCommand::activate_game_overlay_to_web_page("https://steamcommunity.com"),
        SteamworksFriendsCommand::ActivateGameOverlayToWebPage {
            url: "https://steamcommunity.com".to_owned(),
        }
    );
    assert_eq!(
        SteamworksFriendsCommand::activate_game_overlay_to_store(
            app_id,
            SteamworksOverlayToStoreAction::AddToCart,
        ),
        SteamworksFriendsCommand::ActivateGameOverlayToStore {
            app_id,
            action: SteamworksOverlayToStoreAction::AddToCart,
        }
    );
    assert_eq!(
        SteamworksFriendsCommand::activate_game_overlay_to_user("steamid", friend),
        SteamworksFriendsCommand::ActivateGameOverlayToUser {
            dialog: "steamid".to_owned(),
            steam_id: friend,
        }
    );
    assert_eq!(
        SteamworksFriendsCommand::activate_invite_dialog(lobby),
        SteamworksFriendsCommand::ActivateInviteDialog { lobby }
    );
    assert_eq!(
        SteamworksFriendsCommand::activate_invite_dialog_connect_string("join=abc"),
        SteamworksFriendsCommand::ActivateInviteDialogConnectString {
            connect: "join=abc".to_owned(),
        }
    );
    assert_eq!(
        SteamworksFriendsCommand::invite_user_to_game(friend, "join=abc"),
        SteamworksFriendsCommand::InviteUserToGame {
            steam_id: friend,
            connect: "join=abc".to_owned(),
        }
    );
    assert_eq!(
        SteamworksFriendsCommand::set_played_with(friend),
        SteamworksFriendsCommand::SetPlayedWith { steam_id: friend }
    );
    assert_eq!(
        SteamworksFriendsCommand::has_friend(friend, flags),
        SteamworksFriendsCommand::HasFriend {
            steam_id: friend,
            flags,
        }
    );
    assert_eq!(
        SteamworksFriendsCommand::get_friend_avatar(friend, SteamworksAvatarSize::Medium),
        SteamworksFriendsCommand::GetFriendAvatar {
            steam_id: friend,
            size: SteamworksAvatarSize::Medium,
        }
    );
}

#[test]
fn state_records_friend_operations() {
    let mut state = SteamworksFriendsState::default();
    let user = steamworks::SteamId::from_raw(1);
    let friend = steamworks::SteamId::from_raw(2);
    let lobby = steamworks::LobbyId::from_raw(3);
    let app_id = steamworks::AppId(480);
    let flags = steamworks::FriendFlags::IMMEDIATE;
    let initial_friend = SteamworksFriendInfo {
        steam_id: friend,
        name: "Alex".to_owned(),
        nickname: None,
        state: steamworks::FriendState::Online,
        game: None,
    };
    let updated_friend = SteamworksFriendInfo {
        steam_id: friend,
        name: "Alex Updated".to_owned(),
        nickname: Some("A".to_owned()),
        state: steamworks::FriendState::Busy,
        game: None,
    };
    let coplay_friend = SteamworksCoplayFriendInfo {
        friend: SteamworksFriendInfo {
            steam_id: user,
            name: "Morgan".to_owned(),
            nickname: None,
            state: steamworks::FriendState::Away,
            game: None,
        },
        coplay_app_id: app_id,
        coplay_time: 123,
    };
    let avatar = SteamworksAvatar {
        size: SteamworksAvatarSize::Small,
        width: 32,
        height: 32,
        rgba: vec![255; 32 * 32 * 4],
    };

    state.record_operation(&SteamworksFriendsOperation::PersonaNameRead {
        name: "Current User".to_owned(),
    });
    state.record_operation(&SteamworksFriendsOperation::FriendsListed {
        flags,
        friends: vec![initial_friend.clone()],
    });
    state.record_operation(&SteamworksFriendsOperation::FriendRead {
        friend: updated_friend.clone(),
    });
    state.record_operation(&SteamworksFriendsOperation::CoplayFriendsListed {
        friends: vec![coplay_friend.clone()],
    });
    state.record_operation(&SteamworksFriendsOperation::UserInformationRequested {
        steam_id: friend,
        name_only: true,
        requested: false,
    });
    state.record_operation(&SteamworksFriendsOperation::RichPresenceSet {
        key: "status".to_owned(),
        cleared: false,
    });
    state.record_operation(&SteamworksFriendsOperation::RichPresenceCleared);
    state.record_operation(&SteamworksFriendsOperation::FriendRichPresenceRead {
        steam_id: friend,
        key: "connect".to_owned(),
        value: Some("127.0.0.1".to_owned()),
    });
    state.record_operation(&SteamworksFriendsOperation::FriendRichPresenceRead {
        steam_id: friend,
        key: "connect".to_owned(),
        value: None,
    });
    state.record_operation(&SteamworksFriendsOperation::GameOverlayActivated {
        dialog: "Friends".to_owned(),
    });
    state.record_operation(&SteamworksFriendsOperation::GameOverlayToWebPageActivated {
        url: "https://steamcommunity.com".to_owned(),
    });
    state.record_operation(&SteamworksFriendsOperation::GameOverlayToStoreActivated {
        app_id,
        action: SteamworksOverlayToStoreAction::AddToCart,
    });
    state.record_operation(&SteamworksFriendsOperation::GameOverlayToUserActivated {
        dialog: "steamid".to_owned(),
        steam_id: friend,
    });
    state.record_operation(&SteamworksFriendsOperation::InviteDialogActivated { lobby });
    state.record_operation(
        &SteamworksFriendsOperation::InviteDialogConnectStringActivated {
            connect: "join=abc".to_owned(),
        },
    );
    state.record_operation(&SteamworksFriendsOperation::UserInvitedToGame {
        steam_id: friend,
        connect: "join=abc".to_owned(),
    });
    state.record_operation(&SteamworksFriendsOperation::PlayedWithSet { steam_id: friend });
    state.record_operation(&SteamworksFriendsOperation::HasFriendRead {
        steam_id: friend,
        flags,
        has_friend: false,
    });
    state.record_operation(&SteamworksFriendsOperation::HasFriendRead {
        steam_id: friend,
        flags,
        has_friend: true,
    });
    state.record_operation(&SteamworksFriendsOperation::FriendAvatarRead {
        steam_id: friend,
        size: SteamworksAvatarSize::Small,
        avatar: None,
    });
    assert_eq!(
        state.friend_avatar(friend, SteamworksAvatarSize::Small),
        Some(None)
    );
    state.record_operation(&SteamworksFriendsOperation::FriendAvatarRead {
        steam_id: friend,
        size: SteamworksAvatarSize::Small,
        avatar: Some(avatar.clone()),
    });

    assert_eq!(state.last_persona_name(), Some("Current User"));
    assert_eq!(state.friends(), &[initial_friend]);
    assert_eq!(state.friend(friend), Some(&updated_friend));
    assert_eq!(state.known_friends().len(), 2);
    assert_eq!(state.coplay_friends(), std::slice::from_ref(&coplay_friend));
    assert_eq!(state.coplay_friend(user), Some(&coplay_friend));
    assert_eq!(
        state.last_user_information_request(),
        Some(&SteamworksUserInformationRequest {
            steam_id: friend,
            name_only: true,
            requested: false,
        })
    );
    assert_eq!(
        state.last_rich_presence_change(),
        Some(&SteamworksRichPresenceChange::ClearedAll)
    );
    assert_eq!(state.friend_rich_presence(friend, "connect"), Some(None));
    assert_eq!(state.friend_rich_presence_values().len(), 1);
    assert_eq!(state.last_game_overlay_dialog(), Some("Friends"));
    assert_eq!(
        state.last_game_overlay_web_page(),
        Some("https://steamcommunity.com")
    );
    assert_eq!(
        state.last_game_overlay_store(),
        Some(SteamworksOverlayStoreActivation {
            app_id,
            action: SteamworksOverlayToStoreAction::AddToCart,
        })
    );
    assert_eq!(
        state.last_game_overlay_user(),
        Some(&SteamworksOverlayUserActivation {
            dialog: "steamid".to_owned(),
            steam_id: friend,
        })
    );
    assert_eq!(state.last_invite_dialog(), Some(lobby));
    assert_eq!(state.last_invite_connect_string(), Some("join=abc"));
    assert_eq!(
        state.last_user_invite(),
        Some(&SteamworksUserGameInvite {
            steam_id: friend,
            connect: "join=abc".to_owned(),
        })
    );
    assert_eq!(state.last_played_with(), Some(friend));
    assert_eq!(state.has_friend(friend, flags), Some(true));
    assert_eq!(state.has_friend_results().len(), 1);
    assert_eq!(
        state.friend_avatar(friend, SteamworksAvatarSize::Small),
        Some(Some(&avatar))
    );
    assert_eq!(state.friend_avatars().len(), 1);
}

#[test]
fn friends_callbacks_are_bridged_without_client() {
    let mut app = App::new();
    let user = steamworks::SteamId::from_raw(1);
    let friend = steamworks::SteamId::from_raw(2);
    let lobby = steamworks::LobbyId::from_raw(3);
    let flags = steamworks::PersonaChange::NAME | steamworks::PersonaChange::AVATAR;

    app.add_plugins(SteamworksFriendsPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::GameOverlayActivated(
            steamworks::GameOverlayActivated { active: true },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::PersonaStateChange(
            steamworks::PersonaStateChange {
                steam_id: user,
                flags,
            },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::GameLobbyJoinRequested(
            steamworks::GameLobbyJoinRequested {
                lobby_steam_id: lobby,
                friend_steam_id: friend,
            },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::GameRichPresenceJoinRequested(
            steamworks::GameRichPresenceJoinRequested {
                friend_steam_id: friend,
                connect: "join=abc".to_owned(),
            },
        ));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksFriendsResult>>();
    let drained = results.drain().collect::<Vec<_>>();
    assert_eq!(
        drained,
        vec![
            SteamworksFriendsResult::Ok(SteamworksFriendsOperation::GameOverlayActivationChanged {
                active: true
            },),
            SteamworksFriendsResult::Ok(SteamworksFriendsOperation::PersonaStateChanged {
                change: SteamworksPersonaStateChange {
                    steam_id: user,
                    flags,
                },
            },),
            SteamworksFriendsResult::Ok(SteamworksFriendsOperation::GameLobbyJoinRequested {
                request: SteamworksLobbyJoinRequest {
                    lobby,
                    friend_steam_id: friend,
                },
            },),
            SteamworksFriendsResult::Ok(
                SteamworksFriendsOperation::GameRichPresenceJoinRequested {
                    request: SteamworksRichPresenceJoinRequest {
                        friend_steam_id: friend,
                        connect: "join=abc".to_owned(),
                    },
                },
            ),
        ]
    );

    let state = app.world().resource::<SteamworksFriendsState>();
    assert_eq!(state.overlay_active(), Some(true));
    assert_eq!(
        state.last_persona_state_change(),
        Some(&SteamworksPersonaStateChange {
            steam_id: user,
            flags,
        })
    );
    assert_eq!(
        state.last_lobby_join_request(),
        Some(&SteamworksLobbyJoinRequest {
            lobby,
            friend_steam_id: friend,
        })
    );
    assert_eq!(
        state.last_rich_presence_join_request(),
        Some(&SteamworksRichPresenceJoinRequest {
            friend_steam_id: friend,
            connect: "join=abc".to_owned(),
        })
    );
    assert_eq!(state.last_error(), None);
}
