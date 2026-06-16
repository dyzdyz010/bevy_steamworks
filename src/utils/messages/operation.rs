use super::super::{
    SteamworksFloatingGamepadTextInputDismissed, SteamworksFloatingGamepadTextInputShown,
    SteamworksGamepadTextInputDismissed, SteamworksGamepadTextInputShown,
    SteamworksGamepadTextInputSubmitted, SteamworksNotificationPosition, SteamworksUtilsInfo,
};

/// A successfully submitted Steam utility operation or synchronous read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksUtilsOperation {
    /// Common Steam utility information was read.
    CurrentInfoRead {
        /// Utility snapshot.
        info: SteamworksUtilsInfo,
    },
    /// Current Steam app ID was read.
    AppIdRead {
        /// Current Steam app ID.
        app_id: steamworks::AppId,
    },
    /// Steam IP country code was read.
    IpCountryRead {
        /// Country code reported by Steam.
        country: String,
    },
    /// Steam overlay enabled state was read.
    OverlayEnabledRead {
        /// Whether the Steam overlay is enabled.
        enabled: bool,
    },
    /// Steam UI language was read.
    UiLanguageRead {
        /// Current Steam client UI language.
        language: String,
    },
    /// Steam server real time was read.
    ServerRealTimeRead {
        /// Unix epoch seconds reported by Steam.
        unix_epoch_seconds: u32,
    },
    /// Big Picture mode state was read.
    SteamInBigPictureModeRead {
        /// Whether Steam and the overlay are running in Big Picture mode.
        enabled: bool,
    },
    /// Steam Deck state was read.
    SteamRunningOnSteamDeckRead {
        /// Whether Steam reports it is running on Steam Deck.
        enabled: bool,
    },
    /// Steam SDK warning callback was installed.
    WarningCallbackInstalled,
    /// Overlay notification popup position was set.
    OverlayNotificationPositionSet {
        /// Position submitted to Steam.
        position: SteamworksNotificationPosition,
    },
    /// Steam accepted or rejected a Big Picture gamepad text input request.
    GamepadTextInputShown {
        /// Show result.
        shown: SteamworksGamepadTextInputShown,
    },
    /// Steam accepted or rejected a floating gamepad text input request.
    FloatingGamepadTextInputShown {
        /// Show result.
        shown: SteamworksFloatingGamepadTextInputShown,
    },
    /// Submitted text was captured during the Steam callback.
    GamepadTextInputSubmitted {
        /// Submitted text snapshot.
        submitted: SteamworksGamepadTextInputSubmitted,
    },
    /// A gamepad text input dismissal callback was observed.
    GamepadTextInputDismissed {
        /// Callback snapshot.
        dismissed: SteamworksGamepadTextInputDismissed,
    },
    /// A floating gamepad text input dismissal callback was observed.
    FloatingGamepadTextInputDismissed {
        /// Callback snapshot.
        dismissed: SteamworksFloatingGamepadTextInputDismissed,
    },
}
