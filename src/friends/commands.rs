use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::{SteamworksClient, SteamworksEvent};

use super::{
    callbacks::process_friends_steam_events,
    messages::{
        SteamworksFriendsCommand, SteamworksFriendsError, SteamworksFriendsOperation,
        SteamworksFriendsResult,
    },
    state::SteamworksFriendsState,
    types::{
        SteamworksAvatar, SteamworksAvatarSize, SteamworksCoplayFriendInfo, SteamworksFriendInfo,
    },
    validation::validate_command_strings,
};

pub(super) fn process_friends_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksFriendsState>,
    mut commands: ResMut<Messages<SteamworksFriendsCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksFriendsResult>,
) {
    process_friends_steam_events(&mut state, &mut steam_events, &mut results);

    let Some(client) = client else {
        let error = SteamworksFriendsError::ClientUnavailable;
        for command in commands.drain() {
            state.record_error(error.clone());
            results.write(SteamworksFriendsResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        match handle_friends_command(&client, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks friends command"
                );
                results.write(SteamworksFriendsResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks friends command failed"
                );
                results.write(SteamworksFriendsResult::Err { command, error });
            }
        }
    }
}

fn handle_friends_command(
    client: &SteamworksClient,
    command: &SteamworksFriendsCommand,
) -> Result<SteamworksFriendsOperation, SteamworksFriendsError> {
    validate_command_strings(command)?;

    let friends = client.friends();
    match command {
        SteamworksFriendsCommand::GetPersonaName => {
            Ok(SteamworksFriendsOperation::PersonaNameRead {
                name: friends.name(),
            })
        }
        SteamworksFriendsCommand::ListFriends { flags } => {
            let listed = friends
                .get_friends(*flags)
                .iter()
                .map(snapshot_friend)
                .collect();
            Ok(SteamworksFriendsOperation::FriendsListed {
                flags: *flags,
                friends: listed,
            })
        }
        SteamworksFriendsCommand::ListCoplayFriends => {
            let listed = friends
                .get_coplay_friends()
                .iter()
                .map(|friend| SteamworksCoplayFriendInfo {
                    friend: snapshot_friend(friend),
                    coplay_app_id: friend.coplay_game_played(),
                    coplay_time: friend.coplay_time(),
                })
                .collect();
            Ok(SteamworksFriendsOperation::CoplayFriendsListed { friends: listed })
        }
        SteamworksFriendsCommand::GetFriend { steam_id } => {
            let friend = friends.get_friend(*steam_id);
            Ok(SteamworksFriendsOperation::FriendRead {
                friend: snapshot_friend(&friend),
            })
        }
        SteamworksFriendsCommand::RequestUserInformation {
            steam_id,
            name_only,
        } => Ok(SteamworksFriendsOperation::UserInformationRequested {
            steam_id: *steam_id,
            name_only: *name_only,
            requested: friends.request_user_information(*steam_id, *name_only),
        }),
        SteamworksFriendsCommand::SetRichPresence { key, value } => {
            let cleared = value.as_deref().unwrap_or_default().is_empty();
            if friends.set_rich_presence(key, value.as_deref()) {
                Ok(SteamworksFriendsOperation::RichPresenceSet {
                    key: key.clone(),
                    cleared,
                })
            } else {
                Err(SteamworksFriendsError::operation_failed(
                    "friends.set_rich_presence",
                ))
            }
        }
        SteamworksFriendsCommand::ClearRichPresence => {
            friends.clear_rich_presence();
            Ok(SteamworksFriendsOperation::RichPresenceCleared)
        }
        SteamworksFriendsCommand::GetFriendRichPresence { steam_id, key } => {
            let friend = friends.get_friend(*steam_id);
            Ok(SteamworksFriendsOperation::FriendRichPresenceRead {
                steam_id: *steam_id,
                key: key.clone(),
                value: friend.rich_presence(key),
            })
        }
        SteamworksFriendsCommand::ActivateGameOverlay { dialog } => {
            friends.activate_game_overlay(dialog);
            Ok(SteamworksFriendsOperation::GameOverlayActivated {
                dialog: dialog.clone(),
            })
        }
        SteamworksFriendsCommand::ActivateGameOverlayToWebPage { url } => {
            friends.activate_game_overlay_to_web_page(url);
            Ok(SteamworksFriendsOperation::GameOverlayToWebPageActivated { url: url.clone() })
        }
        SteamworksFriendsCommand::ActivateGameOverlayToStore { app_id, action } => {
            friends.activate_game_overlay_to_store(*app_id, action.to_steam());
            Ok(SteamworksFriendsOperation::GameOverlayToStoreActivated {
                app_id: *app_id,
                action: *action,
            })
        }
        SteamworksFriendsCommand::ActivateGameOverlayToUser { dialog, steam_id } => {
            friends.activate_game_overlay_to_user(dialog, *steam_id);
            Ok(SteamworksFriendsOperation::GameOverlayToUserActivated {
                dialog: dialog.clone(),
                steam_id: *steam_id,
            })
        }
        SteamworksFriendsCommand::ActivateInviteDialog { lobby } => {
            friends.activate_invite_dialog(*lobby);
            Ok(SteamworksFriendsOperation::InviteDialogActivated { lobby: *lobby })
        }
        SteamworksFriendsCommand::ActivateInviteDialogConnectString { connect } => {
            friends.activate_invite_dialog_connect_string(connect);
            Ok(
                SteamworksFriendsOperation::InviteDialogConnectStringActivated {
                    connect: connect.clone(),
                },
            )
        }
        SteamworksFriendsCommand::InviteUserToGame { steam_id, connect } => {
            let friend = friends.get_friend(*steam_id);
            friend.invite_user_to_game(connect);
            Ok(SteamworksFriendsOperation::UserInvitedToGame {
                steam_id: *steam_id,
                connect: connect.clone(),
            })
        }
        SteamworksFriendsCommand::SetPlayedWith { steam_id } => {
            let friend = friends.get_friend(*steam_id);
            friend.set_played_with();
            Ok(SteamworksFriendsOperation::PlayedWithSet {
                steam_id: *steam_id,
            })
        }
        SteamworksFriendsCommand::HasFriend { steam_id, flags } => {
            let friend = friends.get_friend(*steam_id);
            Ok(SteamworksFriendsOperation::HasFriendRead {
                steam_id: *steam_id,
                flags: *flags,
                has_friend: friend.has_friend(*flags),
            })
        }
        SteamworksFriendsCommand::GetFriendAvatar { steam_id, size } => {
            let friend = friends.get_friend(*steam_id);
            let rgba = match size {
                SteamworksAvatarSize::Small => friend.small_avatar(),
                SteamworksAvatarSize::Medium => friend.medium_avatar(),
                SteamworksAvatarSize::Large => friend.large_avatar(),
            };
            let avatar = rgba.map(|rgba| {
                let (width, height) = size.dimensions();
                SteamworksAvatar {
                    size: *size,
                    width,
                    height,
                    rgba,
                }
            });
            Ok(SteamworksFriendsOperation::FriendAvatarRead {
                steam_id: *steam_id,
                size: *size,
                avatar,
            })
        }
    }
}

fn snapshot_friend(friend: &steamworks::Friend) -> SteamworksFriendInfo {
    SteamworksFriendInfo {
        steam_id: friend.id(),
        name: friend.name(),
        nickname: friend.nick_name(),
        state: friend.state(),
        game: friend.game_played().map(Into::into),
    }
}
