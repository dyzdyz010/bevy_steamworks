use super::{SteamworksTimelineCommand, SteamworksTimelineError};

pub(super) fn validate_command(
    command: &SteamworksTimelineCommand,
) -> Result<(), SteamworksTimelineError> {
    match command {
        SteamworksTimelineCommand::SetStateDescription { description, .. } => {
            validate_steam_string("description", description)?;
            Ok(())
        }
        SteamworksTimelineCommand::ClearStateDescription { .. } => Ok(()),
        SteamworksTimelineCommand::AddEvent { event } => {
            validate_steam_string("icon", &event.icon)?;
            validate_steam_string("title", &event.title)?;
            validate_steam_string("description", &event.description)?;
            validate_finite_f32("start_offset_seconds", event.start_offset_seconds)
        }
        SteamworksTimelineCommand::SetGameMode { .. } => Ok(()),
    }
}

fn validate_steam_string(field: &'static str, value: &str) -> Result<(), SteamworksTimelineError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksTimelineError::invalid_string(field))
    } else {
        Ok(())
    }
}

fn validate_finite_f32(field: &'static str, value: f32) -> Result<(), SteamworksTimelineError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(SteamworksTimelineError::invalid_float(field))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::super::{SteamworksTimelineEventClipPriority, SteamworksTimelineEventInfo};
    use super::*;

    #[test]
    fn validation_rejects_interior_nul() {
        let command =
            SteamworksTimelineCommand::set_state_description("boss\0phase", Duration::from_secs(1));

        assert_eq!(
            validate_command(&command),
            Err(SteamworksTimelineError::InvalidString {
                field: "description",
            })
        );

        let command = SteamworksTimelineCommand::add_event(SteamworksTimelineEventInfo {
            icon: "skull".to_owned(),
            title: "wipe\0bad".to_owned(),
            description: "party defeated".to_owned(),
            priority: 10,
            start_offset_seconds: 0.0,
            duration: Duration::ZERO,
            clip_priority: SteamworksTimelineEventClipPriority::Featured,
        });

        assert_eq!(
            validate_command(&command),
            Err(SteamworksTimelineError::InvalidString { field: "title" })
        );
    }

    #[test]
    fn validation_rejects_non_finite_values() {
        let command = SteamworksTimelineCommand::add_event(SteamworksTimelineEventInfo {
            icon: "star".to_owned(),
            title: "win".to_owned(),
            description: "match won".to_owned(),
            priority: 1,
            start_offset_seconds: f32::NAN,
            duration: Duration::from_secs(1),
            clip_priority: SteamworksTimelineEventClipPriority::Standard,
        });

        assert_eq!(
            validate_command(&command),
            Err(SteamworksTimelineError::InvalidFloat {
                field: "start_offset_seconds",
            })
        );
    }
}
