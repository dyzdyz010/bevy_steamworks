//! High-level Bevy ECS integration for Steam friends, Rich Presence, overlays,
//! and invites.
//!
//! This module builds on top of the upstream [`steamworks::Friends`] API. Games
//! can keep using the raw Steamworks API through [`SteamworksClient`], while this
//! plugin provides a message-driven layer for common Bevy workflows.

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

/// Bevy plugin for high-level Steam friends, Rich Presence, overlay, and invite commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksFriendsCommand`] and [`SteamworksFriendsResult`] messages and
/// runs its command processor in [`bevy_app::First`] after Steam callbacks. It
/// also mirrors common friends, overlay, and invite callbacks into friends results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksFriendsPlugin;

impl SteamworksFriendsPlugin {
    /// Creates a friends plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksFriendsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksFriendsState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksFriendsCommand>()
            .add_message::<SteamworksFriendsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessFriendsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_friends_commands.in_set(SteamworksSystem::ProcessFriendsCommands),
            );
    }
}

/// Runtime state for [`SteamworksFriendsPlugin`].
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

    fn record_error(&mut self, error: SteamworksFriendsError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksFriendsOperation) {
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

/// A snapshot of a Steam friend suitable for storing in ECS messages/resources.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksFriendInfo {
    /// Friend Steam ID.
    pub steam_id: steamworks::SteamId,
    /// Friend display name.
    pub name: String,
    /// Nickname set by the current user, if one exists.
    pub nickname: Option<String>,
    /// Current persona state.
    pub state: steamworks::FriendState,
    /// Current game information, if the friend is in a game.
    pub game: Option<SteamworksFriendGameInfo>,
}

/// A snapshot of a recently-played-with Steam friend.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksCoplayFriendInfo {
    /// Basic friend information.
    pub friend: SteamworksFriendInfo,
    /// App ID of the game played with this user.
    pub coplay_app_id: steamworks::AppId,
    /// Unix timestamp reported by Steam for the coplay entry.
    pub coplay_time: i32,
}

/// A snapshot of a friend's current game.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksFriendGameInfo {
    /// Game ID reported by Steam.
    pub game: steamworks::GameId,
    /// IPv4 address of the server the friend is in.
    pub game_address: std::net::Ipv4Addr,
    /// Game port of the server the friend is in.
    pub game_port: u16,
    /// Query port of the server the friend is in.
    pub query_port: u16,
    /// Lobby ID the friend is in, when one is reported by Steam.
    pub lobby: steamworks::LobbyId,
}

/// Persona state change callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksPersonaStateChange {
    /// Steam user whose persona state changed.
    pub steam_id: steamworks::SteamId,
    /// Changed persona fields reported by Steam.
    pub flags: steamworks::PersonaChange,
}

/// Lobby join request callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksLobbyJoinRequest {
    /// Lobby Steam ID to join.
    pub lobby: steamworks::LobbyId,
    /// Friend that triggered the request.
    pub friend_steam_id: steamworks::SteamId,
}

/// Rich Presence join request callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksRichPresenceJoinRequest {
    /// Friend that triggered the request, or an invalid Steam ID for non-friend sources.
    pub friend_steam_id: steamworks::SteamId,
    /// Connect string supplied through Rich Presence.
    pub connect: String,
}

impl From<steamworks::FriendGame> for SteamworksFriendGameInfo {
    fn from(game: steamworks::FriendGame) -> Self {
        Self {
            game: game.game,
            game_address: game.game_address,
            game_port: game.game_port,
            query_port: game.query_port,
            lobby: game.lobby,
        }
    }
}

/// Friend avatar size to read from Steam.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksAvatarSize {
    /// Small 32x32 RGBA avatar.
    Small,
    /// Medium 64x64 RGBA avatar.
    Medium,
    /// Large 184x184 RGBA avatar.
    Large,
}

impl SteamworksAvatarSize {
    fn dimensions(self) -> (u32, u32) {
        match self {
            Self::Small => (32, 32),
            Self::Medium => (64, 64),
            Self::Large => (184, 184),
        }
    }
}

/// RGBA avatar bytes returned by Steam.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksAvatar {
    /// Avatar size requested.
    pub size: SteamworksAvatarSize,
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// RGBA pixel bytes.
    pub rgba: Vec<u8>,
}

/// User information refresh request snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SteamworksUserInformationRequest {
    /// User Steam ID.
    pub steam_id: steamworks::SteamId,
    /// Whether only the persona name was requested.
    pub name_only: bool,
    /// Whether Steam reported that an asynchronous refresh is needed.
    pub requested: bool,
}

/// Current-user Rich Presence mutation submitted through this plugin.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksRichPresenceChange {
    /// One key was set or cleared.
    Key {
        /// Rich Presence key.
        key: String,
        /// Whether the key was cleared.
        cleared: bool,
    },
    /// All current-user Rich Presence key/value pairs were cleared.
    ClearedAll,
}

/// Rich Presence value read for a friend/key pair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksFriendRichPresenceValue {
    /// Friend Steam ID.
    pub steam_id: steamworks::SteamId,
    /// Rich Presence key.
    pub key: String,
    /// Value reported by Steam, or `None` when the key is absent.
    pub value: Option<String>,
}

/// Steam overlay store activation snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SteamworksOverlayStoreActivation {
    /// Steam app ID opened.
    pub app_id: steamworks::AppId,
    /// Store overlay behavior.
    pub action: SteamworksOverlayToStoreAction,
}

/// Steam overlay user-dialog activation snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksOverlayUserActivation {
    /// Overlay dialog name.
    pub dialog: String,
    /// Target user Steam ID.
    pub steam_id: steamworks::SteamId,
}

/// Game invite submitted to a Steam user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUserGameInvite {
    /// Target user Steam ID.
    pub steam_id: steamworks::SteamId,
    /// Connect string sent to the target user.
    pub connect: String,
}

/// Friend relationship check snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SteamworksHasFriendResult {
    /// Target user Steam ID.
    pub steam_id: steamworks::SteamId,
    /// Friend flags tested.
    pub flags: steamworks::FriendFlags,
    /// Whether the relationship matched.
    pub has_friend: bool,
}

/// Friend avatar read snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksFriendAvatar {
    /// Friend Steam ID.
    pub steam_id: steamworks::SteamId,
    /// Requested avatar size.
    pub size: SteamworksAvatarSize,
    /// Avatar bytes, or `None` when Steam has no image available yet.
    pub avatar: Option<SteamworksAvatar>,
}

/// How the Steam overlay should handle a store page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksOverlayToStoreAction {
    /// Open the store page without cart behavior.
    None,
    /// Add the app to the user's cart.
    AddToCart,
    /// Add the app to the user's cart and show the cart.
    AddToCartAndShow,
}

impl SteamworksOverlayToStoreAction {
    fn to_steam(self) -> steamworks::OverlayToStoreFlag {
        match self {
            Self::None => steamworks::OverlayToStoreFlag::None,
            Self::AddToCart => steamworks::OverlayToStoreFlag::AddToCart,
            Self::AddToCartAndShow => steamworks::OverlayToStoreFlag::AddToCartAndShow,
        }
    }
}

/// A high-level command for Steam friends, Rich Presence, overlays, and invites.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksFriendsCommand {
    /// Read the current user's Steam persona name.
    GetPersonaName,
    /// Read a friend list snapshot using Steam friend flags.
    ListFriends {
        /// Friend relationship flags to include.
        flags: steamworks::FriendFlags,
    },
    /// Read Steam's recently-played-with list.
    ListCoplayFriends,
    /// Read one friend's current information.
    GetFriend {
        /// Friend Steam ID.
        steam_id: steamworks::SteamId,
    },
    /// Request that Steam refresh a user's persona data.
    RequestUserInformation {
        /// User Steam ID.
        steam_id: steamworks::SteamId,
        /// Whether only the name is needed.
        name_only: bool,
    },
    /// Set or clear one Rich Presence key.
    SetRichPresence {
        /// Rich Presence key.
        key: String,
        /// Rich Presence value. `None` or an empty string clears the key.
        value: Option<String>,
    },
    /// Clear all current-user Rich Presence key/value pairs.
    ClearRichPresence,
    /// Read a Rich Presence value from a friend.
    GetFriendRichPresence {
        /// Friend Steam ID.
        steam_id: steamworks::SteamId,
        /// Rich Presence key.
        key: String,
    },
    /// Open a Steam overlay dialog such as `"Friends"`, `"Community"`, or `"Settings"`.
    ActivateGameOverlay {
        /// Overlay dialog name.
        dialog: String,
    },
    /// Open a URL inside the Steam overlay.
    ActivateGameOverlayToWebPage {
        /// URL to open.
        url: String,
    },
    /// Open an app's Steam store page inside the overlay.
    ActivateGameOverlayToStore {
        /// Steam app ID to show.
        app_id: steamworks::AppId,
        /// Store overlay behavior.
        action: SteamworksOverlayToStoreAction,
    },
    /// Open an overlay dialog scoped to a Steam user.
    ActivateGameOverlayToUser {
        /// Overlay dialog name.
        dialog: String,
        /// Target user Steam ID.
        steam_id: steamworks::SteamId,
    },
    /// Open the Steam lobby invite dialog.
    ActivateInviteDialog {
        /// Lobby to invite friends into.
        lobby: steamworks::LobbyId,
    },
    /// Open the Steam invite dialog using a Rich Presence connect string.
    ActivateInviteDialogConnectString {
        /// Connect string sent through Rich Presence.
        connect: String,
    },
    /// Invite a friend or clan member to the current game.
    InviteUserToGame {
        /// Target user Steam ID.
        steam_id: steamworks::SteamId,
        /// Connect string sent to the target user.
        connect: String,
    },
    /// Mark a Steam user as played with.
    SetPlayedWith {
        /// Target user Steam ID.
        steam_id: steamworks::SteamId,
    },
    /// Check whether a user matches the supplied friend flags.
    HasFriend {
        /// Target user Steam ID.
        steam_id: steamworks::SteamId,
        /// Friend flags to test.
        flags: steamworks::FriendFlags,
    },
    /// Read a friend's avatar bytes.
    GetFriendAvatar {
        /// Friend Steam ID.
        steam_id: steamworks::SteamId,
        /// Avatar size.
        size: SteamworksAvatarSize,
    },
}

impl SteamworksFriendsCommand {
    /// Creates a [`SteamworksFriendsCommand::ListFriends`] command.
    pub fn list_friends(flags: steamworks::FriendFlags) -> Self {
        Self::ListFriends { flags }
    }

    /// Creates a [`SteamworksFriendsCommand::GetFriend`] command.
    pub fn get_friend(steam_id: steamworks::SteamId) -> Self {
        Self::GetFriend { steam_id }
    }

    /// Creates a [`SteamworksFriendsCommand::RequestUserInformation`] command.
    pub fn request_user_information(steam_id: steamworks::SteamId, name_only: bool) -> Self {
        Self::RequestUserInformation {
            steam_id,
            name_only,
        }
    }

    /// Creates a [`SteamworksFriendsCommand::SetRichPresence`] command.
    pub fn set_rich_presence(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::SetRichPresence {
            key: key.into(),
            value: Some(value.into()),
        }
    }

    /// Creates a [`SteamworksFriendsCommand::SetRichPresence`] command that clears one key.
    pub fn clear_rich_presence_key(key: impl Into<String>) -> Self {
        Self::SetRichPresence {
            key: key.into(),
            value: None,
        }
    }

    /// Creates a [`SteamworksFriendsCommand::GetFriendRichPresence`] command.
    pub fn get_friend_rich_presence(steam_id: steamworks::SteamId, key: impl Into<String>) -> Self {
        Self::GetFriendRichPresence {
            steam_id,
            key: key.into(),
        }
    }

    /// Creates a [`SteamworksFriendsCommand::ActivateGameOverlay`] command.
    pub fn activate_game_overlay(dialog: impl Into<String>) -> Self {
        Self::ActivateGameOverlay {
            dialog: dialog.into(),
        }
    }

    /// Creates a [`SteamworksFriendsCommand::ActivateGameOverlayToWebPage`] command.
    pub fn activate_game_overlay_to_web_page(url: impl Into<String>) -> Self {
        Self::ActivateGameOverlayToWebPage { url: url.into() }
    }

    /// Creates a [`SteamworksFriendsCommand::ActivateGameOverlayToStore`] command.
    pub fn activate_game_overlay_to_store(
        app_id: impl Into<steamworks::AppId>,
        action: SteamworksOverlayToStoreAction,
    ) -> Self {
        Self::ActivateGameOverlayToStore {
            app_id: app_id.into(),
            action,
        }
    }

    /// Creates a [`SteamworksFriendsCommand::ActivateGameOverlayToUser`] command.
    pub fn activate_game_overlay_to_user(
        dialog: impl Into<String>,
        steam_id: steamworks::SteamId,
    ) -> Self {
        Self::ActivateGameOverlayToUser {
            dialog: dialog.into(),
            steam_id,
        }
    }

    /// Creates a [`SteamworksFriendsCommand::ActivateInviteDialog`] command.
    pub fn activate_invite_dialog(lobby: steamworks::LobbyId) -> Self {
        Self::ActivateInviteDialog { lobby }
    }

    /// Creates a [`SteamworksFriendsCommand::ActivateInviteDialogConnectString`] command.
    pub fn activate_invite_dialog_connect_string(connect: impl Into<String>) -> Self {
        Self::ActivateInviteDialogConnectString {
            connect: connect.into(),
        }
    }

    /// Creates a [`SteamworksFriendsCommand::InviteUserToGame`] command.
    pub fn invite_user_to_game(steam_id: steamworks::SteamId, connect: impl Into<String>) -> Self {
        Self::InviteUserToGame {
            steam_id,
            connect: connect.into(),
        }
    }

    /// Creates a [`SteamworksFriendsCommand::SetPlayedWith`] command.
    pub fn set_played_with(steam_id: steamworks::SteamId) -> Self {
        Self::SetPlayedWith { steam_id }
    }

    /// Creates a [`SteamworksFriendsCommand::HasFriend`] command.
    pub fn has_friend(steam_id: steamworks::SteamId, flags: steamworks::FriendFlags) -> Self {
        Self::HasFriend { steam_id, flags }
    }

    /// Creates a [`SteamworksFriendsCommand::GetFriendAvatar`] command.
    pub fn get_friend_avatar(steam_id: steamworks::SteamId, size: SteamworksAvatarSize) -> Self {
        Self::GetFriendAvatar { steam_id, size }
    }
}

/// A successfully submitted Steam friends operation or synchronous read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksFriendsOperation {
    /// Current-user persona name was read.
    PersonaNameRead {
        /// Persona name.
        name: String,
    },
    /// Friend list snapshot was read.
    FriendsListed {
        /// Friend relationship flags used for the snapshot.
        flags: steamworks::FriendFlags,
        /// Friend snapshots.
        friends: Vec<SteamworksFriendInfo>,
    },
    /// Recently-played-with snapshot was read.
    CoplayFriendsListed {
        /// Recently-played-with snapshots.
        friends: Vec<SteamworksCoplayFriendInfo>,
    },
    /// One friend snapshot was read.
    FriendRead {
        /// Friend snapshot.
        friend: SteamworksFriendInfo,
    },
    /// A user information refresh was requested.
    UserInformationRequested {
        /// User Steam ID.
        steam_id: steamworks::SteamId,
        /// Whether only the name was requested.
        name_only: bool,
        /// Whether Steam reported that an async refresh is needed.
        requested: bool,
    },
    /// One Rich Presence key was set or cleared.
    RichPresenceSet {
        /// Rich Presence key.
        key: String,
        /// Whether the key was cleared.
        cleared: bool,
    },
    /// All current-user Rich Presence key/value pairs were cleared.
    RichPresenceCleared,
    /// A friend's Rich Presence value was read.
    FriendRichPresenceRead {
        /// Friend Steam ID.
        steam_id: steamworks::SteamId,
        /// Rich Presence key.
        key: String,
        /// Rich Presence value.
        value: Option<String>,
    },
    /// A Steam overlay dialog was activated.
    GameOverlayActivated {
        /// Overlay dialog name.
        dialog: String,
    },
    /// A Steam overlay web page was activated.
    GameOverlayToWebPageActivated {
        /// URL opened.
        url: String,
    },
    /// A Steam overlay store page was activated.
    GameOverlayToStoreActivated {
        /// Steam app ID opened.
        app_id: steamworks::AppId,
        /// Store overlay behavior.
        action: SteamworksOverlayToStoreAction,
    },
    /// A Steam overlay user dialog was activated.
    GameOverlayToUserActivated {
        /// Overlay dialog name.
        dialog: String,
        /// Target user Steam ID.
        steam_id: steamworks::SteamId,
    },
    /// The Steam lobby invite dialog was activated.
    InviteDialogActivated {
        /// Lobby ID.
        lobby: steamworks::LobbyId,
    },
    /// The Steam connect-string invite dialog was activated.
    InviteDialogConnectStringActivated {
        /// Connect string sent through Rich Presence.
        connect: String,
    },
    /// A game invite was submitted to Steam.
    UserInvitedToGame {
        /// Target user Steam ID.
        steam_id: steamworks::SteamId,
        /// Connect string sent to the target user.
        connect: String,
    },
    /// A user was marked as played with.
    PlayedWithSet {
        /// Target user Steam ID.
        steam_id: steamworks::SteamId,
    },
    /// A friend relationship check was read.
    HasFriendRead {
        /// Target user Steam ID.
        steam_id: steamworks::SteamId,
        /// Friend flags tested.
        flags: steamworks::FriendFlags,
        /// Whether the relationship matched.
        has_friend: bool,
    },
    /// A friend avatar was read.
    FriendAvatarRead {
        /// Friend Steam ID.
        steam_id: steamworks::SteamId,
        /// Avatar size requested.
        size: SteamworksAvatarSize,
        /// Avatar bytes, or `None` when Steam has no image available yet.
        avatar: Option<SteamworksAvatar>,
    },
    /// Steam overlay activation state changed.
    GameOverlayActivationChanged {
        /// Whether the Steam overlay is active.
        active: bool,
    },
    /// A persona state change callback was observed.
    PersonaStateChanged {
        /// Callback snapshot.
        change: SteamworksPersonaStateChange,
    },
    /// A lobby join request callback was observed.
    GameLobbyJoinRequested {
        /// Callback snapshot.
        request: SteamworksLobbyJoinRequest,
    },
    /// A Rich Presence join request callback was observed.
    GameRichPresenceJoinRequested {
        /// Callback snapshot.
        request: SteamworksRichPresenceJoinRequest,
    },
}

/// Result message emitted by [`SteamworksFriendsPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksFriendsResult {
    /// The command or observed callback was processed successfully.
    Ok(SteamworksFriendsOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksFriendsCommand,
        /// Failure reason.
        error: SteamworksFriendsError,
    },
}

/// Synchronous errors from [`SteamworksFriendsPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksFriendsError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks friends command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// The upstream Steamworks API rejected the operation.
    #[error("Steamworks friends operation failed: {operation}")]
    OperationFailed {
        /// Operation that failed.
        operation: &'static str,
    },
}

impl SteamworksFriendsError {
    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }
}

fn process_friends_commands(
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

fn process_friends_steam_events(
    state: &mut SteamworksFriendsState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksFriendsResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::GameOverlayActivated(event) => {
                SteamworksFriendsOperation::GameOverlayActivationChanged {
                    active: event.active,
                }
            }
            SteamworksEvent::PersonaStateChange(event) => {
                SteamworksFriendsOperation::PersonaStateChanged {
                    change: SteamworksPersonaStateChange {
                        steam_id: event.steam_id,
                        flags: event.flags,
                    },
                }
            }
            SteamworksEvent::GameLobbyJoinRequested(event) => {
                SteamworksFriendsOperation::GameLobbyJoinRequested {
                    request: SteamworksLobbyJoinRequest {
                        lobby: event.lobby_steam_id,
                        friend_steam_id: event.friend_steam_id,
                    },
                }
            }
            SteamworksEvent::GameRichPresenceJoinRequested(event) => {
                SteamworksFriendsOperation::GameRichPresenceJoinRequested {
                    request: SteamworksRichPresenceJoinRequest {
                        friend_steam_id: event.friend_steam_id,
                        connect: event.connect.clone(),
                    },
                }
            }
            _ => continue,
        };

        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks friends callback"
        );
        results.write(SteamworksFriendsResult::Ok(operation));
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

fn validate_command_strings(
    command: &SteamworksFriendsCommand,
) -> Result<(), SteamworksFriendsError> {
    match command {
        SteamworksFriendsCommand::SetRichPresence { key, value } => {
            validate_steam_string("key", key)?;
            if let Some(value) = value {
                validate_steam_string("value", value)?;
            }
        }
        SteamworksFriendsCommand::GetFriendRichPresence { key, .. } => {
            validate_steam_string("key", key)?;
        }
        SteamworksFriendsCommand::ActivateGameOverlay { dialog } => {
            validate_steam_string("dialog", dialog)?;
        }
        SteamworksFriendsCommand::ActivateGameOverlayToWebPage { url } => {
            validate_steam_string("url", url)?;
        }
        SteamworksFriendsCommand::ActivateGameOverlayToUser { dialog, .. } => {
            validate_steam_string("dialog", dialog)?;
        }
        SteamworksFriendsCommand::ActivateInviteDialogConnectString { connect }
        | SteamworksFriendsCommand::InviteUserToGame { connect, .. } => {
            validate_steam_string("connect", connect)?;
        }
        _ => {}
    }

    Ok(())
}

fn validate_steam_string(field: &'static str, value: &str) -> Result<(), SteamworksFriendsError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksFriendsError::invalid_string(field))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::message::Messages;

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
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksFriendsPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksFriendsCommand>>()
            .write(SteamworksFriendsCommand::GetPersonaName);

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
    fn string_validation_rejects_interior_nul() {
        let command = SteamworksFriendsCommand::SetRichPresence {
            key: "status\0bad".to_owned(),
            value: Some("ok".to_owned()),
        };

        assert_eq!(
            validate_command_strings(&command),
            Err(SteamworksFriendsError::InvalidString { field: "key" })
        );

        let command = SteamworksFriendsCommand::invite_user_to_game(
            steamworks::SteamId::from_raw(1),
            "join\0bad",
        );

        assert_eq!(
            validate_command_strings(&command),
            Err(SteamworksFriendsError::InvalidString { field: "connect" })
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
                SteamworksFriendsResult::Ok(
                    SteamworksFriendsOperation::GameOverlayActivationChanged { active: true },
                ),
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
}
