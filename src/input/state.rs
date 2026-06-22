use bevy_ecs::prelude::Resource;

use crate::cache::trim_oldest;

use super::*;

mod accessors;
mod operations;

pub(in crate::input) const STEAMWORKS_INPUT_STATE_CACHE_LIMIT: usize = 1_024;

/// Runtime state for [`crate::SteamworksInputPlugin`].
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
    action_set_activations: Vec<SteamworksInputActionSetActivation>,
    last_digital_action: Option<SteamworksInputDigitalActionSnapshot>,
    digital_action_snapshots: Vec<SteamworksInputDigitalActionSnapshot>,
    last_analog_action: Option<SteamworksInputAnalogActionSnapshot>,
    analog_action_snapshots: Vec<SteamworksInputAnalogActionSnapshot>,
    last_digital_action_origins: Option<SteamworksInputDigitalActionOriginsSnapshot>,
    digital_action_origin_snapshots: Vec<SteamworksInputDigitalActionOriginsSnapshot>,
    last_analog_action_origins: Option<SteamworksInputAnalogActionOriginsSnapshot>,
    analog_action_origin_snapshots: Vec<SteamworksInputAnalogActionOriginsSnapshot>,
    action_origin_infos: Vec<SteamworksInputActionOriginInfo>,
    last_action_origin_info: Option<SteamworksInputActionOriginInfo>,
    last_motion: Option<SteamworksInputMotionSnapshot>,
    motion_snapshots: Vec<SteamworksInputMotionSnapshot>,
    last_binding_panel_controller: Option<SteamworksInputHandle>,
}

pub(super) fn upsert_controller(
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
        trim_oldest(controllers, STEAMWORKS_INPUT_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_named_action_set(
    handles: &mut Vec<SteamworksInputNamedActionSetHandle>,
    name: String,
    handle: SteamworksInputActionSetHandle,
) {
    if let Some(existing) = handles.iter_mut().find(|existing| existing.name == name) {
        existing.handle = handle;
    } else {
        handles.push(SteamworksInputNamedActionSetHandle { name, handle });
        trim_oldest(handles, STEAMWORKS_INPUT_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_named_digital_action(
    handles: &mut Vec<SteamworksInputNamedDigitalActionHandle>,
    name: String,
    handle: SteamworksInputDigitalActionHandle,
) {
    if let Some(existing) = handles.iter_mut().find(|existing| existing.name == name) {
        existing.handle = handle;
    } else {
        handles.push(SteamworksInputNamedDigitalActionHandle { name, handle });
        trim_oldest(handles, STEAMWORKS_INPUT_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_named_analog_action(
    handles: &mut Vec<SteamworksInputNamedAnalogActionHandle>,
    name: String,
    handle: SteamworksInputAnalogActionHandle,
) {
    if let Some(existing) = handles.iter_mut().find(|existing| existing.name == name) {
        existing.handle = handle;
    } else {
        handles.push(SteamworksInputNamedAnalogActionHandle { name, handle });
        trim_oldest(handles, STEAMWORKS_INPUT_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_action_origin_info(
    infos: &mut Vec<SteamworksInputActionOriginInfo>,
    info: SteamworksInputActionOriginInfo,
) {
    if let Some(existing) = infos
        .iter_mut()
        .find(|existing| existing.origin == info.origin)
    {
        *existing = info;
    } else {
        infos.push(info);
        trim_oldest(infos, STEAMWORKS_INPUT_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_action_set_activation(
    activations: &mut Vec<SteamworksInputActionSetActivation>,
    activation: SteamworksInputActionSetActivation,
) {
    if let Some(existing) = activations
        .iter_mut()
        .find(|existing| existing.controller == activation.controller)
    {
        *existing = activation;
    } else {
        activations.push(activation);
        trim_oldest(activations, STEAMWORKS_INPUT_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_digital_action_snapshot(
    snapshots: &mut Vec<SteamworksInputDigitalActionSnapshot>,
    snapshot: SteamworksInputDigitalActionSnapshot,
) {
    if let Some(existing) = snapshots.iter_mut().find(|existing| {
        existing.controller == snapshot.controller && existing.action == snapshot.action
    }) {
        *existing = snapshot;
    } else {
        snapshots.push(snapshot);
        trim_oldest(snapshots, STEAMWORKS_INPUT_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_analog_action_snapshot(
    snapshots: &mut Vec<SteamworksInputAnalogActionSnapshot>,
    snapshot: SteamworksInputAnalogActionSnapshot,
) {
    if let Some(existing) = snapshots.iter_mut().find(|existing| {
        existing.controller == snapshot.controller && existing.action == snapshot.action
    }) {
        *existing = snapshot;
    } else {
        snapshots.push(snapshot);
        trim_oldest(snapshots, STEAMWORKS_INPUT_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_digital_action_origins_snapshot(
    snapshots: &mut Vec<SteamworksInputDigitalActionOriginsSnapshot>,
    snapshot: SteamworksInputDigitalActionOriginsSnapshot,
) {
    if let Some(existing) = snapshots.iter_mut().find(|existing| {
        existing.controller == snapshot.controller
            && existing.action_set == snapshot.action_set
            && existing.action == snapshot.action
    }) {
        *existing = snapshot;
    } else {
        snapshots.push(snapshot);
        trim_oldest(snapshots, STEAMWORKS_INPUT_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_analog_action_origins_snapshot(
    snapshots: &mut Vec<SteamworksInputAnalogActionOriginsSnapshot>,
    snapshot: SteamworksInputAnalogActionOriginsSnapshot,
) {
    if let Some(existing) = snapshots.iter_mut().find(|existing| {
        existing.controller == snapshot.controller
            && existing.action_set == snapshot.action_set
            && existing.action == snapshot.action
    }) {
        *existing = snapshot;
    } else {
        snapshots.push(snapshot);
        trim_oldest(snapshots, STEAMWORKS_INPUT_STATE_CACHE_LIMIT);
    }
}

pub(super) fn upsert_motion_snapshot(
    snapshots: &mut Vec<SteamworksInputMotionSnapshot>,
    snapshot: SteamworksInputMotionSnapshot,
) {
    if let Some(existing) = snapshots
        .iter_mut()
        .find(|existing| existing.controller == snapshot.controller)
    {
        *existing = snapshot;
    } else {
        snapshots.push(snapshot);
        trim_oldest(snapshots, STEAMWORKS_INPUT_STATE_CACHE_LIMIT);
    }
}
