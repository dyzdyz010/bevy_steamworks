use bevy_app::{App, Plugin};
use bevy_ecs::message::Messages;

use super::*;

#[test]
fn input_plugin_registers_resources_and_messages() {
    let mut app = App::new();

    app.add_plugins(SteamworksInputPlugin::new());

    assert!(app.world().contains_resource::<SteamworksInputState>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksInputCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksInputResult>>());
}

#[test]
fn plugin_name_matches_input_type_path_for_bevy_tracking() {
    let plugin = SteamworksInputPlugin::new();

    assert_eq!(
        plugin.name(),
        std::any::type_name::<SteamworksInputPlugin>()
    );
    assert_eq!(
        plugin.name(),
        "bevy_steamworks::input::SteamworksInputPlugin"
    );
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksInputPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksInputCommand>>()
        .write(SteamworksInputCommand::ListControllers);

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksInputResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksInputResult::Err {
            command: SteamworksInputCommand::ListControllers,
            error: SteamworksInputError::ClientUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksInputState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksInputError::ClientUnavailable)
    );
}

#[test]
fn constructors_preserve_inputs() {
    let controller = SteamworksInputHandle::from_raw(1);
    let action_set = SteamworksInputActionSetHandle::from_raw(2);
    let digital_action = SteamworksInputDigitalActionHandle::from_raw(3);
    let analog_action = SteamworksInputAnalogActionHandle::from_raw(4);
    let origin = SteamworksInputActionOrigin::from_code(1);

    assert_eq!(controller.raw(), 1);
    assert!(controller.is_valid());
    assert!(action_set.is_valid());
    assert!(digital_action.is_valid());
    assert!(analog_action.is_valid());
    assert_eq!(origin.code(), 1);
    assert!(!SteamworksInputHandle::from_raw(0).is_valid());
    assert!(!SteamworksInputActionSetHandle::from_raw(0).is_valid());
    assert!(!SteamworksInputDigitalActionHandle::from_raw(0).is_valid());
    assert!(!SteamworksInputAnalogActionHandle::from_raw(0).is_valid());
    assert_eq!(
        SteamworksInputCommand::init(true),
        SteamworksInputCommand::Init {
            explicitly_call_run_frame: true,
        }
    );
    assert_eq!(
        SteamworksInputCommand::run_frame(),
        SteamworksInputCommand::RunFrame
    );
    assert_eq!(
        SteamworksInputCommand::shutdown(),
        SteamworksInputCommand::Shutdown
    );
    assert_eq!(
        SteamworksInputCommand::list_controllers(),
        SteamworksInputCommand::ListControllers
    );
    assert_eq!(
        SteamworksInputCommand::get_controller_info(controller),
        SteamworksInputCommand::GetControllerInfo { controller }
    );
    assert_eq!(
        SteamworksInputCommand::set_action_manifest_file_path("input_manifest.vdf"),
        SteamworksInputCommand::SetActionManifestFilePath {
            path: "input_manifest.vdf".to_owned(),
        }
    );
    assert_eq!(
        SteamworksInputCommand::get_action_set_handle("gameplay"),
        SteamworksInputCommand::GetActionSetHandle {
            name: "gameplay".to_owned(),
        }
    );
    assert_eq!(
        SteamworksInputCommand::get_digital_action_handle("jump"),
        SteamworksInputCommand::GetDigitalActionHandle {
            name: "jump".to_owned(),
        }
    );
    assert_eq!(
        SteamworksInputCommand::get_analog_action_handle("move"),
        SteamworksInputCommand::GetAnalogActionHandle {
            name: "move".to_owned(),
        }
    );
    assert_eq!(
        SteamworksInputCommand::activate_action_set(controller, action_set),
        SteamworksInputCommand::ActivateActionSet {
            controller,
            action_set,
        }
    );
    assert_eq!(
        SteamworksInputCommand::get_digital_action_data(controller, digital_action),
        SteamworksInputCommand::GetDigitalActionData {
            controller,
            action: digital_action,
        }
    );
    assert_eq!(
        SteamworksInputCommand::get_analog_action_data(controller, analog_action),
        SteamworksInputCommand::GetAnalogActionData {
            controller,
            action: analog_action,
        }
    );
    assert_eq!(
        SteamworksInputCommand::get_digital_action_origins(controller, action_set, digital_action),
        SteamworksInputCommand::GetDigitalActionOrigins {
            controller,
            action_set,
            action: digital_action,
        }
    );
    assert_eq!(
        SteamworksInputCommand::get_analog_action_origins(controller, action_set, analog_action),
        SteamworksInputCommand::GetAnalogActionOrigins {
            controller,
            action_set,
            action: analog_action,
        }
    );
    assert_eq!(
        SteamworksInputCommand::get_motion_data(controller),
        SteamworksInputCommand::GetMotionData { controller }
    );
    assert_eq!(
        SteamworksInputCommand::show_binding_panel(controller),
        SteamworksInputCommand::ShowBindingPanel { controller }
    );
}

#[test]
fn state_upserts_named_handles() {
    let mut state = SteamworksInputState::default();

    state.record_operation(&SteamworksInputOperation::ActionSetHandleRead {
        name: "gameplay".to_owned(),
        handle: SteamworksInputActionSetHandle::from_raw(1),
    });
    state.record_operation(&SteamworksInputOperation::ActionSetHandleRead {
        name: "gameplay".to_owned(),
        handle: SteamworksInputActionSetHandle::from_raw(2),
    });

    assert_eq!(
        state.action_sets(),
        &[SteamworksInputNamedActionSetHandle {
            name: "gameplay".to_owned(),
            handle: SteamworksInputActionSetHandle::from_raw(2),
        }]
    );
    assert_eq!(
        state.action_set_handle("gameplay"),
        Some(SteamworksInputActionSetHandle::from_raw(2))
    );
}

#[test]
fn state_clears_stale_action_data_on_manifest_change_and_shutdown() {
    let mut state = SteamworksInputState::default();
    let controller = SteamworksInputHandle::from_raw(2);
    let action_set = SteamworksInputActionSetHandle::from_raw(1);
    let digital_action = SteamworksInputDigitalActionHandle::from_raw(3);
    let analog_action = SteamworksInputAnalogActionHandle::from_raw(5);
    let origin = SteamworksInputActionOriginInfo {
        origin: SteamworksInputActionOrigin::from_code(9),
        glyph_path: "glyph.png".to_owned(),
        name: "Jump".to_owned(),
    };

    state.record_operation(&SteamworksInputOperation::ActionSetHandleRead {
        name: "gameplay".to_owned(),
        handle: action_set,
    });
    state.record_operation(&SteamworksInputOperation::ActionSetActivated {
        controller,
        action_set,
    });
    state.record_operation(&SteamworksInputOperation::DigitalActionDataRead {
        snapshot: SteamworksInputDigitalActionSnapshot {
            controller,
            action: digital_action,
            data: SteamworksInputDigitalActionData {
                state: true,
                active: true,
            },
        },
    });
    state.record_operation(&SteamworksInputOperation::AnalogActionDataRead {
        snapshot: SteamworksInputAnalogActionSnapshot {
            controller,
            action: analog_action,
            data: SteamworksInputAnalogActionData {
                mode: SteamworksInputSourceMode::JoystickMove,
                x: 1.0,
                y: -1.0,
                active: true,
            },
        },
    });
    state.record_operation(&SteamworksInputOperation::DigitalActionOriginsRead {
        controller,
        action_set,
        action: digital_action,
        origins: vec![origin.clone()],
    });
    state.record_operation(&SteamworksInputOperation::AnalogActionOriginsRead {
        controller,
        action_set,
        action: analog_action,
        origins: vec![origin],
    });
    state.record_operation(&SteamworksInputOperation::MotionDataRead {
        snapshot: SteamworksInputMotionSnapshot {
            controller,
            data: SteamworksInputMotionData {
                rotation_quaternion: [0.0, 0.0, 0.0, 1.0],
                position_acceleration: [0.0, 1.0, 0.0],
                rotation_velocity: [0.0, 0.0, 1.0],
            },
        },
    });
    state.record_operation(&SteamworksInputOperation::BindingPanelShown { controller });

    state.record_operation(&SteamworksInputOperation::ActionManifestFilePathSet {
        path: "new_manifest.vdf".to_owned(),
    });

    assert!(state.action_sets().is_empty());
    assert_eq!(state.action_manifest_path(), Some("new_manifest.vdf"));
    assert!(state.last_action_set_activation().is_none());
    assert!(state.action_set_activations().is_empty());
    assert!(state.last_digital_action().is_none());
    assert!(state.digital_action_data_snapshots().is_empty());
    assert!(state.last_analog_action().is_none());
    assert!(state.analog_action_data_snapshots().is_empty());
    assert!(state.last_digital_action_origins().is_none());
    assert!(state.digital_action_origin_snapshots().is_empty());
    assert!(state.last_analog_action_origins().is_none());
    assert!(state.analog_action_origin_snapshots().is_empty());
    assert!(state.action_origin_infos().is_empty());
    assert!(state.last_action_origin_info().is_none());
    assert!(state.last_motion().is_none());
    assert!(state.motion_snapshots().is_empty());
    assert!(state.last_binding_panel_controller().is_none());

    state.record_operation(&SteamworksInputOperation::Initialized {
        explicitly_call_run_frame: false,
    });
    state.record_operation(&SteamworksInputOperation::ActionSetHandleRead {
        name: "menu".to_owned(),
        handle: SteamworksInputActionSetHandle::from_raw(4),
    });
    state.record_operation(&SteamworksInputOperation::Shutdown);

    assert!(!state.initialized());
    assert!(state.action_manifest_path().is_none());
    assert!(state.action_sets().is_empty());
}

#[test]
fn state_records_input_operations() {
    let mut state = SteamworksInputState::default();
    let controller = SteamworksInputHandle::from_raw(11);
    let action_set = SteamworksInputActionSetHandle::from_raw(22);
    let digital_action = SteamworksInputDigitalActionHandle::from_raw(33);
    let analog_action = SteamworksInputAnalogActionHandle::from_raw(44);
    let controller_info = SteamworksInputControllerInfo {
        handle: controller,
        input_type: SteamworksInputType::SteamDeckController,
    };
    let digital_snapshot = SteamworksInputDigitalActionSnapshot {
        controller,
        action: digital_action,
        data: SteamworksInputDigitalActionData {
            state: true,
            active: true,
        },
    };
    let analog_snapshot = SteamworksInputAnalogActionSnapshot {
        controller,
        action: analog_action,
        data: SteamworksInputAnalogActionData {
            mode: SteamworksInputSourceMode::JoystickMove,
            x: 0.25,
            y: -0.5,
            active: true,
        },
    };
    let origin = SteamworksInputActionOriginInfo {
        origin: SteamworksInputActionOrigin::from_code(7),
        glyph_path: "glyph.svg".to_owned(),
        name: "A Button".to_owned(),
    };
    let updated_origin = SteamworksInputActionOriginInfo {
        origin: origin.origin,
        glyph_path: "glyph-updated.svg".to_owned(),
        name: "A Button Updated".to_owned(),
    };
    let second_origin = SteamworksInputActionOriginInfo {
        origin: SteamworksInputActionOrigin::from_code(8),
        glyph_path: "glyph-b.svg".to_owned(),
        name: "B Button".to_owned(),
    };
    let motion = SteamworksInputMotionSnapshot {
        controller,
        data: SteamworksInputMotionData {
            rotation_quaternion: [0.0, 0.0, 0.0, 1.0],
            position_acceleration: [1.0, 2.0, 3.0],
            rotation_velocity: [4.0, 5.0, 6.0],
        },
    };

    state.record_operation(&SteamworksInputOperation::Initialized {
        explicitly_call_run_frame: true,
    });
    state.record_operation(&SteamworksInputOperation::FrameRun);
    state.record_operation(&SteamworksInputOperation::FrameRun);
    state.record_operation(&SteamworksInputOperation::ControllersListed {
        controllers: vec![controller_info.clone()],
    });
    state.record_operation(&SteamworksInputOperation::ControllerInfoRead {
        controller: SteamworksInputControllerInfo {
            input_type: SteamworksInputType::GenericGamepad,
            ..controller_info.clone()
        },
    });
    state.record_operation(&SteamworksInputOperation::ActionSetHandleRead {
        name: "gameplay".to_owned(),
        handle: action_set,
    });
    state.record_operation(&SteamworksInputOperation::DigitalActionHandleRead {
        name: "jump".to_owned(),
        handle: digital_action,
    });
    state.record_operation(&SteamworksInputOperation::AnalogActionHandleRead {
        name: "move".to_owned(),
        handle: analog_action,
    });
    state.record_operation(&SteamworksInputOperation::ActionSetActivated {
        controller,
        action_set,
    });
    state.record_operation(&SteamworksInputOperation::DigitalActionDataRead {
        snapshot: digital_snapshot.clone(),
    });
    state.record_operation(&SteamworksInputOperation::AnalogActionDataRead {
        snapshot: analog_snapshot.clone(),
    });
    state.record_operation(&SteamworksInputOperation::DigitalActionOriginsRead {
        controller,
        action_set,
        action: digital_action,
        origins: vec![origin.clone()],
    });
    state.record_operation(&SteamworksInputOperation::AnalogActionOriginsRead {
        controller,
        action_set,
        action: analog_action,
        origins: vec![updated_origin.clone(), second_origin.clone()],
    });
    state.record_operation(&SteamworksInputOperation::MotionDataRead {
        snapshot: motion.clone(),
    });
    state.record_operation(&SteamworksInputOperation::BindingPanelShown { controller });

    assert!(state.initialized());
    assert_eq!(state.frame_run_count(), 2);
    assert_eq!(
        state.controller(controller),
        Some(&SteamworksInputControllerInfo {
            input_type: SteamworksInputType::GenericGamepad,
            ..controller_info
        })
    );
    assert_eq!(state.action_set_handle("gameplay"), Some(action_set));
    assert_eq!(state.digital_action_handle("jump"), Some(digital_action));
    assert_eq!(state.analog_action_handle("move"), Some(analog_action));
    assert_eq!(
        state.last_action_set_activation(),
        Some(SteamworksInputActionSetActivation {
            controller,
            action_set,
        })
    );
    assert_eq!(
        state.action_set_activation(controller),
        Some(SteamworksInputActionSetActivation {
            controller,
            action_set,
        })
    );
    assert_eq!(state.action_set_activations().len(), 1);
    assert_eq!(state.last_digital_action(), Some(&digital_snapshot));
    assert_eq!(
        state.digital_action_data(controller, digital_action),
        Some(&digital_snapshot)
    );
    assert_eq!(
        state.digital_action_pressed(controller, digital_action),
        Some(true)
    );
    assert_eq!(
        state.digital_action_active(controller, digital_action),
        Some(true)
    );
    assert_eq!(
        state.digital_action_data_snapshots(),
        &[digital_snapshot.clone()]
    );
    assert_eq!(state.last_analog_action(), Some(&analog_snapshot));
    assert_eq!(
        state.analog_action_data(controller, analog_action),
        Some(&analog_snapshot)
    );
    assert_eq!(
        state.analog_action_mode(controller, analog_action),
        Some(SteamworksInputSourceMode::JoystickMove)
    );
    assert_eq!(
        state.analog_action_vector(controller, analog_action),
        Some((0.25, -0.5))
    );
    assert_eq!(
        state.analog_action_active(controller, analog_action),
        Some(true)
    );
    assert_eq!(
        state.analog_action_data_snapshots(),
        &[analog_snapshot.clone()]
    );
    assert_eq!(
        state.last_digital_action_origins(),
        Some(&SteamworksInputDigitalActionOriginsSnapshot {
            controller,
            action_set,
            action: digital_action,
            origins: vec![origin.clone()],
        })
    );
    assert_eq!(
        state.digital_action_origins(controller, action_set, digital_action),
        Some(&SteamworksInputDigitalActionOriginsSnapshot {
            controller,
            action_set,
            action: digital_action,
            origins: vec![origin.clone()],
        })
    );
    assert_eq!(state.digital_action_origin_snapshots().len(), 1);
    assert_eq!(
        state.last_analog_action_origins(),
        Some(&SteamworksInputAnalogActionOriginsSnapshot {
            controller,
            action_set,
            action: analog_action,
            origins: vec![updated_origin.clone(), second_origin.clone()],
        })
    );
    assert_eq!(
        state.analog_action_origins(controller, action_set, analog_action),
        Some(&SteamworksInputAnalogActionOriginsSnapshot {
            controller,
            action_set,
            action: analog_action,
            origins: vec![updated_origin.clone(), second_origin.clone()],
        })
    );
    assert_eq!(state.analog_action_origin_snapshots().len(), 1);
    assert_eq!(
        state.action_origin_infos(),
        &[updated_origin.clone(), second_origin.clone()]
    );
    assert_eq!(
        state.action_origin_info(updated_origin.origin),
        Some(&updated_origin)
    );
    assert_eq!(
        state.action_origin_info(second_origin.origin),
        Some(&second_origin)
    );
    assert_eq!(state.last_action_origin_info(), Some(&second_origin));
    assert_eq!(state.last_motion(), Some(&motion));
    assert_eq!(state.motion(controller), Some(&motion));
    assert_eq!(
        state.motion_rotation_quaternion(controller),
        Some([0.0, 0.0, 0.0, 1.0])
    );
    assert_eq!(
        state.motion_position_acceleration(controller),
        Some([1.0, 2.0, 3.0])
    );
    assert_eq!(
        state.motion_rotation_velocity(controller),
        Some([4.0, 5.0, 6.0])
    );
    assert_eq!(state.motion_snapshots(), &[motion]);
    assert_eq!(state.last_binding_panel_controller(), Some(controller));
}

#[test]
fn input_state_caches_are_bounded() {
    let mut state = SteamworksInputState::default();

    for raw in 1..=(super::state::STEAMWORKS_INPUT_STATE_CACHE_LIMIT as u64 + 1) {
        let controller = SteamworksInputHandle::from_raw(raw);
        let action_set = SteamworksInputActionSetHandle::from_raw(raw);
        let digital_action = SteamworksInputDigitalActionHandle::from_raw(raw);
        let analog_action = SteamworksInputAnalogActionHandle::from_raw(raw);
        let origin = SteamworksInputActionOrigin::from_code(raw as i32);

        state.record_operation(&SteamworksInputOperation::ControllerInfoRead {
            controller: SteamworksInputControllerInfo {
                handle: controller,
                input_type: SteamworksInputType::GenericGamepad,
            },
        });
        state.record_operation(&SteamworksInputOperation::ActionSetHandleRead {
            name: format!("set-{raw}"),
            handle: action_set,
        });
        state.record_operation(&SteamworksInputOperation::DigitalActionHandleRead {
            name: format!("digital-{raw}"),
            handle: digital_action,
        });
        state.record_operation(&SteamworksInputOperation::AnalogActionHandleRead {
            name: format!("analog-{raw}"),
            handle: analog_action,
        });
        state.record_operation(&SteamworksInputOperation::ActionSetActivated {
            controller,
            action_set,
        });
        state.record_operation(&SteamworksInputOperation::DigitalActionDataRead {
            snapshot: SteamworksInputDigitalActionSnapshot {
                controller,
                action: digital_action,
                data: SteamworksInputDigitalActionData {
                    state: raw % 2 == 0,
                    active: true,
                },
            },
        });
        state.record_operation(&SteamworksInputOperation::AnalogActionDataRead {
            snapshot: SteamworksInputAnalogActionSnapshot {
                controller,
                action: analog_action,
                data: SteamworksInputAnalogActionData {
                    mode: SteamworksInputSourceMode::JoystickMove,
                    x: raw as f32,
                    y: -(raw as f32),
                    active: true,
                },
            },
        });
        state.record_operation(&SteamworksInputOperation::DigitalActionOriginsRead {
            controller,
            action_set,
            action: digital_action,
            origins: vec![SteamworksInputActionOriginInfo {
                origin,
                glyph_path: format!("glyph-{raw}.svg"),
                name: format!("Origin {raw}"),
            }],
        });
        state.record_operation(&SteamworksInputOperation::AnalogActionOriginsRead {
            controller,
            action_set,
            action: analog_action,
            origins: vec![SteamworksInputActionOriginInfo {
                origin,
                glyph_path: format!("analog-glyph-{raw}.svg"),
                name: format!("Analog Origin {raw}"),
            }],
        });
        state.record_operation(&SteamworksInputOperation::MotionDataRead {
            snapshot: SteamworksInputMotionSnapshot {
                controller,
                data: SteamworksInputMotionData {
                    rotation_quaternion: [0.0, 0.0, 0.0, 1.0],
                    position_acceleration: [raw as f32, 0.0, 0.0],
                    rotation_velocity: [0.0, raw as f32, 0.0],
                },
            },
        });
    }

    assert_eq!(
        state.controllers().len(),
        super::state::STEAMWORKS_INPUT_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.action_sets().len(),
        super::state::STEAMWORKS_INPUT_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.digital_actions().len(),
        super::state::STEAMWORKS_INPUT_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.analog_actions().len(),
        super::state::STEAMWORKS_INPUT_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.action_origin_infos().len(),
        super::state::STEAMWORKS_INPUT_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.action_set_activations().len(),
        super::state::STEAMWORKS_INPUT_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.digital_action_data_snapshots().len(),
        super::state::STEAMWORKS_INPUT_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.analog_action_data_snapshots().len(),
        super::state::STEAMWORKS_INPUT_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.digital_action_origin_snapshots().len(),
        super::state::STEAMWORKS_INPUT_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.analog_action_origin_snapshots().len(),
        super::state::STEAMWORKS_INPUT_STATE_CACHE_LIMIT
    );
    assert_eq!(
        state.motion_snapshots().len(),
        super::state::STEAMWORKS_INPUT_STATE_CACHE_LIMIT
    );

    assert_eq!(state.controller(SteamworksInputHandle::from_raw(1)), None);
    assert_eq!(state.action_set_handle("set-1"), None);
    assert_eq!(state.digital_action_handle("digital-1"), None);
    assert_eq!(state.analog_action_handle("analog-1"), None);
    assert_eq!(
        state.action_origin_info(SteamworksInputActionOrigin::from_code(1)),
        None
    );
    assert_eq!(
        state.action_set_activation(SteamworksInputHandle::from_raw(1)),
        None
    );
    assert_eq!(
        state.digital_action_data(
            SteamworksInputHandle::from_raw(1),
            SteamworksInputDigitalActionHandle::from_raw(1),
        ),
        None
    );
    assert_eq!(
        state.digital_action_pressed(
            SteamworksInputHandle::from_raw(1),
            SteamworksInputDigitalActionHandle::from_raw(1),
        ),
        None
    );
    assert_eq!(
        state.digital_action_active(
            SteamworksInputHandle::from_raw(1),
            SteamworksInputDigitalActionHandle::from_raw(1),
        ),
        None
    );
    assert_eq!(
        state.analog_action_data(
            SteamworksInputHandle::from_raw(1),
            SteamworksInputAnalogActionHandle::from_raw(1),
        ),
        None
    );
    assert_eq!(
        state.analog_action_vector(
            SteamworksInputHandle::from_raw(1),
            SteamworksInputAnalogActionHandle::from_raw(1),
        ),
        None
    );
    assert_eq!(
        state.analog_action_active(
            SteamworksInputHandle::from_raw(1),
            SteamworksInputAnalogActionHandle::from_raw(1),
        ),
        None
    );
    assert_eq!(
        state.digital_action_origins(
            SteamworksInputHandle::from_raw(1),
            SteamworksInputActionSetHandle::from_raw(1),
            SteamworksInputDigitalActionHandle::from_raw(1),
        ),
        None
    );
    assert_eq!(
        state.analog_action_origins(
            SteamworksInputHandle::from_raw(1),
            SteamworksInputActionSetHandle::from_raw(1),
            SteamworksInputAnalogActionHandle::from_raw(1),
        ),
        None
    );
    assert_eq!(state.motion(SteamworksInputHandle::from_raw(1)), None);
    assert_eq!(
        state.motion_rotation_quaternion(SteamworksInputHandle::from_raw(1)),
        None
    );
    assert_eq!(
        state.motion_position_acceleration(SteamworksInputHandle::from_raw(1)),
        None
    );
    assert_eq!(
        state.motion_rotation_velocity(SteamworksInputHandle::from_raw(1)),
        None
    );

    assert!(state
        .controller(SteamworksInputHandle::from_raw(2))
        .is_some());
    assert_eq!(
        state.action_set_handle("set-2"),
        Some(SteamworksInputActionSetHandle::from_raw(2))
    );
    assert_eq!(
        state.digital_action_handle("digital-2"),
        Some(SteamworksInputDigitalActionHandle::from_raw(2))
    );
    assert_eq!(
        state.analog_action_handle("analog-2"),
        Some(SteamworksInputAnalogActionHandle::from_raw(2))
    );
    assert!(state
        .action_origin_info(SteamworksInputActionOrigin::from_code(2))
        .is_some());
    assert_eq!(
        state.action_set_activation(SteamworksInputHandle::from_raw(2)),
        Some(SteamworksInputActionSetActivation {
            controller: SteamworksInputHandle::from_raw(2),
            action_set: SteamworksInputActionSetHandle::from_raw(2),
        })
    );
    assert!(state
        .digital_action_data(
            SteamworksInputHandle::from_raw(2),
            SteamworksInputDigitalActionHandle::from_raw(2),
        )
        .is_some());
    assert_eq!(
        state.digital_action_pressed(
            SteamworksInputHandle::from_raw(2),
            SteamworksInputDigitalActionHandle::from_raw(2),
        ),
        Some(true)
    );
    assert_eq!(
        state.digital_action_active(
            SteamworksInputHandle::from_raw(2),
            SteamworksInputDigitalActionHandle::from_raw(2),
        ),
        Some(true)
    );
    assert!(state
        .analog_action_data(
            SteamworksInputHandle::from_raw(2),
            SteamworksInputAnalogActionHandle::from_raw(2),
        )
        .is_some());
    assert_eq!(
        state.analog_action_mode(
            SteamworksInputHandle::from_raw(2),
            SteamworksInputAnalogActionHandle::from_raw(2),
        ),
        Some(SteamworksInputSourceMode::JoystickMove)
    );
    assert_eq!(
        state.analog_action_vector(
            SteamworksInputHandle::from_raw(2),
            SteamworksInputAnalogActionHandle::from_raw(2),
        ),
        Some((2.0, -2.0))
    );
    assert!(state
        .digital_action_origins(
            SteamworksInputHandle::from_raw(2),
            SteamworksInputActionSetHandle::from_raw(2),
            SteamworksInputDigitalActionHandle::from_raw(2),
        )
        .is_some());
    assert!(state
        .analog_action_origins(
            SteamworksInputHandle::from_raw(2),
            SteamworksInputActionSetHandle::from_raw(2),
            SteamworksInputAnalogActionHandle::from_raw(2),
        )
        .is_some());
    assert!(state.motion(SteamworksInputHandle::from_raw(2)).is_some());
    assert_eq!(
        state.motion_position_acceleration(SteamworksInputHandle::from_raw(2)),
        Some([2.0, 0.0, 0.0])
    );
}
