use super::{
    upsert_action_origin_info, upsert_action_set_activation, upsert_analog_action_origins_snapshot,
    upsert_analog_action_snapshot, upsert_controller, upsert_digital_action_origins_snapshot,
    upsert_digital_action_snapshot, upsert_motion_snapshot, upsert_named_action_set,
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
                let activation = SteamworksInputActionSetActivation {
                    controller: *controller,
                    action_set: *action_set,
                };
                upsert_action_set_activation(&mut self.action_set_activations, activation);
                self.last_action_set_activation = Some(activation);
            }
            SteamworksInputOperation::DigitalActionDataRead { snapshot } => {
                upsert_digital_action_snapshot(
                    &mut self.digital_action_snapshots,
                    snapshot.clone(),
                );
                self.last_digital_action = Some(snapshot.clone());
            }
            SteamworksInputOperation::AnalogActionDataRead { snapshot } => {
                upsert_analog_action_snapshot(&mut self.analog_action_snapshots, snapshot.clone());
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
                let snapshot = SteamworksInputDigitalActionOriginsSnapshot {
                    controller: *controller,
                    action_set: *action_set,
                    action: *action,
                    origins: origins.clone(),
                };
                upsert_digital_action_origins_snapshot(
                    &mut self.digital_action_origin_snapshots,
                    snapshot.clone(),
                );
                self.last_digital_action_origins = Some(snapshot);
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
                let snapshot = SteamworksInputAnalogActionOriginsSnapshot {
                    controller: *controller,
                    action_set: *action_set,
                    action: *action,
                    origins: origins.clone(),
                };
                upsert_analog_action_origins_snapshot(
                    &mut self.analog_action_origin_snapshots,
                    snapshot.clone(),
                );
                self.last_analog_action_origins = Some(snapshot);
            }
            SteamworksInputOperation::MotionDataRead { snapshot } => {
                upsert_motion_snapshot(&mut self.motion_snapshots, snapshot.clone());
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
        self.action_set_activations.clear();
        self.last_digital_action = None;
        self.digital_action_snapshots.clear();
        self.last_analog_action = None;
        self.analog_action_snapshots.clear();
        self.last_digital_action_origins = None;
        self.digital_action_origin_snapshots.clear();
        self.last_analog_action_origins = None;
        self.analog_action_origin_snapshots.clear();
        self.action_origin_infos.clear();
        self.last_action_origin_info = None;
        self.last_motion = None;
        self.motion_snapshots.clear();
        self.last_binding_panel_controller = None;
    }
}
