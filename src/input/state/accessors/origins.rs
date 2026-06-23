use crate::input::*;

impl SteamworksInputState {
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

    /// Returns the number of cached digital action origin snapshots.
    pub fn digital_action_origin_snapshot_count(&self) -> usize {
        self.digital_action_origin_snapshots.len()
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

    /// Returns the number of cached origins for a controller/action-set/digital-action triple.
    pub fn digital_action_origin_count(
        &self,
        controller: SteamworksInputHandle,
        action_set: SteamworksInputActionSetHandle,
        action: SteamworksInputDigitalActionHandle,
    ) -> Option<usize> {
        self.digital_action_origins(controller, action_set, action)
            .map(|snapshot| snapshot.origins.len())
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

    /// Returns the number of cached analog action origin snapshots.
    pub fn analog_action_origin_snapshot_count(&self) -> usize {
        self.analog_action_origin_snapshots.len()
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

    /// Returns the number of cached origins for a controller/action-set/analog-action triple.
    pub fn analog_action_origin_count(
        &self,
        controller: SteamworksInputHandle,
        action_set: SteamworksInputActionSetHandle,
        action: SteamworksInputAnalogActionHandle,
    ) -> Option<usize> {
        self.analog_action_origins(controller, action_set, action)
            .map(|snapshot| snapshot.origins.len())
    }

    /// Returns cached action origin presentation data read from origin queries.
    pub fn action_origin_infos(&self) -> &[SteamworksInputActionOriginInfo] {
        &self.action_origin_infos
    }

    /// Returns the number of cached action origin presentation snapshots.
    pub fn action_origin_info_count(&self) -> usize {
        self.action_origin_infos.len()
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

    /// Returns whether action origin presentation data is cached for an origin.
    pub fn has_action_origin_info(&self, origin: SteamworksInputActionOrigin) -> bool {
        self.action_origin_info(origin).is_some()
    }

    /// Returns the cached glyph path for an action origin.
    pub fn action_origin_glyph_path(&self, origin: SteamworksInputActionOrigin) -> Option<&str> {
        self.action_origin_info(origin)
            .map(|info| info.glyph_path.as_str())
    }

    /// Returns the cached localized name for an action origin.
    pub fn action_origin_name(&self, origin: SteamworksInputActionOrigin) -> Option<&str> {
        self.action_origin_info(origin)
            .map(|info| info.name.as_str())
    }

    /// Returns the most recent action origin presentation snapshot.
    pub fn last_action_origin_info(&self) -> Option<&SteamworksInputActionOriginInfo> {
        self.last_action_origin_info.as_ref()
    }
}
