use crate::input::*;

impl SteamworksInputState {
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

    /// Returns the number of cached action set activations.
    pub fn action_set_activation_count(&self) -> usize {
        self.action_set_activations.len()
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

    /// Returns whether an action set activation is cached for a controller.
    pub fn has_action_set_activation(&self, controller: SteamworksInputHandle) -> bool {
        self.action_set_activation(controller).is_some()
    }

    /// Returns the cached active action set handle for a controller.
    pub fn active_action_set(
        &self,
        controller: SteamworksInputHandle,
    ) -> Option<SteamworksInputActionSetHandle> {
        self.action_set_activation(controller)
            .map(|activation| activation.action_set)
    }
}
