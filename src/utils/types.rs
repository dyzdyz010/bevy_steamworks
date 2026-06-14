/// Snapshot of common Steam utility information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUtilsInfo {
    /// Current Steam app ID.
    pub app_id: steamworks::AppId,
    /// Country code inferred by Steam from the current user's IP address.
    pub ip_country: String,
    /// Whether the Steam overlay is enabled.
    pub overlay_enabled: bool,
    /// Current Steam client UI language.
    pub ui_language: String,
    /// Steam server real time as Unix epoch seconds.
    pub server_real_time: u32,
    /// Whether Steam and the Steam overlay are running in Big Picture mode.
    pub steam_in_big_picture_mode: bool,
    /// Whether Steam reports that it is running on a Steam Deck device.
    pub steam_running_on_steam_deck: bool,
}

/// Overlay popup position used by Steam notifications.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksNotificationPosition {
    /// Top-left corner.
    TopLeft,
    /// Top-right corner.
    TopRight,
    /// Bottom-left corner.
    BottomLeft,
    /// Bottom-right corner.
    BottomRight,
}

impl SteamworksNotificationPosition {
    pub(super) fn to_steam(self) -> steamworks::NotificationPosition {
        match self {
            Self::TopLeft => steamworks::NotificationPosition::TopLeft,
            Self::TopRight => steamworks::NotificationPosition::TopRight,
            Self::BottomLeft => steamworks::NotificationPosition::BottomLeft,
            Self::BottomRight => steamworks::NotificationPosition::BottomRight,
        }
    }
}

/// Gamepad text input dismissal callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksGamepadTextInputDismissed {
    /// Submitted text length reported by Steam, or `None` when the input was cancelled.
    ///
    /// The submitted text itself must be read inside Steam's original callback timing
    /// through the upstream `steamworks::Utils` helper.
    pub submitted_text_len: Option<u32>,
}

/// Floating gamepad text input dismissal callback snapshot.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SteamworksFloatingGamepadTextInputDismissed;
