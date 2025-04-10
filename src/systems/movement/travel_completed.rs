use bevy::prelude::*;

use ethnolib::sandbox::EntityId;

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TravelCompleted {
    pub entity_id: EntityId,
}
