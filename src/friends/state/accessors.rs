use super::{
    SteamworksAvatar, SteamworksAvatarSize, SteamworksCoplayFriendInfo, SteamworksFriendAvatar,
    SteamworksFriendGameInfo, SteamworksFriendInfo, SteamworksFriendRichPresenceValue,
    SteamworksFriendsError, SteamworksFriendsState, SteamworksHasFriendResult,
    SteamworksLobbyJoinRequest, SteamworksOverlayStoreActivation, SteamworksOverlayUserActivation,
    SteamworksPersonaStateChange, SteamworksRichPresenceChange, SteamworksRichPresenceJoinRequest,
    SteamworksUserGameInvite, SteamworksUserInformationRequest,
};

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

    /// Returns whether any friend snapshot is cached for a Steam user.
    pub fn has_known_friend(&self, steam_id: steamworks::SteamId) -> bool {
        self.friend(steam_id).is_some()
    }

    /// Returns the latest known display name for a Steam user.
    pub fn friend_name(&self, steam_id: steamworks::SteamId) -> Option<&str> {
        self.friend(steam_id).map(|friend| friend.name.as_str())
    }

    /// Returns the latest known nickname for a Steam user.
    ///
    /// The outer `Option` distinguishes an unknown user from a known user. The
    /// inner `Option` is `None` when the current user has no nickname set.
    pub fn friend_nickname(&self, steam_id: steamworks::SteamId) -> Option<Option<&str>> {
        self.friend(steam_id)
            .map(|friend| friend.nickname.as_deref())
    }

    /// Returns the latest known persona state for a Steam user.
    pub fn friend_state(&self, steam_id: steamworks::SteamId) -> Option<steamworks::FriendState> {
        self.friend(steam_id).map(|friend| friend.state)
    }

    /// Returns the latest known in-game snapshot for a Steam user.
    ///
    /// The outer `Option` distinguishes an unknown user from a known user. The
    /// inner `Option` is `None` when the user is known but not in a game.
    pub fn friend_game(
        &self,
        steam_id: steamworks::SteamId,
    ) -> Option<Option<&SteamworksFriendGameInfo>> {
        self.friend(steam_id).map(|friend| friend.game.as_ref())
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

    /// Returns the latest known app ID for a recently-played-with Steam user.
    pub fn coplay_app_id(&self, steam_id: steamworks::SteamId) -> Option<steamworks::AppId> {
        self.coplay_friend(steam_id)
            .map(|friend| friend.coplay_app_id)
    }

    /// Returns the latest known Unix timestamp for a recently-played-with Steam user.
    pub fn coplay_time(&self, steam_id: steamworks::SteamId) -> Option<i32> {
        self.coplay_friend(steam_id)
            .map(|friend| friend.coplay_time)
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

    /// Returns avatar dimensions for a user and size.
    ///
    /// The outer `Option` distinguishes an unread avatar from a completed read.
    /// The inner `Option` is `None` when Steam had no image available yet.
    pub fn friend_avatar_dimensions(
        &self,
        steam_id: steamworks::SteamId,
        size: SteamworksAvatarSize,
    ) -> Option<Option<(u32, u32)>> {
        self.friend_avatar(steam_id, size)
            .map(|avatar| avatar.map(|avatar| (avatar.width, avatar.height)))
    }

    /// Returns RGBA avatar bytes for a user and size.
    ///
    /// The outer `Option` distinguishes an unread avatar from a completed read.
    /// The inner `Option` is `None` when Steam had no image available yet.
    pub fn friend_avatar_rgba(
        &self,
        steam_id: steamworks::SteamId,
        size: SteamworksAvatarSize,
    ) -> Option<Option<&[u8]>> {
        self.friend_avatar(steam_id, size)
            .map(|avatar| avatar.map(|avatar| avatar.rgba.as_slice()))
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
}
