use bevy::prelude::*;

use ethnolib::sandbox::EntityId;

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UseRequest {
    pub agent_id: EntityId,
    pub prayer_id: u64,
    pub target_id: EntityId,
}
