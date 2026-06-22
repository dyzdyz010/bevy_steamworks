use bevy_ecs::message::Message;

use super::super::{SteamworksAvatarSize, SteamworksOverlayToStoreAction};

mod constructors;

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
