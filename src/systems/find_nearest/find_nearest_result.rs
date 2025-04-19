use bevy::prelude::*;
use ethnolib::{sandbox::EntityId, Number};

type SquaredDistance = Number;

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FindNearestResult {
    pub agent_id: EntityId,
    pub prayer_id: u64,
    pub found_item_id_maybe: Option<(EntityId, SquaredDistance)>,
}
