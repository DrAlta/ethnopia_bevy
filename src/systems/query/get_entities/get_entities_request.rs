use bevy::prelude::*;
use ethnolib::sandbox::EntityId;

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetEntitiesRequest {
    pub agent_id: EntityId,
    pub prayer_id: u64,
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}
