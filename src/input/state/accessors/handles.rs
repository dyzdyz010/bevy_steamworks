use crate::input::*;

impl SteamworksInputState {
    /// Returns action set handles read through the plugin.
    pub fn action_sets(&self) -> &[SteamworksInputNamedActionSetHandle] {
        &self.action_sets
    }

    /// Returns the number of named action set handles cached by this plugin.
    pub fn action_set_count(&self) -> usize {
        self.action_sets.len()
    }

    /// Returns the cached action set handle for a manifest action set name.
    pub fn action_set_handle(&self, name: &str) -> Option<SteamworksInputActionSetHandle> {
        self.action_sets
            .iter()
            .find_map(|handle| (handle.name == name).then_some(handle.handle))
    }

    /// Returns whether a named action set handle is cached.
    pub fn has_action_set_handle(&self, name: &str) -> bool {
        self.action_set_handle(name).is_some()
    }

    /// Returns digital action handles read through the plugin.
    pub fn digital_actions(&self) -> &[SteamworksInputNamedDigitalActionHandle] {
        &self.digital_actions
    }

    /// Returns the number of named digital action handles cached by this plugin.
    pub fn digital_action_count(&self) -> usize {
        self.digital_actions.len()
    }

    /// Returns the cached digital action handle for a manifest action name.
    pub fn digital_action_handle(&self, name: &str) -> Option<SteamworksInputDigitalActionHandle> {
        self.digital_actions
            .iter()
            .find_map(|handle| (handle.name == name).then_some(handle.handle))
    }

    /// Returns whether a named digital action handle is cached.
    pub fn has_digital_action_handle(&self, name: &str) -> bool {
        self.digital_action_handle(name).is_some()
    }

    /// Returns analog action handles read through the plugin.
    pub fn analog_actions(&self) -> &[SteamworksInputNamedAnalogActionHandle] {
        &self.analog_actions
    }

    /// Returns the number of named analog action handles cached by this plugin.
    pub fn analog_action_count(&self) -> usize {
        self.analog_actions.len()
    }

    /// Returns the cached analog action handle for a manifest action name.
    pub fn analog_action_handle(&self, name: &str) -> Option<SteamworksInputAnalogActionHandle> {
        self.analog_actions
            .iter()
            .find_map(|handle| (handle.name == name).then_some(handle.handle))
    }

    /// Returns whether a named analog action handle is cached.
    pub fn has_analog_action_handle(&self, name: &str) -> bool {
        self.analog_action_handle(name).is_some()
    }
}
