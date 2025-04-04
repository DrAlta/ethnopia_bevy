use bevy::prelude::*;
use ethnolib::sandbox::EntityId;

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DropRequest {
    pub agent_id: EntityId,
    pub prayer_id: u64,
    pub object_id: EntityId,
}
