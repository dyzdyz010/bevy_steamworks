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

    pub(super) fn is_valid(self) -> bool {
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

    pub(super) fn is_valid(self) -> bool {
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

    pub(super) fn is_valid(self) -> bool {
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

    pub(super) fn is_valid(self) -> bool {
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
    pub(super) fn from_raw(raw: i32) -> Self {
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
