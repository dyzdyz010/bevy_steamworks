use super::{
    messages::{SteamworksInputCommand, SteamworksInputError},
    types::{
        SteamworksInputActionSetHandle, SteamworksInputAnalogActionHandle,
        SteamworksInputDigitalActionHandle, SteamworksInputHandle,
    },
};

pub(super) fn validate_command(
    command: &SteamworksInputCommand,
) -> Result<(), SteamworksInputError> {
    match command {
        SteamworksInputCommand::SetActionManifestFilePath { path } => {
            validate_steam_string("path", path)
        }
        SteamworksInputCommand::GetActionSetHandle { name } => validate_steam_string("name", name),
        SteamworksInputCommand::GetDigitalActionHandle { name } => {
            validate_steam_string("name", name)
        }
        SteamworksInputCommand::GetAnalogActionHandle { name } => {
            validate_steam_string("name", name)
        }
        SteamworksInputCommand::GetControllerInfo { controller }
        | SteamworksInputCommand::GetMotionData { controller }
        | SteamworksInputCommand::ShowBindingPanel { controller } => {
            validate_input_handle("controller", *controller)
        }
        SteamworksInputCommand::ActivateActionSet {
            controller,
            action_set,
        } => {
            validate_input_handle("controller", *controller)?;
            validate_action_set_handle("action_set", *action_set)
        }
        SteamworksInputCommand::GetDigitalActionData { controller, action } => {
            validate_input_handle("controller", *controller)?;
            validate_digital_action_handle("action", *action)
        }
        SteamworksInputCommand::GetAnalogActionData { controller, action } => {
            validate_input_handle("controller", *controller)?;
            validate_analog_action_handle("action", *action)
        }
        SteamworksInputCommand::GetDigitalActionOrigins {
            controller,
            action_set,
            action,
        } => {
            validate_input_handle("controller", *controller)?;
            validate_action_set_handle("action_set", *action_set)?;
            validate_digital_action_handle("action", *action)
        }
        SteamworksInputCommand::GetAnalogActionOrigins {
            controller,
            action_set,
            action,
        } => {
            validate_input_handle("controller", *controller)?;
            validate_action_set_handle("action_set", *action_set)?;
            validate_analog_action_handle("action", *action)
        }
        SteamworksInputCommand::Init { .. }
        | SteamworksInputCommand::RunFrame
        | SteamworksInputCommand::Shutdown
        | SteamworksInputCommand::ListControllers => Ok(()),
    }
}

fn validate_steam_string(field: &'static str, value: &str) -> Result<(), SteamworksInputError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksInputError::invalid_string(field))
    } else {
        Ok(())
    }
}

fn validate_input_handle(
    field: &'static str,
    handle: SteamworksInputHandle,
) -> Result<(), SteamworksInputError> {
    if handle.is_valid() {
        Ok(())
    } else {
        Err(SteamworksInputError::invalid_handle(field))
    }
}

fn validate_action_set_handle(
    field: &'static str,
    handle: SteamworksInputActionSetHandle,
) -> Result<(), SteamworksInputError> {
    if handle.is_valid() {
        Ok(())
    } else {
        Err(SteamworksInputError::invalid_handle(field))
    }
}

fn validate_digital_action_handle(
    field: &'static str,
    handle: SteamworksInputDigitalActionHandle,
) -> Result<(), SteamworksInputError> {
    if handle.is_valid() {
        Ok(())
    } else {
        Err(SteamworksInputError::invalid_handle(field))
    }
}

fn validate_analog_action_handle(
    field: &'static str,
    handle: SteamworksInputAnalogActionHandle,
) -> Result<(), SteamworksInputError> {
    if handle.is_valid() {
        Ok(())
    } else {
        Err(SteamworksInputError::invalid_handle(field))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_rejects_interior_nul() {
        let command = SteamworksInputCommand::set_action_manifest_file_path("input\0manifest");

        assert_eq!(
            validate_command(&command),
            Err(SteamworksInputError::InvalidString { field: "path" })
        );

        let command = SteamworksInputCommand::get_digital_action_handle("jump\0bad");

        assert_eq!(
            validate_command(&command),
            Err(SteamworksInputError::InvalidString { field: "name" })
        );
    }

    #[test]
    fn validation_rejects_zero_handles() {
        let command = SteamworksInputCommand::activate_action_set(
            SteamworksInputHandle::from_raw(0),
            SteamworksInputActionSetHandle::from_raw(7),
        );

        assert_eq!(
            validate_command(&command),
            Err(SteamworksInputError::InvalidHandle {
                field: "controller",
            })
        );

        let command = SteamworksInputCommand::get_analog_action_data(
            SteamworksInputHandle::from_raw(7),
            SteamworksInputAnalogActionHandle::from_raw(0),
        );

        assert_eq!(
            validate_command(&command),
            Err(SteamworksInputError::InvalidHandle { field: "action" })
        );
    }
}
