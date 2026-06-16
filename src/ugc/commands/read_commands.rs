use super::super::{
    SteamworksUgcError, SteamworksUgcItemDownloadInfo, SteamworksUgcItemDownloadInfoResult,
    SteamworksUgcItemInstallInfo, SteamworksUgcItemInstallInfoResult, SteamworksUgcItemStateInfo,
    SteamworksUgcOperation,
};

pub(super) fn suspend_downloads(suspend: bool) -> SteamworksUgcOperation {
    SteamworksUgcOperation::DownloadsSuspended { suspend }
}

pub(super) fn list_subscribed_items(
    ugc: &steamworks::UGC,
    include_locally_disabled: bool,
) -> SteamworksUgcOperation {
    let items = ugc.subscribed_items(include_locally_disabled);
    SteamworksUgcOperation::SubscribedItemsListed {
        include_locally_disabled,
        items,
    }
}

pub(super) fn read_item_state(
    ugc: &steamworks::UGC,
    item: steamworks::PublishedFileId,
) -> SteamworksUgcOperation {
    SteamworksUgcOperation::ItemStateRead {
        info: SteamworksUgcItemStateInfo {
            item,
            state: ugc.item_state(item),
        },
    }
}

pub(super) fn read_item_download_info(
    ugc: &steamworks::UGC,
    item: steamworks::PublishedFileId,
) -> SteamworksUgcOperation {
    SteamworksUgcOperation::ItemDownloadInfoRead {
        info: SteamworksUgcItemDownloadInfoResult {
            item,
            info: ugc
                .item_download_info(item)
                .map(
                    |(downloaded_bytes, total_bytes)| SteamworksUgcItemDownloadInfo {
                        downloaded_bytes,
                        total_bytes,
                    },
                ),
        },
    }
}

pub(super) fn read_item_install_info(
    ugc: &steamworks::UGC,
    item: steamworks::PublishedFileId,
) -> SteamworksUgcOperation {
    SteamworksUgcOperation::ItemInstallInfoRead {
        info: SteamworksUgcItemInstallInfoResult {
            item,
            info: ugc
                .item_install_info(item)
                .map(|info| SteamworksUgcItemInstallInfo {
                    folder: info.folder,
                    size_on_disk: info.size_on_disk,
                    timestamp: info.timestamp,
                }),
        },
    }
}

pub(super) fn download_item(
    ugc: &steamworks::UGC,
    item: steamworks::PublishedFileId,
    high_priority: bool,
) -> Result<SteamworksUgcOperation, SteamworksUgcError> {
    if !ugc.download_item(item, high_priority) {
        return Err(SteamworksUgcError::operation_failed("ugc.download_item"));
    }
    Ok(SteamworksUgcOperation::DownloadItemSubmitted {
        item,
        high_priority,
    })
}
