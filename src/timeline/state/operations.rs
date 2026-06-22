use super::SteamworksTimelineState;
use crate::timeline::{
    SteamworksTimelineError, SteamworksTimelineOperation, SteamworksTimelineStateDescription,
};

impl SteamworksTimelineState {
    #[cfg(test)]
    pub(in crate::timeline) fn set_event_count_for_test(&mut self, event_count: u64) {
        self.event_count = event_count;
    }

    pub(in crate::timeline) fn record_error(&mut self, error: SteamworksTimelineError) {
        self.last_error = Some(error);
    }

    pub(in crate::timeline) fn record_operation(
        &mut self,
        operation: &SteamworksTimelineOperation,
    ) {
        match operation {
            SteamworksTimelineOperation::GameModeSet { mode } => {
                self.game_mode = Some(*mode);
            }
            SteamworksTimelineOperation::StateDescriptionSet {
                description,
                duration,
            } => {
                self.state_description = Some(SteamworksTimelineStateDescription {
                    description: description.clone(),
                    duration: *duration,
                });
            }
            SteamworksTimelineOperation::StateDescriptionCleared { .. } => {
                self.state_description = None;
            }
            SteamworksTimelineOperation::TimelineEventAdded { event } => {
                self.last_event = Some(event.clone());
                self.event_count = self.event_count.saturating_add(1);
            }
        }
    }
}
