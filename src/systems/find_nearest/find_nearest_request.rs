use bevy::prelude::*;
use ethnolib::sandbox::EntityId;

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FindNearestRequest {
    pub agent_id: EntityId,
    pub prayer_id: u64,
    // search out atleast min_radius from the point at x:y it may or may noe return an item beyond that distance
    pub min_radius: i32,
    pub x: i32,
    pub y: i32,
}
