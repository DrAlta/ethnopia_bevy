use bevy::prelude::*;

use ethnolib::sandbox::EntityId;

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UseRequest {
    pub agent_id: EntityId,
    pub target_id: EntityId,
}
