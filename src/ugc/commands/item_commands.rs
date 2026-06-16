use super::super::{
    async_results::SteamworksUgcAsyncResults, item_updates::apply_item_update,
    update_watches::SteamworksUgcUpdateWatches, SteamworksUgcCommand, SteamworksUgcError,
    SteamworksUgcItemUpdate, SteamworksUgcOperation, SteamworksUgcResult,
};

pub(super) fn create_item(
    ugc: &steamworks::UGC,
    async_results: &SteamworksUgcAsyncResults,
    request_id: u64,
    app_id: steamworks::AppId,
    file_type: steamworks::FileType,
) -> SteamworksUgcOperation {
    let async_results = async_results.clone();
    let command = SteamworksUgcCommand::CreateItem { app_id, file_type };
    ugc.create_item(app_id, file_type, move |result| {
        async_results.push(match result {
            Ok((item, legal)) => SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemCreated {
                request_id,
                item,
                user_needs_to_accept_workshop_legal_agreement: legal,
            }),
            Err(source) => SteamworksUgcResult::Err {
                command,
                error: SteamworksUgcError::steam_error("ugc.create_item", Some(request_id), source),
            },
        });
    });
    SteamworksUgcOperation::ItemCreateRequested {
        request_id,
        app_id,
        file_type,
    }
}

pub(super) fn submit_item_update(
    ugc: &steamworks::UGC,
    async_results: &SteamworksUgcAsyncResults,
    update_watches: &SteamworksUgcUpdateWatches,
    request_id: u64,
    app_id: steamworks::AppId,
    item: steamworks::PublishedFileId,
    update: SteamworksUgcItemUpdate,
) -> Result<SteamworksUgcOperation, SteamworksUgcError> {
    let update_handle = ugc.start_item_update(app_id, item);
    let update_handle = apply_item_update(update_handle, &update)?;
    let async_results = async_results.clone();
    let update_watches_for_callback = update_watches.clone();
    let command = SteamworksUgcCommand::SubmitItemUpdate {
        app_id,
        item,
        update: update.clone(),
    };
    let watch = update_handle.submit(update.change_note.as_deref(), move |result| {
        update_watches_for_callback.remove(request_id);
        async_results.push(match result {
            Ok((updated_item, legal)) => {
                SteamworksUgcResult::Ok(SteamworksUgcOperation::ItemUpdated {
                    request_id,
                    item: updated_item,
                    user_needs_to_accept_workshop_legal_agreement: legal,
                })
            }
            Err(source) => SteamworksUgcResult::Err {
                command,
                error: SteamworksUgcError::steam_error(
                    "ugc.submit_item_update",
                    Some(request_id),
                    source,
                ),
            },
        });
    });
    update_watches.insert(request_id, watch);
    Ok(SteamworksUgcOperation::ItemUpdateSubmitted {
        request_id,
        app_id,
        item,
        update,
    })
}

pub(super) fn read_item_update_progress(
    update_watches: &SteamworksUgcUpdateWatches,
    request_id: u64,
) -> Result<SteamworksUgcOperation, SteamworksUgcError> {
    let progress = update_watches
        .progress(request_id)
        .ok_or(SteamworksUgcError::ItemUpdateNotFound { request_id })?;
    Ok(SteamworksUgcOperation::ItemUpdateProgressRead { progress })
}

pub(super) fn forget_item_update(
    update_watches: &SteamworksUgcUpdateWatches,
    request_id: u64,
) -> Result<SteamworksUgcOperation, SteamworksUgcError> {
    if update_watches.remove(request_id) {
        Ok(SteamworksUgcOperation::ItemUpdateForgotten { request_id })
    } else {
        Err(SteamworksUgcError::ItemUpdateNotFound { request_id })
    }
}
