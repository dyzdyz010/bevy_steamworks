use crate::game_server::*;

impl SteamworksServerState {
    /// Returns the most recent shared-query incoming packet forwarded to Steam.
    pub fn last_incoming_packet(&self) -> Option<&SteamworksServerIncomingPacket> {
        self.last_incoming_packet.as_ref()
    }

    /// Returns how many shared-query incoming packets this command layer forwarded.
    pub fn incoming_packet_count(&self) -> u64 {
        self.incoming_packet_count
    }

    /// Returns packets returned by the most recent outgoing packet drain command.
    pub fn last_outgoing_packets(&self) -> &[SteamworksServerOutgoingPacket] {
        &self.last_outgoing_packets
    }

    /// Returns the most recent shared-query outgoing packet from the latest drain command.
    pub fn last_outgoing_packet(&self) -> Option<&SteamworksServerOutgoingPacket> {
        self.last_outgoing_packets.last()
    }

    /// Returns the target address for the most recent shared-query outgoing packet.
    pub fn last_outgoing_packet_addr(&self) -> Option<std::net::SocketAddrV4> {
        self.last_outgoing_packet().map(|packet| packet.addr)
    }

    /// Returns the byte count for the most recent shared-query outgoing packet.
    pub fn last_outgoing_packet_bytes(&self) -> Option<usize> {
        self.last_outgoing_packet().map(|packet| packet.data.len())
    }

    /// Returns how many shared-query outgoing packets this command layer has drained.
    pub fn outgoing_packet_count(&self) -> u64 {
        self.outgoing_packet_count
    }

    /// Returns how many shared-query outgoing packet drain commands this layer processed.
    pub fn outgoing_packet_drain_count(&self) -> u64 {
        self.outgoing_packet_drain_count
    }
}
