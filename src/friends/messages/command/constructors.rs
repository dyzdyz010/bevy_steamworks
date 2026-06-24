use super::super::super::{
    SteamworksAvatarSize, SteamworksGameOverlayDialog, SteamworksOverlayToStoreAction,
    SteamworksUserOverlayDialog,
};
use super::SteamworksFriendsCommand;

impl SteamworksFriendsCommand {
    /// Creates a [`SteamworksFriendsCommand::GetPersonaName`] command.
    pub fn get_persona_name() -> Self {
        Self::GetPersonaName
    }

    /// Creates a [`SteamworksFriendsCommand::ListFriends`] command.
    pub fn list_friends(flags: steamworks::FriendFlags) -> Self {
        Self::ListFriends { flags }
    }

    /// Creates a [`SteamworksFriendsCommand::ListCoplayFriends`] command.
    pub fn list_coplay_friends() -> Self {
        Self::ListCoplayFriends
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

    /// Creates a [`SteamworksFriendsCommand::ClearRichPresence`] command.
    pub fn clear_rich_presence() -> Self {
        Self::ClearRichPresence
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

    /// Creates a [`SteamworksFriendsCommand::ActivateGameOverlay`] command from
    /// one of Steam's built-in overlay dialog targets.
    pub fn activate_game_overlay_dialog(dialog: SteamworksGameOverlayDialog) -> Self {
        Self::ActivateGameOverlay {
            dialog: dialog.as_str().to_owned(),
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

    /// Creates a [`SteamworksFriendsCommand::ActivateGameOverlayToUser`] command
    /// from one of Steam's built-in user-scoped overlay dialog targets.
    pub fn activate_game_overlay_to_user_dialog(
        dialog: SteamworksUserOverlayDialog,
        steam_id: steamworks::SteamId,
    ) -> Self {
        Self::ActivateGameOverlayToUser {
            dialog: dialog.as_str().to_owned(),
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
