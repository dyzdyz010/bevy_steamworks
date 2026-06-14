use bevy_ecs::prelude::Resource;

use super::*;

/// Runtime state for [`crate::SteamworksInputPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksInputState {
    last_error: Option<SteamworksInputError>,
    initialized: bool,
    frame_run_count: u64,
    controllers: Vec<SteamworksInputControllerInfo>,
    action_sets: Vec<SteamworksInputNamedActionSetHandle>,
    digital_actions: Vec<SteamworksInputNamedDigitalActionHandle>,
    analog_actions: Vec<SteamworksInputNamedAnalogActionHandle>,
    action_manifest_path: Option<String>,
    last_action_set_activation: Option<SteamworksInputActionSetActivation>,
    last_digital_action: Option<SteamworksInputDigitalActionSnapshot>,
    last_analog_action: Option<SteamworksInputAnalogActionSnapshot>,
    last_digital_action_origins: Option<SteamworksInputDigitalActionOriginsSnapshot>,
    last_analog_action_origins: Option<SteamworksInputAnalogActionOriginsSnapshot>,
    last_motion: Option<SteamworksInputMotionSnapshot>,
    last_binding_panel_controller: Option<SteamworksInputHandle>,
}

impl SteamworksInputState {
    /// Returns the most recent synchronous error observed by the input plugin.
    pub fn last_error(&self) -> Option<&SteamworksInputError> {
        self.last_error.as_ref()
    }

    /// Returns whether the last initialization command succeeded and has not
    /// been followed by a shutdown command.
    pub fn initialized(&self) -> bool {
        self.initialized
    }

    /// Returns how many successful [`SteamworksInputCommand::RunFrame`] commands this plugin observed.
    pub fn frame_run_count(&self) -> u64 {
        self.frame_run_count
    }

    /// Returns the known controller snapshots read through the plugin.
    pub fn controllers(&self) -> &[SteamworksInputControllerInfo] {
        &self.controllers
    }

    /// Returns the cached controller snapshot for a handle.
    pub fn controller(
        &self,
        handle: SteamworksInputHandle,
    ) -> Option<&SteamworksInputControllerInfo> {
        self.controllers
            .iter()
            .find(|controller| controller.handle == handle)
    }

    /// Returns action set handles read through the plugin.
    pub fn action_sets(&self) -> &[SteamworksInputNamedActionSetHandle] {
        &self.action_sets
    }

    /// Returns the cached action set handle for a manifest action set name.
    pub fn action_set_handle(&self, name: &str) -> Option<SteamworksInputActionSetHandle> {
        self.action_sets
            .iter()
            .find_map(|handle| (handle.name == name).then_some(handle.handle))
    }

    /// Returns digital action handles read through the plugin.
    pub fn digital_actions(&self) -> &[SteamworksInputNamedDigitalActionHandle] {
        &self.digital_actions
    }

    /// Returns the cached digital action handle for a manifest action name.
    pub fn digital_action_handle(&self, name: &str) -> Option<SteamworksInputDigitalActionHandle> {
        self.digital_actions
            .iter()
            .find_map(|handle| (handle.name == name).then_some(handle.handle))
    }

    /// Returns analog action handles read through the plugin.
    pub fn analog_actions(&self) -> &[SteamworksInputNamedAnalogActionHandle] {
        &self.analog_actions
    }

    /// Returns the cached analog action handle for a manifest action name.
    pub fn analog_action_handle(&self, name: &str) -> Option<SteamworksInputAnalogActionHandle> {
        self.analog_actions
            .iter()
            .find_map(|handle| (handle.name == name).then_some(handle.handle))
    }

    /// Returns the most recent action manifest path accepted by Steam Input.
    pub fn action_manifest_path(&self) -> Option<&str> {
        self.action_manifest_path.as_deref()
    }

    /// Returns the most recent action set activation submitted through this plugin.
    pub fn last_action_set_activation(&self) -> Option<SteamworksInputActionSetActivation> {
        self.last_action_set_activation
    }

    /// Returns the most recent digital action data snapshot.
    pub fn last_digital_action(&self) -> Option<&SteamworksInputDigitalActionSnapshot> {
        self.last_digital_action.as_ref()
    }

    /// Returns the most recent analog action data snapshot.
    pub fn last_analog_action(&self) -> Option<&SteamworksInputAnalogActionSnapshot> {
        self.last_analog_action.as_ref()
    }

    /// Returns the most recent digital action origin snapshot.
    pub fn last_digital_action_origins(
        &self,
    ) -> Option<&SteamworksInputDigitalActionOriginsSnapshot> {
        self.last_digital_action_origins.as_ref()
    }

    /// Returns the most recent analog action origin snapshot.
    pub fn last_analog_action_origins(
        &self,
    ) -> Option<&SteamworksInputAnalogActionOriginsSnapshot> {
        self.last_analog_action_origins.as_ref()
    }

    /// Returns the most recent motion data snapshot.
    pub fn last_motion(&self) -> Option<&SteamworksInputMotionSnapshot> {
        self.last_motion.as_ref()
    }

    /// Returns the most recent controller for which the binding panel was shown.
    pub fn last_binding_panel_controller(&self) -> Option<SteamworksInputHandle> {
        self.last_binding_panel_controller
    }

    pub(super) fn record_error(&mut self, error: SteamworksInputError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksInputOperation) {
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
        self.last_motion = None;
        self.last_binding_panel_controller = None;
    }
}

fn upsert_controller(
    controllers: &mut Vec<SteamworksInputControllerInfo>,
    controller: SteamworksInputControllerInfo,
) {
    if let Some(existing) = controllers
        .iter_mut()
        .find(|existing| existing.handle == controller.handle)
    {
        *existing = controller;
    } else {
        controllers.push(controller);
    }
}

fn upsert_named_action_set(
    handles: &mut Vec<SteamworksInputNamedActionSetHandle>,
    name: String,
    handle: SteamworksInputActionSetHandle,
) {
    if let Some(existing) = handles.iter_mut().find(|existing| existing.name == name) {
        existing.handle = handle;
    } else {
        handles.push(SteamworksInputNamedActionSetHandle { name, handle });
    }
}

fn upsert_named_digital_action(
    handles: &mut Vec<SteamworksInputNamedDigitalActionHandle>,
    name: String,
    handle: SteamworksInputDigitalActionHandle,
) {
    if let Some(existing) = handles.iter_mut().find(|existing| existing.name == name) {
        existing.handle = handle;
    } else {
        handles.push(SteamworksInputNamedDigitalActionHandle { name, handle });
    }
}

fn upsert_named_analog_action(
    handles: &mut Vec<SteamworksInputNamedAnalogActionHandle>,
    name: String,
    handle: SteamworksInputAnalogActionHandle,
) {
    if let Some(existing) = handles.iter_mut().find(|existing| existing.name == name) {
        existing.handle = handle;
    } else {
        handles.push(SteamworksInputNamedAnalogActionHandle { name, handle });
    }
}
