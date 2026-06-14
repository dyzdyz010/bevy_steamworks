use bevy_ecs::prelude::Resource;

use super::{
    messages::{SteamworksFriendsError, SteamworksFriendsOperation},
    types::{
        SteamworksAvatar, SteamworksAvatarSize, SteamworksCoplayFriendInfo, SteamworksFriendAvatar,
        SteamworksFriendInfo, SteamworksFriendRichPresenceValue, SteamworksHasFriendResult,
        SteamworksLobbyJoinRequest, SteamworksOverlayStoreActivation,
        SteamworksOverlayUserActivation, SteamworksPersonaStateChange,
        SteamworksRichPresenceChange, SteamworksRichPresenceJoinRequest, SteamworksUserGameInvite,
        SteamworksUserInformationRequest,
    },
};

/// Runtime state for [`super::SteamworksFriendsPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksFriendsState {
    last_error: Option<SteamworksFriendsError>,
    last_persona_name: Option<String>,
    friends: Vec<SteamworksFriendInfo>,
    known_friends: Vec<SteamworksFriendInfo>,
    coplay_friends: Vec<SteamworksCoplayFriendInfo>,
    last_user_information_request: Option<SteamworksUserInformationRequest>,
    last_rich_presence_change: Option<SteamworksRichPresenceChange>,
    friend_rich_presence: Vec<SteamworksFriendRichPresenceValue>,
    last_game_overlay_dialog: Option<String>,
    last_game_overlay_web_page: Option<String>,
    last_game_overlay_store: Option<SteamworksOverlayStoreActivation>,
    last_game_overlay_user: Option<SteamworksOverlayUserActivation>,
    last_invite_dialog: Option<steamworks::LobbyId>,
    last_invite_connect_string: Option<String>,
    last_user_invite: Option<SteamworksUserGameInvite>,
    last_played_with: Option<steamworks::SteamId>,
    has_friend_results: Vec<SteamworksHasFriendResult>,
    friend_avatars: Vec<SteamworksFriendAvatar>,
    overlay_active: Option<bool>,
    last_persona_state_change: Option<SteamworksPersonaStateChange>,
    last_lobby_join_request: Option<SteamworksLobbyJoinRequest>,
    last_rich_presence_join_request: Option<SteamworksRichPresenceJoinRequest>,
}

impl SteamworksFriendsState {
    /// Returns the most recent synchronous error observed by the friends plugin.
    pub fn last_error(&self) -> Option<&SteamworksFriendsError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent current-user persona name read through the plugin.
    pub fn last_persona_name(&self) -> Option<&str> {
        self.last_persona_name.as_deref()
    }

    /// Returns the last friend list snapshot read through the plugin.
    pub fn friends(&self) -> &[SteamworksFriendInfo] {
        &self.friends
    }

    /// Returns the latest known snapshot for a Steam user.
    ///
    /// This cache is merged from list, coplay, and single-friend reads.
    pub fn friend(&self, steam_id: steamworks::SteamId) -> Option<&SteamworksFriendInfo> {
        self.known_friends
            .iter()
            .find(|friend| friend.steam_id == steam_id)
    }

    /// Returns all latest known friend snapshots cached by this plugin.
    pub fn known_friends(&self) -> &[SteamworksFriendInfo] {
        &self.known_friends
    }

    /// Returns the last recently-played-with snapshot read through the plugin.
    pub fn coplay_friends(&self) -> &[SteamworksCoplayFriendInfo] {
        &self.coplay_friends
    }

    /// Returns the cached coplay snapshot for a Steam user.
    pub fn coplay_friend(
        &self,
        steam_id: steamworks::SteamId,
    ) -> Option<&SteamworksCoplayFriendInfo> {
        self.coplay_friends
            .iter()
            .find(|friend| friend.friend.steam_id == steam_id)
    }

    /// Returns the most recent user information refresh request submitted through this plugin.
    pub fn last_user_information_request(&self) -> Option<&SteamworksUserInformationRequest> {
        self.last_user_information_request.as_ref()
    }

    /// Returns the most recent current-user Rich Presence mutation submitted through this plugin.
    pub fn last_rich_presence_change(&self) -> Option<&SteamworksRichPresenceChange> {
        self.last_rich_presence_change.as_ref()
    }

    /// Returns the latest Rich Presence value read for a friend/key pair.
    ///
    /// The outer `Option` distinguishes an unread key from a completed read. The inner
    /// `Option` is `None` when Steam reported no value for the key.
    pub fn friend_rich_presence(
        &self,
        steam_id: steamworks::SteamId,
        key: &str,
    ) -> Option<Option<&str>> {
        self.friend_rich_presence
            .iter()
            .find(|presence| presence.steam_id == steam_id && presence.key == key)
            .map(|presence| presence.value.as_deref())
    }

    /// Returns all friend Rich Presence reads cached by this plugin.
    pub fn friend_rich_presence_values(&self) -> &[SteamworksFriendRichPresenceValue] {
        &self.friend_rich_presence
    }

    /// Returns the most recent overlay dialog activated through this plugin.
    pub fn last_game_overlay_dialog(&self) -> Option<&str> {
        self.last_game_overlay_dialog.as_deref()
    }

    /// Returns the most recent overlay web page activated through this plugin.
    pub fn last_game_overlay_web_page(&self) -> Option<&str> {
        self.last_game_overlay_web_page.as_deref()
    }

    /// Returns the most recent overlay store activation submitted through this plugin.
    pub fn last_game_overlay_store(&self) -> Option<SteamworksOverlayStoreActivation> {
        self.last_game_overlay_store
    }

    /// Returns the most recent user-scoped overlay activation submitted through this plugin.
    pub fn last_game_overlay_user(&self) -> Option<&SteamworksOverlayUserActivation> {
        self.last_game_overlay_user.as_ref()
    }

    /// Returns the most recent lobby invite dialog activated through this plugin.
    pub fn last_invite_dialog(&self) -> Option<steamworks::LobbyId> {
        self.last_invite_dialog
    }

    /// Returns the most recent connect-string invite dialog activated through this plugin.
    pub fn last_invite_connect_string(&self) -> Option<&str> {
        self.last_invite_connect_string.as_deref()
    }

    /// Returns the most recent game invite submitted through this plugin.
    pub fn last_user_invite(&self) -> Option<&SteamworksUserGameInvite> {
        self.last_user_invite.as_ref()
    }

    /// Returns the most recent user marked as played-with through this plugin.
    pub fn last_played_with(&self) -> Option<steamworks::SteamId> {
        self.last_played_with
    }

    /// Returns the latest relationship check result for a user/flags pair.
    pub fn has_friend(
        &self,
        steam_id: steamworks::SteamId,
        flags: steamworks::FriendFlags,
    ) -> Option<bool> {
        self.has_friend_results.iter().find_map(|result| {
            (result.steam_id == steam_id && result.flags == flags).then_some(result.has_friend)
        })
    }

    /// Returns all friend relationship checks cached by this plugin.
    pub fn has_friend_results(&self) -> &[SteamworksHasFriendResult] {
        &self.has_friend_results
    }

    /// Returns the latest avatar read result for a user and size.
    ///
    /// The outer `Option` distinguishes an unread avatar from a completed read. The inner
    /// `Option` is `None` when Steam has no image available yet.
    pub fn friend_avatar(
        &self,
        steam_id: steamworks::SteamId,
        size: SteamworksAvatarSize,
    ) -> Option<Option<&SteamworksAvatar>> {
        self.friend_avatars
            .iter()
            .find(|avatar| avatar.steam_id == steam_id && avatar.size == size)
            .map(|avatar| avatar.avatar.as_ref())
    }

    /// Returns all friend avatar reads cached by this plugin.
    pub fn friend_avatars(&self) -> &[SteamworksFriendAvatar] {
        &self.friend_avatars
    }

    /// Returns the most recent Steam overlay active state reported by callback.
    pub fn overlay_active(&self) -> Option<bool> {
        self.overlay_active
    }

    /// Returns the most recent persona state change callback snapshot.
    pub fn last_persona_state_change(&self) -> Option<&SteamworksPersonaStateChange> {
        self.last_persona_state_change.as_ref()
    }

    /// Returns the most recent lobby join request callback snapshot.
    pub fn last_lobby_join_request(&self) -> Option<&SteamworksLobbyJoinRequest> {
        self.last_lobby_join_request.as_ref()
    }

    /// Returns the most recent Rich Presence join request callback snapshot.
    pub fn last_rich_presence_join_request(&self) -> Option<&SteamworksRichPresenceJoinRequest> {
        self.last_rich_presence_join_request.as_ref()
    }

    pub(super) fn record_error(&mut self, error: SteamworksFriendsError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksFriendsOperation) {
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
    }
}
