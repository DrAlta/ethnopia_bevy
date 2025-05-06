use bevy::prelude::*;
use ethnolib::sandbox::EntityId;

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetLocationRequest {
    pub agent_id: EntityId,
    pub prayer_id: u64,
    pub subject_id: EntityId,
}
