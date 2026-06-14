//! High-level Bevy ECS integration for Steam Workshop / UGC.
//!
//! This module builds on top of the upstream [`steamworks::UGC`] API. It keeps
//! common Workshop queries, subscriptions, downloads, and playtime tracking in
//! Bevy messages, while converting asynchronous Steam call results and download
//! callbacks into owned ECS-safe result messages.

use bevy_app::{App, First, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{SteamworksEvent, SteamworksSystem};

/// Maximum number of item IDs accepted by one UGC details or playtime command.
///
/// The raw Steam call takes a `u32` count and is not intended for unbounded
/// frame-loop payloads. Larger batches should be split by the caller.
pub const STEAMWORKS_UGC_MAX_ITEMS_PER_COMMAND: usize = 1_000;

/// Maximum item title bytes accepted before the trailing NUL terminator.
pub const STEAMWORKS_UGC_MAX_UPDATE_TITLE_BYTES: usize = 128;

/// Maximum item description bytes accepted before the trailing NUL terminator.
pub const STEAMWORKS_UGC_MAX_UPDATE_DESCRIPTION_BYTES: usize = 7_999;

/// Maximum developer metadata bytes accepted before the trailing NUL terminator.
pub const STEAMWORKS_UGC_MAX_UPDATE_METADATA_BYTES: usize = 4_999;

/// Maximum item tag bytes accepted by Steam.
pub const STEAMWORKS_UGC_MAX_UPDATE_TAG_BYTES: usize = 255;

/// Maximum key/value tag removals accepted by one item update.
pub const STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAG_REMOVALS: usize = 100;

/// Maximum key/value tag additions accepted by one item update.
pub const STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAGS: usize = 100;

/// Bevy plugin for high-level Steam Workshop / UGC commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksUgcCommand`] and [`SteamworksUgcResult`] messages and runs its
/// command processor in [`bevy_app::First`] after Steam callbacks. It also
/// mirrors Workshop download completion callbacks into UGC results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksUgcPlugin;

impl SteamworksUgcPlugin {
    /// Creates a UGC plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksUgcPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksUgcState>()
            .init_resource::<SteamworksUgcAsyncResults>()
            .init_resource::<SteamworksUgcUpdateWatches>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksUgcCommand>()
            .add_message::<SteamworksUgcResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessUgcCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_ugc_commands.in_set(SteamworksSystem::ProcessUgcCommands),
            );
    }
}

mod async_results;
mod callbacks;
mod commands;
mod item_updates;
mod messages;
mod queries;
mod snapshots;
mod state;
mod types;
mod update_watches;
mod validation;

use async_results::SteamworksUgcAsyncResults;
use commands::process_ugc_commands;
use update_watches::SteamworksUgcUpdateWatches;

pub use messages::*;
pub use state::SteamworksUgcState;
pub use types::*;

#[cfg(test)]
mod tests;
