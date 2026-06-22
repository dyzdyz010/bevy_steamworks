use bevy_ecs::prelude::Resource;

use super::{
    messages::SteamworksRemotePlayError,
    types::{
        SteamworksRemotePlayInvite, SteamworksRemotePlaySessionInfo,
        SteamworksRemotePlaySessionSnapshot,
    },
};

mod accessors;
mod operations;

/// Runtime state for [`super::SteamworksRemotePlayPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksRemotePlayState {
    last_error: Option<SteamworksRemotePlayError>,
    sessions: Vec<SteamworksRemotePlaySessionSnapshot>,
    known_sessions: Vec<SteamworksRemotePlaySessionInfo>,
    observed_connected_sessions: Vec<steamworks::RemotePlaySessionId>,
    last_submitted_invite: Option<SteamworksRemotePlayInvite>,
    submitted_invite_count: u64,
}
