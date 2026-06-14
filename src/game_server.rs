//! Bevy ECS integration for Steam Game Server initialization and callbacks.
//!
//! This module builds on top of the upstream [`steamworks::Server`] API. It
//! inserts a Bevy resource for the initialized game server, pumps Steam Game
//! Server callbacks into the shared [`crate::SteamworksEvent`] stream, and
//! mirrors relevant callback confirmations into [`SteamworksServerResult`].

use std::{net::Ipv4Addr, ops::Deref, sync::Mutex};

use bevy_ecs::prelude::Resource;
use thiserror::Error;

use crate::SteamworksFailurePolicy;

mod callbacks;
mod commands;
mod messages;
mod packets;
mod plugin;
mod registry;
mod state;
#[cfg(test)]
mod tests;
mod types;
mod validation;

pub use messages::*;
pub use registry::SteamworksServerCallbackRegistry;
pub use state::SteamworksServerState;
pub use types::*;

/// Required buffer size for Steam Game Server shared-query outgoing packets.
pub const STEAMWORKS_SERVER_QUERY_PACKET_BUFFER_BYTES: usize = 16 * 1024;

/// Configuration used to initialize [`steamworks::Server`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksServerConfig {
    /// IPv4 address the Steam Game Server API should bind to.
    pub ip: Ipv4Addr,
    /// Game traffic port.
    pub game_port: u16,
    /// Server browser query port.
    ///
    /// Use [`steamworks::QUERY_PORT_SHARED`] when game and query traffic share
    /// the same socket.
    pub query_port: u16,
    /// Upstream Steam server mode.
    pub server_mode: steamworks::ServerMode,
    /// Version string reported to Steam.
    pub version: String,
}

impl SteamworksServerConfig {
    /// Creates a Steam Game Server initialization config.
    pub fn new(
        ip: Ipv4Addr,
        game_port: u16,
        query_port: u16,
        server_mode: steamworks::ServerMode,
        version: impl Into<String>,
    ) -> Self {
        Self {
            ip,
            game_port,
            query_port,
            server_mode,
            version: version.into(),
        }
    }
}

/// How [`SteamworksServerPlugin`] should create or locate the Steamworks server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksServerInitMode {
    /// Initialize a Steam Game Server from the supplied config.
    Config(SteamworksServerConfig),
    /// Do not initialize Steamworks.
    ///
    /// This is useful when another layer inserts [`SteamworksServer`] manually,
    /// or for tests that only need plugin schedules and messages.
    Manual,
}

/// Resource inserted when Steam Game Server initialization is explicitly allowed to fail.
#[derive(Clone, Debug, Error, PartialEq, Eq, Resource)]
pub enum SteamworksServerUnavailable {
    /// Manual mode was selected, but no [`SteamworksServer`] resource was present.
    #[error(
        "manual Steam Game Server initialization was selected, but no SteamworksServer resource exists"
    )]
    ManualServerMissing,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steam Game Server config field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// The upstream Steam Game Server initialization call returned an error.
    #[error("Steam Game Server initialization failed with {config:?}: {source}")]
    InitFailed {
        /// Initialization config used for the failed attempt.
        config: SteamworksServerConfig,
        /// Error returned by `steamworks`.
        source: steamworks::SteamAPIInitError,
    },
}

impl SteamworksServerUnavailable {
    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    fn init_failed(config: SteamworksServerConfig, source: steamworks::SteamAPIInitError) -> Self {
        Self::InitFailed { config, source }
    }

    /// Returns true when manual initialization was selected without inserting a server resource.
    pub fn is_manual_server_missing(&self) -> bool {
        matches!(self, Self::ManualServerMissing)
    }

    /// Returns true when Steam Game Server initialization failed before calling upstream Steam.
    pub fn is_invalid_string(&self) -> bool {
        matches!(self, Self::InvalidString { .. })
    }

    /// Returns true when an upstream Steam Game Server initialization call failed.
    pub fn is_init_failed(&self) -> bool {
        matches!(self, Self::InitFailed { .. })
    }

    /// Returns the invalid config field, when initialization failed during validation.
    pub fn invalid_string_field(&self) -> Option<&'static str> {
        match self {
            Self::InvalidString { field } => Some(*field),
            Self::ManualServerMissing | Self::InitFailed { .. } => None,
        }
    }

    /// Returns the config used for a failed upstream Steam Game Server initialization call.
    pub fn init_config(&self) -> Option<&SteamworksServerConfig> {
        match self {
            Self::InitFailed { config, .. } => Some(config),
            Self::ManualServerMissing | Self::InvalidString { .. } => None,
        }
    }

    /// Returns the upstream Steamworks initialization error, when initialization failed.
    pub fn init_error(&self) -> Option<&steamworks::SteamAPIInitError> {
        match self {
            Self::InitFailed { source, .. } => Some(source),
            Self::ManualServerMissing | Self::InvalidString { .. } => None,
        }
    }
}

/// A Bevy resource wrapping [`steamworks::Server`].
#[derive(Clone, Resource)]
pub struct SteamworksServer(steamworks::Server);

impl SteamworksServer {
    /// Creates a Bevy resource from an initialized Steam Game Server.
    pub fn new(server: steamworks::Server) -> Self {
        Self(server)
    }

    /// Returns the underlying Steam Game Server handle.
    pub fn inner(&self) -> &steamworks::Server {
        &self.0
    }

    /// Clones the underlying Steam Game Server handle.
    pub fn clone_inner(&self) -> steamworks::Server {
        self.0.clone()
    }
}

impl From<steamworks::Server> for SteamworksServer {
    fn from(server: steamworks::Server) -> Self {
        Self::new(server)
    }
}

impl Deref for SteamworksServer {
    type Target = steamworks::Server;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

/// A Bevy plugin that integrates Steam Game Server callbacks into an app.
pub struct SteamworksServerPlugin {
    mode: SteamworksServerInitMode,
    failure_policy: SteamworksFailurePolicy,
    run_callbacks: bool,
    server: Mutex<Option<steamworks::Server>>,
}
