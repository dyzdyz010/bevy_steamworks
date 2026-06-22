use std::{path::PathBuf, time::Duration};

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

#[derive(Default, Resource)]
struct UgcExampleState {
    update_request_id: Option<u64>,
    requested_update_progress: bool,
}

fn request_ugc(
    steam: Option<Res<SteamworksClient>>,
    unavailable: Option<Res<SteamworksUnavailable>>,
    mut commands: MessageWriter<SteamworksUgcCommand>,
) {
    if let Some(unavailable) = unavailable {
        eprintln!("Steamworks unavailable: {}", &*unavailable);
        return;
    }

    if steam.is_none() {
        return;
    }

    commands.write(SteamworksUgcCommand::list_subscribed_items(false));

    if let Ok(item) = std::env::var("BEVY_STEAMWORKS_UGC_ITEM") {
        if let Ok(item) = item.parse::<u64>() {
            let item = PublishedFileId(item);
            commands.write(SteamworksUgcCommand::query(
                SteamworksUgcQuery::item(item)
                    .with_metadata(true)
                    .with_key_value_tags(true)
                    .with_statistic(UGCStatisticType::Subscriptions),
            ));
            commands.write(SteamworksUgcCommand::get_item_state(item));
            commands.write(SteamworksUgcCommand::get_item_download_info(item));
            commands.write(SteamworksUgcCommand::get_item_install_info(item));

            if std::env::var("BEVY_STEAMWORKS_UGC_DOWNLOAD").as_deref() == Ok("1") {
                commands.write(SteamworksUgcCommand::download_item(item, true));
            }
            if std::env::var("BEVY_STEAMWORKS_UGC_SUBSCRIBE").as_deref() == Ok("1") {
                commands.write(SteamworksUgcCommand::subscribe_item(item));
            }
            if std::env::var("BEVY_STEAMWORKS_UGC_UNSUBSCRIBE").as_deref() == Ok("1") {
                commands.write(SteamworksUgcCommand::unsubscribe_item(item));
            }
            if std::env::var("BEVY_STEAMWORKS_UGC_UPDATE").as_deref() == Ok("1") {
                commands.write(SteamworksUgcCommand::submit_item_update(
                    AppId(480),
                    item,
                    item_update_from_env(),
                ));
            }
        }
    }

    if let Ok(search_text) = std::env::var("BEVY_STEAMWORKS_UGC_SEARCH") {
        let query = SteamworksUgcQuery::all(
            UGCQueryType::RankedByTextSearch,
            UGCType::Items,
            AppIDs::ConsumerAppId(AppId(480)),
            1,
        )
        .with_search_text(search_text)
        .with_long_description(false);
        commands.write(SteamworksUgcCommand::query(query.clone()));
        if std::env::var("BEVY_STEAMWORKS_UGC_SEARCH_TOTAL").as_deref() == Ok("1") {
            commands.write(SteamworksUgcCommand::query_total(query.clone()));
        }
        if std::env::var("BEVY_STEAMWORKS_UGC_SEARCH_IDS").as_deref() == Ok("1") {
            commands.write(SteamworksUgcCommand::query_ids(query));
        }
    }
}

fn item_update_from_env() -> SteamworksUgcItemUpdate {
    let mut update = SteamworksUgcItemUpdate::new();

    if let Ok(title) = std::env::var("BEVY_STEAMWORKS_UGC_UPDATE_TITLE") {
        update = update.with_title(title);
    }
    if let Ok(description) = std::env::var("BEVY_STEAMWORKS_UGC_UPDATE_DESCRIPTION") {
        update = update.with_description(description);
    }
    if let Ok(content_path) = std::env::var("BEVY_STEAMWORKS_UGC_UPDATE_CONTENT_PATH") {
        update = update.with_content_path(PathBuf::from(content_path));
    }
    if let Ok(preview_path) = std::env::var("BEVY_STEAMWORKS_UGC_UPDATE_PREVIEW_PATH") {
        update = update.with_preview_path(PathBuf::from(preview_path));
    }
    if let Ok(change_note) = std::env::var("BEVY_STEAMWORKS_UGC_UPDATE_CHANGE_NOTE") {
        update = update.with_change_note(change_note);
    }

    update
}

fn log_ugc_results(
    mut state: ResMut<UgcExampleState>,
    mut results: MessageReader<SteamworksUgcResult>,
    mut commands: MessageWriter<SteamworksUgcCommand>,
) {
    for result in results.read() {
        println!("{result:?}");

        let SteamworksUgcResult::Ok(operation) = result else {
            continue;
        };

        if let SteamworksUgcOperation::ItemUpdateSubmitted { request_id, .. } = operation {
            state.update_request_id = Some(*request_id);
        }
    }

    if let Some(request_id) = state.update_request_id {
        if !state.requested_update_progress {
            commands.write(SteamworksUgcCommand::get_item_update_progress(request_id));
            state.requested_update_progress = true;
        }
    }
}

fn exit_after_a_short_run(mut frames: ResMut<FramesRemaining>, mut exit: MessageWriter<AppExit>) {
    if frames.0 == 0 {
        exit.write(AppExit::Success);
    } else {
        frames.0 -= 1;
    }
}

fn main() {
    App::new()
        .insert_resource(FramesRemaining(120))
        .init_resource::<UgcExampleState>()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksUgcPlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, request_ugc)
        .add_systems(Update, (log_ugc_results, exit_after_a_short_run))
        .run();
}
