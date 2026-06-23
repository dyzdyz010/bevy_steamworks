use super::super::{
    SteamworksLeaderboardEntriesDownloadRequest, SteamworksLeaderboardEntriesDownloadResult,
    SteamworksLeaderboardEntry, SteamworksLeaderboardFindOrCreateRequest,
    SteamworksLeaderboardFindOrCreateResult, SteamworksLeaderboardFindRequest,
    SteamworksLeaderboardFindResult, SteamworksLeaderboardId, SteamworksLeaderboardInfo,
    SteamworksLeaderboardScoreUploadRequest, SteamworksLeaderboardScoreUploadResult,
    SteamworksStatsState,
};
use crate::user_stats::{
    SteamworksLeaderboardDisplayType, SteamworksLeaderboardScoreUploaded,
    SteamworksLeaderboardSortMethod,
};

impl SteamworksStatsState {
    /// Returns the number of leaderboard handles currently owned by this plugin.
    pub fn leaderboard_count(&self) -> usize {
        self.leaderboard_count
    }

    /// Returns the plugin-owned leaderboard ID most recently associated with a name.
    pub fn leaderboard_id(&self, name: &str) -> Option<SteamworksLeaderboardId> {
        self.leaderboard_ids
            .iter()
            .find_map(|(known_name, leaderboard)| (known_name == name).then_some(*leaderboard))
    }

    /// Returns whether this plugin has associated a name with a plugin-owned leaderboard ID.
    pub fn has_leaderboard_id(&self, name: &str) -> bool {
        self.leaderboard_id(name).is_some()
    }

    /// Returns leaderboard metadata snapshots read through this plugin.
    pub fn leaderboards(&self) -> &[SteamworksLeaderboardInfo] {
        &self.leaderboard_infos
    }

    /// Returns cached leaderboard metadata for a plugin-owned leaderboard ID.
    pub fn leaderboard_info(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<&SteamworksLeaderboardInfo> {
        self.leaderboard_infos
            .iter()
            .find(|info| info.leaderboard == leaderboard)
    }

    /// Returns whether this plugin has cached metadata for a plugin-owned leaderboard ID.
    pub fn has_leaderboard_info(&self, leaderboard: SteamworksLeaderboardId) -> bool {
        self.leaderboard_info(leaderboard).is_some()
    }

    /// Returns cached leaderboard metadata for a Steamworks leaderboard name.
    pub fn leaderboard_info_by_name(&self, name: &str) -> Option<&SteamworksLeaderboardInfo> {
        self.leaderboard_infos.iter().find(|info| info.name == name)
    }

    /// Returns the cached Steamworks leaderboard name for a plugin-owned leaderboard ID.
    pub fn leaderboard_name(&self, leaderboard: SteamworksLeaderboardId) -> Option<&str> {
        self.leaderboard_info(leaderboard)
            .map(|info| info.name.as_str())
    }

    /// Returns the cached display type for a plugin-owned leaderboard ID.
    pub fn leaderboard_display_type(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<Option<SteamworksLeaderboardDisplayType>> {
        self.leaderboard_info(leaderboard)
            .map(|info| info.display_type)
    }

    /// Returns the cached sort method for a plugin-owned leaderboard ID.
    pub fn leaderboard_sort_method(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<Option<SteamworksLeaderboardSortMethod>> {
        self.leaderboard_info(leaderboard)
            .map(|info| info.sort_method)
    }

    /// Returns the cached total entry count reported by Steam for a plugin-owned leaderboard ID.
    pub fn leaderboard_entry_count(&self, leaderboard: SteamworksLeaderboardId) -> Option<i32> {
        self.leaderboard_info(leaderboard)
            .map(|info| info.entry_count)
    }

    /// Returns the most recent submitted leaderboard find request.
    pub fn last_leaderboard_find_request(&self) -> Option<&SteamworksLeaderboardFindRequest> {
        self.last_leaderboard_find_request.as_ref()
    }

    /// Returns the most recent completed leaderboard find result.
    pub fn last_leaderboard_find_result(&self) -> Option<&SteamworksLeaderboardFindResult> {
        self.last_leaderboard_find_result.as_ref()
    }

    /// Returns the most recent submitted leaderboard find-or-create request.
    pub fn last_leaderboard_find_or_create_request(
        &self,
    ) -> Option<&SteamworksLeaderboardFindOrCreateRequest> {
        self.last_leaderboard_find_or_create_request.as_ref()
    }

    /// Returns the most recent completed leaderboard find-or-create result.
    pub fn last_leaderboard_find_or_create_result(
        &self,
    ) -> Option<&SteamworksLeaderboardFindOrCreateResult> {
        self.last_leaderboard_find_or_create_result.as_ref()
    }

    /// Returns the most recent leaderboard info read through this plugin.
    pub fn last_leaderboard_info(&self) -> Option<&SteamworksLeaderboardInfo> {
        self.last_leaderboard_info.as_ref()
    }

    /// Returns the most recent submitted leaderboard score upload request.
    pub fn last_leaderboard_score_upload_request(
        &self,
    ) -> Option<&SteamworksLeaderboardScoreUploadRequest> {
        self.last_leaderboard_score_upload_request.as_ref()
    }

    /// Returns bounded submitted leaderboard score upload requests keyed by leaderboard.
    pub fn leaderboard_score_upload_requests(&self) -> &[SteamworksLeaderboardScoreUploadRequest] {
        &self.leaderboard_score_upload_requests
    }

    /// Returns the cached score upload request for one leaderboard.
    pub fn leaderboard_score_upload_request(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<&SteamworksLeaderboardScoreUploadRequest> {
        self.leaderboard_score_upload_requests
            .iter()
            .find(|request| request.leaderboard == leaderboard)
    }

    /// Returns the most recent completed leaderboard score upload result.
    pub fn last_leaderboard_score_upload_result(
        &self,
    ) -> Option<&SteamworksLeaderboardScoreUploadResult> {
        self.last_leaderboard_score_upload_result.as_ref()
    }

    /// Returns bounded completed leaderboard score upload results keyed by leaderboard.
    pub fn leaderboard_score_upload_results(&self) -> &[SteamworksLeaderboardScoreUploadResult] {
        &self.leaderboard_score_upload_results
    }

    /// Returns the cached score upload result for one leaderboard.
    pub fn leaderboard_score_upload_result(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<&SteamworksLeaderboardScoreUploadResult> {
        self.leaderboard_score_upload_results
            .iter()
            .find(|result| result.leaderboard == leaderboard)
    }

    /// Returns the cached score upload payload for one leaderboard, preserving a completed upload with no payload as `Some(None)`.
    pub fn leaderboard_score_upload(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<Option<&SteamworksLeaderboardScoreUploaded>> {
        self.leaderboard_score_upload_result(leaderboard)
            .map(|result| result.upload.as_ref())
    }

    /// Returns the score submitted or retained by Steam for the latest cached upload result.
    pub fn leaderboard_uploaded_score(&self, leaderboard: SteamworksLeaderboardId) -> Option<i32> {
        self.leaderboard_score_upload(leaderboard)
            .flatten()
            .map(|upload| upload.score)
    }

    /// Returns whether Steam changed the score for the latest cached upload result.
    pub fn leaderboard_score_was_changed(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<bool> {
        self.leaderboard_score_upload(leaderboard)
            .flatten()
            .map(|upload| upload.was_changed)
    }

    /// Returns the new global rank from the latest cached upload result.
    pub fn leaderboard_uploaded_rank_new(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<i32> {
        self.leaderboard_score_upload(leaderboard)
            .flatten()
            .map(|upload| upload.global_rank_new)
    }

    /// Returns the previous global rank from the latest cached upload result.
    pub fn leaderboard_uploaded_rank_previous(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<i32> {
        self.leaderboard_score_upload(leaderboard)
            .flatten()
            .map(|upload| upload.global_rank_previous)
    }

    /// Returns the most recent submitted leaderboard entries download request.
    pub fn last_leaderboard_entries_download_request(
        &self,
    ) -> Option<&SteamworksLeaderboardEntriesDownloadRequest> {
        self.last_leaderboard_entries_download_request.as_ref()
    }

    /// Returns bounded submitted leaderboard entries download requests keyed by leaderboard.
    pub fn leaderboard_entries_download_requests(
        &self,
    ) -> &[SteamworksLeaderboardEntriesDownloadRequest] {
        &self.leaderboard_entries_download_requests
    }

    /// Returns the cached entries download request for one leaderboard.
    pub fn leaderboard_entries_download_request(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<&SteamworksLeaderboardEntriesDownloadRequest> {
        self.leaderboard_entries_download_requests
            .iter()
            .find(|request| request.leaderboard == leaderboard)
    }

    /// Returns the most recent completed leaderboard entries download result.
    pub fn last_leaderboard_entries_download_result(
        &self,
    ) -> Option<&SteamworksLeaderboardEntriesDownloadResult> {
        self.last_leaderboard_entries_download_result.as_ref()
    }

    /// Returns bounded completed leaderboard entries download results keyed by leaderboard.
    pub fn leaderboard_entries_download_results(
        &self,
    ) -> &[SteamworksLeaderboardEntriesDownloadResult] {
        &self.leaderboard_entries_download_results
    }

    /// Returns the cached entries download result for one leaderboard.
    pub fn leaderboard_entries_download_result(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<&SteamworksLeaderboardEntriesDownloadResult> {
        self.leaderboard_entries_download_results
            .iter()
            .find(|result| result.leaderboard == leaderboard)
    }

    /// Returns the most recent downloaded leaderboard entries.
    pub fn last_leaderboard_entries(&self) -> &[SteamworksLeaderboardEntry] {
        &self.last_leaderboard_entries
    }

    /// Returns the cached downloaded entries for one leaderboard.
    pub fn leaderboard_entries(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<&[SteamworksLeaderboardEntry]> {
        self.leaderboard_entries_download_result(leaderboard)
            .map(|result| result.entries.as_slice())
    }

    /// Returns the number of cached downloaded entries for one leaderboard.
    pub fn leaderboard_downloaded_entry_count(
        &self,
        leaderboard: SteamworksLeaderboardId,
    ) -> Option<usize> {
        self.leaderboard_entries(leaderboard)
            .map(|entries| entries.len())
    }

    /// Returns the number of entries in the most recent leaderboard entry download.
    pub fn last_leaderboard_downloaded_entry_count(&self) -> usize {
        self.last_leaderboard_entries.len()
    }

    /// Returns a cached downloaded leaderboard entry for one Steam user.
    pub fn leaderboard_entry_by_user(
        &self,
        leaderboard: SteamworksLeaderboardId,
        user: steamworks::SteamId,
    ) -> Option<&SteamworksLeaderboardEntry> {
        self.leaderboard_entries(leaderboard)
            .and_then(|entries| entries.iter().find(|entry| entry.user == user))
    }

    /// Returns whether a cached downloaded entry exists for one Steam user.
    pub fn leaderboard_has_entry_for_user(
        &self,
        leaderboard: SteamworksLeaderboardId,
        user: steamworks::SteamId,
    ) -> Option<bool> {
        self.leaderboard_entries(leaderboard)
            .map(|entries| entries.iter().any(|entry| entry.user == user))
    }

    /// Returns a cached downloaded leaderboard entry for one global rank.
    pub fn leaderboard_entry_by_rank(
        &self,
        leaderboard: SteamworksLeaderboardId,
        global_rank: i32,
    ) -> Option<&SteamworksLeaderboardEntry> {
        self.leaderboard_entries(leaderboard).and_then(|entries| {
            entries
                .iter()
                .find(|entry| entry.global_rank == global_rank)
        })
    }

    /// Returns whether a cached downloaded entry exists for one global rank.
    pub fn leaderboard_has_rank(
        &self,
        leaderboard: SteamworksLeaderboardId,
        global_rank: i32,
    ) -> Option<bool> {
        self.leaderboard_entries(leaderboard)
            .map(|entries| entries.iter().any(|entry| entry.global_rank == global_rank))
    }

    /// Returns the cached score for one Steam user on a leaderboard.
    pub fn leaderboard_score_by_user(
        &self,
        leaderboard: SteamworksLeaderboardId,
        user: steamworks::SteamId,
    ) -> Option<i32> {
        self.leaderboard_entry_by_user(leaderboard, user)
            .map(|entry| entry.score)
    }

    /// Returns the cached global rank for one Steam user on a leaderboard.
    pub fn leaderboard_rank_by_user(
        &self,
        leaderboard: SteamworksLeaderboardId,
        user: steamworks::SteamId,
    ) -> Option<i32> {
        self.leaderboard_entry_by_user(leaderboard, user)
            .map(|entry| entry.global_rank)
    }

    /// Returns the cached detail integers for one Steam user on a leaderboard.
    pub fn leaderboard_entry_details(
        &self,
        leaderboard: SteamworksLeaderboardId,
        user: steamworks::SteamId,
    ) -> Option<&[i32]> {
        self.leaderboard_entry_by_user(leaderboard, user)
            .map(|entry| entry.details.as_slice())
    }

    /// Returns the most recent leaderboard ID forgotten by this plugin.
    pub fn last_forgotten_leaderboard(&self) -> Option<SteamworksLeaderboardId> {
        self.last_forgotten_leaderboard
    }
}
