use crate::networking_sockets::*;

impl SteamworksNetworkingSocketsState {
    /// Returns the most recent connection info snapshot read through the plugin.
    pub fn last_connection_info(&self) -> Option<&SteamworksNetworkingSocketsConnectionInfo> {
        self.last_connection_info.as_ref()
    }

    /// Returns bounded connection info snapshots keyed by connection.
    pub fn connection_infos(&self) -> &[SteamworksNetworkingSocketsConnectionInfo] {
        &self.connection_infos
    }

    /// Returns the number of cached connection info snapshots.
    pub fn connection_info_count(&self) -> usize {
        self.connection_infos.len()
    }

    /// Returns the cached connection info for one connection.
    pub fn connection_info(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<&SteamworksNetworkingSocketsConnectionInfo> {
        self.connection_infos
            .iter()
            .find(|info| info.connection == connection)
    }

    /// Returns the latest known high-level state for one connection.
    pub fn connection_state(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<steamworks::networking_types::NetworkingConnectionState> {
        self.connection_info(connection)
            .map(|info| info.state)
            .or_else(|| {
                self.realtime_status(connection)
                    .map(|status| status.connection_state)
            })
    }

    /// Returns the latest known remote identity for one connection, preserving a known connection without identity as `Some(None)`.
    pub fn connection_remote(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<Option<&steamworks::networking_types::NetworkingIdentity>> {
        self.connection_info(connection)
            .map(|info| info.remote.as_ref())
    }

    /// Returns the cached end reason for one connection, preserving a known connection without one as `Some(None)`.
    pub fn connection_end_reason(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<Option<steamworks::networking_types::NetConnectionEnd>> {
        self.connection_info(connection).map(|info| info.end_reason)
    }

    /// Returns the latest known connection user data.
    pub fn connection_user_data(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<i64> {
        if let Some(user_data) = self
            .last_connection_user_data
            .as_ref()
            .filter(|user_data| user_data.connection == connection)
            .map(|user_data| user_data.user_data)
        {
            return Some(user_data);
        }

        self.connection_info(connection).map(|info| info.user_data)
    }

    /// Returns the most recent debug name submitted for one connection.
    pub fn connection_name(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<&str> {
        self.last_connection_name
            .as_ref()
            .filter(|name| name.connection == connection)
            .map(|name| name.name.as_str())
    }

    /// Returns the most recent realtime connection status snapshot.
    pub fn last_realtime_status(&self) -> Option<&SteamworksNetworkingSocketsRealtimeStatus> {
        self.last_realtime_status.as_ref()
    }

    /// Returns bounded realtime connection status snapshots keyed by connection.
    pub fn realtime_statuses(&self) -> &[SteamworksNetworkingSocketsRealtimeStatus] {
        &self.realtime_statuses
    }

    /// Returns the number of cached realtime status snapshots.
    pub fn realtime_status_count(&self) -> usize {
        self.realtime_statuses.len()
    }

    /// Returns the cached realtime status for one connection.
    pub fn realtime_status(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<&SteamworksNetworkingSocketsRealtimeStatus> {
        self.realtime_statuses
            .iter()
            .find(|status| status.connection == connection)
    }

    /// Returns the latest cached ping estimate for one connection.
    pub fn connection_ping(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<i32> {
        self.realtime_status(connection).map(|status| status.ping)
    }

    /// Returns the latest cached local and remote connection quality for one connection.
    pub fn connection_quality(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<(f32, f32)> {
        self.realtime_status(connection).map(|status| {
            (
                status.connection_quality_local,
                status.connection_quality_remote,
            )
        })
    }

    /// Returns the latest cached send capacity for one connection.
    pub fn connection_send_rate_bytes_per_sec(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<i32> {
        self.realtime_status(connection)
            .map(|status| status.send_rate_bytes_per_sec)
    }

    /// Returns the latest cached pending unreliable bytes for one connection.
    pub fn connection_pending_unreliable(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<i32> {
        self.realtime_status(connection)
            .map(|status| status.pending_unreliable)
    }

    /// Returns the latest cached pending reliable bytes for one connection.
    pub fn connection_pending_reliable(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<i32> {
        self.realtime_status(connection)
            .map(|status| status.pending_reliable)
    }

    /// Returns the number of cached realtime lane snapshots for one connection.
    pub fn connection_lane_count(
        &self,
        connection: SteamworksNetworkingSocketsConnectionId,
    ) -> Option<usize> {
        self.realtime_status(connection)
            .map(|status| status.lanes.len())
    }
}
