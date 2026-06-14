use bevy_ecs::{
    message::{MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::SteamworksClient;

use super::{
    messages::{
        SteamworksInputCommand, SteamworksInputError, SteamworksInputOperation,
        SteamworksInputResult,
    },
    state::SteamworksInputState,
    types::{
        SteamworksInputActionOrigin, SteamworksInputActionOriginInfo,
        SteamworksInputActionSetHandle, SteamworksInputAnalogActionData,
        SteamworksInputAnalogActionHandle, SteamworksInputAnalogActionSnapshot,
        SteamworksInputControllerInfo, SteamworksInputDigitalActionData,
        SteamworksInputDigitalActionHandle, SteamworksInputDigitalActionSnapshot,
        SteamworksInputHandle, SteamworksInputMotionData, SteamworksInputMotionSnapshot,
        SteamworksInputSourceMode, SteamworksInputType,
    },
    validation::validate_command,
};

const STEAM_INPUT_MAX_CONTROLLER_COUNT: usize = 16;

pub(super) fn process_input_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksInputState>,
    mut commands: ResMut<Messages<SteamworksInputCommand>>,
    mut results: MessageWriter<SteamworksInputResult>,
) {
    let Some(client) = client else {
        let error = SteamworksInputError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks Input command failed"
            );
            results.write(SteamworksInputResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    let input = client.input();
    for command in commands.drain() {
        match handle_input_command(&input, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks Input command"
                );
                results.write(SteamworksInputResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks Input command failed"
                );
                results.write(SteamworksInputResult::Err { command, error });
            }
        }
    }
}

fn handle_input_command(
    input: &steamworks::Input,
    command: &SteamworksInputCommand,
) -> Result<SteamworksInputOperation, SteamworksInputError> {
    validate_command(command)?;

    match command {
        SteamworksInputCommand::Init {
            explicitly_call_run_frame,
        } => {
            if input.init(*explicitly_call_run_frame) {
                Ok(SteamworksInputOperation::Initialized {
                    explicitly_call_run_frame: *explicitly_call_run_frame,
                })
            } else {
                Err(SteamworksInputError::InitFailed)
            }
        }
        SteamworksInputCommand::RunFrame => {
            input.run_frame();
            Ok(SteamworksInputOperation::FrameRun)
        }
        SteamworksInputCommand::Shutdown => {
            input.shutdown();
            Ok(SteamworksInputOperation::Shutdown)
        }
        SteamworksInputCommand::ListControllers => {
            let controllers = snapshot_connected_controllers(input);
            Ok(SteamworksInputOperation::ControllersListed { controllers })
        }
        SteamworksInputCommand::GetControllerInfo { controller } => {
            Ok(SteamworksInputOperation::ControllerInfoRead {
                controller: snapshot_controller_info(input, *controller),
            })
        }
        SteamworksInputCommand::SetActionManifestFilePath { path } => {
            if input.set_input_action_manifest_file_path(path) {
                Ok(SteamworksInputOperation::ActionManifestFilePathSet { path: path.clone() })
            } else {
                Err(SteamworksInputError::ActionManifestFileRejected)
            }
        }
        SteamworksInputCommand::GetActionSetHandle { name } => {
            let handle =
                SteamworksInputActionSetHandle::from_raw(input.get_action_set_handle(name));
            if handle.is_valid() {
                Ok(SteamworksInputOperation::ActionSetHandleRead {
                    name: name.clone(),
                    handle,
                })
            } else {
                Err(SteamworksInputError::invalid_handle_returned(
                    "GetActionSetHandle",
                ))
            }
        }
        SteamworksInputCommand::GetDigitalActionHandle { name } => {
            let handle =
                SteamworksInputDigitalActionHandle::from_raw(input.get_digital_action_handle(name));
            if handle.is_valid() {
                Ok(SteamworksInputOperation::DigitalActionHandleRead {
                    name: name.clone(),
                    handle,
                })
            } else {
                Err(SteamworksInputError::invalid_handle_returned(
                    "GetDigitalActionHandle",
                ))
            }
        }
        SteamworksInputCommand::GetAnalogActionHandle { name } => {
            let handle =
                SteamworksInputAnalogActionHandle::from_raw(input.get_analog_action_handle(name));
            if handle.is_valid() {
                Ok(SteamworksInputOperation::AnalogActionHandleRead {
                    name: name.clone(),
                    handle,
                })
            } else {
                Err(SteamworksInputError::invalid_handle_returned(
                    "GetAnalogActionHandle",
                ))
            }
        }
        SteamworksInputCommand::ActivateActionSet {
            controller,
            action_set,
        } => {
            input.activate_action_set_handle(controller.raw(), action_set.raw());
            Ok(SteamworksInputOperation::ActionSetActivated {
                controller: *controller,
                action_set: *action_set,
            })
        }
        SteamworksInputCommand::GetDigitalActionData { controller, action } => {
            let data = input.get_digital_action_data(controller.raw(), action.raw());
            let state = data.bState;
            let active = data.bActive;
            Ok(SteamworksInputOperation::DigitalActionDataRead {
                snapshot: SteamworksInputDigitalActionSnapshot {
                    controller: *controller,
                    action: *action,
                    data: SteamworksInputDigitalActionData { state, active },
                },
            })
        }
        SteamworksInputCommand::GetAnalogActionData { controller, action } => {
            let data = input.get_analog_action_data(controller.raw(), action.raw());
            let raw_mode = data.eMode as i32;
            let x = data.x;
            let y = data.y;
            let active = data.bActive;
            Ok(SteamworksInputOperation::AnalogActionDataRead {
                snapshot: SteamworksInputAnalogActionSnapshot {
                    controller: *controller,
                    action: *action,
                    data: SteamworksInputAnalogActionData {
                        mode: SteamworksInputSourceMode::from_raw(raw_mode),
                        x,
                        y,
                        active,
                    },
                },
            })
        }
        SteamworksInputCommand::GetDigitalActionOrigins {
            controller,
            action_set,
            action,
        } => {
            let origins =
                input.get_digital_action_origins(controller.raw(), action_set.raw(), action.raw());
            let origins = origins
                .into_iter()
                .map(|origin| SteamworksInputActionOriginInfo {
                    origin: SteamworksInputActionOrigin::from_code(origin as i32),
                    glyph_path: input.get_glyph_for_action_origin(origin),
                    name: input.get_string_for_action_origin(origin),
                })
                .collect();
            Ok(SteamworksInputOperation::DigitalActionOriginsRead {
                controller: *controller,
                action_set: *action_set,
                action: *action,
                origins,
            })
        }
        SteamworksInputCommand::GetAnalogActionOrigins {
            controller,
            action_set,
            action,
        } => {
            let origins =
                input.get_analog_action_origins(controller.raw(), action_set.raw(), action.raw());
            let origins = origins
                .into_iter()
                .map(|origin| SteamworksInputActionOriginInfo {
                    origin: SteamworksInputActionOrigin::from_code(origin as i32),
                    glyph_path: input.get_glyph_for_action_origin(origin),
                    name: input.get_string_for_action_origin(origin),
                })
                .collect();
            Ok(SteamworksInputOperation::AnalogActionOriginsRead {
                controller: *controller,
                action_set: *action_set,
                action: *action,
                origins,
            })
        }
        SteamworksInputCommand::GetMotionData { controller } => {
            let data = input.get_motion_data(controller.raw());
            let rot_quat_x = data.rotQuatX;
            let rot_quat_y = data.rotQuatY;
            let rot_quat_z = data.rotQuatZ;
            let rot_quat_w = data.rotQuatW;
            let pos_accel_x = data.posAccelX;
            let pos_accel_y = data.posAccelY;
            let pos_accel_z = data.posAccelZ;
            let rot_vel_x = data.rotVelX;
            let rot_vel_y = data.rotVelY;
            let rot_vel_z = data.rotVelZ;
            Ok(SteamworksInputOperation::MotionDataRead {
                snapshot: SteamworksInputMotionSnapshot {
                    controller: *controller,
                    data: SteamworksInputMotionData {
                        rotation_quaternion: [rot_quat_x, rot_quat_y, rot_quat_z, rot_quat_w],
                        position_acceleration: [pos_accel_x, pos_accel_y, pos_accel_z],
                        rotation_velocity: [rot_vel_x, rot_vel_y, rot_vel_z],
                    },
                },
            })
        }
        SteamworksInputCommand::ShowBindingPanel { controller } => {
            if input.show_binding_panel(controller.raw()) {
                Ok(SteamworksInputOperation::BindingPanelShown {
                    controller: *controller,
                })
            } else {
                Err(SteamworksInputError::BindingPanelUnavailable)
            }
        }
    }
}

fn snapshot_connected_controllers(input: &steamworks::Input) -> Vec<SteamworksInputControllerInfo> {
    let mut raw_handles = [0_u64; STEAM_INPUT_MAX_CONTROLLER_COUNT];
    let quantity = input.get_connected_controllers_slice(&mut raw_handles);
    input_handles_from_slice(&raw_handles, quantity)
        .into_iter()
        .map(|handle| snapshot_controller_info(input, handle))
        .collect()
}

fn input_handles_from_slice(raw_handles: &[u64], quantity: usize) -> Vec<SteamworksInputHandle> {
    raw_handles
        .iter()
        .copied()
        .take(quantity)
        .map(SteamworksInputHandle::from_raw)
        .filter(|handle| handle.is_valid())
        .collect()
}

fn snapshot_controller_info(
    input: &steamworks::Input,
    handle: SteamworksInputHandle,
) -> SteamworksInputControllerInfo {
    SteamworksInputControllerInfo {
        handle,
        input_type: SteamworksInputType::from(input.get_input_type_for_handle(handle.raw())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_handle_slice_truncates_and_filters_zero_handles() {
        let raw_handles = [11, 0, 22, 33, 44];

        assert_eq!(
            input_handles_from_slice(&raw_handles, 4),
            vec![
                SteamworksInputHandle::from_raw(11),
                SteamworksInputHandle::from_raw(22),
                SteamworksInputHandle::from_raw(33),
            ]
        );
    }
}
