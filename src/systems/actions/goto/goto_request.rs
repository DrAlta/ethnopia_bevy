use bevy::prelude::*;

use ethnolib::sandbox::{EntityId, world::Movement};

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GotoRequest {
    pub agent_id: EntityId,
    pub prayer_id: u64,
    pub movement: Movement,
}
