use std::fmt;

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

/// Big Picture gamepad text input mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksGamepadTextInputMode {
    /// Standard text entry.
    Normal,
    /// Password-style entry.
    Password,
}

impl SteamworksGamepadTextInputMode {
    pub(super) fn to_steam(self) -> steamworks::GamepadTextInputMode {
        match self {
            Self::Normal => steamworks::GamepadTextInputMode::Normal,
            Self::Password => steamworks::GamepadTextInputMode::Password,
        }
    }
}

/// Big Picture gamepad text input line mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksGamepadTextInputLineMode {
    /// Single-line text entry.
    SingleLine,
    /// Multiple-line text entry.
    MultipleLines,
}

impl SteamworksGamepadTextInputLineMode {
    pub(super) fn to_steam(self) -> steamworks::GamepadTextInputLineMode {
        match self {
            Self::SingleLine => steamworks::GamepadTextInputLineMode::SingleLine,
            Self::MultipleLines => steamworks::GamepadTextInputLineMode::MultipleLines,
        }
    }
}

/// Floating gamepad text input mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksFloatingGamepadTextInputMode {
    /// Single-line text entry.
    SingleLine,
    /// Multiple-line text entry.
    MultipleLines,
    /// Email text entry.
    Email,
    /// Numeric text entry.
    Numeric,
}

impl SteamworksFloatingGamepadTextInputMode {
    pub(super) fn to_steam(self) -> steamworks::FloatingGamepadTextInputMode {
        match self {
            Self::SingleLine => steamworks::FloatingGamepadTextInputMode::SingleLine,
            Self::MultipleLines => steamworks::FloatingGamepadTextInputMode::MultipleLines,
            Self::Email => steamworks::FloatingGamepadTextInputMode::Email,
            Self::Numeric => steamworks::FloatingGamepadTextInputMode::Numeric,
        }
    }
}

/// Request for Steam's Big Picture gamepad text input dialog.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksGamepadTextInputRequest {
    /// Text input mode.
    pub input_mode: SteamworksGamepadTextInputMode,
    /// Text input line mode.
    pub line_mode: SteamworksGamepadTextInputLineMode,
    /// Dialog description shown by Steam.
    pub description: String,
    /// Maximum number of characters Steam should accept.
    pub max_characters: u32,
    /// Optional initial text.
    pub existing_text: Option<String>,
}

impl SteamworksGamepadTextInputRequest {
    /// Creates a normal single-line request.
    pub fn new(description: impl Into<String>, max_characters: u32) -> Self {
        Self {
            input_mode: SteamworksGamepadTextInputMode::Normal,
            line_mode: SteamworksGamepadTextInputLineMode::SingleLine,
            description: description.into(),
            max_characters,
            existing_text: None,
        }
    }

    /// Sets the input mode.
    pub fn with_input_mode(mut self, input_mode: SteamworksGamepadTextInputMode) -> Self {
        self.input_mode = input_mode;
        self
    }

    /// Sets the line mode.
    pub fn with_line_mode(mut self, line_mode: SteamworksGamepadTextInputLineMode) -> Self {
        self.line_mode = line_mode;
        self
    }

    /// Sets the optional initial text.
    pub fn with_existing_text(mut self, existing_text: impl Into<String>) -> Self {
        self.existing_text = Some(existing_text.into());
        self
    }
}

impl fmt::Debug for SteamworksGamepadTextInputRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SteamworksGamepadTextInputRequest")
            .field("input_mode", &self.input_mode)
            .field("line_mode", &self.line_mode)
            .field("description", &self.description)
            .field("max_characters", &self.max_characters)
            .field(
                "existing_text_len",
                &self.existing_text.as_ref().map(String::len),
            )
            .finish()
    }
}

/// Request for Steam's floating gamepad text input overlay.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksFloatingGamepadTextInputRequest {
    /// Floating keyboard mode.
    pub input_mode: SteamworksFloatingGamepadTextInputMode,
    /// Text field x coordinate in game-window pixels.
    pub x: i32,
    /// Text field y coordinate in game-window pixels.
    pub y: i32,
    /// Text field width in pixels.
    pub width: i32,
    /// Text field height in pixels.
    pub height: i32,
}

impl SteamworksFloatingGamepadTextInputRequest {
    /// Creates a floating text input request.
    pub fn new(
        input_mode: SteamworksFloatingGamepadTextInputMode,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Self {
        Self {
            input_mode,
            x,
            y,
            width,
            height,
        }
    }
}

/// Result of showing Steam's Big Picture gamepad text input dialog.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksGamepadTextInputShown {
    /// Request submitted to Steam.
    pub request: SteamworksGamepadTextInputRequest,
    /// Whether Steam accepted the request.
    pub shown: bool,
}

/// Result of showing Steam's floating gamepad text input overlay.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksFloatingGamepadTextInputShown {
    /// Request submitted to Steam.
    pub request: SteamworksFloatingGamepadTextInputRequest,
    /// Whether Steam accepted the request.
    pub shown: bool,
}

/// Gamepad text input dismissal callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksGamepadTextInputDismissed {
    /// Submitted text length reported by Steam, or `None` when the input was cancelled.
    pub submitted_text_len: Option<u32>,
    /// Submitted text captured during Steam's original callback timing, when available.
    pub submitted_text: Option<String>,
}

/// Submitted gamepad text input captured during Steam's callback timing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksGamepadTextInputSubmitted {
    /// Submitted text.
    pub text: String,
    /// Submitted text length reported by Steam.
    pub submitted_text_len: u32,
}

/// Floating gamepad text input dismissal callback snapshot.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SteamworksFloatingGamepadTextInputDismissed;
