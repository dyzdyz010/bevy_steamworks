use std::cell::RefCell;

use super::{
    SteamworksServer, SteamworksServerOutgoingPacket, STEAMWORKS_SERVER_QUERY_PACKET_BUFFER_BYTES,
};

pub(super) fn drain_outgoing_packets(
    server: &SteamworksServer,
) -> Vec<SteamworksServerOutgoingPacket> {
    let mut buffer = vec![0; STEAMWORKS_SERVER_QUERY_PACKET_BUFFER_BYTES];
    let packets = RefCell::new(Vec::new());

    server.get_next_outgoing_packet(&mut buffer, |addr, data| {
        packets.borrow_mut().push(SteamworksServerOutgoingPacket {
            addr,
            data: data.to_vec(),
        });
    });

    packets.into_inner()
}
