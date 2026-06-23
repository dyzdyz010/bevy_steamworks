/// Policy used when a listen socket receives a connection request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksConnectionRequestPolicy {
    /// Accept incoming connection requests immediately.
    Accept,
    /// Reject incoming connection requests immediately.
    Reject {
        /// End reason sent to the remote peer.
        reason: steamworks::networking_types::NetConnectionEnd,
        /// Optional debug string sent to Steam.
        debug: Option<String>,
    },
}

impl Default for SteamworksConnectionRequestPolicy {
    fn default() -> Self {
        Self::Reject {
            reason: steamworks::networking_types::NetConnectionEnd::App(
                steamworks::networking_types::AppNetConnectionEnd::generic_normal(),
            ),
            debug: Some("connection rejected by bevy_steamworks policy".to_owned()),
        }
    }
}

/// Initial Steam Networking Sockets configuration entry for listen/connect commands.
///
/// This owned wrapper keeps command messages comparable and debuggable while
/// converting to upstream [`steamworks::networking_types::NetworkingConfigEntry`]
/// only after validation.
#[derive(Clone, PartialEq)]
pub enum SteamworksNetworkingSocketsConfigEntry {
    /// Signed 32-bit integer config value.
    Int32 {
        /// Upstream config key.
        value: steamworks::networking_types::NetworkingConfigValue,
        /// Config value.
        data: i32,
    },
    /// Signed 64-bit integer config value.
    Int64 {
        /// Upstream config key.
        value: steamworks::networking_types::NetworkingConfigValue,
        /// Config value.
        data: i64,
    },
    /// Floating-point config value.
    Float {
        /// Upstream config key.
        value: steamworks::networking_types::NetworkingConfigValue,
        /// Config value.
        data: f32,
    },
    /// String config value.
    String {
        /// Upstream config key.
        value: steamworks::networking_types::NetworkingConfigValue,
        /// Config value.
        data: String,
    },
}

impl SteamworksNetworkingSocketsConfigEntry {
    /// Creates an integer config entry.
    pub fn int32(value: steamworks::networking_types::NetworkingConfigValue, data: i32) -> Self {
        Self::Int32 { value, data }
    }

    /// Creates a 64-bit integer config entry.
    pub fn int64(value: steamworks::networking_types::NetworkingConfigValue, data: i64) -> Self {
        Self::Int64 { value, data }
    }

    /// Creates a floating-point config entry.
    pub fn float(value: steamworks::networking_types::NetworkingConfigValue, data: f32) -> Self {
        Self::Float { value, data }
    }

    /// Creates a string config entry.
    pub fn string(
        value: steamworks::networking_types::NetworkingConfigValue,
        data: impl Into<String>,
    ) -> Self {
        Self::String {
            value,
            data: data.into(),
        }
    }

    pub(in crate::networking_sockets) fn to_steam(
        &self,
    ) -> steamworks::networking_types::NetworkingConfigEntry {
        match self {
            Self::Int32 { value, data } => {
                steamworks::networking_types::NetworkingConfigEntry::new_int32(*value, *data)
            }
            Self::Int64 { value, data } => {
                steamworks::networking_types::NetworkingConfigEntry::new_int64(*value, *data)
            }
            Self::Float { value, data } => {
                steamworks::networking_types::NetworkingConfigEntry::new_float(*value, *data)
            }
            Self::String { value, data } => {
                steamworks::networking_types::NetworkingConfigEntry::new_string(*value, data)
            }
        }
    }
}
