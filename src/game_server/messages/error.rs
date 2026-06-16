use thiserror::Error;

use crate::user::SteamworksAuthSessionError;

/// Synchronous errors from [`crate::SteamworksServerPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksServerError {
    /// No [`crate::SteamworksServer`] resource exists.
    #[error("SteamworksServer resource is not available")]
    ServerUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steam Game Server command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A remote authentication session was requested with no ticket bytes.
    #[error("Steam Game Server command requires a non-empty authentication ticket")]
    EmptyTicket,
    /// An authentication ticket was requested for an invalid networking identity.
    #[error("Steam Game Server command requires a valid networking identity")]
    InvalidNetworkingIdentity,
    /// Token-based server logon was requested with no token.
    #[error("Steam Game Server token logon requires a non-empty token")]
    EmptyLogonToken,
    /// A count field must be non-negative.
    #[error("Steam Game Server command field {field} must be non-negative, got {value}")]
    InvalidCount {
        /// Field that contained the invalid count.
        field: &'static str,
        /// Invalid count.
        value: i32,
    },
    /// Steam game tags must be non-empty and shorter than 128 bytes.
    #[error("Steam Game Server game tags must be non-empty and shorter than 128 bytes")]
    InvalidGameTags,
    /// The command must be submitted before the server logs on.
    #[error("Steam Game Server command {command} must be submitted before server logon")]
    CommandRequiresPreLogon {
        /// Command that must run before logon.
        command: &'static str,
    },
    /// A server logon command was submitted after logon had already been submitted.
    #[error("Steam Game Server logon has already been submitted")]
    LogonAlreadySubmitted,
    /// The upstream Steamworks API rejected an authentication session.
    #[error("Steam Game Server authentication session failed: {source}")]
    AuthSession {
        /// Authentication session failure reason.
        #[source]
        source: SteamworksAuthSessionError,
    },
}

impl SteamworksServerError {
    pub(in crate::game_server) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    pub(in crate::game_server) fn invalid_count(field: &'static str, value: i32) -> Self {
        Self::InvalidCount { field, value }
    }

    pub(in crate::game_server) fn auth_session(source: steamworks::AuthSessionError) -> Self {
        Self::AuthSession {
            source: source.into(),
        }
    }

    pub(in crate::game_server) fn command_requires_pre_logon(command: &'static str) -> Self {
        Self::CommandRequiresPreLogon { command }
    }
}
