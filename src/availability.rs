use bevy_ecs::prelude::Resource;
use steamworks::{AppId, SteamAPIInitError};
use thiserror::Error;

/// How [`crate::SteamworksPlugin`] should create or locate the Steamworks client.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SteamworksInitMode {
    /// Use [`steamworks::Client::init`] and let Steam determine the app id.
    ///
    /// This requires launching through Steam or providing `steam_appid.txt`.
    #[default]
    Automatic,
    /// Force a specific Steam app id through [`steamworks::Client::init_app`].
    AppId(AppId),
    /// Do not initialize Steamworks.
    ///
    /// This is useful when another layer inserts [`crate::SteamworksClient`] manually,
    /// or for tests that only need the plugin schedules and messages.
    Manual,
}

impl SteamworksInitMode {
    /// Returns the configured Steam app id, when this mode forces one.
    pub fn app_id(self) -> Option<AppId> {
        match self {
            Self::Automatic | Self::Manual => None,
            Self::AppId(app_id) => Some(app_id),
        }
    }

    /// Returns the configured raw Steam app id, when this mode forces one.
    pub fn raw_app_id(self) -> Option<u32> {
        match self {
            Self::Automatic | Self::Manual => None,
            Self::AppId(app_id) => Some(app_id.0),
        }
    }
}

/// How the plugin reacts when Steamworks cannot be initialized.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SteamworksFailurePolicy {
    /// Panic during plugin build.
    ///
    /// This is the default so a Steam-required game cannot silently continue
    /// without Steamworks.
    #[default]
    Panic,
    /// Log the error, insert [`SteamworksUnavailable`], and keep the app running.
    LogAndContinue,
}

/// Resource inserted when Steamworks is explicitly allowed to be unavailable.
#[derive(Clone, Debug, Error, PartialEq, Eq, Resource)]
pub enum SteamworksUnavailable {
    /// Manual mode was selected, but no [`crate::SteamworksClient`] resource was present.
    #[error(
        "manual Steamworks initialization was selected, but no SteamworksClient resource exists"
    )]
    ManualClientMissing,
    /// The upstream Steamworks initialization call returned an error.
    #[error("Steamworks initialization failed in {mode:?}: {source}")]
    InitFailed {
        /// Initialization mode used for the failed attempt.
        mode: SteamworksInitMode,
        /// Error returned by `steamworks`.
        source: SteamAPIInitError,
    },
}

impl SteamworksUnavailable {
    pub(crate) fn init_failed(mode: SteamworksInitMode, source: SteamAPIInitError) -> Self {
        Self::InitFailed { mode, source }
    }

    /// Returns true when manual initialization was selected without inserting a client resource.
    pub fn is_manual_client_missing(&self) -> bool {
        matches!(self, Self::ManualClientMissing)
    }

    /// Returns true when an upstream Steamworks initialization call failed.
    pub fn is_init_failed(&self) -> bool {
        matches!(self, Self::InitFailed { .. })
    }

    /// Returns the initialization mode used for a failed Steamworks initialization call.
    pub fn init_mode(&self) -> Option<SteamworksInitMode> {
        match self {
            Self::ManualClientMissing => None,
            Self::InitFailed { mode, .. } => Some(*mode),
        }
    }

    /// Returns the configured Steam app id for a failed initialization call, when one was forced.
    pub fn app_id(&self) -> Option<AppId> {
        self.init_mode().and_then(SteamworksInitMode::app_id)
    }

    /// Returns the configured raw Steam app id for a failed initialization call, when one was forced.
    pub fn raw_app_id(&self) -> Option<u32> {
        self.init_mode().and_then(SteamworksInitMode::raw_app_id)
    }

    /// Returns the upstream Steamworks initialization error, when initialization failed.
    pub fn init_error(&self) -> Option<&SteamAPIInitError> {
        match self {
            Self::ManualClientMissing => None,
            Self::InitFailed { source, .. } => Some(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_mode_and_unavailable_accessors_expose_structured_status() {
        let app_id = AppId(480);
        let mode = SteamworksInitMode::AppId(app_id);
        let source = SteamAPIInitError::NoSteamClient("Steam is not running".to_string());
        let unavailable = SteamworksUnavailable::InitFailed {
            mode,
            source: source.clone(),
        };

        assert_eq!(SteamworksInitMode::Automatic.app_id(), None);
        assert_eq!(SteamworksInitMode::Automatic.raw_app_id(), None);
        assert_eq!(SteamworksInitMode::Manual.app_id(), None);
        assert_eq!(SteamworksInitMode::Manual.raw_app_id(), None);
        assert_eq!(mode.app_id(), Some(app_id));
        assert_eq!(mode.raw_app_id(), Some(480));

        assert!(!unavailable.is_manual_client_missing());
        assert!(unavailable.is_init_failed());
        assert_eq!(unavailable.init_mode(), Some(mode));
        assert_eq!(unavailable.app_id(), Some(app_id));
        assert_eq!(unavailable.raw_app_id(), Some(480));
        assert_eq!(unavailable.init_error(), Some(&source));
    }
}
