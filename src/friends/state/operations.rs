use super::{
    SteamworksAvatar, SteamworksAvatarSize, SteamworksFriendAvatar, SteamworksFriendInfo,
    SteamworksFriendRichPresenceValue, SteamworksFriendsError, SteamworksFriendsOperation,
    SteamworksFriendsState, SteamworksHasFriendResult, SteamworksOverlayStoreActivation,
    SteamworksOverlayUserActivation, SteamworksRichPresenceChange, SteamworksUserGameInvite,
    SteamworksUserInformationRequest, STEAMWORKS_FRIENDS_STATE_CACHE_LIMIT,
};
use crate::cache::trim_oldest;

impl SteamworksFriendsState {
    pub(in crate::friends) fn record_error(&mut self, error: SteamworksFriendsError) {
        self.last_error = Some(error);
    }

    pub(in crate::friends) fn record_operation(&mut self, operation: &SteamworksFriendsOperation) {
        match operation {
            SteamworksFriendsOperation::PersonaNameRead { name } => {
                self.last_persona_name = Some(name.clone());
            }
            SteamworksFriendsOperation::FriendsListed { friends, .. } => {
                self.friends.clone_from(friends);
                for friend in friends {
                    upsert_friend(&mut self.known_friends, friend.clone());
                }
            }
            SteamworksFriendsOperation::CoplayFriendsListed { friends } => {
                self.coplay_friends.clone_from(friends);
                for friend in friends {
                    upsert_friend(&mut self.known_friends, friend.friend.clone());
                }
            }
            SteamworksFriendsOperation::FriendRead { friend } => {
                upsert_friend(&mut self.known_friends, friend.clone());
            }
            SteamworksFriendsOperation::UserInformationRequested {
                steam_id,
                name_only,
                requested,
            } => {
                self.last_user_information_request = Some(SteamworksUserInformationRequest {
                    steam_id: *steam_id,
                    name_only: *name_only,
                    requested: *requested,
                });
            }
            SteamworksFriendsOperation::RichPresenceSet { key, cleared } => {
                self.last_rich_presence_change = Some(SteamworksRichPresenceChange::Key {
                    key: key.clone(),
                    cleared: *cleared,
                });
            }
            SteamworksFriendsOperation::RichPresenceCleared => {
                self.last_rich_presence_change = Some(SteamworksRichPresenceChange::ClearedAll);
            }
            SteamworksFriendsOperation::FriendRichPresenceRead {
                steam_id,
                key,
                value,
            } => {
                upsert_friend_rich_presence(
                    &mut self.friend_rich_presence,
                    *steam_id,
                    key.clone(),
                    value.clone(),
                );
            }
            SteamworksFriendsOperation::GameOverlayActivated { dialog } => {
                self.last_game_overlay_dialog = Some(dialog.clone());
            }
            SteamworksFriendsOperation::GameOverlayToWebPageActivated { url } => {
                self.last_game_overlay_web_page = Some(url.clone());
            }
            SteamworksFriendsOperation::GameOverlayToStoreActivated { app_id, action } => {
                self.last_game_overlay_store = Some(SteamworksOverlayStoreActivation {
                    app_id: *app_id,
                    action: *action,
                });
            }
            SteamworksFriendsOperation::GameOverlayToUserActivated { dialog, steam_id } => {
                self.last_game_overlay_user = Some(SteamworksOverlayUserActivation {
                    dialog: dialog.clone(),
                    steam_id: *steam_id,
                });
            }
            SteamworksFriendsOperation::InviteDialogActivated { lobby } => {
                self.last_invite_dialog = Some(*lobby);
            }
            SteamworksFriendsOperation::InviteDialogConnectStringActivated { connect } => {
                self.last_invite_connect_string = Some(connect.clone());
            }
            SteamworksFriendsOperation::UserInvitedToGame { steam_id, connect } => {
                self.last_user_invite = Some(SteamworksUserGameInvite {
                    steam_id: *steam_id,
                    connect: connect.clone(),
                });
            }
            SteamworksFriendsOperation::PlayedWithSet { steam_id } => {
                self.last_played_with = Some(*steam_id);
            }
            SteamworksFriendsOperation::HasFriendRead {
                steam_id,
                flags,
                has_friend,
            } => {
                upsert_has_friend_result(
                    &mut self.has_friend_results,
                    *steam_id,
                    *flags,
                    *has_friend,
                );
            }
            SteamworksFriendsOperation::FriendAvatarRead {
                steam_id,
                size,
                avatar,
            } => {
                upsert_friend_avatar(&mut self.friend_avatars, *steam_id, *size, avatar.clone());
            }
            SteamworksFriendsOperation::GameOverlayActivationChanged { active } => {
                self.overlay_active = Some(*active);
            }
            SteamworksFriendsOperation::PersonaStateChanged { change } => {
                self.last_persona_state_change = Some(change.clone());
            }
            SteamworksFriendsOperation::GameLobbyJoinRequested { request } => {
                self.last_lobby_join_request = Some(request.clone());
            }
            SteamworksFriendsOperation::GameRichPresenceJoinRequested { request } => {
                self.last_rich_presence_join_request = Some(request.clone());
            }
        }
    }
}

fn upsert_friend(friends: &mut Vec<SteamworksFriendInfo>, friend: SteamworksFriendInfo) {
    if let Some(known) = friends
        .iter_mut()
        .find(|known| known.steam_id == friend.steam_id)
    {
        *known = friend;
    } else {
        friends.push(friend);
        trim_oldest(friends, STEAMWORKS_FRIENDS_STATE_CACHE_LIMIT);
    }
}

fn upsert_friend_rich_presence(
    values: &mut Vec<SteamworksFriendRichPresenceValue>,
    steam_id: steamworks::SteamId,
    key: String,
    value: Option<String>,
) {
    if let Some(known) = values
        .iter_mut()
        .find(|known| known.steam_id == steam_id && known.key == key)
    {
        known.value = value;
    } else {
        values.push(SteamworksFriendRichPresenceValue {
            steam_id,
            key,
            value,
        });
        trim_oldest(values, STEAMWORKS_FRIENDS_STATE_CACHE_LIMIT);
    }
}

fn upsert_has_friend_result(
    values: &mut Vec<SteamworksHasFriendResult>,
    steam_id: steamworks::SteamId,
    flags: steamworks::FriendFlags,
    has_friend: bool,
) {
    if let Some(known) = values
        .iter_mut()
        .find(|known| known.steam_id == steam_id && known.flags == flags)
    {
        known.has_friend = has_friend;
    } else {
        values.push(SteamworksHasFriendResult {
            steam_id,
            flags,
            has_friend,
        });
        trim_oldest(values, STEAMWORKS_FRIENDS_STATE_CACHE_LIMIT);
    }
}

fn upsert_friend_avatar(
    values: &mut Vec<SteamworksFriendAvatar>,
    steam_id: steamworks::SteamId,
    size: SteamworksAvatarSize,
    avatar: Option<SteamworksAvatar>,
) {
    if let Some(known) = values
        .iter_mut()
        .find(|known| known.steam_id == steam_id && known.size == size)
    {
        known.avatar = avatar;
    } else {
        values.push(SteamworksFriendAvatar {
            steam_id,
            size,
            avatar,
        });
        trim_oldest(values, STEAMWORKS_FRIENDS_STATE_CACHE_LIMIT);
    }
}
