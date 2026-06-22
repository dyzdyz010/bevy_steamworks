use crate::{SteamworksClient, SteamworksServer};

use super::super::{
    handles::SteamworksNetworkingSocketsHandleStorage, snapshots::snapshot_poll_group_message,
    SteamworksNetworkingSocketsError, SteamworksNetworkingSocketsOperation,
    SteamworksNetworkingSocketsPollGroupId, SteamworksNetworkingSocketsPollGroupMessages,
};
use super::helpers::{networking_sockets, server_networking_sockets};

pub(super) fn create_poll_group(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let (sockets, owner) = networking_sockets(client, server)?;
    let poll_group = handles.insert_poll_group(sockets.create_poll_group(), owner);
    Ok(SteamworksNetworkingSocketsOperation::PollGroupCreated { poll_group })
}

pub(super) fn create_server_poll_group(
    server: Option<&SteamworksServer>,
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let (sockets, owner) = server_networking_sockets(server)?;
    let poll_group = handles.insert_poll_group(sockets.create_poll_group(), owner);
    Ok(SteamworksNetworkingSocketsOperation::PollGroupCreated { poll_group })
}

pub(super) fn receive_poll_group_messages(
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    poll_group: SteamworksNetworkingSocketsPollGroupId,
    batch_size: usize,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let poll_group_ref = handles
        .poll_groups
        .get_mut(&poll_group)
        .ok_or(SteamworksNetworkingSocketsError::PollGroupNotFound { id: poll_group })?;
    let messages = poll_group_ref
        .receive_messages(batch_size)
        .into_iter()
        .map(|message| snapshot_poll_group_message(poll_group, message))
        .collect();
    Ok(
        SteamworksNetworkingSocketsOperation::PollGroupMessagesReceived {
            poll_group,
            messages,
        },
    )
}

pub(super) fn receive_all_poll_group_messages(
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    batch_size_per_poll_group: usize,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let mut poll_groups = handles.poll_groups.keys().copied().collect::<Vec<_>>();
    poll_groups.sort_by_key(|poll_group| poll_group.raw());

    let mut batches = Vec::with_capacity(poll_groups.len());
    for poll_group in poll_groups {
        if !handles.poll_groups.contains_key(&poll_group) {
            continue;
        }
        let SteamworksNetworkingSocketsOperation::PollGroupMessagesReceived {
            poll_group,
            messages,
        } = receive_poll_group_messages(handles, poll_group, batch_size_per_poll_group)?
        else {
            unreachable!("receive_poll_group_messages returns PollGroupMessagesReceived");
        };
        batches.push(SteamworksNetworkingSocketsPollGroupMessages {
            poll_group,
            messages,
        });
    }

    Ok(
        SteamworksNetworkingSocketsOperation::AllPollGroupMessagesReceived {
            poll_groups: batches,
        },
    )
}

pub(super) fn close_poll_group(
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    poll_group: SteamworksNetworkingSocketsPollGroupId,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    handles
        .remove_poll_group(&poll_group)
        .ok_or(SteamworksNetworkingSocketsError::PollGroupNotFound { id: poll_group })?;
    Ok(SteamworksNetworkingSocketsOperation::PollGroupClosed { poll_group })
}
