//! High-level Bevy ECS integration for Steam Input.
//!
//! This module builds on top of the upstream [`steamworks::Input`] API. It
//! exposes Bevy messages for common controller, action set, and action data
//! workflows while keeping raw Steamworks SDK binding types out of this crate's
//! public contract.

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksSystem};

const STEAM_INPUT_MAX_CONTROLLER_COUNT: usize = 16;

/// Bevy plugin for high-level Steam Input commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksInputCommand`] and [`SteamworksInputResult`] messages and runs
/// its command processor in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksInputPlugin;

impl SteamworksInputPlugin {
    /// Creates a Steam Input plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksInputState>()
            .add_message::<SteamworksInputCommand>()
            .add_message::<SteamworksInputResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessInputCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_input_commands.in_set(SteamworksSystem::ProcessInputCommands),
            );
    }
}

/// Runtime state for [`SteamworksInputPlugin`].
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

    fn record_error(&mut self, error: SteamworksInputError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksInputOperation) {
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

/// A Steam Input controller handle.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SteamworksInputHandle(u64);

impl SteamworksInputHandle {
    /// Creates a controller handle from a raw Steam Input handle value.
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw Steam Input handle value.
    pub const fn raw(self) -> u64 {
        self.0
    }

    fn is_valid(self) -> bool {
        self.0 != 0
    }
}

/// A Steam Input action set handle.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SteamworksInputActionSetHandle(u64);

impl SteamworksInputActionSetHandle {
    /// Creates an action set handle from a raw Steam Input handle value.
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw Steam Input handle value.
    pub const fn raw(self) -> u64 {
        self.0
    }

    fn is_valid(self) -> bool {
        self.0 != 0
    }
}

/// A Steam Input digital action handle.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SteamworksInputDigitalActionHandle(u64);

impl SteamworksInputDigitalActionHandle {
    /// Creates a digital action handle from a raw Steam Input handle value.
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw Steam Input handle value.
    pub const fn raw(self) -> u64 {
        self.0
    }

    fn is_valid(self) -> bool {
        self.0 != 0
    }
}

/// A Steam Input analog action handle.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SteamworksInputAnalogActionHandle(u64);

impl SteamworksInputAnalogActionHandle {
    /// Creates an analog action handle from a raw Steam Input handle value.
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the raw Steam Input handle value.
    pub const fn raw(self) -> u64 {
        self.0
    }

    fn is_valid(self) -> bool {
        self.0 != 0
    }
}

/// Snapshot of a connected Steam Input controller.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksInputControllerInfo {
    /// Controller handle.
    pub handle: SteamworksInputHandle,
    /// Controller type reported by Steam Input.
    pub input_type: SteamworksInputType,
}

/// Controller type reported by Steam Input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksInputType {
    /// Unknown or unsupported controller.
    Unknown,
    /// Steam Controller.
    SteamController,
    /// Xbox 360 controller.
    XBox360Controller,
    /// Xbox One controller.
    XBoxOneController,
    /// Generic gamepad.
    GenericGamepad,
    /// PlayStation 4 controller.
    PS4Controller,
    /// Apple MFi controller.
    AppleMFiController,
    /// Android controller.
    AndroidController,
    /// Nintendo Switch Joy-Con pair.
    SwitchJoyConPair,
    /// Single Nintendo Switch Joy-Con.
    SwitchJoyConSingle,
    /// Nintendo Switch Pro controller.
    SwitchProController,
    /// Mobile touch input.
    MobileTouch,
    /// PlayStation 3 controller.
    PS3Controller,
    /// PlayStation 5 controller.
    PS5Controller,
    /// Steam Deck built-in controller.
    SteamDeckController,
}

/// Steam Input analog source mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksInputSourceMode {
    /// No source mode.
    None,
    /// D-pad mode.
    Dpad,
    /// Buttons mode.
    Buttons,
    /// Four-buttons mode.
    FourButtons,
    /// Absolute mouse mode.
    AbsoluteMouse,
    /// Relative mouse mode.
    RelativeMouse,
    /// Joystick movement mode.
    JoystickMove,
    /// Joystick mouse mode.
    JoystickMouse,
    /// Joystick camera mode.
    JoystickCamera,
    /// Scroll wheel mode.
    ScrollWheel,
    /// Trigger mode.
    Trigger,
    /// Touch menu mode.
    TouchMenu,
    /// Mouse joystick mode.
    MouseJoystick,
    /// Mouse region mode.
    MouseRegion,
    /// Radial menu mode.
    RadialMenu,
    /// Single button mode.
    SingleButton,
    /// Switches mode.
    Switches,
    /// Unknown source mode value.
    Unknown(i32),
}

/// A named Steam Input action set handle cached in plugin state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksInputNamedActionSetHandle {
    /// Action set name used for lookup.
    pub name: String,
    /// Action set handle returned by Steam Input.
    pub handle: SteamworksInputActionSetHandle,
}

/// A named Steam Input digital action handle cached in plugin state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksInputNamedDigitalActionHandle {
    /// Digital action name used for lookup.
    pub name: String,
    /// Digital action handle returned by Steam Input.
    pub handle: SteamworksInputDigitalActionHandle,
}

/// A named Steam Input analog action handle cached in plugin state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksInputNamedAnalogActionHandle {
    /// Analog action name used for lookup.
    pub name: String,
    /// Analog action handle returned by Steam Input.
    pub handle: SteamworksInputAnalogActionHandle,
}

/// Steam Input digital action data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksInputDigitalActionData {
    /// Whether the digital action is currently pressed.
    pub state: bool,
    /// Whether the digital action is active in the current action set.
    pub active: bool,
}

/// Steam Input analog action data.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksInputAnalogActionData {
    /// Source mode for this analog action.
    pub mode: SteamworksInputSourceMode,
    /// X axis value.
    pub x: f32,
    /// Y axis value.
    pub y: f32,
    /// Whether the analog action is active in the current action set.
    pub active: bool,
}

/// Action set activation context submitted through this plugin.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SteamworksInputActionSetActivation {
    /// Controller configured.
    pub controller: SteamworksInputHandle,
    /// Action set activated.
    pub action_set: SteamworksInputActionSetHandle,
}

/// Steam Input motion data.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksInputMotionData {
    /// Rotation quaternion `[x, y, z, w]`.
    pub rotation_quaternion: [f32; 4],
    /// Position acceleration `[x, y, z]`.
    pub position_acceleration: [f32; 3],
    /// Rotation velocity `[x, y, z]`.
    pub rotation_velocity: [f32; 3],
}

/// Digital action data with the controller and action handle context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksInputDigitalActionSnapshot {
    /// Controller that was sampled.
    pub controller: SteamworksInputHandle,
    /// Digital action that was sampled.
    pub action: SteamworksInputDigitalActionHandle,
    /// Digital action data returned by Steam Input.
    pub data: SteamworksInputDigitalActionData,
}

/// Analog action data with the controller and action handle context.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksInputAnalogActionSnapshot {
    /// Controller that was sampled.
    pub controller: SteamworksInputHandle,
    /// Analog action that was sampled.
    pub action: SteamworksInputAnalogActionHandle,
    /// Analog action data returned by Steam Input.
    pub data: SteamworksInputAnalogActionData,
}

/// Digital action origins with controller, action set, and action context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksInputDigitalActionOriginsSnapshot {
    /// Controller that was inspected.
    pub controller: SteamworksInputHandle,
    /// Active action set context.
    pub action_set: SteamworksInputActionSetHandle,
    /// Digital action that was inspected.
    pub action: SteamworksInputDigitalActionHandle,
    /// Origin presentation data returned by Steam Input.
    pub origins: Vec<SteamworksInputActionOriginInfo>,
}

/// Analog action origins with controller, action set, and action context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksInputAnalogActionOriginsSnapshot {
    /// Controller that was inspected.
    pub controller: SteamworksInputHandle,
    /// Active action set context.
    pub action_set: SteamworksInputActionSetHandle,
    /// Analog action that was inspected.
    pub action: SteamworksInputAnalogActionHandle,
    /// Origin presentation data returned by Steam Input.
    pub origins: Vec<SteamworksInputActionOriginInfo>,
}

/// Motion data with the controller handle context.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksInputMotionSnapshot {
    /// Controller that was sampled.
    pub controller: SteamworksInputHandle,
    /// Motion data returned by Steam Input.
    pub data: SteamworksInputMotionData,
}

/// Action origin presentation data returned by Steam Input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksInputActionOriginInfo {
    /// Opaque Steam Input action origin code.
    pub origin: SteamworksInputActionOrigin,
    /// Glyph path returned by Steam Input.
    pub glyph_path: String,
    /// Localized action origin name returned by Steam Input.
    pub name: String,
}

/// Opaque Steam Input action origin code.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SteamworksInputActionOrigin(i32);

impl SteamworksInputActionOrigin {
    /// Creates an action origin from a Steam Input origin code.
    pub const fn from_code(code: i32) -> Self {
        Self(code)
    }

    /// Returns the Steam Input origin code.
    pub const fn code(self) -> i32 {
        self.0
    }
}

/// A high-level command for Steam Input workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksInputCommand {
    /// Initialize the Steam Input interface.
    Init {
        /// Whether the caller will explicitly run Steam Input frames.
        explicitly_call_run_frame: bool,
    },
    /// Synchronize Steam Input state immediately.
    RunFrame,
    /// Shut down the Steam Input interface.
    Shutdown,
    /// Read all connected controllers and their input types.
    ListControllers,
    /// Read one controller type.
    GetControllerInfo {
        /// Controller to inspect.
        controller: SteamworksInputHandle,
    },
    /// Load a local Steam Input action manifest file.
    SetActionManifestFilePath {
        /// Local path to the action manifest file.
        path: String,
    },
    /// Read an action set handle by name.
    GetActionSetHandle {
        /// Action set name from the action manifest.
        name: String,
    },
    /// Read a digital action handle by name.
    GetDigitalActionHandle {
        /// Digital action name from the action manifest.
        name: String,
    },
    /// Read an analog action handle by name.
    GetAnalogActionHandle {
        /// Analog action name from the action manifest.
        name: String,
    },
    /// Activate an action set for a controller.
    ActivateActionSet {
        /// Controller to configure.
        controller: SteamworksInputHandle,
        /// Action set to activate.
        action_set: SteamworksInputActionSetHandle,
    },
    /// Read digital action data for a controller/action pair.
    GetDigitalActionData {
        /// Controller to sample.
        controller: SteamworksInputHandle,
        /// Digital action to sample.
        action: SteamworksInputDigitalActionHandle,
    },
    /// Read analog action data for a controller/action pair.
    GetAnalogActionData {
        /// Controller to sample.
        controller: SteamworksInputHandle,
        /// Analog action to sample.
        action: SteamworksInputAnalogActionHandle,
    },
    /// Read digital action origins and their presentation strings.
    GetDigitalActionOrigins {
        /// Controller to inspect.
        controller: SteamworksInputHandle,
        /// Active action set context.
        action_set: SteamworksInputActionSetHandle,
        /// Digital action to inspect.
        action: SteamworksInputDigitalActionHandle,
    },
    /// Read analog action origins and their presentation strings.
    GetAnalogActionOrigins {
        /// Controller to inspect.
        controller: SteamworksInputHandle,
        /// Active action set context.
        action_set: SteamworksInputActionSetHandle,
        /// Analog action to inspect.
        action: SteamworksInputAnalogActionHandle,
    },
    /// Read motion data for a controller.
    GetMotionData {
        /// Controller to sample.
        controller: SteamworksInputHandle,
    },
    /// Show the Steam Input binding panel for a controller.
    ShowBindingPanel {
        /// Controller to configure.
        controller: SteamworksInputHandle,
    },
}

impl SteamworksInputCommand {
    /// Creates a [`SteamworksInputCommand::Init`] command.
    pub fn init(explicitly_call_run_frame: bool) -> Self {
        Self::Init {
            explicitly_call_run_frame,
        }
    }

    /// Creates a [`SteamworksInputCommand::SetActionManifestFilePath`] command.
    pub fn set_action_manifest_file_path(path: impl Into<String>) -> Self {
        Self::SetActionManifestFilePath { path: path.into() }
    }

    /// Creates a [`SteamworksInputCommand::GetActionSetHandle`] command.
    pub fn get_action_set_handle(name: impl Into<String>) -> Self {
        Self::GetActionSetHandle { name: name.into() }
    }

    /// Creates a [`SteamworksInputCommand::GetDigitalActionHandle`] command.
    pub fn get_digital_action_handle(name: impl Into<String>) -> Self {
        Self::GetDigitalActionHandle { name: name.into() }
    }

    /// Creates a [`SteamworksInputCommand::GetAnalogActionHandle`] command.
    pub fn get_analog_action_handle(name: impl Into<String>) -> Self {
        Self::GetAnalogActionHandle { name: name.into() }
    }

    /// Creates a [`SteamworksInputCommand::ActivateActionSet`] command.
    pub fn activate_action_set(
        controller: SteamworksInputHandle,
        action_set: SteamworksInputActionSetHandle,
    ) -> Self {
        Self::ActivateActionSet {
            controller,
            action_set,
        }
    }

    /// Creates a [`SteamworksInputCommand::GetDigitalActionData`] command.
    pub fn get_digital_action_data(
        controller: SteamworksInputHandle,
        action: SteamworksInputDigitalActionHandle,
    ) -> Self {
        Self::GetDigitalActionData { controller, action }
    }

    /// Creates a [`SteamworksInputCommand::GetAnalogActionData`] command.
    pub fn get_analog_action_data(
        controller: SteamworksInputHandle,
        action: SteamworksInputAnalogActionHandle,
    ) -> Self {
        Self::GetAnalogActionData { controller, action }
    }
}

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

/// Result message emitted by [`SteamworksInputPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksInputResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksInputOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksInputCommand,
        /// Failure reason.
        error: SteamworksInputError,
    },
}

/// Synchronous errors from [`SteamworksInputPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksInputError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks Input command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A handle passed to Steam is zero.
    #[error("Steamworks Input command field {field} contains an invalid zero handle")]
    InvalidHandle {
        /// Field that contained the invalid handle.
        field: &'static str,
    },
    /// Steam Input initialization returned false.
    #[error("Steam Input initialization failed")]
    InitFailed,
    /// Steam Input rejected the action manifest path.
    #[error("Steam Input rejected the action manifest path")]
    ActionManifestFileRejected,
    /// Steam Input returned an invalid zero handle for a lookup.
    #[error("Steam Input returned an invalid zero handle for {operation}")]
    InvalidHandleReturned {
        /// Lookup operation that returned an invalid handle.
        operation: &'static str,
    },
    /// Steam Input could not show the binding panel.
    #[error("Steam Input binding panel is unavailable")]
    BindingPanelUnavailable,
}

impl SteamworksInputError {
    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    fn invalid_handle(field: &'static str) -> Self {
        Self::InvalidHandle { field }
    }

    fn invalid_handle_returned(operation: &'static str) -> Self {
        Self::InvalidHandleReturned { operation }
    }
}

fn process_input_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksInputState>,
    mut commands: ResMut<Messages<SteamworksInputCommand>>,
    mut results: MessageWriter<SteamworksInputResult>,
) {
    let Some(client) = client else {
        let error = SteamworksInputError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks Input command failed"
            );
            results.write(SteamworksInputResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    let input = client.input();
    for command in commands.drain() {
        match handle_input_command(&input, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks Input command"
                );
                results.write(SteamworksInputResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks Input command failed"
                );
                results.write(SteamworksInputResult::Err { command, error });
            }
        }
    }
}

fn handle_input_command(
    input: &steamworks::Input,
    command: &SteamworksInputCommand,
) -> Result<SteamworksInputOperation, SteamworksInputError> {
    validate_command(command)?;

    match command {
        SteamworksInputCommand::Init {
            explicitly_call_run_frame,
        } => {
            if input.init(*explicitly_call_run_frame) {
                Ok(SteamworksInputOperation::Initialized {
                    explicitly_call_run_frame: *explicitly_call_run_frame,
                })
            } else {
                Err(SteamworksInputError::InitFailed)
            }
        }
        SteamworksInputCommand::RunFrame => {
            input.run_frame();
            Ok(SteamworksInputOperation::FrameRun)
        }
        SteamworksInputCommand::Shutdown => {
            input.shutdown();
            Ok(SteamworksInputOperation::Shutdown)
        }
        SteamworksInputCommand::ListControllers => {
            let controllers = snapshot_connected_controllers(input);
            Ok(SteamworksInputOperation::ControllersListed { controllers })
        }
        SteamworksInputCommand::GetControllerInfo { controller } => {
            Ok(SteamworksInputOperation::ControllerInfoRead {
                controller: snapshot_controller_info(input, *controller),
            })
        }
        SteamworksInputCommand::SetActionManifestFilePath { path } => {
            if input.set_input_action_manifest_file_path(path) {
                Ok(SteamworksInputOperation::ActionManifestFilePathSet { path: path.clone() })
            } else {
                Err(SteamworksInputError::ActionManifestFileRejected)
            }
        }
        SteamworksInputCommand::GetActionSetHandle { name } => {
            let handle =
                SteamworksInputActionSetHandle::from_raw(input.get_action_set_handle(name));
            if handle.is_valid() {
                Ok(SteamworksInputOperation::ActionSetHandleRead {
                    name: name.clone(),
                    handle,
                })
            } else {
                Err(SteamworksInputError::invalid_handle_returned(
                    "GetActionSetHandle",
                ))
            }
        }
        SteamworksInputCommand::GetDigitalActionHandle { name } => {
            let handle =
                SteamworksInputDigitalActionHandle::from_raw(input.get_digital_action_handle(name));
            if handle.is_valid() {
                Ok(SteamworksInputOperation::DigitalActionHandleRead {
                    name: name.clone(),
                    handle,
                })
            } else {
                Err(SteamworksInputError::invalid_handle_returned(
                    "GetDigitalActionHandle",
                ))
            }
        }
        SteamworksInputCommand::GetAnalogActionHandle { name } => {
            let handle =
                SteamworksInputAnalogActionHandle::from_raw(input.get_analog_action_handle(name));
            if handle.is_valid() {
                Ok(SteamworksInputOperation::AnalogActionHandleRead {
                    name: name.clone(),
                    handle,
                })
            } else {
                Err(SteamworksInputError::invalid_handle_returned(
                    "GetAnalogActionHandle",
                ))
            }
        }
        SteamworksInputCommand::ActivateActionSet {
            controller,
            action_set,
        } => {
            input.activate_action_set_handle(controller.raw(), action_set.raw());
            Ok(SteamworksInputOperation::ActionSetActivated {
                controller: *controller,
                action_set: *action_set,
            })
        }
        SteamworksInputCommand::GetDigitalActionData { controller, action } => {
            let data = input.get_digital_action_data(controller.raw(), action.raw());
            let state = data.bState;
            let active = data.bActive;
            Ok(SteamworksInputOperation::DigitalActionDataRead {
                snapshot: SteamworksInputDigitalActionSnapshot {
                    controller: *controller,
                    action: *action,
                    data: SteamworksInputDigitalActionData { state, active },
                },
            })
        }
        SteamworksInputCommand::GetAnalogActionData { controller, action } => {
            let data = input.get_analog_action_data(controller.raw(), action.raw());
            let raw_mode = data.eMode as i32;
            let x = data.x;
            let y = data.y;
            let active = data.bActive;
            Ok(SteamworksInputOperation::AnalogActionDataRead {
                snapshot: SteamworksInputAnalogActionSnapshot {
                    controller: *controller,
                    action: *action,
                    data: SteamworksInputAnalogActionData {
                        mode: SteamworksInputSourceMode::from_raw(raw_mode),
                        x,
                        y,
                        active,
                    },
                },
            })
        }
        SteamworksInputCommand::GetDigitalActionOrigins {
            controller,
            action_set,
            action,
        } => {
            let origins =
                input.get_digital_action_origins(controller.raw(), action_set.raw(), action.raw());
            let origins = origins
                .into_iter()
                .map(|origin| SteamworksInputActionOriginInfo {
                    origin: SteamworksInputActionOrigin::from_code(origin as i32),
                    glyph_path: input.get_glyph_for_action_origin(origin),
                    name: input.get_string_for_action_origin(origin),
                })
                .collect();
            Ok(SteamworksInputOperation::DigitalActionOriginsRead {
                controller: *controller,
                action_set: *action_set,
                action: *action,
                origins,
            })
        }
        SteamworksInputCommand::GetAnalogActionOrigins {
            controller,
            action_set,
            action,
        } => {
            let origins =
                input.get_analog_action_origins(controller.raw(), action_set.raw(), action.raw());
            let origins = origins
                .into_iter()
                .map(|origin| SteamworksInputActionOriginInfo {
                    origin: SteamworksInputActionOrigin::from_code(origin as i32),
                    glyph_path: input.get_glyph_for_action_origin(origin),
                    name: input.get_string_for_action_origin(origin),
                })
                .collect();
            Ok(SteamworksInputOperation::AnalogActionOriginsRead {
                controller: *controller,
                action_set: *action_set,
                action: *action,
                origins,
            })
        }
        SteamworksInputCommand::GetMotionData { controller } => {
            let data = input.get_motion_data(controller.raw());
            let rot_quat_x = data.rotQuatX;
            let rot_quat_y = data.rotQuatY;
            let rot_quat_z = data.rotQuatZ;
            let rot_quat_w = data.rotQuatW;
            let pos_accel_x = data.posAccelX;
            let pos_accel_y = data.posAccelY;
            let pos_accel_z = data.posAccelZ;
            let rot_vel_x = data.rotVelX;
            let rot_vel_y = data.rotVelY;
            let rot_vel_z = data.rotVelZ;
            Ok(SteamworksInputOperation::MotionDataRead {
                snapshot: SteamworksInputMotionSnapshot {
                    controller: *controller,
                    data: SteamworksInputMotionData {
                        rotation_quaternion: [rot_quat_x, rot_quat_y, rot_quat_z, rot_quat_w],
                        position_acceleration: [pos_accel_x, pos_accel_y, pos_accel_z],
                        rotation_velocity: [rot_vel_x, rot_vel_y, rot_vel_z],
                    },
                },
            })
        }
        SteamworksInputCommand::ShowBindingPanel { controller } => {
            if input.show_binding_panel(controller.raw()) {
                Ok(SteamworksInputOperation::BindingPanelShown {
                    controller: *controller,
                })
            } else {
                Err(SteamworksInputError::BindingPanelUnavailable)
            }
        }
    }
}

fn snapshot_connected_controllers(input: &steamworks::Input) -> Vec<SteamworksInputControllerInfo> {
    let mut raw_handles = [0_u64; STEAM_INPUT_MAX_CONTROLLER_COUNT];
    let quantity = input.get_connected_controllers_slice(&mut raw_handles);
    input_handles_from_slice(&raw_handles, quantity)
        .into_iter()
        .map(|handle| snapshot_controller_info(input, handle))
        .collect()
}

fn input_handles_from_slice(raw_handles: &[u64], quantity: usize) -> Vec<SteamworksInputHandle> {
    raw_handles
        .iter()
        .copied()
        .take(quantity)
        .map(SteamworksInputHandle::from_raw)
        .filter(|handle| handle.is_valid())
        .collect()
}

fn snapshot_controller_info(
    input: &steamworks::Input,
    handle: SteamworksInputHandle,
) -> SteamworksInputControllerInfo {
    SteamworksInputControllerInfo {
        handle,
        input_type: SteamworksInputType::from(input.get_input_type_for_handle(handle.raw())),
    }
}

fn validate_command(command: &SteamworksInputCommand) -> Result<(), SteamworksInputError> {
    match command {
        SteamworksInputCommand::SetActionManifestFilePath { path } => {
            validate_steam_string("path", path)
        }
        SteamworksInputCommand::GetActionSetHandle { name } => validate_steam_string("name", name),
        SteamworksInputCommand::GetDigitalActionHandle { name } => {
            validate_steam_string("name", name)
        }
        SteamworksInputCommand::GetAnalogActionHandle { name } => {
            validate_steam_string("name", name)
        }
        SteamworksInputCommand::GetControllerInfo { controller }
        | SteamworksInputCommand::GetMotionData { controller }
        | SteamworksInputCommand::ShowBindingPanel { controller } => {
            validate_input_handle("controller", *controller)
        }
        SteamworksInputCommand::ActivateActionSet {
            controller,
            action_set,
        } => {
            validate_input_handle("controller", *controller)?;
            validate_action_set_handle("action_set", *action_set)
        }
        SteamworksInputCommand::GetDigitalActionData { controller, action } => {
            validate_input_handle("controller", *controller)?;
            validate_digital_action_handle("action", *action)
        }
        SteamworksInputCommand::GetAnalogActionData { controller, action } => {
            validate_input_handle("controller", *controller)?;
            validate_analog_action_handle("action", *action)
        }
        SteamworksInputCommand::GetDigitalActionOrigins {
            controller,
            action_set,
            action,
        } => {
            validate_input_handle("controller", *controller)?;
            validate_action_set_handle("action_set", *action_set)?;
            validate_digital_action_handle("action", *action)
        }
        SteamworksInputCommand::GetAnalogActionOrigins {
            controller,
            action_set,
            action,
        } => {
            validate_input_handle("controller", *controller)?;
            validate_action_set_handle("action_set", *action_set)?;
            validate_analog_action_handle("action", *action)
        }
        SteamworksInputCommand::Init { .. }
        | SteamworksInputCommand::RunFrame
        | SteamworksInputCommand::Shutdown
        | SteamworksInputCommand::ListControllers => Ok(()),
    }
}

fn validate_steam_string(field: &'static str, value: &str) -> Result<(), SteamworksInputError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksInputError::invalid_string(field))
    } else {
        Ok(())
    }
}

fn validate_input_handle(
    field: &'static str,
    handle: SteamworksInputHandle,
) -> Result<(), SteamworksInputError> {
    if handle.is_valid() {
        Ok(())
    } else {
        Err(SteamworksInputError::invalid_handle(field))
    }
}

fn validate_action_set_handle(
    field: &'static str,
    handle: SteamworksInputActionSetHandle,
) -> Result<(), SteamworksInputError> {
    if handle.is_valid() {
        Ok(())
    } else {
        Err(SteamworksInputError::invalid_handle(field))
    }
}

fn validate_digital_action_handle(
    field: &'static str,
    handle: SteamworksInputDigitalActionHandle,
) -> Result<(), SteamworksInputError> {
    if handle.is_valid() {
        Ok(())
    } else {
        Err(SteamworksInputError::invalid_handle(field))
    }
}

fn validate_analog_action_handle(
    field: &'static str,
    handle: SteamworksInputAnalogActionHandle,
) -> Result<(), SteamworksInputError> {
    if handle.is_valid() {
        Ok(())
    } else {
        Err(SteamworksInputError::invalid_handle(field))
    }
}

impl From<steamworks::InputType> for SteamworksInputType {
    fn from(value: steamworks::InputType) -> Self {
        match value {
            steamworks::InputType::Unknown => Self::Unknown,
            steamworks::InputType::SteamController => Self::SteamController,
            steamworks::InputType::XBox360Controller => Self::XBox360Controller,
            steamworks::InputType::XBoxOneController => Self::XBoxOneController,
            steamworks::InputType::GenericGamepad => Self::GenericGamepad,
            steamworks::InputType::PS4Controller => Self::PS4Controller,
            steamworks::InputType::AppleMFiController => Self::AppleMFiController,
            steamworks::InputType::AndroidController => Self::AndroidController,
            steamworks::InputType::SwitchJoyConPair => Self::SwitchJoyConPair,
            steamworks::InputType::SwitchJoyConSingle => Self::SwitchJoyConSingle,
            steamworks::InputType::SwitchProController => Self::SwitchProController,
            steamworks::InputType::MobileTouch => Self::MobileTouch,
            steamworks::InputType::PS3Controller => Self::PS3Controller,
            steamworks::InputType::PS5Controller => Self::PS5Controller,
            steamworks::InputType::SteamDeckController => Self::SteamDeckController,
        }
    }
}

impl SteamworksInputSourceMode {
    fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::None,
            1 => Self::Dpad,
            2 => Self::Buttons,
            3 => Self::FourButtons,
            4 => Self::AbsoluteMouse,
            5 => Self::RelativeMouse,
            6 => Self::JoystickMove,
            7 => Self::JoystickMouse,
            8 => Self::JoystickCamera,
            9 => Self::ScrollWheel,
            10 => Self::Trigger,
            11 => Self::TouchMenu,
            12 => Self::MouseJoystick,
            13 => Self::MouseRegion,
            14 => Self::RadialMenu,
            15 => Self::SingleButton,
            16 => Self::Switches,
            _ => Self::Unknown(raw),
        }
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

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    #[test]
    fn input_plugin_registers_resources_and_messages() {
        let mut app = App::new();

        app.add_plugins(SteamworksInputPlugin::new());

        assert!(app.world().contains_resource::<SteamworksInputState>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksInputCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksInputResult>>());
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksInputPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksInputCommand>>()
            .write(SteamworksInputCommand::ListControllers);

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksInputResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksInputResult::Err {
                command: SteamworksInputCommand::ListControllers,
                error: SteamworksInputError::ClientUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksInputState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksInputError::ClientUnavailable)
        );
    }

    #[test]
    fn validation_rejects_interior_nul() {
        let command = SteamworksInputCommand::set_action_manifest_file_path("input\0manifest");

        assert_eq!(
            validate_command(&command),
            Err(SteamworksInputError::InvalidString { field: "path" })
        );

        let command = SteamworksInputCommand::get_digital_action_handle("jump\0bad");

        assert_eq!(
            validate_command(&command),
            Err(SteamworksInputError::InvalidString { field: "name" })
        );
    }

    #[test]
    fn validation_rejects_zero_handles() {
        let command = SteamworksInputCommand::activate_action_set(
            SteamworksInputHandle::from_raw(0),
            SteamworksInputActionSetHandle::from_raw(7),
        );

        assert_eq!(
            validate_command(&command),
            Err(SteamworksInputError::InvalidHandle {
                field: "controller",
            })
        );

        let command = SteamworksInputCommand::get_analog_action_data(
            SteamworksInputHandle::from_raw(7),
            SteamworksInputAnalogActionHandle::from_raw(0),
        );

        assert_eq!(
            validate_command(&command),
            Err(SteamworksInputError::InvalidHandle { field: "action" })
        );
    }

    #[test]
    fn constructors_preserve_inputs() {
        let controller = SteamworksInputHandle::from_raw(1);
        let action_set = SteamworksInputActionSetHandle::from_raw(2);
        let digital_action = SteamworksInputDigitalActionHandle::from_raw(3);
        let analog_action = SteamworksInputAnalogActionHandle::from_raw(4);

        assert_eq!(controller.raw(), 1);
        assert_eq!(
            SteamworksInputCommand::activate_action_set(controller, action_set),
            SteamworksInputCommand::ActivateActionSet {
                controller,
                action_set,
            }
        );
        assert_eq!(
            SteamworksInputCommand::get_digital_action_data(controller, digital_action),
            SteamworksInputCommand::GetDigitalActionData {
                controller,
                action: digital_action,
            }
        );
        assert_eq!(
            SteamworksInputCommand::get_analog_action_data(controller, analog_action),
            SteamworksInputCommand::GetAnalogActionData {
                controller,
                action: analog_action,
            }
        );
    }

    #[test]
    fn state_upserts_named_handles() {
        let mut state = SteamworksInputState::default();

        state.record_operation(&SteamworksInputOperation::ActionSetHandleRead {
            name: "gameplay".to_owned(),
            handle: SteamworksInputActionSetHandle::from_raw(1),
        });
        state.record_operation(&SteamworksInputOperation::ActionSetHandleRead {
            name: "gameplay".to_owned(),
            handle: SteamworksInputActionSetHandle::from_raw(2),
        });

        assert_eq!(
            state.action_sets(),
            &[SteamworksInputNamedActionSetHandle {
                name: "gameplay".to_owned(),
                handle: SteamworksInputActionSetHandle::from_raw(2),
            }]
        );
        assert_eq!(
            state.action_set_handle("gameplay"),
            Some(SteamworksInputActionSetHandle::from_raw(2))
        );
    }

    #[test]
    fn input_handle_slice_truncates_and_filters_zero_handles() {
        let raw_handles = [11, 0, 22, 33, 44];

        assert_eq!(
            input_handles_from_slice(&raw_handles, 4),
            vec![
                SteamworksInputHandle::from_raw(11),
                SteamworksInputHandle::from_raw(22),
                SteamworksInputHandle::from_raw(33),
            ]
        );
    }

    #[test]
    fn state_clears_stale_action_data_on_manifest_change_and_shutdown() {
        let mut state = SteamworksInputState::default();
        let controller = SteamworksInputHandle::from_raw(2);
        let action_set = SteamworksInputActionSetHandle::from_raw(1);
        let digital_action = SteamworksInputDigitalActionHandle::from_raw(3);
        let analog_action = SteamworksInputAnalogActionHandle::from_raw(5);
        let origin = SteamworksInputActionOriginInfo {
            origin: SteamworksInputActionOrigin::from_code(9),
            glyph_path: "glyph.png".to_owned(),
            name: "Jump".to_owned(),
        };

        state.record_operation(&SteamworksInputOperation::ActionSetHandleRead {
            name: "gameplay".to_owned(),
            handle: action_set,
        });
        state.record_operation(&SteamworksInputOperation::ActionSetActivated {
            controller,
            action_set,
        });
        state.record_operation(&SteamworksInputOperation::DigitalActionDataRead {
            snapshot: SteamworksInputDigitalActionSnapshot {
                controller,
                action: digital_action,
                data: SteamworksInputDigitalActionData {
                    state: true,
                    active: true,
                },
            },
        });
        state.record_operation(&SteamworksInputOperation::AnalogActionDataRead {
            snapshot: SteamworksInputAnalogActionSnapshot {
                controller,
                action: analog_action,
                data: SteamworksInputAnalogActionData {
                    mode: SteamworksInputSourceMode::JoystickMove,
                    x: 1.0,
                    y: -1.0,
                    active: true,
                },
            },
        });
        state.record_operation(&SteamworksInputOperation::DigitalActionOriginsRead {
            controller,
            action_set,
            action: digital_action,
            origins: vec![origin.clone()],
        });
        state.record_operation(&SteamworksInputOperation::AnalogActionOriginsRead {
            controller,
            action_set,
            action: analog_action,
            origins: vec![origin],
        });
        state.record_operation(&SteamworksInputOperation::MotionDataRead {
            snapshot: SteamworksInputMotionSnapshot {
                controller,
                data: SteamworksInputMotionData {
                    rotation_quaternion: [0.0, 0.0, 0.0, 1.0],
                    position_acceleration: [0.0, 1.0, 0.0],
                    rotation_velocity: [0.0, 0.0, 1.0],
                },
            },
        });
        state.record_operation(&SteamworksInputOperation::BindingPanelShown { controller });

        state.record_operation(&SteamworksInputOperation::ActionManifestFilePathSet {
            path: "new_manifest.vdf".to_owned(),
        });

        assert!(state.action_sets().is_empty());
        assert_eq!(state.action_manifest_path(), Some("new_manifest.vdf"));
        assert!(state.last_action_set_activation().is_none());
        assert!(state.last_digital_action().is_none());
        assert!(state.last_analog_action().is_none());
        assert!(state.last_digital_action_origins().is_none());
        assert!(state.last_analog_action_origins().is_none());
        assert!(state.last_motion().is_none());
        assert!(state.last_binding_panel_controller().is_none());

        state.record_operation(&SteamworksInputOperation::Initialized {
            explicitly_call_run_frame: false,
        });
        state.record_operation(&SteamworksInputOperation::ActionSetHandleRead {
            name: "menu".to_owned(),
            handle: SteamworksInputActionSetHandle::from_raw(4),
        });
        state.record_operation(&SteamworksInputOperation::Shutdown);

        assert!(!state.initialized());
        assert!(state.action_manifest_path().is_none());
        assert!(state.action_sets().is_empty());
    }

    #[test]
    fn state_records_input_operations() {
        let mut state = SteamworksInputState::default();
        let controller = SteamworksInputHandle::from_raw(11);
        let action_set = SteamworksInputActionSetHandle::from_raw(22);
        let digital_action = SteamworksInputDigitalActionHandle::from_raw(33);
        let analog_action = SteamworksInputAnalogActionHandle::from_raw(44);
        let controller_info = SteamworksInputControllerInfo {
            handle: controller,
            input_type: SteamworksInputType::SteamDeckController,
        };
        let digital_snapshot = SteamworksInputDigitalActionSnapshot {
            controller,
            action: digital_action,
            data: SteamworksInputDigitalActionData {
                state: true,
                active: true,
            },
        };
        let analog_snapshot = SteamworksInputAnalogActionSnapshot {
            controller,
            action: analog_action,
            data: SteamworksInputAnalogActionData {
                mode: SteamworksInputSourceMode::JoystickMove,
                x: 0.25,
                y: -0.5,
                active: true,
            },
        };
        let origin = SteamworksInputActionOriginInfo {
            origin: SteamworksInputActionOrigin::from_code(7),
            glyph_path: "glyph.svg".to_owned(),
            name: "A Button".to_owned(),
        };
        let motion = SteamworksInputMotionSnapshot {
            controller,
            data: SteamworksInputMotionData {
                rotation_quaternion: [0.0, 0.0, 0.0, 1.0],
                position_acceleration: [1.0, 2.0, 3.0],
                rotation_velocity: [4.0, 5.0, 6.0],
            },
        };

        state.record_operation(&SteamworksInputOperation::Initialized {
            explicitly_call_run_frame: true,
        });
        state.record_operation(&SteamworksInputOperation::FrameRun);
        state.record_operation(&SteamworksInputOperation::FrameRun);
        state.record_operation(&SteamworksInputOperation::ControllersListed {
            controllers: vec![controller_info.clone()],
        });
        state.record_operation(&SteamworksInputOperation::ControllerInfoRead {
            controller: SteamworksInputControllerInfo {
                input_type: SteamworksInputType::GenericGamepad,
                ..controller_info.clone()
            },
        });
        state.record_operation(&SteamworksInputOperation::ActionSetHandleRead {
            name: "gameplay".to_owned(),
            handle: action_set,
        });
        state.record_operation(&SteamworksInputOperation::DigitalActionHandleRead {
            name: "jump".to_owned(),
            handle: digital_action,
        });
        state.record_operation(&SteamworksInputOperation::AnalogActionHandleRead {
            name: "move".to_owned(),
            handle: analog_action,
        });
        state.record_operation(&SteamworksInputOperation::ActionSetActivated {
            controller,
            action_set,
        });
        state.record_operation(&SteamworksInputOperation::DigitalActionDataRead {
            snapshot: digital_snapshot.clone(),
        });
        state.record_operation(&SteamworksInputOperation::AnalogActionDataRead {
            snapshot: analog_snapshot.clone(),
        });
        state.record_operation(&SteamworksInputOperation::DigitalActionOriginsRead {
            controller,
            action_set,
            action: digital_action,
            origins: vec![origin.clone()],
        });
        state.record_operation(&SteamworksInputOperation::AnalogActionOriginsRead {
            controller,
            action_set,
            action: analog_action,
            origins: vec![origin.clone()],
        });
        state.record_operation(&SteamworksInputOperation::MotionDataRead {
            snapshot: motion.clone(),
        });
        state.record_operation(&SteamworksInputOperation::BindingPanelShown { controller });

        assert!(state.initialized());
        assert_eq!(state.frame_run_count(), 2);
        assert_eq!(
            state.controller(controller),
            Some(&SteamworksInputControllerInfo {
                input_type: SteamworksInputType::GenericGamepad,
                ..controller_info
            })
        );
        assert_eq!(state.action_set_handle("gameplay"), Some(action_set));
        assert_eq!(state.digital_action_handle("jump"), Some(digital_action));
        assert_eq!(state.analog_action_handle("move"), Some(analog_action));
        assert_eq!(
            state.last_action_set_activation(),
            Some(SteamworksInputActionSetActivation {
                controller,
                action_set,
            })
        );
        assert_eq!(state.last_digital_action(), Some(&digital_snapshot));
        assert_eq!(state.last_analog_action(), Some(&analog_snapshot));
        assert_eq!(
            state.last_digital_action_origins(),
            Some(&SteamworksInputDigitalActionOriginsSnapshot {
                controller,
                action_set,
                action: digital_action,
                origins: vec![origin.clone()],
            })
        );
        assert_eq!(
            state.last_analog_action_origins(),
            Some(&SteamworksInputAnalogActionOriginsSnapshot {
                controller,
                action_set,
                action: analog_action,
                origins: vec![origin],
            })
        );
        assert_eq!(state.last_motion(), Some(&motion));
        assert_eq!(state.last_binding_panel_controller(), Some(controller));
    }
}
