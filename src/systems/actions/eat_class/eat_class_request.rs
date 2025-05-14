use bevy::prelude::*;
use ethnolib::sandbox::{EntityId, Item};

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EatClassRequest {
    pub agent_id: EntityId,
    pub prayer_id: u64,
    pub item_class: Item,
}
