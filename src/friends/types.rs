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
    pub(super) fn dimensions(self) -> (u32, u32) {
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

/// Built-in Steam overlay dialogs for the current user.
///
/// These map to the `pchDialog` values accepted by Steam's
/// `ISteamFriends::ActivateGameOverlay` API.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksGameOverlayDialog {
    /// Open the friends list.
    Friends,
    /// Open the Steam community overlay.
    Community,
    /// Open the players dialog.
    Players,
    /// Open the Steam settings dialog.
    Settings,
    /// Open the official game group or community hub.
    OfficialGameGroup,
    /// Open the current user's stats.
    Stats,
    /// Open the current user's achievements.
    Achievements,
}

impl SteamworksGameOverlayDialog {
    /// Returns the Steam dialog string for this overlay target.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Friends => "friends",
            Self::Community => "community",
            Self::Players => "players",
            Self::Settings => "settings",
            Self::OfficialGameGroup => "officialgamegroup",
            Self::Stats => "stats",
            Self::Achievements => "achievements",
        }
    }
}

/// Steam overlay user-dialog activation snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksOverlayUserActivation {
    /// Overlay dialog name.
    pub dialog: String,
    /// Target user Steam ID.
    pub steam_id: steamworks::SteamId,
}

/// Built-in Steam overlay dialogs scoped to another Steam user or group.
///
/// These map to the `pchDialog` values accepted by Steam's
/// `ISteamFriends::ActivateGameOverlayToUser` API.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksUserOverlayDialog {
    /// Open the target user's or group's profile.
    Profile,
    /// Open chat with the target user or group.
    Chat,
    /// Open a Steam Trading session window.
    JoinTrade,
    /// Open the target user's stats.
    Stats,
    /// Open the target user's achievements.
    Achievements,
    /// Prompt to add the target user as a friend.
    FriendAdd,
    /// Prompt to remove the target friend.
    FriendRemove,
    /// Prompt to accept an incoming friend request.
    FriendRequestAccept,
    /// Prompt to ignore an incoming friend request.
    FriendRequestIgnore,
}

impl SteamworksUserOverlayDialog {
    /// Returns the Steam dialog string for this user-scoped overlay target.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Profile => "steamid",
            Self::Chat => "chat",
            Self::JoinTrade => "jointrade",
            Self::Stats => "stats",
            Self::Achievements => "achievements",
            Self::FriendAdd => "friendadd",
            Self::FriendRemove => "friendremove",
            Self::FriendRequestAccept => "friendrequestaccept",
            Self::FriendRequestIgnore => "friendrequestignore",
        }
    }
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
    pub(super) fn to_steam(self) -> steamworks::OverlayToStoreFlag {
        self.into()
    }
}

impl From<SteamworksOverlayToStoreAction> for steamworks::OverlayToStoreFlag {
    fn from(value: SteamworksOverlayToStoreAction) -> Self {
        match value {
            SteamworksOverlayToStoreAction::None => Self::None,
            SteamworksOverlayToStoreAction::AddToCart => Self::AddToCart,
            SteamworksOverlayToStoreAction::AddToCartAndShow => Self::AddToCartAndShow,
        }
    }
}
