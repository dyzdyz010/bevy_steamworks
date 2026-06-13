//! High-level Bevy ECS integration for Steam friends, Rich Presence, overlays,
//! and invites.
//!
//! This module builds on top of the upstream [`steamworks::Friends`] API. Games
//! can keep using the raw Steamworks API through [`SteamworksClient`], while this
//! plugin provides a message-driven layer for common Bevy workflows.

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksSystem};

/// Bevy plugin for high-level Steam friends, Rich Presence, overlay, and invite commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksFriendsCommand`] and [`SteamworksFriendsResult`] messages and
/// runs its command processor in [`bevy_app::First`] after Steam callbacks.
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
    coplay_friends: Vec<SteamworksCoplayFriendInfo>,
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

    /// Returns the last recently-played-with snapshot read through the plugin.
    pub fn coplay_friends(&self) -> &[SteamworksCoplayFriendInfo] {
        &self.coplay_friends
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
            }
            SteamworksFriendsOperation::CoplayFriendsListed { friends } => {
                self.coplay_friends.clone_from(friends);
            }
            _ => {}
        }
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
        /// Avatar bytes, or `None` when Steam has no image available yet.
        avatar: Option<SteamworksAvatar>,
    },
}

/// Result message emitted by [`SteamworksFriendsPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksFriendsResult {
    /// The command was submitted to Steamworks or a value was read.
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
    mut results: MessageWriter<SteamworksFriendsResult>,
) {
    let Some(client) = client else {
        let error = SteamworksFriendsError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
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
}
