use bevy_ecs::message::Message;

use super::super::{
    SteamworksInputActionSetHandle, SteamworksInputAnalogActionHandle,
    SteamworksInputDigitalActionHandle, SteamworksInputHandle,
};

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
    /// Creates a [`crate::SteamworksInputCommand::Init`] command.
    pub fn init(explicitly_call_run_frame: bool) -> Self {
        Self::Init {
            explicitly_call_run_frame,
        }
    }

    /// Creates a [`crate::SteamworksInputCommand::RunFrame`] command.
    pub fn run_frame() -> Self {
        Self::RunFrame
    }

    /// Creates a [`crate::SteamworksInputCommand::Shutdown`] command.
    pub fn shutdown() -> Self {
        Self::Shutdown
    }

    /// Creates a [`crate::SteamworksInputCommand::ListControllers`] command.
    pub fn list_controllers() -> Self {
        Self::ListControllers
    }

    /// Creates a [`crate::SteamworksInputCommand::GetControllerInfo`] command.
    pub fn get_controller_info(controller: SteamworksInputHandle) -> Self {
        Self::GetControllerInfo { controller }
    }

    /// Creates a [`crate::SteamworksInputCommand::SetActionManifestFilePath`] command.
    pub fn set_action_manifest_file_path(path: impl Into<String>) -> Self {
        Self::SetActionManifestFilePath { path: path.into() }
    }

    /// Creates a [`crate::SteamworksInputCommand::GetActionSetHandle`] command.
    pub fn get_action_set_handle(name: impl Into<String>) -> Self {
        Self::GetActionSetHandle { name: name.into() }
    }

    /// Creates a [`crate::SteamworksInputCommand::GetDigitalActionHandle`] command.
    pub fn get_digital_action_handle(name: impl Into<String>) -> Self {
        Self::GetDigitalActionHandle { name: name.into() }
    }

    /// Creates a [`crate::SteamworksInputCommand::GetAnalogActionHandle`] command.
    pub fn get_analog_action_handle(name: impl Into<String>) -> Self {
        Self::GetAnalogActionHandle { name: name.into() }
    }

    /// Creates a [`crate::SteamworksInputCommand::ActivateActionSet`] command.
    pub fn activate_action_set(
        controller: SteamworksInputHandle,
        action_set: SteamworksInputActionSetHandle,
    ) -> Self {
        Self::ActivateActionSet {
            controller,
            action_set,
        }
    }

    /// Creates a [`crate::SteamworksInputCommand::GetDigitalActionData`] command.
    pub fn get_digital_action_data(
        controller: SteamworksInputHandle,
        action: SteamworksInputDigitalActionHandle,
    ) -> Self {
        Self::GetDigitalActionData { controller, action }
    }

    /// Creates a [`crate::SteamworksInputCommand::GetAnalogActionData`] command.
    pub fn get_analog_action_data(
        controller: SteamworksInputHandle,
        action: SteamworksInputAnalogActionHandle,
    ) -> Self {
        Self::GetAnalogActionData { controller, action }
    }

    /// Creates a [`crate::SteamworksInputCommand::GetDigitalActionOrigins`] command.
    pub fn get_digital_action_origins(
        controller: SteamworksInputHandle,
        action_set: SteamworksInputActionSetHandle,
        action: SteamworksInputDigitalActionHandle,
    ) -> Self {
        Self::GetDigitalActionOrigins {
            controller,
            action_set,
            action,
        }
    }

    /// Creates a [`crate::SteamworksInputCommand::GetAnalogActionOrigins`] command.
    pub fn get_analog_action_origins(
        controller: SteamworksInputHandle,
        action_set: SteamworksInputActionSetHandle,
        action: SteamworksInputAnalogActionHandle,
    ) -> Self {
        Self::GetAnalogActionOrigins {
            controller,
            action_set,
            action,
        }
    }

    /// Creates a [`crate::SteamworksInputCommand::GetMotionData`] command.
    pub fn get_motion_data(controller: SteamworksInputHandle) -> Self {
        Self::GetMotionData { controller }
    }

    /// Creates a [`crate::SteamworksInputCommand::ShowBindingPanel`] command.
    pub fn show_binding_panel(controller: SteamworksInputHandle) -> Self {
        Self::ShowBindingPanel { controller }
    }
}
