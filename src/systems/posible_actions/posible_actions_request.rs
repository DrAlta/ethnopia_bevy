use bevy::prelude::*;

use ethnolib::sandbox::EntityId;

#[derive(Event, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PosibleActionsRequest {
    pub agent_id: EntityId,
    pub target_id: EntityId,
}
