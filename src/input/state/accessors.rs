use super::{
    SteamworksInputActionOrigin, SteamworksInputActionOriginInfo,
    SteamworksInputActionSetActivation, SteamworksInputActionSetHandle,
    SteamworksInputAnalogActionHandle, SteamworksInputAnalogActionOriginsSnapshot,
    SteamworksInputAnalogActionSnapshot, SteamworksInputDigitalActionHandle,
    SteamworksInputDigitalActionOriginsSnapshot, SteamworksInputDigitalActionSnapshot,
    SteamworksInputError, SteamworksInputHandle, SteamworksInputMotionSnapshot,
    SteamworksInputNamedActionSetHandle, SteamworksInputNamedAnalogActionHandle,
    SteamworksInputNamedDigitalActionHandle, SteamworksInputSourceMode, SteamworksInputState,
};
use crate::SteamworksInputControllerInfo;

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

    /// Returns cached action set activations keyed by controller.
    pub fn action_set_activations(&self) -> &[SteamworksInputActionSetActivation] {
        &self.action_set_activations
    }

    /// Returns the cached action set activation for a controller.
    pub fn action_set_activation(
        &self,
        controller: SteamworksInputHandle,
    ) -> Option<SteamworksInputActionSetActivation> {
        self.action_set_activations
            .iter()
            .find(|activation| activation.controller == controller)
            .copied()
    }

    /// Returns the most recent digital action data snapshot.
    pub fn last_digital_action(&self) -> Option<&SteamworksInputDigitalActionSnapshot> {
        self.last_digital_action.as_ref()
    }

    /// Returns cached digital action data snapshots keyed by controller and action.
    pub fn digital_action_data_snapshots(&self) -> &[SteamworksInputDigitalActionSnapshot] {
        &self.digital_action_snapshots
    }

    /// Returns the cached digital action data snapshot for a controller/action pair.
    pub fn digital_action_data(
        &self,
        controller: SteamworksInputHandle,
        action: SteamworksInputDigitalActionHandle,
    ) -> Option<&SteamworksInputDigitalActionSnapshot> {
        self.digital_action_snapshots
            .iter()
            .find(|snapshot| snapshot.controller == controller && snapshot.action == action)
    }

    /// Returns whether a cached digital action is currently pressed.
    pub fn digital_action_pressed(
        &self,
        controller: SteamworksInputHandle,
        action: SteamworksInputDigitalActionHandle,
    ) -> Option<bool> {
        self.digital_action_data(controller, action)
            .map(|snapshot| snapshot.data.state)
    }

    /// Returns whether a cached digital action is active in the current action set.
    pub fn digital_action_active(
        &self,
        controller: SteamworksInputHandle,
        action: SteamworksInputDigitalActionHandle,
    ) -> Option<bool> {
        self.digital_action_data(controller, action)
            .map(|snapshot| snapshot.data.active)
    }

    /// Returns the most recent analog action data snapshot.
    pub fn last_analog_action(&self) -> Option<&SteamworksInputAnalogActionSnapshot> {
        self.last_analog_action.as_ref()
    }

    /// Returns cached analog action data snapshots keyed by controller and action.
    pub fn analog_action_data_snapshots(&self) -> &[SteamworksInputAnalogActionSnapshot] {
        &self.analog_action_snapshots
    }

    /// Returns the cached analog action data snapshot for a controller/action pair.
    pub fn analog_action_data(
        &self,
        controller: SteamworksInputHandle,
        action: SteamworksInputAnalogActionHandle,
    ) -> Option<&SteamworksInputAnalogActionSnapshot> {
        self.analog_action_snapshots
            .iter()
            .find(|snapshot| snapshot.controller == controller && snapshot.action == action)
    }

    /// Returns the cached analog action source mode.
    pub fn analog_action_mode(
        &self,
        controller: SteamworksInputHandle,
        action: SteamworksInputAnalogActionHandle,
    ) -> Option<SteamworksInputSourceMode> {
        self.analog_action_data(controller, action)
            .map(|snapshot| snapshot.data.mode)
    }

    /// Returns the cached analog action x/y values.
    pub fn analog_action_vector(
        &self,
        controller: SteamworksInputHandle,
        action: SteamworksInputAnalogActionHandle,
    ) -> Option<(f32, f32)> {
        self.analog_action_data(controller, action)
            .map(|snapshot| (snapshot.data.x, snapshot.data.y))
    }

    /// Returns whether a cached analog action is active in the current action set.
    pub fn analog_action_active(
        &self,
        controller: SteamworksInputHandle,
        action: SteamworksInputAnalogActionHandle,
    ) -> Option<bool> {
        self.analog_action_data(controller, action)
            .map(|snapshot| snapshot.data.active)
    }

    /// Returns the most recent digital action origin snapshot.
    pub fn last_digital_action_origins(
        &self,
    ) -> Option<&SteamworksInputDigitalActionOriginsSnapshot> {
        self.last_digital_action_origins.as_ref()
    }

    /// Returns cached digital action origin snapshots keyed by controller, action set, and action.
    pub fn digital_action_origin_snapshots(
        &self,
    ) -> &[SteamworksInputDigitalActionOriginsSnapshot] {
        &self.digital_action_origin_snapshots
    }

    /// Returns the cached digital action origins for a controller/action-set/action triple.
    pub fn digital_action_origins(
        &self,
        controller: SteamworksInputHandle,
        action_set: SteamworksInputActionSetHandle,
        action: SteamworksInputDigitalActionHandle,
    ) -> Option<&SteamworksInputDigitalActionOriginsSnapshot> {
        self.digital_action_origin_snapshots
            .iter()
            .find(|snapshot| {
                snapshot.controller == controller
                    && snapshot.action_set == action_set
                    && snapshot.action == action
            })
    }

    /// Returns the most recent analog action origin snapshot.
    pub fn last_analog_action_origins(
        &self,
    ) -> Option<&SteamworksInputAnalogActionOriginsSnapshot> {
        self.last_analog_action_origins.as_ref()
    }

    /// Returns cached analog action origin snapshots keyed by controller, action set, and action.
    pub fn analog_action_origin_snapshots(&self) -> &[SteamworksInputAnalogActionOriginsSnapshot] {
        &self.analog_action_origin_snapshots
    }

    /// Returns the cached analog action origins for a controller/action-set/action triple.
    pub fn analog_action_origins(
        &self,
        controller: SteamworksInputHandle,
        action_set: SteamworksInputActionSetHandle,
        action: SteamworksInputAnalogActionHandle,
    ) -> Option<&SteamworksInputAnalogActionOriginsSnapshot> {
        self.analog_action_origin_snapshots.iter().find(|snapshot| {
            snapshot.controller == controller
                && snapshot.action_set == action_set
                && snapshot.action == action
        })
    }

    /// Returns cached action origin presentation data read from origin queries.
    pub fn action_origin_infos(&self) -> &[SteamworksInputActionOriginInfo] {
        &self.action_origin_infos
    }

    /// Returns cached presentation data for one action origin.
    pub fn action_origin_info(
        &self,
        origin: SteamworksInputActionOrigin,
    ) -> Option<&SteamworksInputActionOriginInfo> {
        self.action_origin_infos
            .iter()
            .find(|info| info.origin == origin)
    }

    /// Returns the most recent action origin presentation snapshot.
    pub fn last_action_origin_info(&self) -> Option<&SteamworksInputActionOriginInfo> {
        self.last_action_origin_info.as_ref()
    }

    /// Returns the most recent motion data snapshot.
    pub fn last_motion(&self) -> Option<&SteamworksInputMotionSnapshot> {
        self.last_motion.as_ref()
    }

    /// Returns cached motion snapshots keyed by controller.
    pub fn motion_snapshots(&self) -> &[SteamworksInputMotionSnapshot] {
        &self.motion_snapshots
    }

    /// Returns the cached motion snapshot for a controller.
    pub fn motion(
        &self,
        controller: SteamworksInputHandle,
    ) -> Option<&SteamworksInputMotionSnapshot> {
        self.motion_snapshots
            .iter()
            .find(|snapshot| snapshot.controller == controller)
    }

    /// Returns the cached rotation quaternion for a controller.
    pub fn motion_rotation_quaternion(
        &self,
        controller: SteamworksInputHandle,
    ) -> Option<[f32; 4]> {
        self.motion(controller)
            .map(|snapshot| snapshot.data.rotation_quaternion)
    }

    /// Returns the cached position acceleration for a controller.
    pub fn motion_position_acceleration(
        &self,
        controller: SteamworksInputHandle,
    ) -> Option<[f32; 3]> {
        self.motion(controller)
            .map(|snapshot| snapshot.data.position_acceleration)
    }

    /// Returns the cached rotation velocity for a controller.
    pub fn motion_rotation_velocity(&self, controller: SteamworksInputHandle) -> Option<[f32; 3]> {
        self.motion(controller)
            .map(|snapshot| snapshot.data.rotation_velocity)
    }

    /// Returns the most recent controller for which the binding panel was shown.
    pub fn last_binding_panel_controller(&self) -> Option<SteamworksInputHandle> {
        self.last_binding_panel_controller
    }
}
