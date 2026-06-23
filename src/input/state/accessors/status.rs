use crate::input::*;

impl SteamworksInputState {
    /// Returns the most recent synchronous error observed by the input plugin.
    pub fn last_error(&self) -> Option<&SteamworksInputError> {
        self.last_error.as_ref()
    }

    /// Returns whether the last initialization command succeeded and has not
    /// been followed by a shutdown command.
    pub fn initialized(&self) -> bool {
        self.initialized
    }

    /// Returns how many successful [`SteamworksInputCommand::RunFrame`] commands this plugin observed.
    pub fn frame_run_count(&self) -> u64 {
        self.frame_run_count
    }
}
