use super::super::{
    SteamworksAvatar, SteamworksAvatarSize, SteamworksCoplayFriendInfo, SteamworksFriendInfo,
    SteamworksLobbyJoinRequest, SteamworksOverlayToStoreAction, SteamworksPersonaStateChange,
    SteamworksRichPresenceJoinRequest,
};

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
