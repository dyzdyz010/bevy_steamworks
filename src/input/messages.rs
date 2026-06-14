use bevy_ecs::message::Message;
use thiserror::Error;

use super::*;

/// A high-level command for Steam Input workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksInputCommand {
    /// Initialize the Steam Input interface.
    Init {
        /// Whether the caller will explicitly run Steam Input frames.
        explicitly_call_run_frame: bool,
    },
    /// Synchronize Steam Input state immediately.
    RunFrame,
    /// Shut down the Steam Input interface.
    Shutdown,
    /// Read all connected controllers and their input types.
    ListControllers,
    /// Read one controller type.
    GetControllerInfo {
        /// Controller to inspect.
        controller: SteamworksInputHandle,
    },
    /// Load a local Steam Input action manifest file.
    SetActionManifestFilePath {
        /// Local path to the action manifest file.
        path: String,
    },
    /// Read an action set handle by name.
    GetActionSetHandle {
        /// Action set name from the action manifest.
        name: String,
    },
    /// Read a digital action handle by name.
    GetDigitalActionHandle {
        /// Digital action name from the action manifest.
        name: String,
    },
    /// Read an analog action handle by name.
    GetAnalogActionHandle {
        /// Analog action name from the action manifest.
        name: String,
    },
    /// Activate an action set for a controller.
    ActivateActionSet {
        /// Controller to configure.
        controller: SteamworksInputHandle,
        /// Action set to activate.
        action_set: SteamworksInputActionSetHandle,
    },
    /// Read digital action data for a controller/action pair.
    GetDigitalActionData {
        /// Controller to sample.
        controller: SteamworksInputHandle,
        /// Digital action to sample.
        action: SteamworksInputDigitalActionHandle,
    },
    /// Read analog action data for a controller/action pair.
    GetAnalogActionData {
        /// Controller to sample.
        controller: SteamworksInputHandle,
        /// Analog action to sample.
        action: SteamworksInputAnalogActionHandle,
    },
    /// Read digital action origins and their presentation strings.
    GetDigitalActionOrigins {
        /// Controller to inspect.
        controller: SteamworksInputHandle,
        /// Active action set context.
        action_set: SteamworksInputActionSetHandle,
        /// Digital action to inspect.
        action: SteamworksInputDigitalActionHandle,
    },
    /// Read analog action origins and their presentation strings.
    GetAnalogActionOrigins {
        /// Controller to inspect.
        controller: SteamworksInputHandle,
        /// Active action set context.
        action_set: SteamworksInputActionSetHandle,
        /// Analog action to inspect.
        action: SteamworksInputAnalogActionHandle,
    },
    /// Read motion data for a controller.
    GetMotionData {
        /// Controller to sample.
        controller: SteamworksInputHandle,
    },
    /// Show the Steam Input binding panel for a controller.
    ShowBindingPanel {
        /// Controller to configure.
        controller: SteamworksInputHandle,
    },
}

impl SteamworksInputCommand {
    /// Creates a [`SteamworksInputCommand::Init`] command.
    pub fn init(explicitly_call_run_frame: bool) -> Self {
        Self::Init {
            explicitly_call_run_frame,
        }
    }

    /// Creates a [`SteamworksInputCommand::SetActionManifestFilePath`] command.
    pub fn set_action_manifest_file_path(path: impl Into<String>) -> Self {
        Self::SetActionManifestFilePath { path: path.into() }
    }

    /// Creates a [`SteamworksInputCommand::GetActionSetHandle`] command.
    pub fn get_action_set_handle(name: impl Into<String>) -> Self {
        Self::GetActionSetHandle { name: name.into() }
    }

    /// Creates a [`SteamworksInputCommand::GetDigitalActionHandle`] command.
    pub fn get_digital_action_handle(name: impl Into<String>) -> Self {
        Self::GetDigitalActionHandle { name: name.into() }
    }

    /// Creates a [`SteamworksInputCommand::GetAnalogActionHandle`] command.
    pub fn get_analog_action_handle(name: impl Into<String>) -> Self {
        Self::GetAnalogActionHandle { name: name.into() }
    }

    /// Creates a [`SteamworksInputCommand::ActivateActionSet`] command.
    pub fn activate_action_set(
        controller: SteamworksInputHandle,
        action_set: SteamworksInputActionSetHandle,
    ) -> Self {
        Self::ActivateActionSet {
            controller,
            action_set,
        }
    }

    /// Creates a [`SteamworksInputCommand::GetDigitalActionData`] command.
    pub fn get_digital_action_data(
        controller: SteamworksInputHandle,
        action: SteamworksInputDigitalActionHandle,
    ) -> Self {
        Self::GetDigitalActionData { controller, action }
    }

    /// Creates a [`SteamworksInputCommand::GetAnalogActionData`] command.
    pub fn get_analog_action_data(
        controller: SteamworksInputHandle,
        action: SteamworksInputAnalogActionHandle,
    ) -> Self {
        Self::GetAnalogActionData { controller, action }
    }
}

/// A successfully submitted Steam Input operation or synchronous read.
#[derive(Clone, Debug, PartialEq)]
pub enum SteamworksInputOperation {
    /// Steam Input was initialized.
    Initialized {
        /// Whether manual Steam Input frame updates were requested.
        explicitly_call_run_frame: bool,
    },
    /// Steam Input frame state was synchronized.
    FrameRun,
    /// Steam Input was shut down.
    Shutdown,
    /// Connected controllers were listed.
    ControllersListed {
        /// Connected controllers and their input types.
        controllers: Vec<SteamworksInputControllerInfo>,
    },
    /// One controller's input type was read.
    ControllerInfoRead {
        /// Controller information.
        controller: SteamworksInputControllerInfo,
    },
    /// Action manifest path was accepted by Steam Input.
    ActionManifestFilePathSet {
        /// Local path submitted to Steam Input.
        path: String,
    },
    /// Action set handle was read.
    ActionSetHandleRead {
        /// Action set name used for lookup.
        name: String,
        /// Action set handle returned by Steam Input.
        handle: SteamworksInputActionSetHandle,
    },
    /// Digital action handle was read.
    DigitalActionHandleRead {
        /// Digital action name used for lookup.
        name: String,
        /// Digital action handle returned by Steam Input.
        handle: SteamworksInputDigitalActionHandle,
    },
    /// Analog action handle was read.
    AnalogActionHandleRead {
        /// Analog action name used for lookup.
        name: String,
        /// Analog action handle returned by Steam Input.
        handle: SteamworksInputAnalogActionHandle,
    },
    /// Action set was activated for a controller.
    ActionSetActivated {
        /// Controller configured.
        controller: SteamworksInputHandle,
        /// Action set activated.
        action_set: SteamworksInputActionSetHandle,
    },
    /// Digital action data was read.
    DigitalActionDataRead {
        /// Digital action snapshot.
        snapshot: SteamworksInputDigitalActionSnapshot,
    },
    /// Analog action data was read.
    AnalogActionDataRead {
        /// Analog action snapshot.
        snapshot: SteamworksInputAnalogActionSnapshot,
    },
    /// Digital action origins were read.
    DigitalActionOriginsRead {
        /// Controller inspected.
        controller: SteamworksInputHandle,
        /// Action set context.
        action_set: SteamworksInputActionSetHandle,
        /// Digital action inspected.
        action: SteamworksInputDigitalActionHandle,
        /// Origin presentation data.
        origins: Vec<SteamworksInputActionOriginInfo>,
    },
    /// Analog action origins were read.
    AnalogActionOriginsRead {
        /// Controller inspected.
        controller: SteamworksInputHandle,
        /// Action set context.
        action_set: SteamworksInputActionSetHandle,
        /// Analog action inspected.
        action: SteamworksInputAnalogActionHandle,
        /// Origin presentation data.
        origins: Vec<SteamworksInputActionOriginInfo>,
    },
    /// Motion data was read.
    MotionDataRead {
        /// Motion data snapshot.
        snapshot: SteamworksInputMotionSnapshot,
    },
    /// Steam Input binding panel was shown.
    BindingPanelShown {
        /// Controller configured.
        controller: SteamworksInputHandle,
    },
}

/// Result message emitted by [`crate::SteamworksInputPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksInputResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksInputOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksInputCommand,
        /// Failure reason.
        error: SteamworksInputError,
    },
}

/// Synchronous errors from [`crate::SteamworksInputPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksInputError {
    /// No [`crate::SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks Input command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A handle passed to Steam is zero.
    #[error("Steamworks Input command field {field} contains an invalid zero handle")]
    InvalidHandle {
        /// Field that contained the invalid handle.
        field: &'static str,
    },
    /// Steam Input initialization returned false.
    #[error("Steam Input initialization failed")]
    InitFailed,
    /// Steam Input rejected the action manifest path.
    #[error("Steam Input rejected the action manifest path")]
    ActionManifestFileRejected,
    /// Steam Input returned an invalid zero handle for a lookup.
    #[error("Steam Input returned an invalid zero handle for {operation}")]
    InvalidHandleReturned {
        /// Lookup operation that returned an invalid handle.
        operation: &'static str,
    },
    /// Steam Input could not show the binding panel.
    #[error("Steam Input binding panel is unavailable")]
    BindingPanelUnavailable,
}

impl SteamworksInputError {
    pub(super) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    pub(super) fn invalid_handle(field: &'static str) -> Self {
        Self::InvalidHandle { field }
    }

    pub(super) fn invalid_handle_returned(operation: &'static str) -> Self {
        Self::InvalidHandleReturned { operation }
    }
}
