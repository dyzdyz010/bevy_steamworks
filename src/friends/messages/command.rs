use bevy_ecs::message::Message;

use super::super::{SteamworksAvatarSize, SteamworksOverlayToStoreAction};

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
