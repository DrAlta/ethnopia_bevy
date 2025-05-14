use bevy::prelude::*;

use ethnolib::sandbox::EntityId;

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UseOnRequest {
    pub agent_id: EntityId,
    pub prayer_id: u64,
    pub tool_id: EntityId,
    pub target_id: EntityId,
}
