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

mod accessors;
mod operations;

pub(in crate::friends) const STEAMWORKS_FRIENDS_STATE_CACHE_LIMIT: usize = 1_024;

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
