use std::time::Duration;

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::prelude::*;

#[derive(Resource)]
struct FramesRemaining(u32);

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
                SteamworksUgcQuery::item(item).with_options(
                    SteamworksUgcQueryOptions::new()
                        .with_metadata(true)
                        .with_key_value_tags(true)
                        .with_statistic(UGCStatisticType::Subscriptions),
                ),
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
        }
    }

    if let Ok(search_text) = std::env::var("BEVY_STEAMWORKS_UGC_SEARCH") {
        commands.write(SteamworksUgcCommand::query(
            SteamworksUgcQuery::all(
                UGCQueryType::RankedByTextSearch,
                UGCType::Items,
                AppIDs::ConsumerAppId(AppId(480)),
                1,
            )
            .with_options(
                SteamworksUgcQueryOptions::new()
                    .with_search_text(search_text)
                    .with_long_description(false),
            ),
        ));
    }
}

fn log_ugc_results(mut results: MessageReader<SteamworksUgcResult>) {
    for result in results.read() {
        println!("{result:?}");
    }
}

fn log_ugc_callbacks(mut events: MessageReader<SteamworksEvent>) {
    for event in events.read() {
        if let SteamworksEvent::DownloadItemResult(event) = event {
            println!("DownloadItemResult: {event:?}");
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
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksUgcPlugin::new())
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
        .add_systems(Startup, request_ugc)
        .add_systems(
            Update,
            (log_ugc_results, log_ugc_callbacks, exit_after_a_short_run),
        )
        .run();
}
