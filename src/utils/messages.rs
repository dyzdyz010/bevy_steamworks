use bevy_ecs::message::Message;
use thiserror::Error;

use super::types::{
    SteamworksFloatingGamepadTextInputDismissed, SteamworksGamepadTextInputDismissed,
    SteamworksNotificationPosition, SteamworksUtilsInfo,
};

/// A high-level command for Steam utility queries and overlay helpers.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksUtilsCommand {
    /// Read a snapshot of common Steam utility information.
    GetCurrentInfo,
    /// Read the current Steam app ID.
    GetAppId,
    /// Read the country code inferred by Steam from the current user's IP address.
    GetIpCountry,
    /// Read whether the Steam overlay is enabled.
    IsOverlayEnabled,
    /// Read the current Steam client UI language.
    GetUiLanguage,
    /// Read Steam server real time as Unix epoch seconds.
    GetServerRealTime,
    /// Read whether Steam and the Steam overlay are running in Big Picture mode.
    IsSteamInBigPictureMode,
    /// Read whether Steam reports that it is running on a Steam Deck device.
    IsSteamRunningOnSteamDeck,
    /// Set where Steam overlay notification popups should appear.
    SetOverlayNotificationPosition {
        /// Popup position to submit to Steam.
        position: SteamworksNotificationPosition,
    },
}

impl SteamworksUtilsCommand {
    /// Creates a [`SteamworksUtilsCommand::SetOverlayNotificationPosition`] command.
    pub fn set_overlay_notification_position(position: SteamworksNotificationPosition) -> Self {
        Self::SetOverlayNotificationPosition { position }
    }
}

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
    /// Overlay notification popup position was set.
    OverlayNotificationPositionSet {
        /// Position submitted to Steam.
        position: SteamworksNotificationPosition,
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

/// Result message emitted by [`crate::SteamworksUtilsPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksUtilsResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksUtilsOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksUtilsCommand,
        /// Failure reason.
        error: SteamworksUtilsError,
    },
}

/// Synchronous errors from [`crate::SteamworksUtilsPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksUtilsError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
}
