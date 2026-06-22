use super::super::super::{
    SteamworksInputActionSetHandle, SteamworksInputAnalogActionHandle,
    SteamworksInputDigitalActionHandle, SteamworksInputHandle,
};
use super::SteamworksInputCommand;

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
