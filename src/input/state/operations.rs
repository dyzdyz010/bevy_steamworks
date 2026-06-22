use super::{
    upsert_action_origin_info, upsert_controller, upsert_named_action_set,
    upsert_named_analog_action, upsert_named_digital_action, SteamworksInputActionSetActivation,
    SteamworksInputAnalogActionOriginsSnapshot, SteamworksInputDigitalActionOriginsSnapshot,
    SteamworksInputError, SteamworksInputOperation, SteamworksInputState,
};

impl SteamworksInputState {
    pub(in crate::input) fn record_error(&mut self, error: SteamworksInputError) {
        self.last_error = Some(error);
    }

    pub(in crate::input) fn record_operation(&mut self, operation: &SteamworksInputOperation) {
        match operation {
            SteamworksInputOperation::Initialized { .. } => {
                self.clear_cached_input_data();
                self.action_manifest_path = None;
                self.initialized = true;
            }
            SteamworksInputOperation::FrameRun => {
                self.frame_run_count = self.frame_run_count.saturating_add(1);
            }
            SteamworksInputOperation::Shutdown => {
                self.initialized = false;
                self.action_manifest_path = None;
                self.clear_cached_input_data();
            }
            SteamworksInputOperation::ControllersListed { controllers } => {
                self.controllers.clone_from(controllers);
            }
            SteamworksInputOperation::ControllerInfoRead { controller } => {
                upsert_controller(&mut self.controllers, controller.clone());
            }
            SteamworksInputOperation::ActionManifestFilePathSet { path } => {
                self.clear_action_cache();
                self.action_manifest_path = Some(path.clone());
            }
            SteamworksInputOperation::ActionSetHandleRead { name, handle } => {
                upsert_named_action_set(&mut self.action_sets, name.clone(), *handle);
            }
            SteamworksInputOperation::DigitalActionHandleRead { name, handle } => {
                upsert_named_digital_action(&mut self.digital_actions, name.clone(), *handle);
            }
            SteamworksInputOperation::AnalogActionHandleRead { name, handle } => {
                upsert_named_analog_action(&mut self.analog_actions, name.clone(), *handle);
            }
            SteamworksInputOperation::ActionSetActivated {
                controller,
                action_set,
            } => {
                self.last_action_set_activation = Some(SteamworksInputActionSetActivation {
                    controller: *controller,
                    action_set: *action_set,
                });
            }
            SteamworksInputOperation::DigitalActionDataRead { snapshot } => {
                self.last_digital_action = Some(snapshot.clone());
            }
            SteamworksInputOperation::AnalogActionDataRead { snapshot } => {
                self.last_analog_action = Some(snapshot.clone());
            }
            SteamworksInputOperation::DigitalActionOriginsRead {
                controller,
                action_set,
                action,
                origins,
            } => {
                for origin in origins {
                    upsert_action_origin_info(&mut self.action_origin_infos, origin.clone());
                }
                self.last_action_origin_info = origins.last().cloned();
                self.last_digital_action_origins =
                    Some(SteamworksInputDigitalActionOriginsSnapshot {
                        controller: *controller,
                        action_set: *action_set,
                        action: *action,
                        origins: origins.clone(),
                    });
            }
            SteamworksInputOperation::AnalogActionOriginsRead {
                controller,
                action_set,
                action,
                origins,
            } => {
                for origin in origins {
                    upsert_action_origin_info(&mut self.action_origin_infos, origin.clone());
                }
                self.last_action_origin_info = origins.last().cloned();
                self.last_analog_action_origins =
                    Some(SteamworksInputAnalogActionOriginsSnapshot {
                        controller: *controller,
                        action_set: *action_set,
                        action: *action,
                        origins: origins.clone(),
                    });
            }
            SteamworksInputOperation::MotionDataRead { snapshot } => {
                self.last_motion = Some(snapshot.clone());
            }
            SteamworksInputOperation::BindingPanelShown { controller } => {
                self.last_binding_panel_controller = Some(*controller);
            }
        }
    }

    fn clear_cached_input_data(&mut self) {
        self.controllers.clear();
        self.clear_action_cache();
    }

    fn clear_action_cache(&mut self) {
        self.action_sets.clear();
        self.digital_actions.clear();
        self.analog_actions.clear();
        self.last_action_set_activation = None;
        self.last_digital_action = None;
        self.last_analog_action = None;
        self.last_digital_action_origins = None;
        self.last_analog_action_origins = None;
        self.action_origin_infos.clear();
        self.last_action_origin_info = None;
        self.last_motion = None;
        self.last_binding_panel_controller = None;
    }
}
