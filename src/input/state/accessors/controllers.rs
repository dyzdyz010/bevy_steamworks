use crate::input::*;

impl SteamworksInputState {
    /// Returns the known controller snapshots read through the plugin.
    pub fn controllers(&self) -> &[SteamworksInputControllerInfo] {
        &self.controllers
    }

    /// Returns the number of known controller snapshots cached by this plugin.
    pub fn controller_count(&self) -> usize {
        self.controllers.len()
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

    /// Returns whether a controller snapshot is cached for a handle.
    pub fn has_controller(&self, handle: SteamworksInputHandle) -> bool {
        self.controller(handle).is_some()
    }

    /// Returns the cached controller type for a handle.
    pub fn controller_input_type(
        &self,
        handle: SteamworksInputHandle,
    ) -> Option<SteamworksInputType> {
        self.controller(handle)
            .map(|controller| controller.input_type)
    }
}
