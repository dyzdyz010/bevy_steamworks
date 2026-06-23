use crate::input::*;

impl SteamworksInputState {
    /// Returns the most recent motion data snapshot.
    pub fn last_motion(&self) -> Option<&SteamworksInputMotionSnapshot> {
        self.last_motion.as_ref()
    }

    /// Returns cached motion snapshots keyed by controller.
    pub fn motion_snapshots(&self) -> &[SteamworksInputMotionSnapshot] {
        &self.motion_snapshots
    }

    /// Returns the number of cached motion snapshots.
    pub fn motion_snapshot_count(&self) -> usize {
        self.motion_snapshots.len()
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

    /// Returns whether motion data is cached for a controller.
    pub fn has_motion(&self, controller: SteamworksInputHandle) -> bool {
        self.motion(controller).is_some()
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

    /// Returns whether this plugin observed a successful binding panel request.
    pub fn binding_panel_was_shown(&self) -> bool {
        self.last_binding_panel_controller.is_some()
    }
}
