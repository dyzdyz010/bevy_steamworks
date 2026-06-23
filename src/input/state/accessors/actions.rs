use crate::input::*;

impl SteamworksInputState {
    /// Returns the most recent digital action data snapshot.
    pub fn last_digital_action(&self) -> Option<&SteamworksInputDigitalActionSnapshot> {
        self.last_digital_action.as_ref()
    }

    /// Returns cached digital action data snapshots keyed by controller and action.
    pub fn digital_action_data_snapshots(&self) -> &[SteamworksInputDigitalActionSnapshot] {
        &self.digital_action_snapshots
    }

    /// Returns the number of cached digital action data snapshots.
    pub fn digital_action_snapshot_count(&self) -> usize {
        self.digital_action_snapshots.len()
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

    /// Returns the number of cached analog action data snapshots.
    pub fn analog_action_snapshot_count(&self) -> usize {
        self.analog_action_snapshots.len()
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
}
