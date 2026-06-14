use super::super::{
    SteamworksInputActionOriginInfo, SteamworksInputActionSetHandle,
    SteamworksInputAnalogActionHandle, SteamworksInputAnalogActionSnapshot,
    SteamworksInputControllerInfo, SteamworksInputDigitalActionHandle,
    SteamworksInputDigitalActionSnapshot, SteamworksInputHandle, SteamworksInputMotionSnapshot,
};

/// A successfully submitted Steam Input operation or synchronous read.
#[derive(Clone, Debug, PartialEq)]
pub enum SteamworksInputOperation {
    /// Steam Input was initialized.
    Initialized {
        /// Whether manual Steam Input frame updates were requested.
        explicitly_call_run_frame: bool,
    },
    /// Steam Input frame state was synchronized.
    FrameRun,
    /// Steam Input was shut down.
    Shutdown,
    /// Connected controllers were listed.
    ControllersListed {
        /// Connected controllers and their input types.
        controllers: Vec<SteamworksInputControllerInfo>,
    },
    /// One controller's input type was read.
    ControllerInfoRead {
        /// Controller information.
        controller: SteamworksInputControllerInfo,
    },
    /// Action manifest path was accepted by Steam Input.
    ActionManifestFilePathSet {
        /// Local path submitted to Steam Input.
        path: String,
    },
    /// Action set handle was read.
    ActionSetHandleRead {
        /// Action set name used for lookup.
        name: String,
        /// Action set handle returned by Steam Input.
        handle: SteamworksInputActionSetHandle,
    },
    /// Digital action handle was read.
    DigitalActionHandleRead {
        /// Digital action name used for lookup.
        name: String,
        /// Digital action handle returned by Steam Input.
        handle: SteamworksInputDigitalActionHandle,
    },
    /// Analog action handle was read.
    AnalogActionHandleRead {
        /// Analog action name used for lookup.
        name: String,
        /// Analog action handle returned by Steam Input.
        handle: SteamworksInputAnalogActionHandle,
    },
    /// Action set was activated for a controller.
    ActionSetActivated {
        /// Controller configured.
        controller: SteamworksInputHandle,
        /// Action set activated.
        action_set: SteamworksInputActionSetHandle,
    },
    /// Digital action data was read.
    DigitalActionDataRead {
        /// Digital action snapshot.
        snapshot: SteamworksInputDigitalActionSnapshot,
    },
    /// Analog action data was read.
    AnalogActionDataRead {
        /// Analog action snapshot.
        snapshot: SteamworksInputAnalogActionSnapshot,
    },
    /// Digital action origins were read.
    DigitalActionOriginsRead {
        /// Controller inspected.
        controller: SteamworksInputHandle,
        /// Action set context.
        action_set: SteamworksInputActionSetHandle,
        /// Digital action inspected.
        action: SteamworksInputDigitalActionHandle,
        /// Origin presentation data.
        origins: Vec<SteamworksInputActionOriginInfo>,
    },
    /// Analog action origins were read.
    AnalogActionOriginsRead {
        /// Controller inspected.
        controller: SteamworksInputHandle,
        /// Action set context.
        action_set: SteamworksInputActionSetHandle,
        /// Analog action inspected.
        action: SteamworksInputAnalogActionHandle,
        /// Origin presentation data.
        origins: Vec<SteamworksInputActionOriginInfo>,
    },
    /// Motion data was read.
    MotionDataRead {
        /// Motion data snapshot.
        snapshot: SteamworksInputMotionSnapshot,
    },
    /// Steam Input binding panel was shown.
    BindingPanelShown {
        /// Controller configured.
        controller: SteamworksInputHandle,
    },
}
